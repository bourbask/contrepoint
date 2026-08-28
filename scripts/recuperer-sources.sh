#!/usr/bin/env bash
# Composant [0] de docs/architecture.md — récupération des archives sources.
#
#   ./scripts/recuperer-sources.sh
#
# Ne fait aucun parsing, aucune transformation, aucune écriture dans data/
# hors du cache. Les archives ne sont pas versionnées (.gitignore).
#
# Deux moitiés, et la séparation est volontaire :
#   - les fonctions pures ci-dessous, testées hors ligne par
#     scripts/test-recuperer-sources.sh ;
#   - `telecharger` et `principal`, qui touchent le réseau et n'ont pas de test
#     hors ligne (brique0/ingestion-votes.md §9d).
#
# Le script est sourçable : sourcé, il ne fait rien d'autre que définir ses
# fonctions.
set -euo pipefail

# CONTREPOINT_CACHE est relatif : sans cela, lancé depuis ailleurs, le
# script écrirait son cache hors du dépôt.
cd "$(dirname "$0")/.."

: "${CONTREPOINT_CACHE:=data/cache}"
: "${CONTREPOINT_TENTATIVES:=8}"

# Les quatre sources du pipeline — URL relevées dans docs/sources.md §1.1 et
# dans `sources[]` du registre d'entités. Aucune n'est construite par
# concaténation, et chacune porte le nom que son producteur publie (RG-76) et
# la licence sous laquelle elle le publie.
#
#   nom|url|forme|producteur|licence
#
# `forme` vaut `zip` — un conteneur, dont l'empreinte de contenu se calcule sur
# les fichiers extraits (contrats.md §2.8) — ou `fichier` : la ressource est un
# fichier unique, son contenu EST le fichier, et les deux empreintes coïncident
# par définition (I22).
#
# La licence décide de la redistribution, et elle seule (RG-118, ADR 0000 §8) :
# `scripts/archives.sh` ne dépose que les archives dont l'URL est celle de
# l'Assemblée nationale, par liste blanche. CHES, qui ne publie aucune licence,
# et le nuancier, qui n'est pas dans cette liste, ne sont jamais déposés — le
# dépôt distribue ce script, l'URL, la date et les empreintes, jamais la copie.
SOURCES=(
  "scrutins|https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip|zip|Assemblée nationale|Licence Ouverte v1.0"
  "amo30|https://data.assemblee-nationale.fr/static/openData/repository/17/amo/tous_acteurs_mandats_organes_xi_legislature/AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip|zip|Assemblée nationale|Licence Ouverte v1.0"
  # CHES ne publie aucune licence. La condition obtenue par échange écrit le
  # 2026-08-27 est une exigence de citation, pas une cession de droits
  # (ADR 0000 §8, RG-118). Le libellé reste court : il entre dans
  # `mention_paternite`, plafonnée à 200 caractères par I20, et le détail
  # vit dans docs/sources.md §4.
  "ches_2024|https://github.com/chesdata/chesdata.github.io/releases/download/ches-europe/CHES_2024_final_v2.csv|fichier|Chapel Hill Expert Survey|aucune licence publiée, citation exigée"
  "nuance_leg2024|https://static.data.gouv.fr/resources/elections-legislatives-des-30-juin-et-7-juillet-2024-resultats-definitifs-du-2nd-tour/20240710-170536/resultats-definitifs-par-region.csv|fichier|Ministère de l'intérieur|Licence Ouverte v2.0"
)

# ---- Fonctions pures --------------------------------------------------------

# La taille annoncée est la porte, pas le MD5 du producteur : celui-ci varie
# avec la construction servie (brique0/verification-2026-08-27.md §0).
telechargement_complet() { # fichier octets_annonces
  [ -f "$1" ] && [ "$(stat -c%s "$1")" -eq "$2" ]
}

