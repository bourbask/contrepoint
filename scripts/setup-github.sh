#!/usr/bin/env bash
# Configure le dépôt GitHub : crochet local, branche par défaut, branches
# protégées, source de publication des Pages, dependabot, étiquettes.
# Ce qui n'est pas scriptable est dans docs/mise-en-route.md.
# Idempotent sur les étiquettes et les réglages. Les protections de branches
# ne sont PAS écrasées si elles diffèrent du gabarit : PUT remplace au lieu de
# fusionner, et rabaisserait ce qui a été durci à la main. FORCE=1 pour forcer.
#
#   ./scripts/setup-github.sh            applique
#   DRY_RUN=1 ./scripts/setup-github.sh  affiche seulement
#
# Prérequis : gh authentifié avec le scope `repo`.

set -euo pipefail

REPO="${REPO:-bourbask/contrepoint}"
# Vérifications de statut exigées avant merge. `ci-ok` et lui seul : il agrège
# les autres travaux de ci.yml par `needs`. Y ajouter un nom de travail ferait
# refuser tout merge tant que ce travail n'a pas publié de contexte — un travail
# gardé par hashFiles n'en publie pas quand il saute.
CHECKS="${CHECKS:-ci-ok}"

run() {
  if [[ -n "${DRY_RUN:-}" ]]; then
    printf 'DRY_RUN: %s\n' "$*"
  else
    "$@"
  fi
}

contexts_json() {
  if [[ -z "$CHECKS" ]]; then
    printf '[]'
  else
    printf '%s' "$CHECKS" | tr ',' '\n' | sed 's/^ *//;s/ *$//' \
      | python3 -c 'import json,sys; print(json.dumps([l for l in sys.stdin.read().split() if l]))'
  fi
}

protect() {
  local branch="$1"
  echo "→ protection de $branch"
  # required_approving_review_count = 0 : dépôt solo, l'auteur ne peut pas
  # approuver sa propre PR. La PR reste obligatoire, l'approbation non.
  # enforce_admins = true : la règle s'applique aussi au propriétaire, sinon
  # elle ne protège de rien.
  local payload
  payload=$(cat <<JSON
{
  "required_status_checks": { "strict": true, "contexts": $(contexts_json) },
  "enforce_admins": true,
  "required_pull_request_reviews": {
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": false,
    "required_approving_review_count": 0,
    "require_last_push_approval": false
  },
  "restrictions": null,
  "required_linear_history": false,
  "allow_force_pushes": false,
  "allow_deletions": false,
  "block_creations": false,
  "required_conversation_resolution": true,
  "lock_branch": false,
  "allow_fork_syncing": false
}
JSON
)
  if [[ -n "${DRY_RUN:-}" ]]; then
    printf 'DRY_RUN: PUT repos/%s/branches/%s/protection\n%s\n' "$REPO" "$branch" "$payload"
  else
    # PUT remplace, il ne fusionne pas. Toute valeur durcie à la main serait
    # silencieusement rabaissée par un script annoncé « relançable sans effet
    # de bord » : nombre d'approbations exigées, historique linéaire, verrou de
    # branche, restrictions de poussée. On refuse d'écraser sans consentement.
    # PUT remplace, il ne fusionne pas. Comparer les objets entiers ne sert à
    # rien : la réponse de l'API porte des champs que le gabarit n'a pas, donc
    # elle diffère toujours, et un garde-fou toujours bloquant finit supprimé.
    # On ne refuse que si l'existant est PLUS STRICT que le gabarit.
    local actuel plus_strict
    if actuel=$(gh api "repos/$REPO/branches/$branch/protection" 2>/dev/null); then
      plus_strict=$(printf '%s' "$actuel" | python3 - "$payload" <<'PY'
import json, sys
vivant = json.load(sys.stdin)
gabarit = json.loads(sys.argv[1])
perdus = []

def b(o, *chemin):
    for c in chemin:
        if not isinstance(o, dict): return None
        o = o.get(c)
    return o.get('enabled') if isinstance(o, dict) and 'enabled' in o else o

# Un booléen de durcissement vrai en vivant et faux au gabarit serait perdu.
for champ in ('required_linear_history', 'block_creations', 'lock_branch',
              'enforce_admins', 'required_conversation_resolution'):
    if b(vivant, champ) and not b(gabarit, champ):
        perdus.append(champ)
# Ceux-là sont inversés : autoriser est le relâchement.
for champ in ('allow_force_pushes', 'allow_deletions'):
    if b(vivant, champ) is False and b(gabarit, champ) is not False:
        perdus.append(f"{champ} (actuellement interdit)")

vr = (vivant.get('required_pull_request_reviews') or {})
gr = (gabarit.get('required_pull_request_reviews') or {})
if vr.get('required_approving_review_count', 0) > gr.get('required_approving_review_count', 0):
    perdus.append(f"required_approving_review_count {vr['required_approving_review_count']} → {gr.get('required_approving_review_count', 0)}")
if vr.get('require_last_push_approval') and not gr.get('require_last_push_approval'):
    perdus.append('require_last_push_approval')
if vivant.get('restrictions') and gabarit.get('restrictions') is None:
    perdus.append('restrictions de poussée (seraient supprimées)')

vc = set((vivant.get('required_status_checks') or {}).get('contexts') or [])
gc = set((gabarit.get('required_status_checks') or {}).get('contexts') or [])
if vc - gc:
    perdus.append(f"vérifications exigées perdues : {', '.join(sorted(vc - gc))}")

print('\n'.join(perdus))
PY
)
      if [ -n "$plus_strict" ] && [ -z "${FORCE:-}" ]; then
        echo "→ $branch : la protection en place est PLUS STRICTE que le gabarit." >&2
        printf '%s\n' "$plus_strict" | sed 's/^/     perdu : /' >&2
        echo "   PUT remplace au lieu de fusionner. FORCE=1 pour écraser délibérément." >&2
        return 0
      fi
    fi
    printf '%s' "$payload" | gh api -X PUT "repos/$REPO/branches/$branch/protection" --input - >/dev/null
  fi
}

