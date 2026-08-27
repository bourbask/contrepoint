"""Construit les artefacts d'exemple du front.

Source des valeurs, et rien d'autre :
  - les cinq lignes de preuve reelles de docs/brique0/contrats.md §2.6,
    reprises telles quelles, octet pour octet (I16) ;
  - les valeurs par groupe de docs/brique0/positionnement.md §6 ;
  - les deux valeurs d'exemple de contrats.md §4.2 (experts et nuance de LFI) ;
  - la regle de construction des bandes de contrats.md §4.3, appliquee sur
    data/registre/partis.json.

Les identifiants sont recalcules par la regle de deduplication du §3 : le
script reproduit les cinq identifiants publies avant d'en produire d'autres,
et le test `contrat.test.ts` refait ce calcul sur chaque ligne livree.

Ces artefacts servent au developpement et aux tests. En production le front lit
`public/api/`, produit par le pipeline.

    python3 web/src/fixtures/construire.py
"""
import hashlib, json, re, os, shutil, pathlib

W = str(pathlib.Path(__file__).resolve().parents[3])
OUT = f"{W}/web/src/fixtures"
CON = open(f"{W}/docs/brique0/contrats.md", encoding="utf-8").read()

bloc = CON[CON.index("### 2.6"):CON.index("### 2.7")]
textes_reels = re.findall(r'^\{"schema":"contrepoint/preuve/1".*\}$', bloc, re.M)
reelles = {json.loads(t)["id"]: t for t in textes_reels}
par_cle = {}
for t in textes_reels:
    l = json.loads(t)
    par_cle[(l["famille"], l["entite"])] = (t, l)

def canonique(o):
    return json.dumps(o, sort_keys=True, separators=(",", ":"), ensure_ascii=False)

def compact(o):
    return json.dumps(o, separators=(",", ":"), ensure_ascii=False)

def identifiant(l):
    cle = "\x1f".join([
        l["famille"], l["entite"], l["observation"]["debut"], l["observation"]["fin"],
        l["methode"]["id"], l["methode"]["version"], canonique(l["methode"]["parametres"]),
        ",".join(sorted(e["empreinte_contenu_sha256"] for e in l["entrees"])),
    ])
    return hashlib.sha256(cle.encode("utf-8")).hexdigest()

ORDRE = ["schema","id","contrat","famille","entite","valeur","valeur_code","echelle",
         "motif_code","motif","dispersion","observation","date_source","date_calcul",
         "methode","epingles","entrees","logiciel"]

def ligne(modele, **maj):
    """Une ligne calquee sur une ligne reelle, dans l'ordre de cles impose (§7)."""
    l = json.loads(json.dumps(modele))
    l.update(maj)
    l["id"] = identifiant(l)
    return {k: l[k] for k in ORDRE}

_, MOD_VOTES = par_cle[("votes", "groupe.an17.lfi-nfp")]
_, MOD_EXPERTS = par_cle[("experts", "parti.rn")]
_, MOD_ADMIN = par_cle[("administratif", "coalition.nfp")]

# positionnement.md §6 : effectif, mediane ancree, IQR, ecart-type de reechantillonnage.
# La regle de non-publication du meme §6 : IQR <= 0,25, ecart-type <= 0,05, effectif >= 10.
GROUPES = [
    ("groupe.an17.lfi-nfp", "LFI-NFP",  73, -1.0,     0.047, 0.0),
    ("groupe.an17.ecos",    "ECOS",     38, -0.9876,  0.033, 0.0061),
    ("groupe.an17.gdr",     "GDR",      18, -0.8619,  0.083, 0.0109),
    ("groupe.an17.soc",     "SOC",      70, -0.8352,  0.057, 0.0098),
    ("groupe.an17.liot",    "LIOT",     25,  0.1435,  0.687, 0.0176),
    ("groupe.an17.dem",     "DEM",      41,  0.1814,  0.205, 0.0158),
    ("groupe.an17.epr",     "EPR",     115,  0.2408,  0.226, 0.0207),
    ("groupe.an17.ni",      "NI",        9,  0.2664,  0.623, 0.0438),
    ("groupe.an17.hor",     "HOR",      43,  0.3837,  0.200, 0.0151),
    ("groupe.an17.dr",      "DR",       63,  0.7013,  0.113, 0.0123),
    ("groupe.an17.uddplr",  "UDDPLR",   18,  0.9900,  0.042, 0.0060),
    ("groupe.an17.rn",      "RN",      129,  1.0,     0.052, 0.0),
]