# Méthode imposée par brique0/contrats.md §2.8 : chemins relatifs à la racine
# de l'archive, tri en ordre d'octets — `LC_ALL=C`, jamais la collation de la
# locale —, SHA-256 de la concaténation des contenus, sans séparateur.
empreinte_contenu() { # repertoire_d_extraction
  (
    cd "$1" || exit 1
    # -print0 et sort -z : un nom contenant un saut de ligne serait sinon
    # coupé en deux chemins. `--` : un fichier nommé `-n` dans l'archive
    # atteindrait sinon cat comme un drapeau, et l'empreinte publiée serait
    # celle de `cat -n`, silencieusement et avec un code de retour nul.
    # Le préfixe `./` est constant, il ne change pas l'ordre d'octets.
    find . -type f -print0 | LC_ALL=C sort -z | xargs -0 cat -- |
      sha256sum | cut -d' ' -f1
  )
}

# Une empreinte de contenu qui change alors que la date de source n'a pas
# changé signale une donnée modifiée sans annonce : le script s'arrête.
verifier_stabilite() { # index nom date_source empreinte_contenu
  [ -f "$1" ] || return 0
  local n d c
  while IFS=$'\t' read -r n d c _; do
    [ "$n" = "$2" ] && [ "$d" = "$3" ] && [ "$c" != "$4" ] || continue
    echo "::error::$2 : empreinte de contenu $c -> $4 sans changement de la date de source ($3). Donnée modifiée sans annonce." >&2
    return 1
  done < "$1"
}

# La mention de paternité est dérivée de la source, jamais écrite en dur : deux
# des quatre sources ne sont pas de l'Assemblée nationale, et une mention
# recopiée d'une source sur une autre est une fausse attribution (RG-76).
paternite() { # producteur licence date_source
  printf '%s — %s — données du %s' "$1" "$2" "$3"
}

# Trié, donc identique quel que soit l'ordre d'appel.
ecrire_descripteur() { # fichier cle=valeur...
  local f="$1"
  shift
  printf '%s\n' "$@" | LC_ALL=C sort > "$f"
}

iso_depuis_http() { # date_http
  date -u -d "$1" +%Y-%m-%dT%H:%M:%SZ
}

# Les horodatages ISO en Z se comparent en ordre d'octets.
derniere_source() { # iso...
  printf '%s\n' "$@" | LC_ALL=C sort | tail -1
}

# Le cache est indexé par empreinte d'archive et immuable : une entrée déjà
# présente n'est jamais réécrite.
deposer_cache() { # empreinte_archive fichier
  # L'empreinte devient un nom de répertoire : vérifié, `../../evade` créait
  # bien un répertoire hors du cache. Inexploitable tant que l'empreinte vient
  # d'un sha256sum local, dangereux le jour où elle viendra de l'index distant.
  [[ "$1" =~ ^[0-9a-f]{64}$ ]] || { echo "::error::empreinte invalide : $1" >&2; return 1; }
  local d="$CONTREPOINT_CACHE/$1"
  mkdir -p "$d"
  [ -e "$d/$(basename "$2")" ] || cp "$2" "$d/"
  echo "$d"
}

# ---- Enveloppe réseau, sans test hors ligne ---------------------------------

entete() { # entetes nom
  printf '%s' "$1" | tr -d '\r' | grep -i "^$2:" | tail -1 | cut -d' ' -f2-
}

# Le serveur de l'Assemblée ferme la connexion en cours de transfert sans
# erreur (ADR 0001 §1.5) : reprise `curl -C -` jusqu'à la taille annoncée.
telecharger() { # url destination -> "octets_annonces last_modified etag"
  local url="$1" dest="$2" entetes annonce lm et i
  entetes=$(curl -sSIL --retry 3 "$url")
  annonce=$(entete "$entetes" content-length)
  lm=$(entete "$entetes" last-modified)
  et=$(entete "$entetes" etag)
  if [ -z "$annonce" ]; then
    echo "::error::$url n'annonce pas de content-length — la porte de complétude n'existe pas." >&2
    return 1
  fi
  for ((i = 1; i <= CONTREPOINT_TENTATIVES; i++)); do
    telechargement_complet "$dest" "$annonce" && break
    echo "  tentative $i : $(stat -c%s "$dest" 2>/dev/null || echo 0) / $annonce octets" >&2
    # `-L` : la ressource CHES est servie par une redirection 302 de GitHub
    # Releases. Sans lui, la reprise télécharge le corps vide de la redirection
    # et la boucle épuise ses huit tentatives sur zéro octet.
    curl -sSL -C - -o "$dest" "$url" || true
  done
  if ! telechargement_complet "$dest" "$annonce"; then
    echo "::error::$url tronqué après $CONTREPOINT_TENTATIVES tentatives : $(stat -c%s "$dest" 2>/dev/null || echo 0) / $annonce octets." >&2
    return 1
  fi
  printf '%s\t%s\t%s\n' "$annonce" "$lm" "$et"
}

