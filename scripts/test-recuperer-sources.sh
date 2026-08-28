#!/usr/bin/env bash
# Suite hors ligne de scripts/recuperer-sources.sh — niveau 1 du plan de tests.
# Aucun réseau, aucune clé, aucun jeton, aucune horloge : le script à tester est
# sourcé, et seules ses fonctions pures sont exercées. L'enveloppe réseau
# (`telecharger`) n'a pas de test hors ligne, par construction
# (brique0/ingestion-votes.md §9d).
#
#   ./scripts/test-recuperer-sources.sh
#
# Identifiants dans docs/brique0/plan-de-tests.md §4bis.
set -uo pipefail
cd "$(dirname "$0")/.."

# Sourcé, donc `principal` ne s'exécute pas : le script le vérifie lui-même.
# shellcheck source=recuperer-sources.sh
. scripts/recuperer-sources.sh

echecs=0
bac=$(mktemp -d)
trap 'rm -rf "$bac"' EXIT

verifier() { # identifiant invariant attendu obtenu
  if [ "$3" = "$4" ]; then
    printf '  ✓ %s %s\n' "$1" "$2"
  else
    echecs=$((echecs + 1))
    printf '  ✗ %s %s\n      attendu : %s\n      obtenu  : %s\n' "$1" "$2" "$3" "$4"
  fi
}

# ---- REC-01 — la taille annoncée est la porte -------------------------------
printf 'x%.0s' $(seq 1 100) > "$bac/archive.bin"
telechargement_complet "$bac/archive.bin" 100 && r=complet || r=incomplet
verifier REC-01 "100 octets reçus sur 100 annoncés : le téléchargement est complet" complet "$r"
telechargement_complet "$bac/archive.bin" 101 && r=complet || r=incomplet
verifier REC-01 "100 octets reçus sur 101 annoncés : la troncature est vue" incomplet "$r"
telechargement_complet "$bac/absente.bin" 100 && r=complet || r=incomplet
verifier REC-01 "archive absente : jamais complète" incomplet "$r"

# ---- REC-02 — empreinte de contenu en ordre d'octets ------------------------
# Sous fr_FR.UTF-8 la ponctuation est ignorée dans la collation : V100 précède
# V10, qui précède V1 — l'inverse de l'ordre d'octets. L'attendu est écrit à la
# main dans l'ordre d'octets, il n'est pas relevé d'une exécution.
mkdir -p "$bac/extrait/json"
printf 'un'   > "$bac/extrait/json/V1.json"
printf 'dix'  > "$bac/extrait/json/V10.json"
printf 'cent' > "$bac/extrait/json/V100.json"
attendu=$(printf 'undixcent' | sha256sum | cut -d' ' -f1)
verifier REC-02 "concaténation en ordre d'octets, pas en ordre de locale" \
  "$attendu" "$(empreinte_contenu "$bac/extrait")"
verifier REC-02 "empreinte inchangée sous une locale à collation exotique" \
  "$attendu" "$(LC_ALL=fr_FR.UTF-8 empreinte_contenu "$bac/extrait")"
cp -r "$bac/extrait" "$bac/ailleurs"
verifier REC-02 "empreinte indépendante du répertoire d'extraction" \
  "$attendu" "$(empreinte_contenu "$bac/ailleurs")"

# Noms de fichiers hostiles. Une archive distante décide de ces noms : sans
# terminateur d'options, un fichier nommé `-n` atteint cat comme un drapeau et
# l'empreinte publiée devient celle de `cat -n`, silencieusement et avec un
# code de retour nul. Sans séparateur NUL, un nom contenant un saut de ligne
# est coupé en deux chemins inexistants et l'empreinte devient celle du vide.
# Les deux défauts ont été démontrés le 2026-08-27 avant correction.
mkdir -p "$bac/hostile"
printf 'ligne\n' > "$bac/hostile/z.json"
touch "$bac/hostile/-n"
attendu_h=$(printf 'ligne\n' | sha256sum | cut -d' ' -f1)
verifier REC-02 "un fichier nommé -n n'atteint pas cat comme une option" \
  "$attendu_h" "$(empreinte_contenu "$bac/hostile")"

