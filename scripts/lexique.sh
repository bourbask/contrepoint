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
QUALIF='biais d[eu]s? (media|média|journal|rédaction|redaction)|partial(e|es|ité|ités)?\b|militant|palmar|note globale|extrémit|extremit|extrémis|extremis'
# Agrégations qui écrasent les familles de mesure (règle non négociable n°6).
AGREG='score global|indice de position|consensus des mesures|moyenne des familles'

declare -a FICHIERS=()
cible="${1:-code}"
case "$cible" in
  code)
    mapfile -d '' -t FICHIERS < <(git ls-files -z 'pipeline/*' 'web/*' 'schemas/*' 'data/*' 'public/*' 2>/dev/null)
    motifs="$AXES|$QUALIF|$AGREG"
    ;;
  docs)
    # juridique.md et ton.md définissent le lexique : ils citent les termes interdits.
    mapfile -d '' -t FICHIERS < <(git ls-files -z '*.md')
    # juridique.md et ton.md définissent le lexique : ils citent les termes interdits.
    declare -a garde=()
    for f in "${FICHIERS[@]}"; do
      case "$f" in docs/juridique.md|docs/ton.md|CHANGELOG.md) ;; *) garde+=("$f") ;; esac
    done
    FICHIERS=("${garde[@]}")
    # RG-91 interdit les axes dépréciatifs dans la documentation aussi.
    # QUALIF reste hors du mode docs : la documentation discute légitimement
    # de la partialité comme concept, et « biais de sélection » est un terme
    # retenu du projet.
    motifs="$AXES|$AGREG"
    ;;
  *) echo "usage: $0 [code|docs]" >&2; exit 2 ;;
esac

if [ "${#FICHIERS[@]}" -eq 0 ]; then
  echo "lexique ($cible) : aucun fichier à vérifier"
  exit 0
fi

# Un lien symbolique ferait lire hors du dépôt ; un fichier supprimé mais encore
# indexé ferait échouer grep, et l'échec était avalé.
declare -a LISIBLES=()
for f in "${FICHIERS[@]}"; do
  if [ -L "$f" ]; then
    echo "::error::lien symbolique suivi : $f"; exit 1
  elif [ -f "$f" ]; then
    LISIBLES+=("$f")
  fi
done
[ "${#LISIBLES[@]}" -eq 0 ] && { echo "lexique ($cible) : aucun fichier lisible"; exit 0; }

# Une ligne qui ÉNONCE l'interdiction n'est pas une violation : la
# documentation doit pouvoir dire « aucun axe de fiabilité ». On ne retient
# donc que les occurrences dépourvues de marqueur de prohibition.
PROHIBITION='aucun|jamais|interdit|proscrit|ne (le |la |les )?(sert|fait|fera|produit|publie)|pas de|ni de|hors périmètre|sans axe'

# Même défaut que celui trouvé le 2026-08-27 dans une porte de CI puis dans
# securite.sh : une liste séparée par des sauts de ligne casse sur un nom
# contenant un espace ou un guillemet, et le `|| true` transformait la casse en
# succès. grep rend 0 s'il a trouvé, 1 sinon, plus de 1 s'il a échoué — les
# trois ne sont pas la même chose.
# `code=$?` sur la même ligne ne protège pas : avec `set -e`, l'affectation qui
# échoue tue le script avant. Le `|| code=$?` en fait une commande composée.
code=0
brut=$(grep -inE -- "$motifs" "${LISIBLES[@]}") || code=$?
if [ "$code" -gt 1 ]; then
  echo "::error::grep en erreur (code $code) — le contrôle de lexique n'a rien pu affirmer"
  exit 1
fi
violations=$(printf '%s' "$brut" | grep -ivE "$PROHIBITION" || true)
if [ -n "$violations" ]; then
  echo "$violations"
  echo "::error::terme interdit par docs/juridique.md — voir scripts/lexique.sh"
  echo "Si l'occurrence énonce l'interdiction, formuler la ligne avec « aucun » ou « jamais »."
  exit 1
fi
echo "lexique ($cible) : aucun terme interdit sur ${#LISIBLES[@]} fichiers"
