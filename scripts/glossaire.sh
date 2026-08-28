#!/usr/bin/env bash
# Porte du glossaire normatif. docs/glossaire.md EST la source de vérité : rien
# n'est recopié ici, ni un terme, ni une interdiction. Une liste recopiée
# diverge — trois copies du lexique avaient déjà divergé, et l'une refusait un
# terme retenu du projet.
#
#   ./scripts/glossaire.sh              vérifie docs/
#   ./scripts/glossaire.sh <répertoire> vérifie un autre arbre (mutant, essai)
#
# Deux contrôles :
#   1. aucun terme déclaré « Interdit » dans le glossaire n'apparaît ailleurs ;
#   2. aucun terme défini par le glossaire n'est redéfini dans un autre document
#      (ISO/IEC Directives Part 2 : un seul article « Termes et définitions »).
#
# Conventions reprises de scripts/lexique.sh et scripts/securite.sh : sortie
# `::error::`, une ligne de bilan chiffrée en cas de succès, code 1 en échec,
# code 2 en erreur d'usage.
set -uo pipefail
cd "$(dirname "$0")/.."

# LC_ALL=C partout : un tri ou une comparaison dépendant de la locale rend un
# ordre différent d'une machine à l'autre, donc une porte non reproductible.
export LC_ALL=C

RACINE="${1:-docs}"
# La source de vérité est docs/glossaire.md ; un arbre d'essai qui porte sa
# propre copie (mutant) l'emporte, ce qui permet de vérifier la porte hors dépôt.
GLOSSAIRE="docs/glossaire.md"
[ -f "$RACINE/glossaire.md" ] && GLOSSAIRE="$RACINE/glossaire.md"

echec=0
signaler() { echec=1; printf '::error::%s\n' "$1"; }

if [ ! -f "$GLOSSAIRE" ]; then
  echo "::error::$GLOSSAIRE absent — la source de vérité du contrôle n'existe pas" >&2
  exit 2
fi

# ---- Lecture de la source de vérité ----------------------------------------
# Termes définis : un titre de niveau 3 par entrée.
TERMES=$(sed -n 's/^### \(.*\)$/\1/p' "$GLOSSAIRE")

# Termes interdits : les fragments entre accents graves des lignes « Interdit ».
# Les lignes « À migrer » sont délibérément ignorées : elles nomment des formes
# encore présentes, et une porte rouge le jour de sa naissance se fait désarmer.
INTERDITS=$(sed -n 's/^\*\*Interdit :\*\*//p' "$GLOSSAIRE" \
  | tr ',' '\n' | sed -n 's/.*`\(.*\)`.*/\1/p')

nb_termes=$(printf '%s\n' "$TERMES" | grep -c . )
nb_interdits=$(printf '%s\n' "$INTERDITS" | grep -c . )

# Une porte qui ne lit rien affirme le vert sans avoir rien vérifié. C'est le
# défaut exact que ce script est chargé de ne pas reproduire.
if [ "$nb_termes" -lt 5 ] || [ "$nb_interdits" -lt 5 ]; then
  echo "::error::$GLOSSAIRE illisible : $nb_termes termes, $nb_interdits interdits — le contrôle n'a rien pu affirmer" >&2
  exit 2
fi

# ---- Fichiers à examiner ----------------------------------------------------
# Séparateur NUL et tableau : une liste séparée par des sauts de ligne casse sur
# un nom contenant un espace ou un guillemet. Jamais `xargs` sans -0, jamais
# `|| true` sur une commande de contrôle : ce couple avait rendu vertes trois
# portes du projet, un seul fichier malformé suffisant.
declare -a SUIVIS=() FICHIERS=()
if git rev-parse --git-dir >/dev/null 2>&1; then
  mapfile -d '' -t SUIVIS < <(git ls-files -z -- "$RACINE" 2>/dev/null)
fi
if [ "${#SUIVIS[@]}" -eq 0 ]; then
  mapfile -d '' -t SUIVIS < <(find "$RACINE" -type f -print0)
fi
for f in "${SUIVIS[@]}"; do
  case "$f" in *.md) FICHIERS+=("$f") ;; esac
done

declare -a LISIBLES=()
for f in "${FICHIERS[@]}"; do
  if [ -L "$f" ]; then
    signaler "lien symbolique suivi : $f"
  elif [ -f "$f" ] && [ "$f" != "$GLOSSAIRE" ]; then
    LISIBLES+=("$f")
  fi
done

if [ "${#LISIBLES[@]}" -eq 0 ]; then
  echo "::error::aucun fichier à examiner sous $RACINE — le contrôle n'a rien pu affirmer" >&2
  exit 2
fi

echo "glossaire : $nb_termes termes définis, $nb_interdits formes interdites, ${#LISIBLES[@]} fichiers examinés"

# grep rend 0 s'il a trouvé, 1 s'il n'a rien trouvé, plus de 1 s'il a échoué.
# « rien trouvé » et « rien lu » ne sont pas la même chose : le second doit
# rendre la porte rouge. Sous `set -e`, `x=$(cmd); code=$?` tuerait le script
# avant la lecture de $? — d'où la forme `code=0; x=$(cmd) || code=$?`.
chercher() { # $1 = drapeaux, $2 = motif → imprime les lignes, rend 2 si erreur
  local sortie code=0
  sortie=$(grep -n "$1" -- "$2" "${LISIBLES[@]}") || code=$?
  if [ "$code" -gt 1 ]; then
    signaler "grep en erreur (code $code) sur « $2 » — le contrôle n'a rien pu affirmer"
    return 2
  fi
  printf '%s' "$sortie"
}

# ---- Contrôle 1 — formes interdites ----------------------------------------
# -F : les formes portent des apostrophes et des accents, pas des motifs.
while IFS= read -r terme; do
  [ -z "$terme" ] && continue
  r=$(chercher -iHF "$terme")
  if [ -n "$r" ]; then
    signaler "forme interdite par docs/glossaire.md : « $terme »"
    printf '%s\n' "$r" | sed 's/^/      /'
  fi
done <<< "$INTERDITS"

# ---- Contrôle 2 — définition en double -------------------------------------
# Une définition est un terme en gras suivi d'un marqueur définitoire. Le motif
# n'attrape pas une simple mention en gras dans une phrase : sans le marqueur,
# pas de violation — c'est ce qui évite le faux positif sur les tableaux de
# formes canoniques de docs/ton.md §3, qui nomment sans définir.
while IFS= read -r terme; do
  [ -z "$terme" ] && continue
  r=$(chercher -inHE "^[[:space:]]*([-*>|][[:space:]]*)?\*\*$terme\*\*[[:space:]]*(:|=|est |désigne )")
  if [ -n "$r" ]; then
    signaler "« $terme » est défini une seconde fois hors de docs/glossaire.md"
    printf '%s\n' "$r" | sed 's/^/      /'
  fi
done <<< "$TERMES"

if [ "$echec" -eq 0 ]; then
  echo "  ✓ aucune forme interdite, aucune définition en double"
  exit 0
fi
echo
echo "::error::porte du glossaire en échec — voir docs/glossaire.md"
exit 1