principal() {
  mkdir -p "$CONTREPOINT_CACHE"
  local index="$CONTREPOINT_CACHE/index.txt"
  local fichiers tmp dates=() nom url forme producteur licence meta annonce lm et iso
  local archive sha md5 dossier extrait contenu recupere_le

  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' RETURN

  # La date de récupération est celle de la machine qui télécharge, et c'est le
  # seul endroit du projet où une horloge est lue : le pipeline, lui, n'en lit
  # aucune (contrats.md §8.1). Elle est consignée une fois par entrée de cache,
  # immuable comme le reste du descripteur.
  recupere_le=$(date -u +%Y-%m-%d)

  for entree in "${SOURCES[@]}"; do
    IFS='|' read -r nom url forme producteur licence <<< "$entree"
    archive="$tmp/$(basename "$url")"
    echo "== $nom"

    meta=$(telecharger "$url" "$archive")
    IFS=$'\t' read -r annonce lm et <<< "$meta"
    iso=$(iso_depuis_http "$lm")

    sha=$(sha256sum "$archive" | cut -d' ' -f1)
    # Le MD5 est consigné à titre documentaire et n'échoue jamais le script :
    # la source répond en plusieurs constructions du même contenu, et le MD5
    # publié suit celle que son propre serveur voit (verification §0).
    md5=$(md5sum "$archive" | cut -d' ' -f1)

    dossier=$(deposer_cache "$sha" "$archive")
    if [ "$forme" = "zip" ]; then
      extrait="$dossier/extrait"
      # find -type f les exclut de l'empreinte, mais toute étape ultérieure lisant
      # extrait/ hériterait du piège : une entrée pointant vers /etc/passwd est
      # bien matérialisée par unzip.
      [ -d "$extrait" ] || { unzip -oq "$dossier/$(basename "$url")" -d "$extrait" &&
        find "$extrait" -type l -delete; }
      contenu=$(empreinte_contenu "$extrait")
      fichiers=$(find "$extrait" -type f | wc -l)
    else
      # Fichier unique : rien à décompresser, et le contenu EST le fichier. Les
      # deux empreintes coïncident par définition, et une inégalité serait un
      # refus du contrat (contrats.md §2.8, I22).
      contenu="$sha"
      fichiers=1
    fi

    verifier_stabilite "$index" "$nom" "$iso" "$contenu"

    ecrire_descripteur "$dossier/descripteur.txt" \
      "source=$nom" \
      "url=$url" \
      "octets_annonces=$annonce" \
      "octets_recus=$(stat -c%s "$dossier/$(basename "$url")")" \
      "last_modified=$lm" \
      "date_source=$iso" \
      "etag=$et" \
      "md5_producteur_documentaire=$md5" \
      "empreinte_sha256=$sha" \
      "empreinte_contenu_sha256=$contenu" \
      "forme=$forme" \
      "fichier=$(basename "$url")" \
      "producteur=$producteur" \
      "licence=$licence" \
      "recupere_le=$recupere_le" \
      "paternite=$(paternite "$producteur" "$licence" "$iso")" \
      "fichiers_extraits=$fichiers"

    printf '%s\t%s\t%s\t%s\n' "$nom" "$iso" "$contenu" "$sha" >> "$index"
    LC_ALL=C sort -u "$index" -o "$index"
    dates+=("$iso")

    echo "   $annonce octets, archive $sha, contenu $contenu, source du $iso"
  done

  # pipeline.yml en dérive CONTREPOINT_DATE_CALCUL. Le pipeline ne lit jamais
  # l'horloge : la date de calcul vient de la source (contrats.md §8.1).
  derniere_source "${dates[@]}" > "$CONTREPOINT_CACHE/derniere-source.txt"
  echo "date de source retenue : $(cat "$CONTREPOINT_CACHE/derniere-source.txt")"
}

if [ "${BASH_SOURCE[0]}" = "$0" ]; then
  principal "$@"
fi
