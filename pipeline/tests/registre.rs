//! REG-01 à REG-22 — le registre d'entités et ses 25 règles de validation
//! (`docs/brique0/registre-entites.md` §6).
//!
//! Le registre est le seul fichier du projet édité à la main : une erreur s'y
//! propage dans toutes les mesures. Chaque test porte **une** règle, et chaque
//! variante fautive est construite par mutation du registre réel — un cas
//! fautif écrit de zéro échouerait pour dix raisons et n'en démontrerait
//! aucune.

use contrepoint::estimateur::ancres;
use contrepoint::registre::{CLES, canoniser, confronter, valider, valider_texte};
use serde_json::{Value, json};

fn texte_reel() -> String {
    let chemin =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/registre/partis.json");
    std::fs::read_to_string(&chemin).unwrap_or_else(|e| panic!("{} : {e}", chemin.display()))
}

fn reel() -> Value {
    serde_json::from_str(&texte_reel()).expect("registre réel conforme au JSON")
}

fn fixture() -> Value {
    let chemin = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../docs/brique0/echantillons/registre-l17.json");
    serde_json::from_str(&std::fs::read_to_string(&chemin).expect("fixture lisible"))
        .expect("fixture conforme au JSON")
}

/// Le refus attendu est nommé : un test qui se contente de « ça refuse » passe
/// aussi quand une autre règle a mordu, et la règle visée peut être morte.
fn refuse_par(registre: &Value, regle: &str) {
    let refus = valider(registre);
    assert!(
        refus.iter().any(|r| r.starts_with(regle)),
        "{regle} devait refuser ; refus observés : {refus:?}"
    );
}

fn accepte(registre: &Value) {
    let refus = valider(registre);
    assert!(
        refus.is_empty(),
        "registre refusé sans motif attendu : {refus:?}"
    );
}

/// Le groupe d'indice donné, muté en place.
fn groupe<'a>(registre: &'a mut Value, id: &str) -> &'a mut Value {
    registre["groupes"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|g| g["id"] == id)
        .unwrap_or_else(|| panic!("groupe {id} absent"))
}

fn entite<'a>(registre: &'a mut Value, id: &str) -> &'a mut Value {
    registre["entites"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|e| e["id"] == id)
        .unwrap_or_else(|| panic!("entité {id} absente"))
}

// ---------------------------------------------------------------- REG-01 ----

#[test]
fn extrait_de_reference_valide() {
    // REG-01 : sans un cas vert, la suite de refus passerait aussi sur un
    // validateur qui refuse tout.
    accepte(&reel());
    accepte(&fixture());
    assert!(
        valider_texte(&texte_reel()).is_ok(),
        "le registre réel doit passer les contrôles de forme du fichier"
    );
}

#[test]
fn liste_blanche_transcrit_le_schema_publie() {
    // REG-01, second volet : V1 est transcrite dans
    // `schemas/registre-partis-1.schema.json`. Si le validateur et le schéma
    // divergent, l'un des deux ment, et le fichier passe l'un en violant
    // l'autre. Le contrôle est mécanique.
    let schema: Value = serde_json::from_str(
        &std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../schemas/registre-partis-1.schema.json"),
        )
        .expect("schéma lisible"),
    )
    .expect("schéma conforme au JSON");

    // `required` et non `properties` : un objet JSON relu perd son ordre de
    // déclaration, un tableau le garde. Les deux listent les mêmes clés — le
    // schéma n'a aucune propriété facultative — et seule `required` permet de
    // vérifier aussi l'ordre, qui est ce que la forme canonique de V23 impose.
    let cles_de = |noeud: &Value| -> Vec<String> {
        noeud["required"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|c| c.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    };
    let attendu = [
        ("racine", cles_de(&schema)),
        (
            "legislatures",
            cles_de(&schema["properties"]["legislatures"]["items"]),
        ),
        (
            "sources",
            cles_de(&schema["properties"]["sources"]["items"]),
        ),
        (
            "entites",
            cles_de(&schema["properties"]["entites"]["items"]),
        ),
        (
            "groupes",
            cles_de(&schema["properties"]["groupes"]["items"]),
        ),
        (
            "relations",
            cles_de(&schema["properties"]["relations"]["items"]),
        ),
        ("identifiants", cles_de(&schema["$defs"]["identifiant"])),
        ("composition", cles_de(&schema["$defs"]["composition"])),
        ("ancre_axe", cles_de(&schema["$defs"]["ancre_axe"])),
    ];
    for (bloc, cles_schema) in attendu {
        let cles_code: Vec<String> = CLES
            .iter()
            .find(|(nom, _)| *nom == bloc)
            .unwrap_or_else(|| panic!("bloc {bloc} absent de la liste blanche du validateur"))
            .1
            .iter()
            .map(|c| (*c).to_owned())
            .collect();
        assert_eq!(
            cles_code, cles_schema,
            "bloc {bloc} : la liste blanche du validateur diverge du schéma publié"
        );
    }
}

