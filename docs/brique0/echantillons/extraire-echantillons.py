#!/usr/bin/env python3
"""Reconstruit les fichiers de docs/brique0/echantillons/ depuis les archives AN.

Aucun fichier de ce repertoire n'est ecrit a la main. Usage :

    python3 extraire-echantillons.py <racine_scrutins> <racine_amo30> <cible>

ou <racine_scrutins> contient json/VTANR5L17V*.json (Scrutins.json.zip decompresse)
et <racine_amo30> contient json/organe/ et json/acteur/ (AMO30...json.zip decompresse).

Les cinq scrutins sont copies **octet pour octet** : ce sont des extraits verbatim
de la source, pas des reductions. Les deux index de referentiel sont derives, et
portent un champ `_provenance` qui le dit.
"""
import json, os, shutil, sys

SCRUTINS = [
    # uid, ce que le fichier documente
    ("VTANR5L17V5268", "votant serialise en objet nu ; les trois causePositionVote ; parDelegation"),
    ("VTANR5L17V2767", "minorite vide (unanime) ; mise au point nominative"),
    ("VTANR5L17V6256", "organeRef 'PO0' sur tous les blocs de groupe"),
    ("VTANR5L17V156",  "scrutin ordinaire de reference, decomptes coherents"),
    ("VTANR5L17V1",    "premier scrutin de la legislature, motion de censure"),
]
# les 14 groupes politiques de la XVIIe legislature (AD inclus, absent des scrutins)
GROUPES = ["PO840056", "PO845520", "PO845454", "PO845425", "PO845439", "PO845407",
           "PO845514", "PO845470", "PO845413", "PO845485", "PO845401", "PO845419",
           "PO847173", "PO872880"]
# acteurs retenus pour l'index des mandats, chacun pour un cas de rattachement
ACTEURS = {
    "PA330240": "trois groupes successifs : AD -> UDR -> UDDPLR, plus doublons de mandat",
    "PA267285": "mandat GP duplique a periodes identiques",
    "PA642868": "retour dans le meme groupe apres passage NI, deux dateFin concurrentes",
    "PA642725": "la ventilation dit DR le 2024-10-22, le mandat AMO30 dit encore NI",
    "PA721908": "presidente de l'Assemblee : 7508 non-votants de cause PAN",
}
ONE = lambda x: [] if x is None else (x if isinstance(x, list) else [x])


def main(src_scr, src_amo, out):
    os.makedirs(out, exist_ok=True)
    for uid, _ in SCRUTINS:
        shutil.copyfile(os.path.join(src_scr, "json", uid + ".json"),
                        os.path.join(out, uid + ".json"))

    organes = []
    for uid in GROUPES:
        organes.append(json.load(open(os.path.join(src_amo, "json", "organe", uid + ".json"),
                                      encoding="utf-8"))["organe"])
    organes.sort(key=lambda o: (o["viMoDe"]["dateDebut"], o["libelleAbrev"]))
    write(os.path.join(out, "organes-groupes-l17.json"), {
        "_provenance": "derive de AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip"
                       " par docs/brique0/echantillons/extraire-echantillons.py ;"
                       " chaque element est l'objet `organe` verbatim",
        "organes": organes,
    })

    mandats = []
    for uid, motif in sorted(ACTEURS.items()):
        a = json.load(open(os.path.join(src_amo, "json", "acteur", uid + ".json"),
                           encoding="utf-8"))["acteur"]
        ident = a["etatCivil"]["ident"]
        mandats.append({
            "acteurRef": uid,
            "_cas": motif,
            "nom": ident["nom"], "prenom": ident["prenom"],
            "mandatsGP": sorted(
                ({"uid": m["uid"], "organeRef": m["organes"]["organeRef"],
                  "dateDebut": m["dateDebut"], "dateFin": m["dateFin"]}
                 for m in ONE(a["mandats"]["mandat"])
                 if m["typeOrgane"] == "GP" and m.get("legislature") == "17"),
                key=lambda m: (m["dateDebut"], m["uid"])),
            "mandatsAssemblee": sorted(
                ({"uid": m["uid"], "organeRef": m["organes"]["organeRef"],
                  "dateDebut": m["dateDebut"], "dateFin": m["dateFin"]}
                 for m in ONE(a["mandats"]["mandat"])
                 if m["typeOrgane"] == "ASSEMBLEE" and m.get("legislature") == "17"),
                key=lambda m: (m["dateDebut"], m["uid"])),
        })
    write(os.path.join(out, "mandats-gp-l17.json"), {
        "_provenance": "derive de AMO30... par docs/brique0/echantillons/extraire-echantillons.py ;"
                       " seuls les mandats typeOrgane=GP et typeOrgane=ASSEMBLEE de la"
                       " legislature 17 sont conserves",
        "acteurs": mandats,
    })


def write(path, obj):
    with open(path, "w", encoding="utf-8") as f:
        json.dump(obj, f, ensure_ascii=False, indent=2, sort_keys=False)
        f.write("\n")


if __name__ == "__main__":
    main(*sys.argv[1:4])
