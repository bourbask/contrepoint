#!/usr/bin/env bash
# Dépôt et récupération des archives sources, indexées par leur SHA-256.
#
#   ./scripts/archives.sh deposer <dossier-de-cache>   dépose archive + descripteur
#   ./scripts/archives.sh retrouver <empreinte> <dest> récupère une archive
#   ./scripts/archives.sh present <empreinte>          0 si déjà déposée
#   ./scripts/archives.sh deposer-index <fichier>      dépose l'index (remplacé)
#   ./scripts/archives.sh retrouver-index <fichier>    récupère l'index s'il existe
#
# Pourquoi une release et pas le cache d'Actions : rejouer en 2027 le calcul de
# 2026 exige de conserver l'archive de 2026 (brique0/ingestion-votes.md §9e), et
# le cache d'Actions évince à sept jours sous un plafond de dix gigaoctets. Un
# asset de release n'est ni évincé ni plafonné de la même façon.
#
# Redistribution : autorisée pour ces archives par la Licence Ouverte v1.0 sous
# laquelle l'Assemblée nationale les publie (ADR 0000 §8). Aucune archive dont
# la licence n'autorise pas la redistribution n'est déposée ici — CHES en
# particulier reste dehors.
#
# Exige `gh` authentifié. En intégration continue, `GH_TOKEN` vaut
# `${{ github.token }}` et le travail déclare `permissions: contents: write` —
# la même permission que `tag.yml` utilise déjà pour `gh release create`.
#
# `A VERIFIER`, trois points, aucun n'a pu l'être sans écrire sur le dépôt
# distant, ce qui n'était pas dans le périmètre du ticket :
#   1. que `${{ github.token }}` avec `contents: write` suffise à créer la
#      release et à téléverser un asset. Vérifier par un
#      `workflow_dispatch` de `pipeline.yml` et la lecture du journal, ou en
#      local par `gh release create archives-sources --notes test` puis
#      `gh release delete archives-sources --cleanup-tag` ;
#   2. le plafond de taille d'un asset et le volume total toléré sur un dépôt
#      public. Vérifier sur la documentation GitHub en vigueur ; les archives
#      pèsent 26 et 14 mégaoctets, très en dessous de tout plafond connu ;
#   3. que `gh release create` sur le tag `archives-sources` ne déclenche pas
#      `tag.yml`, qui n'écoute que `push` sur `main` — donc a priori non.
#      Vérifier par la même exécution manuelle.
set -euo pipefail

: "${CONTREPOINT_TAG_ARCHIVES:=archives-sources}"

assurer_release() {
  gh release view "$CONTREPOINT_TAG_ARCHIVES" >/dev/null 2>&1 && return 0
  gh release create "$CONTREPOINT_TAG_ARCHIVES" \
    --title "Archives sources" \
    --notes "Archives des sources publiques, indexées par leur SHA-256. Assemblée nationale — Licence Ouverte v1.0. Conservées pour la rejouabilité : le descripteur \`<empreinte>.txt\` porte l'URL, la taille et la date de source." \
    --latest=false
}

# Une empreinte est 64 hexadécimaux. Sans cette garde, `retrouver '*' /chemin`
# téléchargerait un asset arbitraire, et le motif de `present` serait un glob.
empreinte_valide() { [[ "$1" =~ ^[0-9a-f]{64}$ ]] || {
  echo "::error::empreinte invalide : $1" >&2; return 1; }; }

present() { # empreinte
  empreinte_valide "$1" || return 1
  gh release view "$CONTREPOINT_TAG_ARCHIVES" --json assets \
    --jq '.assets[].name' 2>/dev/null | grep -qxF "$1.zip"
}

deposer() { # dossier de cache, nommé par l'empreinte
  local dossier="${1%/}" empreinte archive
  empreinte=$(basename "$dossier")
  # Le refus de redistribuer vient AVANT la recherche d'archive. Sinon une
  # source non redistribuable — donc de forme `fichier`, sans zip — fait
  # echouer l'etape, le pipeline s'arrete sous `bash -e`, et `deposer-index`
  # n'est jamais atteint : la garde anti-modification-silencieuse se desarme
  # sans que rien ne le dise. Refus par liste blanche, mais refus propre.
  if ! grep -qE '^url=https://data\.assemblee-nationale\.fr/' "$dossier/descripteur.txt"; then
    echo "$dossier : source hors de la liste redistribuable (RG-118, ADR 0000 §8) — non déposée"
    return 0
  fi
  archive=$(find "$dossier" -maxdepth 1 -name '*.zip' | head -1)
  if [ -z "$archive" ]; then
    echo "::error::aucune archive dans $dossier" >&2
    return 1
  fi
  # Un asset de release est immuable et publié : un asset mal nommé est
  # définitif. L'empreinte est recalculée, jamais déduite du nom du dossier.
  local reelle
  reelle=$(sha256sum "$archive" | cut -d' ' -f1)
  if [ "$reelle" != "$empreinte" ]; then
    echo "::error::$dossier : l'archive vaut $reelle, le dossier annonce $empreinte." >&2
    return 1
  fi
  if present "$empreinte"; then
    echo "$empreinte déjà déposée — un asset est immuable, rien à faire"
    return 0
  fi
  assurer_release
  local tmp
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' RETURN
  cp "$archive" "$tmp/$empreinte.zip"
  cp "$dossier/descripteur.txt" "$tmp/$empreinte.txt"
  gh release upload "$CONTREPOINT_TAG_ARCHIVES" "$tmp/$empreinte.zip" "$tmp/$empreinte.txt"
  echo "$empreinte déposée"
}

retrouver() { # empreinte destination
  empreinte_valide "$1" || return 1
  gh release download "$CONTREPOINT_TAG_ARCHIVES" --pattern "$1.zip" --output "$2" --clobber
  local obtenue
  obtenue=$(sha256sum "$2" | cut -d' ' -f1)
  if [ "$obtenue" != "$1" ]; then
    echo "::error::asset $1 récupéré avec l'empreinte $obtenue — l'index de la release ment." >&2
    return 1
  fi
}

deposer_index() { # fichier
  assurer_release
  local tmp
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' RETURN
  cp "$1" "$tmp/index.txt"
  gh release upload "$CONTREPOINT_TAG_ARCHIVES" "$tmp/index.txt" --clobber
}

retrouver_index() { # fichier
  # Distinguer « pas encore de release » d'une panne. Avaler les deux ferait
  # échouer en ouvert la garde anti-modification-silencieuse : index absent,
  # verifier_stabilite accepte tout, et personne ne le voit avant un an.
  if ! gh release view "$CONTREPOINT_TAG_ARCHIVES" >/dev/null 2>&1; then
    echo "aucune release d'archives — première exécution"
    return 0
  fi
  gh release download "$CONTREPOINT_TAG_ARCHIVES" --pattern index.txt \
    --output "$1" --clobber
}

case "${1:-}" in
  deposer)         deposer "$2" ;;
  retrouver)       retrouver "$2" "$3" ;;
  present)         present "$2" ;;
  deposer-index)   deposer_index "$2" ;;
  retrouver-index) retrouver_index "$2" ;;
  *) sed -n '2,9p' "$0" >&2; exit 2 ;;
esac
