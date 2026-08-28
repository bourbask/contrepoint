#!/usr/bin/env bash
# Contrôle de sécurité déterministe, à passer avant chaque commit.
#
#   ./scripts/securite.sh            vérifie l'index (ce qui va être committé)
#   ./scripts/securite.sh --arbre    vérifie tous les fichiers suivis
#
# Le dépôt est public et porte le nom de son auteur. Le risque n'est pas
# l'intrusion — il n'y a ni serveur ni compte — c'est ce qu'un tiers hostile
# pourrait reconstituer sur l'auteur. Ce script attrape ce qui se voit par
# motif ; l'agent sécurité traite le reste.
#
# Aucune donnée personnelle n'est inscrite dans ce fichier : les motifs sont
# génériques, précisément pour que le contrôle ne soit pas lui-même une fuite.
set -uo pipefail

echec=0
signaler() { echec=1; printf '  ✗ %s\n' "$1"; }

# Les listes sont séparées par NUL et portées par des tableaux. Une liste
# séparée par des sauts de ligne casse sur un nom contenant un espace, un
# guillemet ou un saut de ligne — et l'erreur, avalée, transformait un contrôle
# en succès. Défaut trouvé le 2026-08-27 dans une porte de CI, puis ici même :
# ce script portait le défaut qu'il est chargé de détecter.
declare -a FICHIERS=()
if [ "${1:-}" = "--arbre" ]; then
  mapfile -d '' -t FICHIERS < <(git ls-files -z)
  mode=arbre
else
  mapfile -d '' -t FICHIERS < <(git diff --cached --name-only --diff-filter=ACMR -z)
  mode=index
fi

if [ "${#FICHIERS[@]}" -eq 0 ]; then
  echo "sécurité ($mode) : rien à vérifier"
  exit 0
fi

# Ne retient que les fichiers réguliers : un lien symbolique ferait lire hors
# du dépôt, un fichier supprimé ferait planter grep.
declare -a LISIBLES=()
for f in "${FICHIERS[@]}"; do
  if [ -L "$f" ]; then
    signaler "lien symbolique suivi : $f"
  elif [ -f "$f" ]; then
    LISIBLES+=("$f")
  fi
done

# grep rend 0 s'il a trouvé, 1 s'il n'a rien trouvé, >1 s'il a échoué. Les trois
# ne sont pas la même chose : un contrôle qui a échoué n'affirme rien.
contenu() {
  [ "${#LISIBLES[@]}" -eq 0 ] && return 0
  local sortie code
  sortie=$(grep -nHIE -- "$1" "${LISIBLES[@]}"); code=$?
  if [ "$code" -gt 1 ]; then
    signaler "grep en erreur (code $code) sur le motif « $1 » — le contrôle n'a rien pu affirmer"
    return 0
  fi
  printf '%s' "$sortie"
}

echo "sécurité ($mode) : ${#FICHIERS[@]} fichiers"

# ---- 1. Secrets -------------------------------------------------------------
r=$(contenu '(BEGIN (RSA|OPENSSH|EC|PGP) PRIVATE KEY)|gh[pousr]_[A-Za-z0-9]{20,}|glpat-[A-Za-z0-9_-]{15,}|AKIA[0-9A-Z]{16}|sk-[A-Za-z0-9]{20,}|xox[baprs]-[A-Za-z0-9-]{10,}')
[ -n "$r" ] && { signaler "secret probable :"; echo "$r" | sed 's/^/      /'; }

r=$(contenu '(api[_-]?key|secret|password|passwd|token)[[:space:]]*[:=][[:space:]]*['"'"'"][^'"'"'"]{8,}')
[ -n "$r" ] && { signaler "valeur sensible affectée en clair :"; echo "$r" | sed 's/^/      /'; }

# ---- 1bis. Outillage shell et intégration continue -------------------------
# Ajouté après la revue du 2026-08-27 : les scripts manipulent des valeurs
# venues du réseau, et les workflows manipulent le jeton.

# Une liste de fichiers passée à une commande sans terminateur d'options ni
# séparateur NUL : un nom venu d'une archive distante devient un drapeau.
r=$(contenu 'find [^|]*\| *(xargs|while)' | grep -vE 'print0|-z |xargs -0[^|]* --')
[ -n "$r" ] && { signaler "liste de fichiers sans -print0 ni -- : un nom hostile devient une option :"; echo "$r" | sed 's/^/      /'; }

