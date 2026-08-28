#!/usr/bin/env bash
# Recupere la couleur d'identite de chaque entite depuis Wikidata (P465).
#
# Pourquoi un fichier SEPARE du registre d'entites : l'empreinte de
# `data/registre/partis.json` entre dans la cle de deduplication du §3. Y
# ajouter un champ re-emettrait TOUS les identifiants de preuve. Or une couleur
# est de la presentation, pas une entree de mesure : elle n'a rien a faire dans
# la chaine de preuve.
#
# Pourquoi Wikidata : la source est deja declaree au registre, les douze
# identifiants y sont deja mappes, et P465 est en CC0 — donc redistribuable,
# contrairement aux logos, qui sont des marques deposees.
#
#   ./scripts/couleurs-identite.sh
#
# Le reseau est ici, jamais dans le pipeline (contrats.md §8.1).
set -euo pipefail
cd "$(dirname "$0")/.."

registre=data/registre/partis.json
sortie=data/identite/couleurs.json
[ -f "$registre" ] || { echo "::error::$registre absent" >&2; exit 1; }
mkdir -p "$(dirname "$sortie")"

python3 - "$registre" "$sortie" <<'PY'
import json, sys, time, urllib.request, hashlib

registre, sortie = sys.argv[1], sys.argv[2]
d = json.load(open(registre, encoding="utf-8"))

# L'identifiant Wikidata vient du registre, jamais devine.
cibles = []
for e in d["entites"]:
    for i in e.get("identifiants", []):
        if i.get("source") == "wikidata" and i.get("valeur"):
            cibles.append((e["id"], e["nom"], i["valeur"]))
            break

# Wikimedia refuse l'agent par defaut de Python — 403. Leur politique exige un
# agent identifiable, avec un moyen de joindre le responsable.
AGENT = "Contrepoint/0.1 (https://github.com/bourbask/contrepoint) python-urllib"

def p465(qid):
    url = ("https://www.wikidata.org/w/api.php?action=wbgetclaims"
           f"&entity={qid}&property=P465&format=json")
    requete = urllib.request.Request(url, headers={"User-Agent": AGENT})
    with urllib.request.urlopen(requete, timeout=30) as r:
        c = json.load(r).get("claims", {}).get("P465", [])
    return c[0]["mainsnak"]["datavalue"]["value"] if c else None

entrees, sans = [], []
for eid, nom, qid in cibles:
    v = p465(qid)
    time.sleep(0.3)                      # courtoisie envers un service gratuit
    if v is None:
        sans.append(eid)
        print(f"  sans couleur : {eid} ({qid})")
        continue
    entrees.append({"entite": eid, "wikidata": qid, "srgb": f"#{v.upper()}"})
    print(f"  {eid:26s} {qid:12s} #{v.upper()}")

entrees.sort(key=lambda x: x["entite"])
doc = {
    "schema": "contrepoint/couleurs-identite/1",
    "source": "wikidata",
    "propriete": "P465",
    "licence": "CC0 1.0",
    "note": ("Couleur d'identite declaree par la source, jamais choisie ici. "
             "Presentation seule : aucune valeur mesuree n'en depend, et ce "
             "fichier n'entre pas dans la cle de deduplication du §3."),
    "recupere_le": time.strftime("%Y-%m-%d", time.gmtime()),
    "couleurs": entrees,
}
texte = json.dumps(doc, ensure_ascii=False, indent=1, sort_keys=False) + "\n"
open(sortie, "w", encoding="utf-8").write(texte)
print(f"\n{len(entrees)} couleurs, {len(sans)} sans : {sans}")
print("empreinte", hashlib.sha256(texte.encode()).hexdigest()[:16], "|", len(texte.encode()), "octets")
PY