// ---------------------------------------------------------------- REG-02 ----

#[test]
fn cle_inconnue_refusee() {
    // REG-02 [C] — V1. Le champ de valorisation qui apparaît en silence est le
    // défaut que la liste blanche existe pour attraper.
    let mut r = reel();
    entite(&mut r, "parti.rn")["score"] = json!(0.42);
    refuse_par(&r, "V1");

    let mut r = reel();
    r["axe_de_synthese"] = json!(1);
    refuse_par(&r, "V1");
}

// ---------------------------------------------------------------- REG-03 ----

#[test]
fn schema_litteral() {
    // REG-03 — V2.
    let mut r = reel();
    r["schema"] = json!("contrepoint/registre-partis/2");
    refuse_par(&r, "V2");
}

// ---------------------------------------------------------------- REG-04 ----

#[test]
fn date_reelle() {
    // REG-04 — V3. Une date impossible passe le motif et casse toute
    // comparaison de période en aval.
    let mut r = reel();
    r["date_registre"] = json!("2026-02-30");
    refuse_par(&r, "V3");

    let mut r = reel();
    groupe(&mut r, "groupe.an17.rn")["debut"] = json!("2024-13-01");
    refuse_par(&r, "V3");
}

// ---------------------------------------------------------------- REG-05 ----

#[test]
fn id_unique_et_prefixe_coherent() {
    // REG-05 — V4.
    let mut r = reel();
    entite(&mut r, "parti.rn")["id"] = json!("coalition.x");
    refuse_par(&r, "V4");

    let mut r = reel();
    entite(&mut r, "parti.rn")["id"] = json!("parti.ps");
    refuse_par(&r, "V4");
}

// ---------------------------------------------------------------- REG-06 ----

#[test]
fn reference_pendante_refusee() {
    // REG-06 — V5 et V6, dans les deux sens.
    let mut r = reel();
    groupe(&mut r, "groupe.an17.rn")["composition"][0]["entite"] = json!("parti.inexistant");
    refuse_par(&r, "V5");

    let mut r = reel();
    r["sources"].as_array_mut().unwrap().push(json!({
        "id": "source_jamais_citee",
        "libelle": "Source déclarée et jamais employée",
        "url": null,
        "recupere_le": "2026-08-27",
        "empreinte_sha256": null,
        "cardinalite": "un_par_date",
        "licence": "s.o.",
        "remarque": null
    }));
    refuse_par(&r, "V6");
}

// ---------------------------------------------------------------- REG-07 ----

#[test]
fn injectivite_par_source_et_par_date() {
    // REG-07 [C] — V7. L'erreur fatale du projet : deux partis appariés au même
    // identifiant externe. Rien en aval ne peut la détecter.
    let mut r = reel();
    for ident in entite(&mut r, "parti.ps")["identifiants"]
        .as_array_mut()
        .unwrap()
    {
        if ident["source"] == "ches_2024" {
            ident["valeur"] = json!("610"); // celui du RN
        }
    }
    refuse_par(&r, "V7");
}

// ---------------------------------------------------------------- REG-08 ----

#[test]
fn cardinalite_un_par_date() {
    // REG-08 — V8. Les deux lignes `an_organe` du RN passent parce que leurs
    // périodes sont disjointes ; les mêmes qui se chevauchent sont un refus.
    accepte(&reel());

    let mut r = reel();
    for ident in entite(&mut r, "parti.rn")["identifiants"]
        .as_array_mut()
        .unwrap()
    {
        if ident["valeur"] == "PO684946" {
            ident["fin"] = json!(null);
        }
    }
    refuse_par(&r, "V8");
}

// ---------------------------------------------------------------- REG-09 ----