mkdir -p "$bac/hostile2"
printf 'X' > "$bac/hostile2/$(printf 'a\nb')"
printf 'Y' > "$bac/hostile2/c.json"
attendu_h2=$(printf 'XY' | sha256sum | cut -d' ' -f1)
verifier REC-02 "un nom contenant un saut de ligne ne coupe pas la liste" \
  "$attendu_h2" "$(empreinte_contenu "$bac/hostile2")"

# ---- REC-03 — contenu modifié sans annonce = échec bruyant ------------------
index="$bac/index.txt"
printf 'scrutins\t2026-08-27T04:25:40Z\taaa\tzzz\n' > "$index"
verifier_stabilite "$index" scrutins 2026-08-27T04:25:40Z aaa 2>/dev/null && r=accepte || r=refuse
verifier REC-03 "même date, même contenu : accepté" accepte "$r"
verifier_stabilite "$index" scrutins 2026-08-27T04:25:40Z bbb 2>/dev/null && r=accepte || r=refuse
verifier REC-03 "même date, contenu différent : refusé" refuse "$r"
verifier_stabilite "$index" scrutins 2026-08-28T04:25:40Z bbb 2>/dev/null && r=accepte || r=refuse
verifier REC-03 "date nouvelle, contenu différent : accepté" accepte "$r"
verifier_stabilite "$bac/inexistant.txt" scrutins 2026-08-27T04:25:40Z bbb 2>/dev/null && r=accepte || r=refuse
verifier REC-03 "index absent : accepté, il n'y a rien à contredire" accepte "$r"

# ---- REC-04 — descripteur déterministe --------------------------------------
ecrire_descripteur "$bac/d1.txt" "url=u" "octets_recus=3" "empreinte_sha256=a"
ecrire_descripteur "$bac/d2.txt" "empreinte_sha256=a" "url=u" "octets_recus=3"
verifier REC-04 "descripteur déterministe, quel que soit l'ordre des champs" \
  "empreinte_sha256=a|octets_recus=3|url=u" "$(paste -sd'|' "$bac/d1.txt")"
verifier REC-04 "les deux ordres donnent le même octet" \
  "$(cat "$bac/d1.txt")" "$(cat "$bac/d2.txt")"

# ---- REC-05 — date de source, format exigé par pipeline.yml -----------------
d=$(derniere_source 2026-08-27T00:34:47Z 2026-08-27T04:25:40Z 2026-08-26T23:00:00Z)
verifier REC-05 "la source la plus récente l'emporte" 2026-08-27T04:25:40Z "$d"
printf '%s' "$d" | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$' \
  && r=conforme || r=invalide
verifier REC-05 "horodatage ISO complet, exigé par le schéma preuve-1" conforme "$r"
verifier REC-05 "relu à l'identique par le \`date -u -d\` de pipeline.yml" \
  "$d" "$(date -u -d "${d:-rien}" +%Y-%m-%dT%H:%M:%SZ 2>/dev/null)"
verifier REC-05 "un last-modified HTTP devient un horodatage ISO complet" \
  2026-08-27T04:25:40Z "$(iso_depuis_http 'Thu, 27 Aug 2026 04:25:40 GMT')"

# ---- REC-06 — cache immuable ------------------------------------------------
CONTREPOINT_CACHE="$bac/cache"
mkdir -p "$CONTREPOINT_CACHE/fa0fa18ace89abc8216fdc5924626538855574538b99618b54d0c769973edbb6"
printf 'archive d origine' > "$CONTREPOINT_CACHE/fa0fa18ace89abc8216fdc5924626538855574538b99618b54d0c769973edbb6/Scrutins.json.zip"
printf 'autre chose' > "$bac/Scrutins.json.zip"
deposer_cache fa0fa18ace89abc8216fdc5924626538855574538b99618b54d0c769973edbb6 "$bac/Scrutins.json.zip" >/dev/null
verifier REC-06 "une entrée de cache déjà présente n'est jamais réécrite" \
  "archive d origine" "$(cat "$CONTREPOINT_CACHE/fa0fa18ace89abc8216fdc5924626538855574538b99618b54d0c769973edbb6/Scrutins.json.zip")"
