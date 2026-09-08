#!/usr/bin/env bash
# Source unique des termes interdits par docs/juridique.md.
# Appelé par la CI et cité par docs/definition-of-done.md et docs/ton.md,
# qui ne doivent PAS recopier la liste : trois copies avaient déjà divergé.
#
#   ./scripts/lexique.sh code       vérifie l'arbre de code et de données
#   ./scripts/lexique.sh docs       vérifie la documentation
#   ./scripts/lexique.sh public     T7 sur les chaînes affichées (docs/ton.md §1 bis)
#   ./scripts/lexique.sh personne   T2 sur le diff, hors chemins du régime public
set -euo pipefail
cd "$(dirname "$0")/.."

# Portée du régime public de docs/ton.md §0, déclarée ICI et nulle part ailleurs :
# le document la cite, il ne la recopie pas. Une seconde copie divergerait, comme
# les trois copies du lexique avant elle.
# Une ligne qui ÉNONCE l'interdiction n'est pas une violation : la
# documentation doit pouvoir dire « aucun axe de fiabilité ». On ne retient
# donc que les occurrences dépourvues de marqueur de prohibition.
PROHIBITION='aucun|jamais|interdit|proscrit|ne (le |la |les )?(sert|fait|fera|produit|publie)|pas de|ni de|hors périmètre|sans axe'

PUBLIC_CHEMINS=(web/src)
# Même raison que LOT plus bas : la liste complète dépasse ARG_MAX.
LOT_PUBLIC=400