#[test]
fn valeur_nulle_exige_un_motif() {
    // REG-09 [C] — V9. L'absence est dite, jamais comblée.
    let mut r = reel();
    for ident in entite(&mut r, "parti.place-publique")["identifiants"]
        .as_array_mut()
        .unwrap()
    {
        if ident["valeur"].is_null() {
            ident["motif"] = json!(null);
        }
    }
    refuse_par(&r, "V9");

    let mut r = reel();
    for ident in entite(&mut r, "parti.rn")["identifiants"]
        .as_array_mut()
        .unwrap()
    {
        if ident["source"] == "ches_2024" {
            ident["motif"] = json!("Un motif sur une valeur renseignée.");
        }
    }
    refuse_par(&r, "V9");
}

// ---------------------------------------------------------------- REG-10 ----

#[test]
fn etabli_le_present_et_pas_dans_le_futur() {
    // REG-10 — V10.
    let mut r = reel();
    groupe(&mut r, "groupe.an17.rn")["ancre_axe"]["etabli_le"] = json!("2099-01-01");
    refuse_par(&r, "V10");
}

// ---------------------------------------------------------------- REG-11 ----

#[test]
fn bornes_ordonnees_et_incluses() {
    // REG-11 — V11, V12, V13.
    let mut r = reel();
    groupe(&mut r, "groupe.an17.udr")["debut"] = json!("2025-09-04");
    groupe(&mut r, "groupe.an17.udr")["fin"] = json!("2024-09-12");
    refuse_par(&r, "V11");

    let mut r = reel();
    groupe(&mut r, "groupe.an17.rn")["composition"][0]["debut"] = json!("2024-07-01");
    refuse_par(&r, "V12");

    let mut r = reel();
    groupe(&mut r, "groupe.an17.rn")["debut"] = json!("2024-07-01");
    refuse_par(&r, "V13");
}

#[test]
fn exception_v13_nommee_sur_uid() {
    // REG-11b — V13. `PO840056` ouvre le 2024-07-01, dix-sept jours avant
    // l'ouverture de la XVIIe : l'exception porte sur l'`uid_an`, jamais sur un
    // libellé. Sans le second cas, l'exception se généralise et V13 ne dit plus
    // rien ; sans le premier, aucun registre conforme n'est constructible.
    accepte(&reel());

    let mut r = reel();
    // Le même écart, sur un autre organe : refusé.
    groupe(&mut r, "groupe.an17.rn")["debut"] = json!("2024-07-01");
    groupe(&mut r, "groupe.an17.rn")["composition"][0]["debut"] = json!("2024-07-01");
    refuse_par(&r, "V13");

    // L'exception ne se transporte pas avec le sigle : renommer NI en RN ne
    // dispense pas RN de V13.
    let mut r = reel();
    groupe(&mut r, "groupe.an17.rn")["sigle"] = json!("NI");
    groupe(&mut r, "groupe.an17.rn")["debut"] = json!("2024-07-01");
    groupe(&mut r, "groupe.an17.rn")["composition"][0]["debut"] = json!("2024-07-01");
    refuse_par(&r, "V13");
}

// ---------------------------------------------------------------- REG-12 ----

#[test]
fn aucun_chevauchement() {
    // REG-12 [C] — V14. Deux périodes du même couple (porteur, source, valeur)
    // qui se chevauchent rendent la jointure par date non déterministe.
    let mut r = reel();
    let identifiants = entite(&mut r, "parti.lr")["identifiants"]
        .as_array_mut()
        .unwrap();
    let ligne = identifiants
        .iter()
        .find(|i| i["source"] == "ches_2024")
        .cloned()
        .unwrap();
    identifiants.push(ligne);
    refuse_par(&r, "V14");
}

// ---------------------------------------------------------------- REG-13 ----

#[test]
fn groupe_egal_a_la_source() {
    // REG-13 [C] — V15 et V16. C'est ce qui rend le registre falsifiable contre
    // sa source : une divergence est soit une source qui a bougé, soit une main
    // qui a édité.
    let organes = organes_l17();
    assert!(
        confronter(&reel(), &organes).is_empty(),
        "le registre réel doit être égal à sa source : {:?}",
        confronter(&reel(), &organes)
    );

    let mut r = reel();
    groupe(&mut r, "groupe.an17.rn")["nom"] = json!("Rassemblement national");
    let refus = confronter(&r, &organes);
    assert!(refus.iter().any(|x| x.starts_with("V16")), "{refus:?}");

    let mut r = reel();
    groupe(&mut r, "groupe.an17.rn")["debut"] = json!("2024-07-19");
    assert!(
        confronter(&r, &organes)
            .iter()
            .any(|x| x.starts_with("V16"))
    );

    let mut r = reel();
    groupe(&mut r, "groupe.an17.rn")["uid_an"] = json!("PO999999");
    assert!(
        confronter(&r, &organes)
            .iter()
            .any(|x| x.starts_with("V15"))
    );
}

