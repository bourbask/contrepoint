#!/usr/bin/env bash
# Portes déclarées bloquantes par la documentation et vérifiables SANS le
# binaire du pipeline. Appelé par le travail `portes` de .github/workflows/ci.yml
# et par .githooks/pre-commit.
#
#   ./scripts/portes-de-ci.sh                 lance les trois portes
#   ./scripts/portes-de-ci.sh identifiants    plan-de-tests.md §15 porte 1
#   ./scripts/portes-de-ci.sh invariants      contrats.md §6 I12 et I13
#   ./scripts/portes-de-ci.sh tests-ignores   plan-de-tests.md §15
#
# Ce qui N'EST PAS ici, et pourquoi : la couverture (§15 portes 2 et 3) et les
# trois contrôles de déterminisme (contrats.md §8.2) exigent le binaire
# `contrepoint`, qui n'existe pas. Leurs squelettes sont dans ci.yml, gardés par
# hashFiles, avec la commande exacte qu'ils exécuteront. Une porte déclarée sans
# travail en face est contournée dès la première PR pressée ; un travail qui
# saute en le disant ne trompe personne.
set -uo pipefail
cd "$(dirname "$0")/.."

echec=0
signaler() { echec=1; printf '::error::%s\n' "$1"; }

# ---- Porte 1 — la liste nominative des invariants ---------------------------
# plan-de-tests.md §15 : « les identifiants sont dans le document, les noms de
# fonctions sont dans le code, l'un cite l'autre ». La boucle va dans les deux
# sens : un identifiant déclaré et non écrit est une porte fantôme, un
# identifiant écrit et non déclaré est un test que le plan ne connaît pas.
PLAN=docs/brique0/plan-de-tests.md
PREFIXES='REC|POR|ADA|ING|MAT|EST|AGR|REG|PRE|FAM|EXP'

# Une suite par préfixe, désignée par SON FICHIER et non par le répertoire qui
# la contiendra. C'est ce qui évite la falaise : pointer `pipeline` faisait
# basculer six préfixes — 96 identifiants — au premier fichier Rust créé, et un
# crochet qui refuse 96 fois d'un coup se fait contourner, pas corriger. Chaque
# suite mord le jour où elle naît, et ce tableau dit aussi où chacune doit vivre.
suites_de() {
  case "$1" in
    REC) echo scripts/test-recuperer-sources.sh ;;
    POR) echo scripts/test-portes.sh ;;
    ADA) echo pipeline/tests/adaptateurs.rs ;;
    ING) echo pipeline/tests/ingestion.rs ;;
    MAT) echo pipeline/tests/matrice.rs ;;
    EST) echo pipeline/tests/estimateur.rs ;;
    AGR) echo pipeline/tests/agregation.rs ;;
    REG) echo pipeline/tests/registre.rs ;;
    PRE) echo pipeline/tests/preuves.rs ;;
    FAM) echo pipeline/tests/familles.rs ;;
    EXP) echo pipeline/tests/export.rs web/src/contrat.test.ts ;;
    *)   echo "" ;;
  esac
}

identifiants() {
  echo "── porte 1 : identifiants de test déclarés contre suites écrites"
  local prefixe presents id trouve suites s
  # Dérivé de `PREFIXES`, jamais recopié : les deux listes ont divergé au premier
  # préfixe ajouté — `POR` était dans le tableau des suites et absent de la
  # boucle, donc la porte l'ignorait en silence tout en affichant « tout passe ».
  for prefixe in $(printf '%s' "$PREFIXES" | tr '|' ' '); do
    presents=""
    for s in $(suites_de "$prefixe"); do
      [ -e "$s" ] && presents="$presents $s"
    done

    declares=$(grep -ohE "\\b$prefixe-[0-9]{2}\\b" "$PLAN" | sort -u)
    n=$(printf '%s\n' "$declares" | grep -c .)

    if [ -z "$presents" ]; then
      printf '  ⧗ %s : %s identifiants déclarés, suite absente (%s) — en attente\n' \
        "$prefixe" "$n" "$(suites_de "$prefixe" | tr ' ' ',')"
      continue
    fi

    manquants=""
    for id in $declares; do
      # -r sur un fichier simple est sans effet, -F fixe la chaîne.
      if ! grep -qrF -- "$id" $presents; then
        manquants="$manquants $id"
      fi
    done
    if [ -n "$manquants" ]; then
      signaler "$PLAN déclare des identifiants sans test dans$presents :$manquants"
    fi

    # Sens inverse : cité dans la suite, absent du plan.
    cites=$(grep -ohrE -- "\\b$prefixe-[0-9]{2}\\b" $presents | sort -u)
    intrus=$(comm -13 <(printf '%s\n' "$declares") <(printf '%s\n' "$cites") | grep . || true)
    if [ -n "$intrus" ]; then
      signaler "identifiants cités dans$presents et absents de $PLAN : $(echo $intrus)"
    fi

    [ -z "$manquants" ] && [ -z "$intrus" ] \
      && printf '  ✓ %s : %s identifiants, tous écrits dans%s\n' "$prefixe" "$n" "$presents" || true
  done
}

