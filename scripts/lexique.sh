#!/usr/bin/env bash
# Source unique des termes interdits par docs/juridique.md.
# Appelé par la CI et cité par docs/definition-of-done.md et docs/ton.md,
# qui ne doivent PAS recopier la liste : trois copies avaient déjà divergé.
#
#   ./scripts/lexique.sh code   vérifie l'arbre de code et de données
#   ./scripts/lexique.sh docs   vérifie la documentation
set -euo pipefail

# Axes à pôle dépréciatif : interdits partout, sans exception.
AXES='fiabilit|credibilit|crédibilit|veracit|véracit|desinformation|désinformation|fake.?news|infox'
# Qualifications d'organisation plutôt que mesures : interdites dans le produit.
QUALIF='biais d[eu]s? (media|média|journal|rédaction|redaction)|partial|militant|palmar|note globale|extrémit|extremit|extrémis|extremis'
# Agrégations qui écrasent les familles de mesure (règle non négociable n°6).
AGREG='score global|indice de position|consensus des mesures|moyenne des familles'

cible="${1:-code}"
case "$cible" in
  code)
    fichiers=$(git ls-files 'pipeline/*' 'web/*' 'schemas/*' 'data/*' 'public/*' 2>/dev/null || true)
    motifs="$AXES|$QUALIF|$AGREG"
    ;;
  docs)
    # juridique.md et ton.md définissent le lexique : ils citent les termes interdits.
    fichiers=$(git ls-files '*.md' | grep -vE '^(docs/juridique\.md|docs/ton\.md|CHANGELOG\.md)$' || true)
    # RG-91 interdit les axes dépréciatifs dans la documentation aussi.
    # QUALIF reste hors du mode docs : la documentation discute légitimement
    # de la partialité comme concept, et « biais de sélection » est un terme
    # retenu du projet.
    motifs="$AXES|$AGREG"
    ;;
  *) echo "usage: $0 [code|docs]" >&2; exit 2 ;;
esac

if [ -z "$fichiers" ]; then
  echo "lexique ($cible) : aucun fichier à vérifier"
  exit 0
fi

# Une ligne qui ÉNONCE l'interdiction n'est pas une violation : la
# documentation doit pouvoir dire « aucun axe de fiabilité ». On ne retient
# donc que les occurrences dépourvues de marqueur de prohibition.
PROHIBITION='aucun|jamais|interdit|proscrit|ne (le |la |les )?(sert|fait|fera|produit|publie)|pas de|ni de|hors périmètre|sans axe'

violations=$(echo "$fichiers" | xargs -r grep -inE "$motifs" | grep -ivE "$PROHIBITION" || true)
if [ -n "$violations" ]; then
  echo "$violations"
  echo "::error::terme interdit par docs/juridique.md — voir scripts/lexique.sh"
  echo "Si l'occurrence énonce l'interdiction, formuler la ligne avec « aucun » ou « jamais »."
  exit 1
fi
echo "lexique ($cible) : aucun terme interdit sur $(echo "$fichiers" | grep -c .) fichiers"