fn organes_l17() -> Vec<Value> {
    let chemin = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../docs/brique0/echantillons/organes-groupes-l17.json");
    let fichier: Value =
        serde_json::from_str(&std::fs::read_to_string(&chemin).expect("échantillon lisible"))
            .expect("échantillon conforme au JSON");
    fichier["organes"]
        .as_array()
        .expect("tableau `organes`")
        .clone()
}

// ---------------------------------------------------------------- REG-14 ----

#[test]
fn sigle_nest_pas_une_cle() {
    // REG-14 — `PO872880` porte `libelleAbrev` UDDPLR et `libelleAbrege` UDR.
    // Deux entités distinctes portent la même abréviation dans la source : une
    // clé sur l'abréviation les fusionne.
    let organes = organes_l17();
    let uddplr = organes
        .iter()
        .find(|o| o["uid"] == "PO872880")
        .expect("PO872880 dans l'échantillon");
    assert_eq!(uddplr["libelleAbrev"], "UDDPLR");
    assert_eq!(uddplr["libelleAbrege"], "UDR");

    // La confrontation se fait sur `uid_an`, jamais sur le sigle : un sigle
    // dupliqué entre deux groupes ne fusionne rien et ne déclenche V15 ni V16.
    let mut r = reel();
    groupe(&mut r, "groupe.an17.uddplr")["sigle"] = json!("UDR");
    let refus = confronter(&r, &organes);
    assert!(
        refus.iter().any(|x| x.starts_with("V16")),
        "le sigle est recopié de libelleAbrev, pas de libelleAbrege : {refus:?}"
    );
}

// ---------------------------------------------------------------- REG-15 ----

#[test]
fn succession_relie_deux_periodes_contigues() {
    // REG-15 — V18. `organePrecedentRef` est nul sur les 63 GP : la chaîne de
    // succession est déclarée à la main, donc fausse si rien ne la contrôle.
    accepte(&reel()); // AD → UDR (2024-09-11 / 2024-09-12) passe.

    let mut r = reel();
    for relation in r["relations"].as_array_mut().unwrap() {
        if relation["type"] == "succession_groupe" {
            relation["date"] = json!("2024-09-20");
        }
    }
    refuse_par(&r, "V18");
}

// ---------------------------------------------------------------- REG-16 ----

#[test]
fn lexique_interdit_absent_cles_comprises() {
    // REG-16 [C] — V20. Le grep de la definition of done porte sur le diff ;
    // ce test porte sur le fichier entier.
    let mut r = reel();
    let proscrit = ["cre", "dibilite"].concat(); // terme proscrit, jamais écrit en clair
    entite(&mut r, "parti.rn")["remarque"] = json!(format!("Axe de {proscrit} du parti."));
    refuse_par(&r, "V20");

    let mut r = reel();
    let cible = entite(&mut r, "parti.rn").as_object_mut().unwrap();
    cible.insert(["vera", "cite"].concat(), json!(1)); // clé proscrite, jamais écrite en clair
    refuse_par(&r, "V20");
}

// ---------------------------------------------------------------- REG-17 ----

#[test]
fn longueurs_maximales() {
    // REG-17 — V21. Le graphe est dimensionné sur ces limites.
    let mut r = reel();
    entite(&mut r, "parti.rn")["sigle"] = json!("R".repeat(41));
    refuse_par(&r, "V21");

    let mut r = reel();
    groupe(&mut r, "groupe.an17.rn")["remarque"] = json!("R".repeat(141));
    refuse_par(&r, "V21");
}

// ---------------------------------------------------------------- REG-18 ----

#[test]
fn aucun_champ_de_valorisation_ni_nom_de_personne() {
    // REG-18 [C] — V22. Le registre ne contient aucune personne.
    let mut r = reel();
    let cible = entite(&mut r, "parti.rn").as_object_mut().unwrap();
    cible.insert("position".to_owned(), json!(0.82));
    refuse_par(&r, "V22");

    let mut r = reel();
    for ident in entite(&mut r, "parti.rn")["identifiants"]
        .as_array_mut()
        .unwrap()
    {
        if ident["source"] == "an_organe" {
            ident["valeur"] = json!("PA793290");
        }
    }
    refuse_par(&r, "V22");
}

// ---------------------------------------------------------------- REG-19 ----