label() {
  local name="$1" color="$2" desc="$3"
  if gh label list --repo "$REPO" --limit 200 | cut -f1 | grep -qxF "$name"; then
    run gh label edit "$name" --repo "$REPO" --color "$color" --description "$desc"
  else
    run gh label create "$name" --repo "$REPO" --color "$color" --description "$desc"
  fi
}

echo "== crochets git locaux =="
# Le crochet rend les contrôles bloquants. core.hooksPath est une configuration
# locale : elle n'est pas clonée, il faut la poser sur chaque poste.
run git -C "$(git rev-parse --show-toplevel)" config core.hooksPath .githooks

echo "== dépôt $REPO =="
run gh repo edit "$REPO" --default-branch develop

echo "== branches protégées =="
protect main
protect develop

echo "== publication : source des Pages sur Actions =="
# Sans ce basculement, GitHub sert la branche `gh-pages` (ou rien) et
# deploy.yml échoue à `actions/deploy-pages` : le site n'est jamais publié.
# `build_type: workflow` est ce que l'interface appelle « Source : GitHub
# Actions ». Créer si absent, mettre à jour sinon — 404 signifie « pas de site ».
if gh api "repos/$REPO/pages" >/dev/null 2>&1; then
  run gh api -X PUT "repos/$REPO/pages" -f build_type=workflow
else
  run gh api -X POST "repos/$REPO/pages" -f build_type=workflow
fi
# L'environnement `github-pages` référencé par deploy.yml est créé par GitHub
# lors de la configuration ci-dessus, avec sa politique de branche. Le script
# ne le fabrique pas : un environnement créé à la main naît sans politique.
echo "→ environnements déclarés : $(gh api "repos/$REPO/environments" --jq '[.environments[].name] | join(", ")' 2>/dev/null || echo '(illisible)')"

echo "== dependabot =="
# dependabot.yml pilote les montées de version ; il ne suffit pas. Les alertes
# de vulnérabilité et les correctifs de sécurité sont des réglages du dépôt,
# désactivés par défaut, et invisibles tant que personne ne les lit.
run gh api -X PUT "repos/$REPO/vulnerability-alerts"
run gh api -X PUT "repos/$REPO/automated-security-fixes"

echo "== étiquettes =="
label correction       "0e8a16" "Correction d'une donnée ou d'une preuve"
label bug              "d73a4a" "L'outil ne fonctionne pas comme décrit"
label brique-0         "1d76db" "Acteurs — partis et députés"
label brique-1         "1d76db" "Presse écrite"
label brique-2         "1d76db" "YouTube"
label brique-3         "1d76db" "TV / radio"
label methode          "5319e7" "Touche à la méthode de mesure"
label juridique        "b60205" "Exposition juridique ou lexique"
label release          "fbca04" "Montée de version (develop → main)"

echo
echo "Fait. Ce que ce script ne peut pas faire est dans docs/mise-en-route.md."
echo "Rappel : ci-ok est la seule vérification exigée — tout nouveau travail de"
echo "ci.yml s'y ajoute en \`needs\`, jamais dans CHECKS."
