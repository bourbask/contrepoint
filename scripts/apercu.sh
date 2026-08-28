#!/usr/bin/env bash
# Rastérise une page en image, pour qu'elle puisse être REGARDÉE.
#
# Le front est jugé sur des nombres — 29 tests de contrat, un rendu SVG figé —
# et jamais sur ce qu'il donne à voir. Un axe juste et illisible reste un échec,
# et rien dans le dépôt ne pouvait l'attraper.
#
#   ./scripts/apercu.sh <fichier.html|url> <sortie.png> [largeur] [hauteur]
#
# Firefox et non un convertisseur SVG : `rsvg-convert` ignore les variables CSS
# et rend tout en noir. Mesuré le 2026-08-28.
#
# Un profil jetable à chaque appel : sans lui, Firefox refuse de démarrer quand
# une session est déjà ouverte, et le rendu dépendrait des préférences locales.
set -euo pipefail

source="${1:?usage: apercu.sh <fichier.html|url> <sortie.png> [largeur] [hauteur]}"
sortie="${2:?sortie png attendue}"
# Chemin ABSOLU : avec un chemin relatif, Firefox rend 0 et n'écrit rien du tout.
mkdir -p "$(dirname "$sortie")"
sortie="$(readlink -f "$sortie")"
largeur="${3:-760}"
hauteur="${4:-1600}"
theme="${APERCU_THEME:-0}"   # 0 = clair, 1 = sombre

command -v firefox >/dev/null || { echo "::error::firefox absent" >&2; exit 1; }
case "$source" in
  http://*|https://*|file://*) cible="$source" ;;
  *) cible="file://$(readlink -f "$source")" ;;
esac

profil=$(mktemp -d)
trap 'rm -rf "$profil"' EXIT
# Le thème est une entrée, jamais la preference de la machine : deux apercus
# pris sur deux postes doivent etre comparables.
printf 'user_pref("ui.systemUsesDarkTheme", %s);\n' "$theme" > "$profil/user.js"

timeout 120 firefox --headless --profile "$profil" \
  --window-size="$largeur,$hauteur" --screenshot "$sortie" "$cible" >/dev/null 2>&1 || true

[ -s "$sortie" ] || { echo "::error::aucune image produite pour $cible" >&2; exit 1; }
printf 'aperçu : %s (%s×%s, thème %s) → %s octets\n' \
  "$sortie" "$largeur" "$hauteur" "$([ "$theme" = 1 ] && echo sombre || echo clair)" "$(stat -c%s "$sortie")"