#[test]
fn forme_canonique_idempotente() {
    // REG-19 [P] — V23. Une main qui édite sans passer par le formateur rend le
    // diff illisible, ce qui tue la relecture ligne par ligne.
    let texte = texte_reel();
    let une = canoniser(&reel());
    assert_eq!(
        une, texte,
        "le fichier commité n'est pas sa propre forme canonique"
    );
    let deux = canoniser(&serde_json::from_str(&une).unwrap());
    assert_eq!(une, deux, "format ∘ format ≠ format");

    for (nom, fautif) in [
        ("BOM", format!("\u{feff}{texte}")),
        ("CRLF", texte.replace('\n', "\r\n")),
        ("sans fin de ligne finale", texte.trim_end().to_owned()),
        ("indenté à 4 espaces", texte.replace("\n  ", "\n    ")),
    ] {
        assert!(
            valider_texte(&fautif).is_err(),
            "V23 devait refuser un fichier {nom}"
        );
    }

    // Réordonné : `sources` triée autrement que par `id`.
    let mut r = reel();
    r["sources"].as_array_mut().unwrap().reverse();
    refuse_par(&r, "V23");
}

// ---------------------------------------------------------------- REG-20 ----

#[test]
fn famille_absente_jamais_remplie_par_une_autre() {
    // REG-20 [C] — une famille de mesure absente ne se remplit pas avec une
    // autre. La violer ici est indétectable en aval : la valeur arrive dans le
    // graphe comme une mesure.
    let r = reel();
    let udr = r["entites"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["id"] == "parti.udr")
        .unwrap();
    let ches = udr["identifiants"]
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["source"] == "ches_2024")
        .expect("ligne ches_2024");
    let nuance = udr["identifiants"]
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["source"] == "nuance_leg2024")
        .expect("ligne nuance_leg2024");
    assert!(ches["valeur"].is_null() && nuance["valeur"].is_null());
    assert_ne!(
        ches["motif"], nuance["motif"],
        "deux absences de causes différentes portent deux motifs différents"
    );

    // Reprendre la valeur de la famille voisine est un refus : V7 attrape le
    // report d'un identifiant d'une entité sur une autre, et rien ne rattrape
    // un report entre sources — c'est pourquoi le motif est obligatoire.
    let mut r = reel();
    for ident in entite(&mut r, "parti.udr")["identifiants"]
        .as_array_mut()
        .unwrap()
    {
        if ident["source"] == "nuance_leg2024" {
            ident["valeur"] = json!("LR"); // celui de parti.lr
        }
    }
    refuse_par(&r, "V7");
}

// ---------------------------------------------------------------- REG-21 ----

#[test]
fn au_plus_une_ancre_par_pole_et_par_date() {
    // REG-21 [C] — V24. Deux ancres du même pôle à la même date rendent la
    // transformation d'ancrage non définie : le pipeline choisirait selon
    // l'ordre de lecture du fichier.
    let mut r = reel();
    groupe(&mut r, "groupe.an17.soc")["ancre_axe"] = json!({
        "pole": "gauche",
        "debut": "2024-07-18",
        "fin": null,
        "etabli_le": "2026-08-27",
        "remarque": "Seconde ancre gauche, sur une période qui chevauche la première."
    });
    refuse_par(&r, "V24");
    assert!(
        ancres(&r, "2026-07-21").is_err(),
        "deux ancres du pôle gauche : le calcul doit s'arrêter"
    );

    // Les deux mêmes sur périodes disjointes : accepté.
    let mut r = reel();
    groupe(&mut r, "groupe.an17.lfi-nfp")["ancre_axe"]["fin"] = json!("2025-01-01");
    groupe(&mut r, "groupe.an17.soc")["ancre_axe"] = json!({
        "pole": "gauche",
        "debut": "2025-01-02",
        "fin": null,
        "etabli_le": "2026-08-27",
        "remarque": "Ancre gauche à compter du 2025-01-02, période disjointe de la précédente."
    });
    accepte(&r);
}

// ---------------------------------------------------------------- REG-22 ----