# ---- Porte 2 — invariants mécaniques du contrat de sortie -------------------
# contrats.md §6 : I13 est un grep, I12 une expression rationnelle sur les clés.
# Les deux sont vérifiables aujourd'hui : la boucle porte sur les artefacts qui
# existent, et elle mord dès le premier artefact produit.
# Les artefacts publiés sont ceux que git suit, pas ceux que le disque porte.
# Un glob sur le système de fichiers ramassait `data/cache/`, ignoré par git et
# rempli de 22 000 fichiers extraits par le pipeline : la porte dépassait la
# limite d'arguments du noyau et se déclarait incapable d'affirmer. Constaté le
# 2026-08-27 après la première exécution réelle du pipeline.
artefacts_publies() {
  local c
  while IFS= read -r -d '' c; do
    case "$c" in
      public/api/*.json|public/api/*/*.json|data/*.json|data/*/*.json|data/preuves/*.jsonl) ;;
      *) continue ;;
    esac
    if [ -L "$c" ]; then
      signaler "lien symbolique dans un chemin publié : $c"
    elif [ -f "$c" ]; then
      printf '%s\0' "$c"
    fi
  done < <(git ls-files -z)
  return 0
}


invariants() {
  echo "── porte 2 : invariants I12 et I13 sur les artefacts publiés"
  local -a liste=()
  mapfile -d '' -t liste < <(artefacts_publies)
  if [ "${#liste[@]}" -eq 0 ]; then
    echo "  ⧗ aucun artefact publié pour l'instant — la porte ne peut rien affirmer"
    return
  fi
  printf '  %s artefact(s) examiné(s)\n' "${#liste[@]}"

  # Le contrôle distingue « rien trouvé » de « rien lu ». Sans cette distinction,
  # un nom de fichier contenant un espace ou un guillemet, ou un seul JSON
  # illisible, rendait la porte verte alors qu'une violation était présente —
  # vérifié le 2026-08-27 avant correction. Une porte qui plante n'affirme rien.
  local r rc

  # I13 — aucune coordonnée individuelle. ADR 0000 §2, RG-41.
  r=$(grep -nHE -- '\bPA[0-9]{4,}\b' "${liste[@]}"); rc=$?
  if [ "$rc" -gt 1 ]; then
    signaler "I13 — grep en erreur (code $rc) : la porte n'a rien pu affirmer"
  elif [ -n "$r" ]; then
    signaler "I13 — identifiant d'acteur dans un artefact publié (RG-41) :"
    printf '%s\n' "$r" | sed 's/^/      /'
  else
    echo "  ✓ I13 — aucun identifiant d'acteur"
  fi

  # I12 — aucune clé ni valeur d'énumération suggérant une agrégation entre
  # familles de mesure. Sur les clés, pas sur le texte : une URL contenant
  # « score » n'est pas une violation, un champ nommé `score` en est une.
  # Les chemins passent par argv, jamais par l'entrée standard.
  r=$(python3 - "${liste[@]}" <<'PY'
import json, re, sys
motif = re.compile(r"moyenne|score|synth[eè]se|global|consensus|indice", re.I)
identifiant = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")
fautes = []
def visiter(n, chemin, fichier):
    if isinstance(n, dict):
        for k, v in n.items():
            if motif.search(k):
                fautes.append(f"{fichier}: clé {chemin}.{k}")
            visiter(v, f"{chemin}.{k}", fichier)
    elif isinstance(n, list):
        for i, v in enumerate(n):
            visiter(v, f"{chemin}[{i}]", fichier)
    elif isinstance(n, str) and identifiant.match(n) and motif.search(n):
        fautes.append(f"{fichier}: valeur {chemin} = {n}")
for f in sys.argv[1:]:
    try:
        with open(f, encoding="utf-8") as fh:
            if f.endswith(".jsonl"):
                for i, ligne in enumerate(fh, 1):
                    if ligne.strip():
                        visiter(json.loads(ligne), f"$[{i}]", f)
            else:
                visiter(json.load(fh), "$", f)
    except Exception as e:
        # Un artefact illisible ne prouve pas l'absence de violation : il empêche
        # de l'affirmer. C'est une faute, pas un fichier à sauter.
        fautes.append(f"{f}: illisible ({type(e).__name__}) — la porte ne peut rien affirmer")
print("\n".join(fautes))
PY
); rc=$?
  if [ "$rc" -ne 0 ]; then
    signaler "I12 — le contrôle a planté (code $rc) : la porte n'a rien pu affirmer"
  elif [ -n "$r" ]; then
    signaler "I12 — nom suggérant une agrégation entre familles de mesure (RG-05) :"
    printf '%s\n' "$r" | sed 's/^/      /'
  else
    echo "  ✓ I12 — aucune clé ni valeur d'agrégation inter-familles"
  fi
}

# ---- Porte 3 — aucun test sauté sans motif ni date de reprise ---------------
# plan-de-tests.md §15, definition-of-done.md « non fini ».
tests_ignores() {
  echo "── porte 3 : tests sautés sans motif écrit ni date de reprise"
  local r
  # find plutôt que `git ls-files` : un fichier ajouté et non encore indexé doit
  # être vu. -print0 et -- : un nom de fichier hostile ne devient pas un drapeau.
  if [ -z "$(find pipeline web -name '*.rs' -o -name '*.ts' -o -name '*.tsx' 2>/dev/null | head -1)" ]; then
    echo "  ⧗ aucune source Rust ni TypeScript — rien à vérifier"
    return
  fi
  # Un motif seul ne suffit pas : la date de reprise est ce qui empêche
  # « à réactiver plus tard » de durer trois ans.
  r=$(find pipeline web \( -name target -o -name node_modules -o -name dist \) -prune -o \
        \( -name '*.rs' -o -name '*.ts' -o -name '*.tsx' \) -print0 2>/dev/null \
      | xargs -0 -r grep -nHE -- '#\[ignore|\b(it|test|describe)\.(skip|todo)\(|\bxit\(|\bxdescribe\(' \
      | grep -vE '[0-9]{4}-[0-9]{2}-[0-9]{2}' || true)
  if [ -n "$r" ]; then
    signaler "test sauté sans date de reprise sur la ligne :"
    printf '%s\n' "$r" | sed 's/^/      /'
  else
    echo "  ✓ aucun test sauté sans date de reprise"
  fi
}


# ---- porte 4 : la taille du registre citée dans la documentation ------------
#
# `data/registre/partis.json` voit sa taille citée dans trois documents. Elle a
# été fausse trois fois : 42 972 pour un fichier de 42 970, puis 42 970 après
# une bascule d'URL qui l'avait mis à 42 961, puis encore après la correction de
# l'empreinte du nuancier. C'est le seul chiffre du dépôt qui bouge à chaque
# correction du registre, et personne ne pense à le suivre.
#
# La porte ne regarde **que** les lignes qui nomment `partis.json` : une première
# version, qui balayait toutes les tailles de la documentation, signalait
# vingt-sept lignes justes. Une porte qui crie faux se fait désactiver.
#
# L'empreinte n'est volontairement pas contrôlée ici : REG-23 la compare entre
# l'échantillon et le registre, et l'invariant I3 la confronte à la source à
# chaque exécution du pipeline. Un troisième motif, forcément approximatif sur
# de la prose, n'ajouterait que du bruit.
taille_du_registre() {
  printf '\n── porte 4 : taille du registre citée dans la documentation\n'
  local fichier=data/registre/partis.json
  if [ ! -f "$fichier" ]; then
    signaler "$fichier absent — la porte ne peut rien affirmer"
    return
  fi
  local octets attendu code=0 lignes r
  octets=$(LC_ALL=C wc -c < "$fichier" | tr -d ' ')
  # La prose groupe les milliers par une espace ordinaire, insécable ou fine.
  attendu="${octets:0:2}[  ]?${octets:2}"

  lignes=$(grep -rn "partis\.json" docs/ 2>/dev/null) || code=$?
  if [ "$code" -gt 1 ]; then
    signaler "grep en erreur — la porte n'a rien pu affirmer"
    return
  fi
  r=$(printf '%s\n' "$lignes" \
        | grep -E "[0-9]{2}[  ]?[0-9]{3} (o|octets)" \
        | grep -vE "$attendu (o|octets)" || true)
  if [ -n "$r" ]; then
    signaler "taille citée pour partis.json qui n'est pas la sienne ($octets octets) :"
    printf '%s\n' "$r" | cut -c1-150 | sed 's/^/      /'
  else
    echo "  ✓ toute taille citée pour partis.json vaut $octets octets"
  fi
}

case "${1:-tout}" in
  identifiants)   identifiants ;;
  invariants)     invariants ;;
  tests-ignores)  tests_ignores ;;
  registre)       taille_du_registre ;;
  tout)           identifiants; echo; invariants; echo; tests_ignores; echo; taille_du_registre ;;
  *) echo "usage: $0 [identifiants|invariants|tests-ignores|registre|tout]" >&2; exit 2 ;;
esac

echo
if [ "$echec" -ne 0 ]; then
  echo "portes de CI : au moins une porte refuse"
  exit 1
fi
echo "portes de CI : tout passe"
