#!/usr/bin/env bash
# Suite hors ligne des portes de contrôle — niveau 1 du plan de tests.
#
# Ce que cette suite existe pour tenir : un contrôle doit distinguer « rien
# trouvé » de « rien lu », **y compris** quand l'arbre porte beaucoup de
# fichiers. Le 2026-08-28, une copie de cache égarée a mis 22 490 fichiers dans
# l'index : `securite.sh` et `lexique.sh` ont dépassé `ARG_MAX` et rendu 126 sur
# chaque motif. Ils ont échoué bruyamment — c'est le comportement voulu — mais
# aucun des deux ne pouvait plus rien affirmer.
#
#   ./scripts/test-portes.sh
#
# Identifiants dans docs/brique0/plan-de-tests.md §4ter.
set -uo pipefail
cd "$(dirname "$0")/.."

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

# ---- POR-01 — le découpage tient au-delà d'ARG_MAX --------------------------
#
# 23 000 chemins longs, soit environ 3,3 Mio d'`argv` : au-delà de la limite du
# noyau. Sans découpage, `grep` rend 126 et le contrôle n'affirme plus rien.
long="$bac/bbecd01274d2bc9f46fcaa276b06868862ae7680131da3162e35b5cbef663061/extrait/json/organe"
mkdir -p "$long"
python3 - "$long" <<'PY'
import sys
for n in range(23000):
    open(f"{sys.argv[1]}/PO{800000 + n}.json", "w").write("{}\n")
PY
mapfile -t TOUS < <(find "$bac" -name '*.json')

# La primitive, isolée : même forme que `contenu()` de securite.sh et que la
# boucle de lexique.sh.
par_lots() { # motif fichiers...
  local motif="$1"; shift
  local -a f=("$@")
  local sortie="" partiel code n=0
  while [ "$n" -lt "${#f[@]}" ]; do
    code=0
    partiel=$(grep -nHIE -- "$motif" "${f[@]:n:400}") || code=$?
    [ "$code" -gt 1 ] && { printf 'echec:%s' "$code"; return 0; }
    [ -n "$partiel" ] && sortie+="$partiel"$'\n'
    n=$((n + 400))
  done
  printf '%s' "${sortie%$'\n'}"
}

# La reproduction du defaut est un **constat**, pas un invariant : `ARG_MAX`
# depend du noyau et de la limite de pile, donc du coureur. Chez moi 23 000
# chemins la depassent, sur un coureur GitHub non — la premiere version de ce
# test l'affirmait et echouait en CI, ce qui etait ma faute et non la sienne.
# On mesure, on l'affiche, on n'en fait pas une assertion.
code=0
grep -nHIE -- 'motif-absent' "${TOUS[@]}" >/dev/null 2>&1 || code=$?
octets=$(printf '%s\0' "${TOUS[@]}" | wc -c)
if [ "$code" -gt 1 ]; then
  printf '  ⧗ POR-01 constat : sans découpage, %s chemins (%s Kio) font échouer grep\n' \
    "${#TOUS[@]}" "$((octets / 1024))"
else
  printf '  ⧗ POR-01 constat : %s chemins (%s Kio) passent sans découpage sur cette machine — ARG_MAX y est plus haut\n' \
    "${#TOUS[@]}" "$((octets / 1024))"
fi

# L'invariant, lui, ne depend d'aucune limite : avec decoupage, « rien trouve »
# reste distinct de « rien lu », quelle que soit la taille de la liste.
r=$(par_lots 'motif-absent' "${TOUS[@]}")
verifier POR-01 "avec découpage, rien trouvé se distingue de rien lu" "" "$r"

# ---- POR-02 — un défaut reste vu à cette échelle ----------------------------
#
# Le découpage ne doit pas seulement ne pas planter : il doit encore trouver.
# Une aiguille dans le dernier lot, là où une boucle qui s'arrête trop tôt la
# manquerait.
# Le jeton est **assemblé à l'exécution**, jamais écrit en clair : un littéral
# dans un fichier suivi est un secret pour `securite.sh`, qui a refusé le commit
# — et il avait raison, il ne peut pas savoir qu'il est factice.
printf '%s_%s\n' 'ghp' "$(printf 'A%.0s' $(seq 1 30))" > "$long/zzz-derniere.json"
mapfile -t AVEC < <(find "$bac" -name '*.json' | LC_ALL=C sort)
r=$(par_lots 'gh[pousr]_[A-Za-z0-9]{20,}' "${AVEC[@]}")
verifier POR-02 "une occurrence dans le dernier lot est trouvée" oui \
  "$([ -n "$r" ] && echo oui || echo non)"