#[test]
fn ancre_manquante_arrete_le_pipeline() {
    // REG-22 [C] — V25, RG-31. Une substitution silencieuse d'ancre change
    // l'échelle de toutes les positions publiées sans changer leur identifiant
    // d'échelle.
    let mut r = reel();
    groupe(&mut r, "groupe.an17.rn")["ancre_axe"]["fin"] = json!("2025-01-01");
    let erreur = ancres(&r, "2026-07-21").expect_err("l'ancre droite manque au 2026-07-21");
    assert!(
        erreur.contains("droite") && erreur.contains("RG-31"),
        "l'échec doit être bruyant et nommer le pôle manquant : {erreur}"
    );
    // Aucune ancre de remplacement : la gauche seule ne suffit pas.
    assert!(ancres(&r, "2026-07-21").is_err());
    // À une date couverte, les deux existent toujours.
    assert!(ancres(&r, "2024-12-31").is_ok());
}

#[test]
fn forme_de_fichier_refusee_avec_son_motif() {
    // V23 et V3, versant « message ». Un refus qui ne nomme pas sa règle ne
    // sert à personne : deux mutants survivaient parce que les tests se
    // contentaient d'un `is_err()`, qu'une autre règle rendait vrai.
    let texte = texte_reel();
    let refus = |fautif: &str| {
        valider_texte(fautif)
            .expect_err("refus attendu")
            .join(" ; ")
    };

    let avec_bom = refus(&format!("\u{feff}{texte}"));
    assert!(
        avec_bom.contains("marque d'ordre d'octets"),
        "le refus doit nommer le BOM : {avec_bom}"
    );
    let avec_crlf = refus(&texte.replace('\n', "\r\n"));
    assert!(
        avec_crlf.contains("CRLF"),
        "le refus doit nommer la fin de ligne : {avec_crlf}"
    );

    // V3 — une chaîne qui n'a pas la forme d'une date est refusée, jamais
    // acceptée et jamais fatale : un registre fautif se relit, il ne plante pas.
    for fautive in [
        "",
        "2026",
        "2026-8-1",
        "hier",
        "2026-08-27T00:00:00Z",
        "20260827",
    ] {
        assert!(
            !contrepoint::registre::date_reelle(fautive),
            "{fautive:?} n'est pas une date réelle"
        );
    }
    let mut r = reel();
    groupe(&mut r, "groupe.an17.rn")["debut"] = json!("2024-7-1");
    refuse_par(&r, "V3");
}

/// REG-23 [C] — l'échantillon de `docs/brique0/echantillons/registre-l17.json`
/// déclare les **mêmes** sources que `data/registre/partis.json` : même URL,
/// même empreinte, même remarque.
///
/// Ce qui casse si le test disparaît : l'échantillon est ce qu'un tiers lit
/// pour reproduire le travail. La bascule du nuancier vers le fichier régional
/// du 2nd tour — la correction RG-111, qui écarte un fichier portant des noms
/// de candidats — a été appliquée au registre et **pas** à l'échantillon, qui a
/// gardé pendant une fusion l'empreinte de la source nominative et un décompte
/// de 22 codes au lieu de 17. Aucun test ne comparait les deux : le défaut
/// était muet, et l'échantillon renvoyait vers le fichier écarté.
#[test]
fn echantillon_et_registre_declarent_les_memes_sources() {
    let racine = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let lire = |chemin: std::path::PathBuf| -> Value {
        serde_json::from_str(
            &std::fs::read_to_string(&chemin)
                .unwrap_or_else(|e| panic!("{} : {e}", chemin.display())),
        )
        .unwrap_or_else(|e| panic!("{} : {e}", chemin.display()))
    };
    let echantillon = lire(racine.join("docs/brique0/echantillons/registre-l17.json"));
    let reel = lire(racine.join("data/registre/partis.json"));

    let par_id = |r: &Value| -> std::collections::BTreeMap<String, Value> {
        r["sources"]
            .as_array()
            .expect("`sources` est un tableau")
            .iter()
            .map(|s| (s["id"].as_str().unwrap_or_default().to_owned(), s.clone()))
            .collect()
    };
    let (e, v) = (par_id(&echantillon), par_id(&reel));
    assert!(!e.is_empty(), "REG-23 : échantillon sans source");

    for (id, source) in &e {
        let Some(vraie) = v.get(id) else {
            panic!("REG-23 : la source « {id} » de l'échantillon n'existe pas dans le registre");
        };
        // `url`, `empreinte_sha256` et `remarque` sont ce qu'un reproducteur
        // suit. Une divergence l'envoie sur un autre fichier que le nôtre.
        for cle in ["url", "empreinte_sha256", "remarque", "licence"] {
            assert_eq!(
                source[cle], vraie[cle],
                "REG-23 : « {id} » diverge sur `{cle}` entre l'échantillon et le registre"
            );
        }
    }
}
