#!/usr/bin/env bash
# Configure le dépôt GitHub : branches protégées, étiquettes, branche par défaut.
# Idempotent — relançable sans effet de bord.
#
#   ./scripts/setup-github.sh            applique
#   DRY_RUN=1 ./scripts/setup-github.sh  affiche seulement
#
# Prérequis : gh authentifié avec le scope `repo`.

set -euo pipefail

REPO="${REPO:-bourbask/contrepoint}"
# Vérifications de statut exigées avant merge. Vide au départ : la protection
# refuserait tout merge tant que la CI n'a pas publié de contexte de ce nom.
# À remplir avec les noms de jobs réels dès que .github/workflows/ci.yml existe.
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
echo "Fait. Rappel : ajouter les contextes de CI dès que ci.yml existe, par ex."
echo "  CHECKS='ci-ok' ./scripts/setup-github.sh"