verifier POR-02 "et une seule ligne est rapportée" 1 "$(printf '%s\n' "$r" | grep -c '')"

# ---- POR-03 — un échec de lecture reste un échec ----------------------------
#
# Le découpage ne doit pas avaler une erreur : un fichier illisible au milieu de
# la liste rend le contrôle rouge, jamais vert.
if [ "$(id -u)" -eq 0 ]; then
  printf '  ⧗ POR-03 non exécuté sous root : les droits ne bloquent rien\n'
else
  illisible="$bac/illisible.json"
  echo '{}' > "$illisible"
  chmod 000 "$illisible"
  r=$(par_lots 'motif-absent' "${AVEC[@]}" "$illisible")
  chmod 644 "$illisible"
  verifier POR-03 "un fichier illisible rend « rien lu », pas « rien trouvé »" oui \
    "$(case "$r" in echec:*) echo oui ;; *) echo non ;; esac)"
fi

# ---- POR-04 — la porte de T2 ne s'ouvre pas sur un mot ----------------------
# Le 2026-09-08, une revue a prouve que le mode `personne` exemptait TOUTE ligne
# portant un mot de prohibition — « aucun », « jamais », « pas de ». « Aucune
# donnee personnelle ne vous est demandee », phrase type d'une mention de
# confidentialite, passait ; la meme privee de son « Aucune » etait refusee.
# La seule porte de T2 s'ouvrait sur un mot. L'exemption porte desormais sur le
# pronom CITE, et ces quatre cas la tiennent.
essai_personne() { # texte -> rc
  local f=docs/sources.md sauve="$bac/sources.md" r
  cp "$f" "$sauve"
  printf '\n%s\n' "$1" >> "$f"
  ./scripts/lexique.sh personne >/dev/null 2>&1; r=$?
  cp "$sauve" "$f"
  echo "$r"
}
if [ -f docs/sources.md ]; then
  verifier POR-04 "une prohibition ne blanchit pas le pronom qui la suit" 1 \
    "$(essai_personne 'Aucune donnee personnelle ne vous est demandee.')"
  verifier POR-04 "ni « pas de », ni « jamais »" 1 \
    "$(essai_personne 'Il n a pas de tri : nous classons par ordre alphabetique.')"
  verifier POR-04 "un pronom cite entre guillemets reste permis" 0 \
    "$(essai_personne 'Une regle interdit « nous » et « vous » dans une chaine.')"
else
  printf '  ⧗ POR-04 non execute : docs/sources.md absent\n'
fi

# ---- POR-05 — la porte des codes d'absence lit bien le rendu ----------------
# Meme revue : le mode `public` exigeait un guillemet colle au code, si bien que
# `<p>Non mesure : hors_source</p>` passait — le cas meme que T7 vise.
if [ -f web/src/fixtures/rendu-fige.html ]; then
  sauve="$bac/rendu-fige.html"
  cp web/src/fixtures/rendu-fige.html "$sauve"
  printf '<p>Non mesure : hors_source pour ce groupe</p>' >> web/src/fixtures/rendu-fige.html
  ./scripts/lexique.sh public >/dev/null 2>&1; r=$?
  cp "$sauve" web/src/fixtures/rendu-fige.html
  verifier POR-05 "un code d'absence en texte nu est refuse" 1 "$r"
  ./scripts/lexique.sh public >/dev/null 2>&1
  verifier POR-05 "et le rendu intact passe" 0 "$?"
else
  printf '  ⧗ POR-05 non execute : aucun rendu fige\n'
fi

echo
if [ "$echecs" -ne 0 ]; then
  echo "portes : $echecs invariant(s) en échec"
  exit 1
fi
echo "portes : tous les invariants tiennent"