# Un jeton d'intégration continue déclaré plus largement que l'étape qui en a
# besoin. Une ligne seule ne dit pas si elle est dans un bloc `env:` : la
# première version de ce contrôle était un grep, et elle signalait des jetons
# correctement portés. Il faut lire le YAML.
r=$(for f in "${LISIBLES[@]}"; do
  case "$f" in .github/workflows/*.yml|.github/workflows/*.yaml) ;; *) continue ;; esac
  python3 - "$f" <<'PYEOF'
import sys, yaml
f = sys.argv[1]
try:
    d = yaml.safe_load(open(f)) or {}
except Exception:
    sys.exit(0)
JETONS = ('GH_TOKEN', 'GITHUB_TOKEN')
def porte(env, ou):
    for k in (env or {}):
        if k in JETONS:
            print(f"{f}: {k} déclaré au niveau {ou}")
porte(d.get('env'), 'du workflow')
for nom, job in (d.get('jobs') or {}).items():
    if not isinstance(job, dict): continue
    porte(job.get('env'), f"du travail « {nom} »")
PYEOF
done)
[ -n "$r" ] && { signaler "jeton déclaré plus largement que l'étape qui en a besoin :"; echo "$r" | sed 's/^/      /'; }

# Toute liste poussée dans xargs sans -0 : un nom avec espace, guillemet ou
# saut de ligne casse la liste. Combiné à un `|| true` en aval, la casse devient
# un succès. Défaut trouvé deux fois le 2026-08-27, dont une fois dans ce script.
r=$(contenu '\|[[:space:]]*xargs' | grep -vE 'xargs +(-[a-zA-Z]+ +)*-[a-zA-Z]*0')
[ -n "$r" ] && { signaler "xargs sans -0 — un nom de fichier hostile désarme le contrôle :"; echo "$r" | sed 's/^/      /'; }

# Un contrôle dont l'échec est avalé devient vert quand il plante. « Rien
# trouvé » et « rien lu » ne sont pas la même chose.
r=$(contenu '(python3|jq|grep -[a-zA-Z]*r)[^#]*\|\|[[:space:]]*true')
[ -n "$r" ] && { signaler "échec avalé par || true sur une commande de contrôle — distinguer « rien trouvé » de « rien lu » :"; echo "$r" | sed 's/^/      /'; }

# ---- 2. Fichiers qui ne doivent jamais être suivis --------------------------
r=$(printf '%s\n' "${FICHIERS[@]}" | grep -E '(^|/)\.env($|\.)|(^|/)\.netrc$|(^|/)id_(rsa|ed25519)$|\.pem$|\.p12$|(^|/)\.direnv/' || true)
[ -n "$r" ] && { signaler "fichier à ne pas suivre :"; echo "$r" | sed 's/^/      /'; }

# ---- 3. Empreinte de la machine et de l'auteur ------------------------------
# Un chemin absolu révèle le nom d'utilisateur et l'arborescence personnelle.
r=$(contenu '/(home|Users)/[a-zA-Z0-9._-]+/' | grep -vE '\$HOME|~/|<utilisateur>|/home/runner/')
[ -n "$r" ] && { signaler "chemin absolu personnel — utiliser \$HOME ou un chemin relatif :"; echo "$r" | sed 's/^/      /'; }

# Toute adresse de courriel autre que la forme anonyme de la plateforme.
r=$(contenu '[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}' \
      | grep -vE 'users\.noreply\.github\.com|@schemas\.|@xmlns|@xsi|example\.(com|org)|chesdata@|@[a-z]+\.invalid' \
      | grep -vE '(^|[^a-zA-Z0-9._%+-])git@github\.com:')
[ -n "$r" ] && { signaler "adresse de courriel — seule la forme noreply de la plateforme est admise :"; echo "$r" | sed 's/^/      /'; }

# ---- 4. Exposition de personnes physiques dans les artefacts ----------------
# Les fixtures de docs/ en contiennent légitimement et sont signalées.
# Les artefacts publiés et les données, jamais.
declare -a PUBLIES=()
for f in "${LISIBLES[@]}"; do
  case "$f" in public/api/*|data/*) PUBLIES+=("$f") ;; esac
done
if [ "${#PUBLIES[@]}" -gt 0 ]; then
  r=$(grep -nHIE -- '\bPA[0-9]{4,}\b' "${PUBLIES[@]}"); [ $? -gt 1 ] &&
    signaler "grep en erreur sur les artefacts publiés — le contrôle n'a rien pu affirmer"
  [ -n "$r" ] && { signaler "identifiant d'acteur dans un artefact publié ou dans data/ (RG-110) :"; echo "$r" | sed 's/^/      /'; }
  # `nom` seul est légitime : une organisation en a un. Le risque est le nom
  # d'une personne physique. Faux positif constaté le 2026-08-27 sur les noms
  # de partis du registre d'entités.
  r=$(grep -nHIE -- '"(depute|deputes|acteur|acteurs|membre|membres|personne|personnes|prenom|nom_complet|nom_depute|nom_personne|civilite|patronyme)"[[:space:]]*:' "${PUBLIES[@]}" || true)
  [ -n "$r" ] && { signaler "clé nominative dans un artefact publié ou dans data/ (RG-110) :"; echo "$r" | sed 's/^/      /'; }
fi

# ---- 4bis. Champs du contrat de preuve -------------------------------------
# Durcissements ajoutés après la revue de sécurité du 2026-08-27 : ce que la
# relecture avait attrapé et qu'aucun motif ne voyait.

# `dispersion` ne porte que trois clés. Toute autre est une statistique
# d'ordre : un minimum, un maximum ou un rang est la coordonnée d'un membre
# identifiable (I19, RG-42, ADR 0003 §3). `preuves.rs` garde le producteur ;
# ceci garde l'artefact, qui est ce qui part en ligne. `echelle.min` et
# `echelle.max` sont légitimes et hors de ce motif — vérifié, pas supposé.
r=$(contenu '"dispersion"[[:space:]]*:[[:space:]]*\{[^}]*"(etendue|minimum|maximum|min|max|rang|q1|q3|position)"')
[ -n "$r" ] && { signaler "clé de dispersion hors des trois autorisées — statistique d'ordre exposée (I19) :"; echo "$r" | sed 's/^/      /'; }

# `citation` porte du texte de tiers, sous une exception étroite plafonnée.
r=$(contenu '"citation"[[:space:]]*:[[:space:]]*"([^"\\]|\\.){401,}"')
[ -n "$r" ] && { signaler "\`citation\` au-delà du plafond de 400 caractères (RG-74, I20) :"; echo "$r" | sed 's/^/      /'; }

# `producteur` est une organisation. Une virgule ou une initiale trahit un nom
# de personne physique, que RG-76 refuse.
r=$(contenu '"producteur"[[:space:]]*:[[:space:]]*"[^"]*(,[[:space:]]|\b[A-Z]\.)')
[ -n "$r" ] && { signaler "\`producteur\` en forme de nom de personne — un producteur est une organisation (RG-76) :"; echo "$r" | sed 's/^/      /'; }

# Une heure locale de récupération révèle des horaires de travail. Les
# horodatages de la source (GMT, UTC, ISO) sont légitimes.
r=$(contenu '\b([01][0-9]|2[0-3]):[0-5][0-9](:[0-5][0-9])?\b' \
      | grep -vE 'GMT|UTC|[T ][0-9]{2}:[0-9]{2}:[0-9]{2}Z|cron|schedule|[0-9]{4}-[0-9]{2}-[0-9]{2}[ T]')
[ -n "$r" ] && { signaler "heure locale dans un document — un horodatage de récupération révèle des horaires :"; echo "$r" | sed 's/^/      /'; }

# RG-112 — ni `echelle.id` ni `echelle.libelle` ne nomme une entité mesurée.
# Le libellé avait survécu au renommage de l'identifiant : la règle portait sur
# les identifiants, le risque portait sur la chaîne publiée.
# Le motif exige l'accolade ouvrante : seul le libelle DE L'OBJET echelle est
# vise. Le libelle d'un marqueur nomme le groupe de sa propre bande, ce qui est
# legitime -- premiere version du motif trop large, corrigee le 2026-08-27.
r=$(contenu '"echelle"[[:space:]]*:[[:space:]]*\{[^}]*"(id|libelle)"[[:space:]]*:[[:space:]]*"[^"]*\b[A-Z]{2,8}(-[A-Z]{2,8})?\b' \
      | grep -vE '\b(XVI+e?|CHES)\b')
[ -n "$r" ] && { signaler "sigle d'entité dans \`echelle.id\` ou \`echelle.libelle\` (RG-112) :"; echo "$r" | sed 's/^/      /'; }

# ---- 5. Adresses privées et matériel ---------------------------------------
r=$(contenu '\b(192\.168|10\.[0-9]{1,3})\.[0-9]{1,3}\.[0-9]{1,3}\b|\b([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}\b')
[ -n "$r" ] && { signaler "adresse privée ou matérielle :"; echo "$r" | sed 's/^/      /'; }

if [ "$echec" -eq 0 ]; then
  echo "  ✓ aucun motif de fuite détecté"
  echo
  echo "Rappel : ce contrôle est déterministe et partiel. Un diff touchant data/,"
  echo "schemas/, public/api/ ou les fixtures demande en plus l'agent sécurité."
  exit 0
fi
echo
echo "Contrôle de sécurité en échec. Ne pas committer en l'état."
exit 1