deposer_cache d98c02036530900ba8d8083fd4edf6eb41f332b122ec11472ae035b1f0abff5b "$bac/Scrutins.json.zip" >/dev/null
verifier REC-06 "une empreinte inconnue crée son entrée" \
  "autre chose" "$(cat "$CONTREPOINT_CACHE/d98c02036530900ba8d8083fd4edf6eb41f332b122ec11472ae035b1f0abff5b/Scrutins.json.zip")"

# ---- REC-07 — la mention de paternité est dérivée, jamais recopiée ----------
# Deux des quatre sources ne sont pas de l'Assemblée nationale. Une mention
# écrite en dur les attribuait toutes à elle, et à une licence qu'elles n'ont
# pas : c'est une fausse attribution, pas une approximation (RG-76, RG-118).
verifier REC-07 "la paternité vaut producteur, licence et date de la source" \
  "Assemblée nationale — Licence Ouverte v1.0 — données du 2026-08-27T04:25:40Z" \
  "$(paternite 'Assemblée nationale' 'Licence Ouverte v1.0' 2026-08-27T04:25:40Z)"
p_ches=$(paternite 'Chapel Hill Expert Survey' 'aucune licence publiée — réutilisation soumise à citation' 2026-08-04T17:28:50Z)
printf '%s' "$p_ches" | grep -q 'Assemblée nationale' && r=contaminee || r=propre
verifier REC-07 "la paternité de CHES ne nomme pas l'Assemblée nationale" propre "$r"
printf '%s' "$p_ches" | grep -q 'Licence Ouverte' && r=contaminee || r=propre
verifier REC-07 "la paternité de CHES n'annonce aucune Licence Ouverte" propre "$r"

# ---- REC-08 — seule une source redistribuable est déposée en release --------
# CHES ne publie aucune licence (ADR 0000 §8, RG-118) et le nuancier n'est pas
# dans la liste blanche : le dépôt distribue l'URL, la date et les empreintes,
# jamais la copie. Le contrôle est exercé sur `scripts/archives.sh` lui-même,
# et il échoue AVANT tout appel réseau à `gh`.
deposable() { # url -> depose|refuse
  local url="$1" d sha sortie
  # Un .csv et non un .zip : les sources non redistribuables sont de forme
  # `fichier`. Avec un faux zip, le refus venait de la recherche d'archive et
  # non de la liste blanche — la porte testée n'était pas celle qui s'ouvre.
  printf 'contenu' > "$bac/donnees.csv"
  sha=$(sha256sum "$bac/donnees.csv" | cut -d' ' -f1)
  d="$bac/depot/$sha"
  mkdir -p "$d"
  cp "$bac/donnees.csv" "$d/"
  printf 'url=%s\n' "$url" > "$d/descripteur.txt"
  sortie=$(./scripts/archives.sh deposer "$d" 2>&1 || true)
  if printf '%s' "$sortie" | grep -q 'hors de la liste redistribuable'; then
    echo refuse
  else
    echo depose
  fi
  rm -rf "$bac/depot"
}
verifier REC-08 "CHES n'est jamais déposé : aucune licence publiée" refuse \
  "$(deposable 'https://github.com/chesdata/chesdata.github.io/releases/download/ches-europe/CHES_2024_final_v2.csv')"
verifier REC-08 "le nuancier n'est pas dans la liste blanche non plus" refuse \
  "$(deposable 'https://static.data.gouv.fr/resources/x/resultats-definitifs-par-circonscription.csv')"
verifier REC-08 "une URL qui ressemble à celle de l'Assemblée sans en être une est refusée" refuse \
  "$(deposable 'https://data.assemblee-nationale.fr.exemple.invalid/Scrutins.json.zip')"

echo
if [ "$echecs" -eq 0 ]; then
  echo "recuperer-sources : tous les tests passent"
  exit 0
fi
echo "recuperer-sources : $echecs test(s) en échec"
exit 1