# Regle de construction des bandes, contrats.md §4.3, appliquee sur
# data/registre/partis.json : composition designant exactement un parti que
# nul autre groupe valide ne designe -> bande du parti ; sinon bande du groupe.
BANDE = {
    "groupe.an17.lfi-nfp": ("parti.lfi",      "La France insoumise"),
    "groupe.an17.ecos":    ("groupe.an17.ecos", "Écologiste et Social"),
    "groupe.an17.gdr":     ("groupe.an17.gdr",  "Gauche Démocrate et Républicaine"),
    "groupe.an17.soc":     ("parti.ps",       "Parti socialiste"),
    "groupe.an17.liot":    ("groupe.an17.liot", "LIOT"),
    "groupe.an17.dem":     ("groupe.an17.dem",  "Les Démocrates"),
    "groupe.an17.epr":     ("groupe.an17.epr",  "Ensemble pour la République"),
    "groupe.an17.ni":      ("groupe.an17.ni",   "Non inscrit"),
    "groupe.an17.hor":     ("parti.horizons", "Horizons"),
    "groupe.an17.dr":      ("parti.lr",       "Les Républicains"),
    "groupe.an17.uddplr":  ("parti.udr",      "Union des droites pour la République"),
    "groupe.an17.rn":      ("parti.rn",       "Rassemblement national"),
}

lignes = {}          # id -> texte de la ligne
marqueurs = {}       # bande_id -> [(famille, marqueur)]
libelles = {}

def poser(bande_id, libelle, marqueur):
    libelles[bande_id] = libelle
    marqueurs.setdefault(bande_id, []).append(marqueur)

def enregistrer(l):
    texte = reelles.get(l["id"]) or compact(l)
    lignes[l["id"]] = texte
    return l["id"]

for gid, sigle, n, mediane, iqr, ect in GROUPES:
    publiable = iqr <= 0.25 and ect <= 0.05 and n >= 10
    motifs = []
    if iqr > 0.25:
        motifs.append(f"IQR {iqr:.3f} pour un maximum de 0,25".replace(".", ","))
    if n < 10:
        motifs.append(f"effectif retenu {n} pour un minimum de 10")
    l = ligne(MOD_VOTES, entite=gid,
              valeur=mediane if publiable else None,
              motif_code=None if publiable else "sous_seuil_de_publication",
              motif=None if publiable else
                    "Dispersion interne au-delà du seuil publié : " + ", et ".join(motifs) + ".",
              dispersion={"effectif": n, "iqr": iqr, "ecart_type_reechantillonnage": ect})
    bande_id, libelle = BANDE[gid]
    poser(bande_id, libelle, {
        "famille": "votes", "echelle": "votes_an17_ancre_v1",
        "valeur": mediane if publiable else None, "valeur_code": None,
        "libelle": f"Votes du groupe {sigle}",
        "motif_code": l["motif_code"], "motif": l["motif"],
        "dispersion": {"effectif": n, "iqr": iqr},
        "preuve": enregistrer(l), "_dec": 4})

# Famille experts : les deux seules valeurs ecrites dans le depot
# (contrats.md §2.6 pour parti.rn, §4.2 pour parti.lfi).
for entite, valeur, libelle_bande in [("parti.lfi", 0.82, "La France insoumise"),
                                      ("parti.rn", 8.82, "Rassemblement national")]:
    l = ligne(MOD_EXPERTS, entite=entite, valeur=valeur)
    poser(entite, libelle_bande, {
        "famille": "experts", "echelle": "ches_lrgen_0_10",
        "valeur": valeur, "valeur_code": None, "libelle": "CHES 2024, lrgen",
        "motif_code": None, "motif": None, "dispersion": None,
        "preuve": enregistrer(l), "_dec": 2})

# Famille administrative : coalition.nfp = UG (§2.6), parti.lfi = FI (§4.2).
for entite, code, libelle_bande in [("coalition.nfp", "UG", "Nouveau Front populaire"),
                                    ("parti.lfi", "FI", "La France insoumise")]:
    l = ligne(MOD_ADMIN, entite=entite, valeur_code=code)
    poser(entite, libelle_bande, {
        "famille": "administratif", "echelle": "nuance_leg2024",
        "valeur": None, "valeur_code": code, "libelle": "Nuance 2024",
        "motif_code": None, "motif": None, "dispersion": None,
        "preuve": enregistrer(l), "_dec": None})

# --- Assemblage de l'instantane -------------------------------------------
FAM = ["votes", "experts", "administratif"]

def cle_bande(bid):
    v = next((m["valeur"] for m in marqueurs[bid]
              if m["famille"] == "votes" and m["valeur"] is not None), None)
    return (1, 0.0, bid) if v is None else (0, v, bid)

class Brut(float):
    """Un nombre dont le rendu JSON est fige (§7 : decimales de l'echelle)."""
    def __new__(cls, texte):
        o = super().__new__(cls, texte); o.texte = texte; return o

