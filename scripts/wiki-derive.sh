#!/usr/bin/env bash
# Détecte la dérive entre la documentation du dépôt et le miroir du wiki.
#
#   ./scripts/wiki-derive.sh
#
# Ne pousse rien, ne modifie rien. Il dit quelles pages sont à reprendre et de
# quelle façon — parce que toutes ne se régénèrent pas mécaniquement.
#
# Correspondance et règles : docs/wiki-miroir.md
set -uo pipefail

WIKI="${WIKI:-/tmp/contrepoint-wiki}"
ETAT=".sources.sha256"

# page|fichier source|nature
TABLE='
Methode.md|docs/methode.md|miroir
Architecture.md|docs/architecture.md|miroir
Feuille-de-route.md|ROADMAP.md|miroir
Contraintes-de-publication.md|docs/juridique.md|adapte
Comprendre-le-projet.md|docs/utilisation.md|enrichi
Home.md|README.md|propre
'

if [ ! -d "$WIKI/.git" ]; then
  echo "Le wiki n'est pas cloné dans $WIKI."
  echo "  git clone git@github.com:bourbask/contrepoint.wiki.git $WIKI"
  echo "  (ou WIKI=<chemin> $0)"
  exit 2
fi

derive=0
printf '%-32s %-28s %s\n' "PAGE" "SOURCE" "ÉTAT"
printf '%-32s %-28s %s\n' "--------------------------------" "----------------------------" "----"

while IFS='|' read -r page source nature; do
  [ -z "$page" ] && continue
  [ -f "$source" ] || { printf '%-32s %-28s source absente\n' "$page" "$source"; derive=1; continue; }
  actuel=$(sha256sum "$source" | cut -d' ' -f1)
  connu=$(grep -F " $source" "$WIKI/$ETAT" 2>/dev/null | cut -d' ' -f1 | head -1)
  if [ -z "$connu" ]; then
    etat="jamais synchronisée"
    derive=1
  elif [ "$actuel" = "$connu" ]; then
    etat="à jour"
  else
    derive=1
    case "$nature" in
      miroir)  etat="DÉRIVE — régénérable mécaniquement" ;;
      adapte)  etat="DÉRIVE — reprise à la main (exemptions de lexique non transportables)" ;;
      enrichi) etat="DÉRIVE — reprise à la main (contient un apport propre au wiki)" ;;
      propre)  etat="source modifiée — page propre au wiki, à relire" ;;
    esac
  fi
  printf '%-32s %-28s %s\n' "$page" "$source" "$etat"
done <<< "$TABLE"

echo
if [ "$derive" -eq 0 ]; then
  echo "Miroir à jour."
  exit 0
fi
echo "Le wiki dérive. Reprendre les pages ci-dessus, puis enregistrer l'état :"
echo "  ./scripts/wiki-derive.sh --enregistrer"
[ "${1:-}" = "--enregistrer" ] || exit 1

: > "$WIKI/$ETAT"
while IFS='|' read -r page source nature; do
  [ -z "$page" ] && continue
  [ -f "$source" ] && sha256sum "$source" >> "$WIKI/$ETAT"
done <<< "$TABLE"
echo "État enregistré dans $WIKI/$ETAT — à committer dans le wiki."