# Les fichiers qui ÉNONCENT les règles en citent forcément les termes et les
# motifs. Déclarés une fois, employés par `docs` et par `personne` : la
# première version portait la liste deux fois et les deux avaient divergé.
DEFINISSENT_LA_REGLE=(docs/juridique.md docs/ton.md CHANGELOG.md scripts/lexique.sh)

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
      exempt=0
      for d in "${DEFINISSENT_LA_REGLE[@]}"; do [ "$f" = "$d" ] && exempt=1; done
      [ "$exempt" -eq 0 ] && garde+=("$f")
    done
    FICHIERS=("${garde[@]}")
    # RG-91 interdit les axes dépréciatifs dans la documentation aussi.
    # QUALIF reste hors du mode docs : la documentation discute légitimement
    # de la partialité comme concept, et « biais de sélection » est un terme
    # retenu du projet.
    motifs="$AXES|$AGREG"
    ;;
  public)
    # T7 en régime public : le `motif_code` reste dans l'artefact, jamais à
    # l'écran. Le motif s'affiche en français.
    #
    # Ce contrôle vise le RENDU, pas la source. La version du 2026-08-31 grepait
    # les `.tsx` et laissait passer `<p>Non mesuré : hors_source</p>`, faute
    # d'un guillemet collé au code — le cas même que T7 vise, prouvé le
    # 2026-09-01. Élargir le motif sur la source rendait alors huit lignes
    # justes : déclarations de type de `contrat.ts`, assertions de test, et la
    # fonction de traduction elle-même. Un motif qui signale du code correct est
    # un motif à changer, pas une exception à ajouter.
    #
    # Le rendu figé est la sortie du composant, telle qu'un visiteur la lit. Un
    # code qui s'y trouve est à l'écran, sans interprétation possible ; un code
    # absent n'y est pas. Aucun faux positif n'est possible par construction.
    declare -a rendus=()
    for c in "${PUBLIC_CHEMINS[@]}"; do
      declare -a ajout=()
      mapfile -d '' -t ajout < <(git ls-files -z -- "$c/*.html")
      [ "${#ajout[@]}" -gt 0 ] && rendus+=("${ajout[@]}")
    done
    if [ "${#rendus[@]}" -eq 0 ]; then
      echo "::error::aucun rendu figé sous ${PUBLIC_CHEMINS[*]} — le contrôle n'a rien pu affirmer"
      exit 1
    fi
    for f in "${rendus[@]}"; do
      [ -L "$f" ] && { echo "::error::lien symbolique suivi : $f"; exit 1; }
      [ -f "$f" ] || { echo "::error::fichier indexé mais absent : $f"; exit 1; }
    done
    # La liste des codes est dérivée du schéma, jamais recopiée : elle a déjà
    # gagné deux valeurs depuis la version 1 du contrat. `re.escape` est
    # indispensable — une valeur portant `)` ou `[` rendrait l'ERE invalide, et
    # l'échec de grep passait pour un succès.
    codes=$(python3 -c 'import json,re;e=json.load(open("schemas/preuve-2.schema.json"))["properties"]["motif_code"]["enum"];print("|".join(re.escape(c) for c in e if c))')
    [ -z "$codes" ] && { echo "::error::liste de motif_code vide — le contrôle n'a rien pu affirmer"; exit 1; }
    code=0
    faute=$(grep -nHE -- "\\b($codes)\\b" "${rendus[@]}") || code=$?
    if [ "$code" -gt 1 ]; then
      echo "::error::grep en erreur (code $code) — le contrôle public n'a rien pu affirmer"
      exit 1
    fi
    if [ -n "$faute" ]; then
      printf '%s\n' "$faute"
      echo "::error::code d'absence à l'écran — docs/ton.md T7 : le motif s'affiche en français"
      exit 1
    fi
    echo "lexique (public) : aucun code d'absence dans ${#rendus[@]} rendu(s) figé(s)"
    exit 0
    ;;
  personne)
    # T2. Sur le diff, jamais sur l'arbre : l'arbre porte 130 occurrences, dont
    # les `on` anglais de LICENSE, des citations, et les lignes de docs/ton.md
    # qui énoncent la règle. Un contrôle qui crie faux 130 fois se fait
    # désactiver.
    #
    # Les chemins du régime public sont exclus : docs/ton.md §1 bis y autorise
    # la deuxième personne à l'indicatif, une fois par écran. Le plafond d'une
    # occurrence par écran n'est pas automatisable — un écran n'est pas un
    # fichier — et se relit.
    #
    # DEUX motifs, deux portées, et c'est délibéré. Le resserrage aux cinq
    # extensions a été mesuré pour `on` seul : il attrapait 33 lignes justes —
    # la clé `on:` de chaque workflow YAML, et le « on » anglais des citations
    # académiques de `public/api/preuves/*.json` et `data/preuves/positions.jsonl`.
    # L'appliquer aussi à `nous|vous|votre` avait retiré du contrôle les
    # libellés de `data/registre/*.json`, que docs/ton.md §0 place pourtant en
    # régime interne — un garde-fou perdu en passant, prouvé le 2026-09-01.
    # La première personne est donc contrôlée partout hors régime public.
    PREMIERE='nous|notre|nos|vous|votre|vos'
    # Sensible à la casse, contrairement à PREMIERE. Le pronom français s'écrit
    # « on » ou « On » en tête de phrase, jamais « ON ». Sans cette distinction,
    # toute citation anglaise en capitales fait crier la porte : le codebook du
    # Chapel Hill dit « GENERAL POSITION ON EUROPEAN INTEGRATION », et mutiler
    # une citation pour faire taire un motif est le pire des deux maux.
    IMPERSONNEL='on|On'
    base="${2:-origin/develop}"
    if ! git rev-parse --verify -q "$base" >/dev/null; then
      echo "::error::référence de comparaison absente : $base"
      echo "usage: $0 personne [base]"
      exit 1
    fi
    declare -a exclus=()
    for c in "${PUBLIC_CHEMINS[@]}"; do exclus+=(":(exclude)$c"); done
    for d in "${DEFINISSENT_LA_REGLE[@]}"; do exclus+=(":(exclude)$d"); done
    # `grep` rend 1 quand il ne trouve rien : c'est le cas nominal ici. Au-delà
    # de 1, il a échoué, et un échec avalé rendrait la porte verte à tort.
    ajoutes() {
      local sortie code=0
      sortie=$(git diff -U0 --merge-base "$base" -- "$@" | grep -E '^\+' | grep -v '^+++') || code=$?
      [ "$code" -gt 1 ] && { echo "::error::git diff en erreur (code $code)" >&2; return 2; }
      printf '%s' "$sortie"
    }
    cherche() {
      # `${3-i}` et non `${3:-i}` : la seconde forme substitue aussi sur une
      # chaîne vide, si bien que passer '' redonnait « i » et le motif restait
      # insensible à la casse.
      local dans="$1" motif="$2" casse="${3-i}" sortie code=0
      # Une ligne qui ÉNONCE l'interdiction n'est pas une violation : une règle
      # doit pouvoir écrire « pas de "nous" ». Même motif que le corps
      # principal, déclaré une seule fois — deux listes recopiées ont déjà
      # divergé ici.
      sortie=$(printf '%s' "$dans" | grep -nw${casse}E -- "$motif" | grep -viE -- "$PROHIBITION") || code=$?
      [ "$code" -gt 1 ] && { echo "::error::grep en erreur (code $code)" >&2; return 2; }
      printf '%s' "$sortie"
    }
    # Première personne : partout, sauf le régime public et les textes qui
    # énoncent la règle.
    v1=$(cherche "$(ajoutes "${exclus[@]}")" "$PREMIERE") || exit 1
    # `on` : seulement là où il ne se confond pas avec la clé YAML ni l'anglais.
    v2=$(cherche "$(ajoutes '*.md' '*.rs' '*.ts' '*.tsx' '*.sh' "${exclus[@]}")" "$IMPERSONNEL" '') || exit 1
    if [ -n "$v1$v2" ]; then
      [ -n "$v1" ] && printf '%s\n' "$v1"
      [ -n "$v2" ] && printf '%s\n' "$v2"
      echo "::error::personne grammaticale hors régime public — docs/ton.md T2"
      echo "Une citation entre guillemets se justifie dans la description de la PR."
      exit 1
    fi
    echo "lexique (personne) : aucune personne grammaticale ajoutée hors ${PUBLIC_CHEMINS[*]}"
    exit 0
    ;;
  *) echo "usage: $0 [code|docs|public|personne]" >&2; exit 2 ;;
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