def rendre(o, ind=0):
    if isinstance(o, Brut):  return o.texte
    if o is None:            return "null"
    if isinstance(o, bool):  return "true" if o else "false"
    if isinstance(o, (int, float)): return json.dumps(o)
    if isinstance(o, str):   return json.dumps(o, ensure_ascii=False)
    if isinstance(o, list):  return "[" + ",".join(rendre(x) for x in o) + "]"
    return "{" + ",".join(f"{json.dumps(k,ensure_ascii=False)}:{rendre(v)}" for k, v in o.items()) + "}"

def dec(v, n):
    return None if v is None else Brut(f"{v:.{n}f}")

bandes = []
for bid in sorted(marqueurs, key=cle_bande):
    ms = sorted(marqueurs[bid], key=lambda m: FAM.index(m["famille"]))
    sortie = []
    for m in ms:
        n = m.pop("_dec")
        d = m["dispersion"]
        sortie.append({
            "famille": m["famille"], "echelle": m["echelle"],
            "valeur": dec(m["valeur"], n) if n else m["valeur"],
            "valeur_code": m["valeur_code"], "libelle": m["libelle"],
            "motif_code": m["motif_code"], "motif": m["motif"],
            "dispersion": None if d is None else {"effectif": d["effectif"], "iqr": dec(d["iqr"], 4)},
            "preuve": m["preuve"]})
    bandes.append({"id": bid, "libelle": libelles[bid], "marqueurs": sortie})

instantane = {
    "schema": "contrepoint/instantane/1", "contrat": "0.3.0", "id": "an17-2026-07-21",
    "chambre": "AN", "legislature": "17", "date": "2026-07-21",
    "date_arretee": "2026-08-27T00:00:00Z",
    "ancrage": {"famille": "votes", "ancre_gauche": "groupe.an17.lfi-nfp",
                "ancre_droite": "groupe.an17.rn",
                "note": "Médiane du groupe LFI-NFP à −1,0000, médiane du groupe RN à +1,0000, au 2026-07-21."},
    "bandes": bandes,
    "sans_mesure": [{"entite": "parti.place-publique", "libelle": "Place publique",
                     "motif_code": "hors_source",
                     "motif": "Entité absente des quatre sources d'identifiants et de la grille de nuances, au 2026-08-27."}],
}

texte_inst = rendre(instantane) + "\n"
octets = len(texte_inst.encode("utf-8"))
empreinte = hashlib.sha256(texte_inst.encode("utf-8")).hexdigest()

manifeste = {
    "schema": "contrepoint/manifeste/1", "contrat": "0.3.0",
    "schemas": ["contrepoint/preuve/1", "contrepoint/instantane/1", "contrepoint/eclat-preuves/1"],
    "date_arretee": "2026-08-27T00:00:00Z",
    "licence": "Licence Ouverte / Open Licence (Etalab)",
    "mention_paternite": "Assemblée nationale — Licence Ouverte v1.0 — données du 2026-08-27",
    "familles": [
        {"id": "votes", "libelle": "Votes nominatifs", "echelle": "votes_an17_ancre_v1"},
        {"id": "experts", "libelle": "Enquête d'experts", "echelle": "ches_lrgen_0_10"},
        {"id": "administratif", "libelle": "Nuance administrative", "echelle": "nuance_leg2024"}],
    "instantanes": [{"id": "an17-2026-07-21", "chambre": "AN", "legislature": "17",
                     "date": "2026-07-21", "url": "instantanes/an17-2026-07-21.json",
                     "empreinte_sha256": empreinte, "octets": octets, "bandes": len(bandes)}],
    "preuves": {"racine": "preuves/", "eclats": 256,
                "fonction": "deux premiers caractères hexadécimaux de l'id"},
}

# Le rendu fige de EXP-07 vit dans le meme dossier : seuls les artefacts
# produits ici sont effaces.
shutil.rmtree(f"{OUT}/instantanes", ignore_errors=True)
shutil.rmtree(f"{OUT}/preuves", ignore_errors=True)
os.makedirs(f"{OUT}/instantanes"); os.makedirs(f"{OUT}/preuves")
open(f"{OUT}/index.json", "w", encoding="utf-8").write(rendre(manifeste) + "\n")
open(f"{OUT}/instantanes/an17-2026-07-21.json", "w", encoding="utf-8").write(texte_inst)

eclats = {}
for i, t in sorted(lignes.items()):
    eclats.setdefault(i[:2], []).append(t)
for prefixe, textes in sorted(eclats.items()):
    open(f"{OUT}/preuves/{prefixe}.json", "w", encoding="utf-8").write("[" + ",".join(textes) + "]\n")

print(f"{len(bandes)} bandes, {sum(len(b['marqueurs']) for b in bandes)} marqueurs, "
      f"{len(lignes)} lignes de preuve, {len(eclats)} éclats")
print("instantané:", octets, "octets, empreinte", empreinte[:16])
print("lignes reelles du §2.6 reprises telles quelles:",
      sum(1 for i in lignes if i in reelles), "/ 5")
