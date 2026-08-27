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

if [ "${1:-}" = "--arbre" ]; then
  fichiers=$(git ls-files)
  mode=arbre
else
  fichiers=$(git diff --cached --name-only --diff-filter=ACMR)
  mode=index
fi

if [ -z "$fichiers" ]; then
  echo "sécurité ($mode) : rien à vérifier"
  exit 0
fi

contenu() { echo "$fichiers" | xargs -r -I{} sh -c '[ -f "{}" ] && grep -nHIE "$1" "{}"' _ "$1" 2>/dev/null; }

echo "sécurité ($mode) : $(echo "$fichiers" | grep -c .) fichiers"

# ---- 1. Secrets -------------------------------------------------------------
r=$(contenu '(BEGIN (RSA|OPENSSH|EC|PGP) PRIVATE KEY)|gh[pousr]_[A-Za-z0-9]{20,}|glpat-[A-Za-z0-9_-]{15,}|AKIA[0-9A-Z]{16}|sk-[A-Za-z0-9]{20,}|xox[baprs]-[A-Za-z0-9-]{10,}')
[ -n "$r" ] && { signaler "secret probable :"; echo "$r" | sed 's/^/      /'; }

r=$(contenu '(api[_-]?key|secret|password|passwd|token)[[:space:]]*[:=][[:space:]]*['"'"'"][^'"'"'"]{8,}')
[ -n "$r" ] && { signaler "valeur sensible affectée en clair :"; echo "$r" | sed 's/^/      /'; }

# ---- 2. Fichiers qui ne doivent jamais être suivis --------------------------
r=$(echo "$fichiers" | grep -E '(^|/)\.env($|\.)|(^|/)\.netrc$|(^|/)id_(rsa|ed25519)$|\.pem$|\.p12$|(^|/)\.direnv/')
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
publies=$(echo "$fichiers" | grep -E '^(public/api/|data/)' || true)
if [ -n "$publies" ]; then
  r=$(echo "$publies" | xargs -r grep -nHIE '\bPA[0-9]{4,}\b' 2>/dev/null)
  [ -n "$r" ] && { signaler "identifiant d'acteur dans un artefact publié ou dans data/ (RG-110) :"; echo "$r" | sed 's/^/      /'; }
  # `nom` seul est légitime : une organisation en a un. Le risque est le nom
  # d'une personne physique. Faux positif constaté le 2026-08-27 sur les noms
  # de partis du registre d'entités.
  r=$(echo "$publies" | xargs -r grep -nHIE '"(depute|deputes|acteur|acteurs|membre|membres|personne|personnes|prenom|nom_complet|nom_depute|nom_personne|civilite|patronyme)"[[:space:]]*:' 2>/dev/null)
  [ -n "$r" ] && { signaler "clé nominative dans un artefact publié ou dans data/ (RG-110) :"; echo "$r" | sed 's/^/      /'; }
fi

# ---- 4bis. Champs du contrat de preuve -------------------------------------
# Durcissements ajoutés après la revue de sécurité du 2026-08-27 : ce que la
# relecture avait attrapé et qu'aucun motif ne voyait.

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