# Même défaut que celui trouvé le 2026-08-27 dans une porte de CI puis dans
# securite.sh : une liste séparée par des sauts de ligne casse sur un nom
# contenant un espace ou un guillemet, et le `|| true` transformait la casse en
# succès. grep rend 0 s'il a trouvé, 1 sinon, plus de 1 s'il a échoué — les
# trois ne sont pas la même chose.
# `code=$?` sur la même ligne ne protège pas : avec `set -e`, l'affectation qui
# échoue tue le script avant. Le `|| code=$?` en fait une commande composée.
#
# Toute la liste d'un coup dépasse `ARG_MAX` dès qu'un grand répertoire non
# suivi traîne dans l'arbre — mesuré le 2026-08-28 sur 23 000 fichiers à chemin
# long, où le contrôle rendait 126 et n'affirmait plus rien. Le découpage est
# fait ici et non par `xargs`, qui rend 123 dès qu'une invocation rend entre 1
# et 125 : « rien trouvé » et « grep en erreur » y deviendraient le même code.
LOT=400
brut=""
n=0
while [ "$n" -lt "${#LISIBLES[@]}" ]; do
  code=0
  partiel=$(grep -inE -- "$motifs" "${LISIBLES[@]:n:LOT}") || code=$?
  if [ "$code" -gt 1 ]; then
    echo "::error::grep en erreur (code $code) — le contrôle de lexique n'a rien pu affirmer"
    exit 1
  fi
  [ -n "$partiel" ] && brut+="$partiel"$'\n'
  n=$((n + LOT))
done
brut="${brut%$'\n'}"
violations=$(printf '%s' "$brut" | grep -ivE "$PROHIBITION" || true)
if [ -n "$violations" ]; then
  echo "$violations"
  echo "::error::terme interdit par docs/juridique.md — voir scripts/lexique.sh"
  echo "Si l'occurrence énonce l'interdiction, formuler la ligne avec « aucun » ou « jamais »."
  exit 1
fi
echo "lexique ($cible) : aucun terme interdit sur ${#LISIBLES[@]} fichiers"
