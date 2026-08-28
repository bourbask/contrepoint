//! EXP-01 à EXP-08 — le contrat d'export : manifeste, instantané, éclats de
//! preuves. Spécification : `docs/brique0/contrats.md` §4, §6 et §7.
//!
//! Les trois artefacts sont des **projections** du registre de preuves et ne
//! portent aucune valeur qui ne soit d'abord une ligne du registre : un
//! marqueur sans ligne ne s'affiche pas.
//!
//! EXP-08 — `aucune_couleur_seule_porteuse_dinformation` — porte sur le rendu
//! du graphe : forme et `aria-label` distincts par marqueur. Il n'est pas
//! écrivable ici, le pipeline n'ayant aucun rendu ; sa suite est
//! `web/src/contrat.test.ts`, et il est cité ici pour que la porte 1 de
//! `scripts/portes-de-ci.sh` sache où il vit. Ce qui **est** vérifiable côté
//! pipeline l'est : l'instantané fournit à chaque marqueur sa famille, son
//! échelle et son libellé, donc de quoi porter une forme et une étiquette
//! distinctes sans recourir à une couleur.

use contrepoint::export::{
    Description, FAMILLES, construire_eclats, construire_instantane, construire_manifeste,
    verifier_artefacts,
};
use contrepoint::preuves::construire;
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn chemin(relatif: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relatif)
}

fn registre() -> Value {
    serde_json::from_str(&std::fs::read_to_string(chemin("../data/registre/partis.json")).unwrap())
        .unwrap()
}

const DATE: &str = "2026-07-21";
const CALCUL: &str = "2026-08-27T00:00:00Z";

fn entree_registre() -> Value {
    json!({
        "source": "registre_partis",
        "url": "https://raw.githubusercontent.com/bourbask/contrepoint/v0.3.0/data/registre/partis.json",
        "producteur": "Contrepoint",
        "derniere_mise_a_jour": "2026-08-27",
        "citation": null,
        "empreinte_sha256": "b7bdb819be8b6773a8af5d2a939a78120e710e6f3cf6e86e87db0443168aaf2b",
        "empreinte_contenu_sha256": "b7bdb819be8b6773a8af5d2a939a78120e710e6f3cf6e86e87db0443168aaf2b",
        "recupere_le": "2026-08-27"
    })
}

fn entrees_votes() -> Value {
    json!([
        entree_registre(),
        {
            "source": "an_scrutins_17",
            "url": "https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip",
            "producteur": "Assemblée nationale",
            "derniere_mise_a_jour": "2026-08-27",
            "citation": null,
            "empreinte_sha256": "aa767a2a05f25e38badca738af3535cb9ab89b5fa95d0810a60af05eab1e4721",
            "empreinte_contenu_sha256": "c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c",
            "recupere_le": "2026-08-27"
        },
        {
            "source": "an_organe",
            "url": "https://data.assemblee-nationale.fr/static/openData/repository/17/amo/tous_acteurs_mandats_organes_xi_legislature/AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip",
            "producteur": "Assemblée nationale",
            "derniere_mise_a_jour": "2026-08-27",
            "citation": null,
            "empreinte_sha256": "bbecd01274d2bc9f46fcaa276b06868862ae7680131da3162e35b5cbef663061",
            "empreinte_contenu_sha256": "0f49c00a8227d6cb8e658d374bacfec35238fe4e2dd6305f7df6ac4f515c5de6",
            "recupere_le": "2026-08-27"
        }
    ])
}

fn entrees_ches() -> Value {
    json!([
        {
            "source": "ches_2024",
            "url": "https://github.com/chesdata/chesdata.github.io/releases/download/ches-europe/CHES_2024_final_v2.csv",
            "producteur": "Chapel Hill Expert Survey",
            "derniere_mise_a_jour": "2026-08-04",
            "citation": "Rovny, Jan, Jonathan Polk, Ryan Bakker, Liesbet Hooghe, Seth Jolly, Gary Marks, Marco Steenbergen, and Milada Anna Vachudova. 2025. \"The 2024 Chapel Hill Expert Survey on political party positioning in Europe: Twenty-five years of party positional data.\" Electoral Studies 97 (October). doi:10.1016/j.electstud.2025.102981",
            "empreinte_sha256": "1c1ec0532afa2a0a13317122cbbe40eb9ff35425191892d1fff24fbef6acc6a8",
            "empreinte_contenu_sha256": "1c1ec0532afa2a0a13317122cbbe40eb9ff35425191892d1fff24fbef6acc6a8",
            "recupere_le": "2026-08-27"
        },
        entree_registre()
    ])
}

fn ligne_votes(entite: &str, valeur: Value, iqr: f64, effectif: u64) -> String {
    let publiee = !valeur.is_null();
    construire(json!({
        "contrat": "0.4.0",
        "famille": "votes",
        "entite": entite,
        "valeur": valeur,
        "valeur_code": null,
        "echelle": {
            "id": "votes_an17_ancre_v1",
            "min": -1.0,
            "max": 1.0,
            "decimales": 2,
            "libelle": "Votes XVIIe législature, unités médianes ancrées"
        },
        "motif_code": if publiee { json!(null) } else { json!("sous_seuil_de_publication") },
        "motif": if publiee {
            json!(null)
        } else {
            json!("Dispersion interne au-delà du seuil publié : IQR 0,687 pour un maximum de 0,25.")
        },
        "dispersion": {"effectif": effectif, "iqr": iqr, "ecart_type_reechantillonnage": 0.0},
        "observation": {"debut": "2024-10-08", "fin": DATE},
        "date_source": "2026-08-27",
        "date_calcul": CALCUL,
        "methode": {
            "id": "votes_rang1_ancre",
            "version": "1.0.0",
            "parametres": {
                "ancre_droite": "groupe.an17.rn",
                "ancre_gauche": "groupe.an17.lfi-nfp",
                "codage": "pour=+1;contre=-1;abstention=0;non_votant=manquant;absent=manquant",
                "filtre_scrutins": "minorite_non_vide",
                "iterations_als": 300,
                "scrutins_ecartes": 455,
                "scrutins_retenus": 7979
            }
        },
        "epingles": [],
        "entrees": entrees_votes(),
        "logiciel": {"version": "0.1.0", "commit": null}
    }))
    .unwrap_or_else(|e| panic!("{entite} : {e}"))
}

fn ligne_experts(entite: &str, valeur: f64) -> String {
    construire(json!({
        "contrat": "0.4.0",
        "famille": "experts",
        "entite": entite,
        "valeur": valeur,
        "valeur_code": null,
        "echelle": {
            "id": "ches_lrgen_0_10",
            "min": 0.0,
            "max": 10.0,
            "decimales": 2,
            "libelle": "CHES 2024, variable lrgen, échelle 0 à 10"
        },
        "motif_code": null,
        "motif": null,
        "dispersion": null,
        "observation": {"debut": "2024-01-01", "fin": "2024-12-31"},
        "date_source": "2026-08-04",
        "date_calcul": CALCUL,
        "methode": {
            "id": "ches_lrgen",
            "version": "1.0.0",
            "parametres": {"colonne": "lrgen", "pays": 6, "vague": "2024"}
        },
        "epingles": [],
        "entrees": entrees_ches(),
        "logiciel": {"version": "0.1.0", "commit": null}
    }))
    .unwrap()
}

/// Une absence dite, avec son code : Place publique est absente des quatre
/// sources d'identifiants et de la grille de nuances (registre-entites.md §5.2).
fn ligne_absence(entite: &str) -> String {
    construire(json!({
        "contrat": "0.4.0",
        "famille": "experts",
        "entite": entite,
        "valeur": null,
        "valeur_code": null,
        "echelle": {
            "id": "ches_lrgen_0_10",
            "min": 0.0,
            "max": 10.0,
            "decimales": 2,
            "libelle": "CHES 2024, variable lrgen, échelle 0 à 10"
        },
        "motif_code": "hors_source",
        "motif": "Absente des quatre sources d'identifiants et de la grille de nuances.",
        "dispersion": null,
        "observation": {"debut": "2024-01-01", "fin": "2024-12-31"},
        "date_source": "2026-08-04",
        "date_calcul": CALCUL,
        "methode": {
            "id": "ches_lrgen",
            "version": "1.0.0",
            "parametres": {"colonne": "lrgen", "pays": 6, "vague": "2024"}
        },
        "epingles": [],
        "entrees": entrees_ches(),
        "logiciel": {"version": "0.1.0", "commit": null}
    }))
    .unwrap()
}

fn description() -> Description {
    Description {
        id: "an17-2026-07-21".to_owned(),
        chambre: "AN".to_owned(),
        legislature: "17".to_owned(),
        date: DATE.to_owned(),
        note_ancrage: "Échelle ancrée sur deux groupes de cette législature : deux instantanés ne se superposent pas.".to_owned(),
    }
}

/// Le jeu minimal : les deux ancres, un groupe non publié, un parti mesuré par
/// les experts, une entité sans aucune mesure.
fn lignes() -> Vec<String> {
    vec![
        ligne_votes("groupe.an17.lfi-nfp", json!(-1.0), 0.047, 73),
        ligne_votes("groupe.an17.rn", json!(1.0), 0.052, 129),
        ligne_votes("groupe.an17.liot", json!(null), 0.687, 25),
        ligne_experts("parti.rn", 8.82),
        ligne_absence("parti.place-publique"),
    ]
}

fn instantane() -> String {
    construire_instantane(&description(), "0.4.0", &lignes(), &registre())
        .expect("instantané construit")
}

// ---------------------------------------------------------------- EXP-01 ----

/// Les licences telles que les descripteurs de cache les portent. CHES n'en
/// publie **aucune** : la nommer « Licence Ouverte » serait la fausse
/// attribution que REC-07 refuse.
fn licences() -> BTreeMap<String, String> {
    [
        ("Assemblée nationale", "Licence Ouverte v1.0"),
        (
            "Chapel Hill Expert Survey",
            "aucune licence publiée, citation exigée",
        ),
        ("Ministère de l'intérieur", "Licence Ouverte v2.0"),
    ]
    .into_iter()
    .map(|(p, l)| (p.to_owned(), l.to_owned()))
    .collect()
}

#[test]
fn trois_marqueurs_trois_echelles_nommees() {
    // EXP-01 [C] — chaque marqueur porte sa famille, son échelle nommée et sa
    // preuve. Aucune graduation commune, aucun écart chiffré entre familles :
    // la bibliothèque de graphiques qui refuse de moyenner trois séries sur un
    // axe commun n'existe pas, c'est le code du projet qui doit refuser.
    let vue: Value = serde_json::from_str(&instantane()).unwrap();
    let mut echelles = std::collections::BTreeMap::new();
    let mut marqueurs = 0;
    for bande in vue["bandes"].as_array().unwrap() {
        for marqueur in bande["marqueurs"].as_array().unwrap() {
            marqueurs += 1;
            let famille = marqueur["famille"].as_str().expect("famille");
            let echelle = marqueur["echelle"].as_str().expect("échelle nommée");
            assert!(marqueur["preuve"].as_str().is_some_and(|p| p.len() == 64));
            let precedente = echelles.insert(echelle.to_owned(), famille.to_owned());
            assert!(
                precedente.is_none_or(|f| f == famille),
                "deux familles partagent l'échelle {echelle}"
            );
        }
    }
    assert!(marqueurs > 0);
    assert!(
        echelles.len() >= 2,
        "au moins deux échelles nommées distinctes"
    );

    // Aucun nombre atteignable depuis `bandes[]` hors d'un `marqueurs[]`, hors
    // `effectif` (I11) : il n'existe donc pas d'emplacement pour une valeur
    // agrégeant deux familles.
    for bande in vue["bandes"].as_array().unwrap() {
        for (cle, valeur) in bande.as_object().unwrap() {
            assert!(
                cle == "marqueurs" || !valeur.is_number(),
                "I11 — nombre `{cle}` hors d'un marqueur"
            );
        }
    }
}

// ---------------------------------------------------------------- EXP-02 ----

#[test]
fn aucune_comparaison_inter_legislature() {
    // EXP-02 [C] — I18. Aucun champ d'écart, de ratio ni de flèche entre deux
    // législatures, deux dates ou deux instantanés.
    let vue = instantane();
    let manifeste = construire_manifeste(
        "0.4.0",
        &[(description(), vue.clone())],
        &lignes(),
        &licences(),
    )
    .expect("manifeste");
    for artefact in [&vue, &manifeste] {
        let racine: Value = serde_json::from_str(artefact).unwrap();
        assert!(
            !contient_nom(
                &racine,
                &["ecart", "variation", "evolution", "delta", "tendance"]
            ),
            "un champ d'écart entre deux dates est apparu"
        );
    }
    // Le manifeste ne porte qu'une entrée d'instantané en v0, et l'ajout d'une
    // seconde n'introduit aucun champ nouveau : le curseur temporel est déjà là.
    let mut seconde = description();
    seconde.id = "an17-2026-01-01".to_owned();
    seconde.date = "2026-01-01".to_owned();
    let deux = construire_manifeste(
        "0.4.0",
        &[(description(), vue.clone()), (seconde, vue)],
        &lignes(),
        &licences(),
    )
    .unwrap();
    let racine: Value = serde_json::from_str(&deux).unwrap();
    assert_eq!(racine["instantanes"].as_array().unwrap().len(), 2);
    let cles_une: Vec<&String> = racine["instantanes"][0]
        .as_object()
        .unwrap()
        .keys()
        .collect();
    let cles_deux: Vec<&String> = racine["instantanes"][1]
        .as_object()
        .unwrap()
        .keys()
        .collect();
    assert_eq!(
        cles_une, cles_deux,
        "aucun champ nouveau avec une seconde date"
    );
}

fn contient_nom(noeud: &Value, noms: &[&str]) -> bool {
    match noeud {
        Value::Object(map) => map.iter().any(|(cle, valeur)| {
            noms.iter().any(|n| cle.split('_').any(|j| j == *n)) || contient_nom(valeur, noms)
        }),
        Value::Array(elements) => elements.iter().any(|e| contient_nom(e, noms)),
        _ => false,
    }
}

// ---------------------------------------------------------------- EXP-03 ----

#[test]
fn absence_dite_jamais_comblee() {
    // EXP-03 [C] — une case vide se lit comme un centre. C'est la façon la plus
    // directe de publier une position que personne n'a mesurée.
    let vue: Value = serde_json::from_str(&instantane()).unwrap();

    // §4.3 règle 4 : un groupe dont le seul marqueur porte
    // `sous_seuil_de_publication` a une bande — la mesure existe, sa
    // non-publication est le résultat et il s'affiche.
    let liot = vue["bandes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|b| b["id"] == "groupe.an17.liot")
        .expect("LIOT occupe une bande : sa non-publication est un résultat");
    let marqueur = &liot["marqueurs"][0];
    assert!(marqueur["valeur"].is_null() && marqueur["valeur_code"].is_null());
    assert_eq!(marqueur["motif_code"], "sous_seuil_de_publication");
    assert!(marqueur["motif"].as_str().is_some_and(|m| !m.is_empty()));

    // §4.3 règle 5 : une entité sans aucune mesure n'a pas de bande, elle est
    // dite dans `sans_mesure`, avec sa raison — jamais « neutre » ni « centre ».
    let sans = vue["sans_mesure"].as_array().unwrap();
    assert!(!sans.is_empty(), "les entités sans mesure sont dites");
    for entite in sans {
        assert!(
            entite["motif"].as_str().is_some_and(|m| !m.is_empty()),
            "une absence est dite avec sa raison"
        );
        let motif = entite["motif"].as_str().unwrap().to_lowercase();
        for interdit in ["neutre", "centre", "au milieu", "sans opinion"] {
            assert!(!motif.contains(interdit), "motif complaisant : {motif}");
        }
        assert!(
            vue["bandes"]
                .as_array()
                .unwrap()
                .iter()
                .all(|b| b["id"] != entite["entite"]),
            "une entité sans mesure n'occupe pas de bande"
        );
    }
}

// ---------------------------------------------------------------- EXP-04 ----

#[test]
fn limites_de_longueur() {
    // EXP-04 — ADR 0000 §5. Le graphe est dimensionné sur ces limites :
    // étiquette ≤ 40 caractères, légende ≤ 140.
    let vue: Value = serde_json::from_str(&instantane()).unwrap();
    assert!(vue["ancrage"]["note"].as_str().unwrap().chars().count() <= 140);
    for bande in vue["bandes"].as_array().unwrap() {
        assert!(
            bande["libelle"].as_str().unwrap().chars().count() <= 40,
            "{bande}"
        );
        for marqueur in bande["marqueurs"].as_array().unwrap() {
            assert!(
                marqueur["libelle"].as_str().unwrap().chars().count() <= 40,
                "{marqueur}"
            );
            if let Some(motif) = marqueur["motif"].as_str() {
                assert!(motif.chars().count() <= 140);
            }
        }
    }
    for entite in vue["sans_mesure"].as_array().unwrap() {
        assert!(entite["libelle"].as_str().unwrap().chars().count() <= 40);
        assert!(entite["motif"].as_str().unwrap().chars().count() <= 140);
    }

    // Le libellé d'une bande est le `nom` du registre s'il tient en
    // 40 caractères, sinon le `sigle` — cas de LIOT, dont le nom en compte 48.
    let liot = vue["bandes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|b| b["id"] == "groupe.an17.liot")
        .unwrap();
    assert_eq!(liot["libelle"], "LIOT");
}

// ---------------------------------------------------------------- EXP-05 ----

#[test]
fn lexique_interdit_absent_de_lexport() {
    // EXP-05 [C] — I14 et RG-112. Le grep de la definition of done porte sur le
    // diff : un terme introduit par une donnée plutôt que par du code y échappe.
    let vue = instantane();
    let manifeste = construire_manifeste(
        "0.4.0",
        &[(description(), vue.clone())],
        &lignes(),
        &licences(),
    )
    .unwrap();
    let eclats = construire_eclats(&lignes(), std::slice::from_ref(&vue)).unwrap();

    // Les termes sont assemblés et non écrits en clair : `scripts/lexique.sh`
    // vérifie l'arbre de code, et une liste littérale ferait de ce test une
    // violation de la règle qu'il défend. Termes proscrits par docs/juridique.md.
    let proscrits: Vec<String> = [
        ["fiab", "ilit"],
        ["cred", "ibilit"],
        ["créd", "ibilit"],
        ["vera", "cit"],
        ["véra", "cit"],
        ["inf", "ox"],
    ]
    .iter()
    .map(|p| p.concat())
    .collect();
    for (nom, artefact) in [("instantané", &vue), ("manifeste", &manifeste)]
        .into_iter()
        .chain(eclats.iter().map(|(k, v)| (k.as_str(), v)))
    {
        let bas = artefact.to_lowercase();
        for terme in &proscrits {
            assert!(
                !bas.contains(terme.as_str()),
                "terme proscrit `{terme}` dans {nom}"
            );
        }
    }

    // RG-112 — aucun identifiant technique ne nomme une entité mesurée : les
    // sigles des deux ancres n'apparaissent dans aucun identifiant d'échelle.
    let racine: Value = serde_json::from_str(&vue).unwrap();
    for bande in racine["bandes"].as_array().unwrap() {
        for marqueur in bande["marqueurs"].as_array().unwrap() {
            let echelle = marqueur["echelle"].as_str().unwrap().to_lowercase();
            for sigle in ["rn", "lfi", "nfp", "ps", "lr"] {
                assert!(
                    !echelle.split(['_', '-']).any(|j| j == sigle),
                    "l'identifiant d'échelle {echelle} nomme une entité mesurée"
                );
            }
        }
    }
}

// ---------------------------------------------------------------- EXP-06 ----

#[test]
fn schema_publie_et_verifie_a_la_construction() {
    // EXP-06 — les types du contrat sont définis deux fois, sans compilateur
    // entre les deux : ici et dans `schemas/`. Sans ce contrôle, la divergence
    // se découvre en production.
    for fichier in ["manifeste-1", "instantane-1", "eclat-preuves-1"] {
        let schema: Value = serde_json::from_str(
            &std::fs::read_to_string(chemin(&format!("../schemas/{fichier}.schema.json"))).unwrap(),
        )
        .unwrap();
        assert!(
            schema["additionalProperties"] == false || schema["type"] == "array",
            "{fichier} : le schéma publié doit être strict"
        );
    }

    // Les clés produites sont exactement les clés requises par le schéma, dans
    // l'ordre déclaré — c'est ce que la forme canonique du §7 impose.
    let attendues = |fichier: &str, chemin_dans_le_schema: &[&str]| -> Vec<String> {
        let schema: Value = serde_json::from_str(
            &std::fs::read_to_string(chemin(&format!("../schemas/{fichier}.schema.json"))).unwrap(),
        )
        .unwrap();
        let mut noeud = &schema;
        for pas in chemin_dans_le_schema {
            noeud = &noeud[*pas];
        }
        noeud["required"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c.as_str().unwrap().to_owned())
            .collect()
    };

    let vue = instantane();
    let ordre_ecrit = |texte: &str, cles: &[String]| {
        let mut precedent = 0;
        for cle in cles {
            let position = texte
                .find(&format!("\"{cle}\":"))
                .unwrap_or_else(|| panic!("clé {cle} absente de l'artefact"));
            assert!(position >= precedent, "clé {cle} hors de l'ordre du schéma");
            precedent = position;
        }
    };
    ordre_ecrit(&vue, &attendues("instantane-1", &[]));
    let manifeste = construire_manifeste(
        "0.4.0",
        &[(description(), vue.clone())],
        &lignes(),
        &licences(),
    )
    .unwrap();
    ordre_ecrit(&manifeste, &attendues("manifeste-1", &[]));

    // Un artefact qui ne correspond pas au schéma publié échoue bruyamment.
    let mut fautif: Value = serde_json::from_str(&vue).unwrap();
    fautif["moyenne_des_familles"] = json!(0.5);
    let refus = verifier_artefacts(
        &manifeste,
        &[fautif.to_string()],
        &Default::default(),
        &lignes(),
    );
    assert!(
        !refus.is_empty(),
        "un artefact hors schéma doit être refusé"
    );
}

// ---------------------------------------------------------------- EXP-07 ----

#[test]
fn instantane_de_lexport_complet() {
    // EXP-07 [T1] — filet de non-régression global : attrape ce que les tests
    // nominatifs ne prévoient pas. Il ne remplace aucun d'entre eux : un
    // instantané dit qu'une sortie a changé, jamais qu'elle est fausse.
    //
    // La référence est écrite à la main depuis le §4.2 du contrat, lue et
    // contrôlée, jamais recopiée d'une sortie (docs/tdd.md §2).
    let vue = instantane();
    let attendu = concat!(
        r#"{"schema":"contrepoint/instantane/1","contrat":"0.4.0","id":"an17-2026-07-21","#,
        r#""chambre":"AN","legislature":"17","date":"2026-07-21","#,
        r#""date_arretee":"2026-08-27T00:00:00Z","#,
        r#""ancrage":{"famille":"votes","ancre_gauche":"groupe.an17.lfi-nfp","#,
        r#""ancre_droite":"groupe.an17.rn","#,
        r#""note":"Échelle ancrée sur deux groupes de cette législature : deux instantanés ne se superposent pas."},"#,
        r#""bandes":[{"id":"parti.lfi","libelle":"La France insoumise","marqueurs":["#,
        r#"{"famille":"votes","echelle":"votes_an17_ancre_v1","valeur":-1.00,"valeur_code":null,"#,
        r#""libelle":"Votes du groupe LFI-NFP","motif_code":null,"motif":null,"#,
        r#""dispersion":{"effectif":73,"iqr":0.05},"#,
        r#""preuve":"58dddb470ebac0b3da987e46a74a1bf48b0ebfb6945219f1d10ba1eb3f6466e9"}]},"#,
        r#"{"id":"parti.rn","libelle":"Rassemblement national","marqueurs":["#,
        r#"{"famille":"votes","echelle":"votes_an17_ancre_v1","valeur":1.00,"valeur_code":null,"#,
        r#""libelle":"Votes du groupe RN","motif_code":null,"motif":null,"#,
        r#""dispersion":{"effectif":129,"iqr":0.05},"#,
        r#""preuve":"8fde0f5f78afe28503821f8194a91dca7022105eaaad831bad9b6d4ef8489e2b"},"#,
        r#"{"famille":"experts","echelle":"ches_lrgen_0_10","valeur":8.82,"valeur_code":null,"#,
        r#""libelle":"CHES 2024, lrgen","motif_code":null,"motif":null,"dispersion":null,"#,
        r#""preuve":"4367cde19edba83604ea4e88b557e1491332af708e929a07e5b2aac349173c2b"}]},"#,
        r#"{"id":"groupe.an17.liot","libelle":"LIOT","marqueurs":["#,
        r#"{"famille":"votes","echelle":"votes_an17_ancre_v1","valeur":null,"valeur_code":null,"#,
        r#""libelle":"Votes du groupe LIOT","motif_code":"sous_seuil_de_publication","#,
        r#""motif":"Dispersion interne au-delà du seuil publié : IQR 0,687 pour un maximum de 0,25.","#,
        r#""dispersion":{"effectif":25,"iqr":0.69},"#,
        r#""preuve":"32fb76691440af055b9863b258a59c400b00775d7598630c2cdcc1441a192b61"}]}],"#,
        // §4.3 règle 5 — l'univers de `sans_mesure` est le **registre**, pas les
        // seules entités qui portent une ligne : une entité sans aucune ligne
        // n'a aucun marqueur, donc aucun marqueur ne porte de valeur. Le jeu
        // minimal ne mesure que LFI, RN et LIOT, donc les dix autres entités
        // `parti.*` et `coalition.*` valides à la date sont dites. Place
        // publique garde le motif de **sa** ligne, qui est plus précis.
        // Les Écologistes est dit par son sigle : son nom compte 43 caractères.
        r#""sans_mesure":["#,
        r#"{"entite":"coalition.ensemble","libelle":"Ensemble","motif_code":"aucune_mesure","#,
        r#""motif":"Aucune ligne de preuve ne porte cette entité : aucune famille de mesure ne l'a produite."},"#,
        r#"{"entite":"coalition.nfp","libelle":"Nouveau Front populaire","motif_code":"aucune_mesure","#,
        r#""motif":"Aucune ligne de preuve ne porte cette entité : aucune famille de mesure ne l'a produite."},"#,
        r#"{"entite":"parti.ecologistes","libelle":"Les Écologistes","motif_code":"aucune_mesure","#,
        r#""motif":"Aucune ligne de preuve ne porte cette entité : aucune famille de mesure ne l'a produite."},"#,
        r#"{"entite":"parti.horizons","libelle":"Horizons","motif_code":"aucune_mesure","#,
        r#""motif":"Aucune ligne de preuve ne porte cette entité : aucune famille de mesure ne l'a produite."},"#,
        r#"{"entite":"parti.lr","libelle":"Les Républicains","motif_code":"aucune_mesure","#,
        r#""motif":"Aucune ligne de preuve ne porte cette entité : aucune famille de mesure ne l'a produite."},"#,
        r#"{"entite":"parti.pcf","libelle":"Parti communiste français","motif_code":"aucune_mesure","#,
        r#""motif":"Aucune ligne de preuve ne porte cette entité : aucune famille de mesure ne l'a produite."},"#,
        r#"{"entite":"parti.place-publique","libelle":"Place publique","#,
        r#""motif_code":"hors_source","motif":"Absente des quatre sources d'identifiants et de la grille de nuances."},"#,
        r#"{"entite":"parti.ps","libelle":"Parti socialiste","motif_code":"aucune_mesure","#,
        r#""motif":"Aucune ligne de preuve ne porte cette entité : aucune famille de mesure ne l'a produite."},"#,
        r#"{"entite":"parti.renaissance","libelle":"Renaissance","motif_code":"aucune_mesure","#,
        r#""motif":"Aucune ligne de preuve ne porte cette entité : aucune famille de mesure ne l'a produite."},"#,
        r#"{"entite":"parti.udr","libelle":"Union des droites pour la République","motif_code":"aucune_mesure","#,
        r#""motif":"Aucune ligne de preuve ne porte cette entité : aucune famille de mesure ne l'a produite."}]}"#
    );
    assert_eq!(vue, attendu);
}

// -------------------------------------------- invariants d'artefact --------

#[test]
fn aucun_orphelin_dans_les_deux_sens() {
    // I16 — tout `preuve` d'un marqueur existe dans un éclat publié, la ligne y
    // est identique octet pour octet à celle du registre, et tout éclat publié
    // est référencé par au moins un marqueur.
    let lignes = lignes();
    let vue = instantane();
    let manifeste = construire_manifeste(
        "0.4.0",
        &[(description(), vue.clone())],
        &lignes,
        &licences(),
    )
    .unwrap();
    let eclats = construire_eclats(&lignes, std::slice::from_ref(&vue)).unwrap();

    let racine: Value = serde_json::from_str(&vue).unwrap();
    let mut references = std::collections::BTreeSet::new();
    for bande in racine["bandes"].as_array().unwrap() {
        for marqueur in bande["marqueurs"].as_array().unwrap() {
            let preuve = marqueur["preuve"].as_str().unwrap().to_owned();
            let eclat = eclats
                .get(&preuve[..2])
                .unwrap_or_else(|| panic!("éclat {} absent", &preuve[..2]));
            let attendue = lignes
                .iter()
                .find(|l| l.contains(&format!("\"id\":\"{preuve}\"")))
                .expect("ligne du registre");
            assert!(
                eclat.contains(attendue.as_str()),
                "la ligne servie doit être identique octet pour octet à celle du registre"
            );
            references.insert(preuve);
        }
    }
    for (prefixe, eclat) in &eclats {
        let contenu: Value = serde_json::from_str(eclat).unwrap();
        for ligne in contenu.as_array().unwrap() {
            let id = ligne["id"].as_str().unwrap();
            assert!(id.starts_with(prefixe.as_str()));
            assert!(references.contains(id), "éclat orphelin : {id}");
        }
    }
    let refus = verifier_artefacts(&manifeste, std::slice::from_ref(&vue), &eclats, &lignes);
    assert!(refus.is_empty(), "{refus:?}");
}

#[test]
fn date_arretee_derivee_et_jamais_saisie() {
    // I17 — `date_arretee` du manifeste et de chaque instantané = maximum des
    // `date_calcul` des lignes référencées. Aucune valeur saisie.
    let vue: Value = serde_json::from_str(&instantane()).unwrap();
    assert_eq!(vue["date_arretee"], CALCUL);
    let manifeste: Value = serde_json::from_str(
        &construire_manifeste(
            "0.4.0",
            &[(description(), instantane())],
            &lignes(),
            &licences(),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(manifeste["date_arretee"], CALCUL);

    // I17 porte sur « le manifeste **et** chaque instantané » (§6). La porte le
    // vérifie sur les deux : une date saisie à la main dans l'un ou l'autre est
    // refusée.
    //
    // Mutant qui survivait avant ce test : `date_arretee` du manifeste
    // remplacée par n'importe quel horodatage, `verifier_artefacts` muet.
    let vue_texte = instantane();
    let lignes = lignes();
    let eclats = construire_eclats(&lignes, std::slice::from_ref(&vue_texte)).unwrap();
    for saisie in ["manifeste", "instantane"] {
        let mut manifeste_faux = construire_manifeste(
            "0.4.0",
            &[(description(), vue_texte.clone())],
            &lignes,
            &licences(),
        )
        .unwrap();
        let mut instantane_faux = vue_texte.clone();
        let cible = if saisie == "manifeste" {
            &mut manifeste_faux
        } else {
            &mut instantane_faux
        };
        *cible = cible.replace(CALCUL, "2030-01-01T00:00:00Z");
        let refus = verifier_artefacts(
            &manifeste_faux,
            std::slice::from_ref(&instantane_faux),
            &eclats,
            &lignes,
        );
        assert!(
            refus.iter().any(|r| r.starts_with("I17")),
            "{saisie} : une `date_arretee` saisie est refusée — {refus:?}"
        );
    }

    // I20 est produit par le validateur de ligne **et retenu** par la porte
    // d'artefact : les plafonds de longueur sont épinglés.
    //
    // Mutant qui survivait avant ce test : le filtre de `verifier_artefacts`
    // jetait I20 avec le reste.
    let mut trop_long: Value = serde_json::from_str(&vue_texte).unwrap();
    trop_long["ancrage"]["note"] = json!("é".repeat(201));
    let refus = verifier_artefacts(
        &construire_manifeste(
            "0.4.0",
            &[(description(), vue_texte.clone())],
            &lignes,
            &licences(),
        )
        .unwrap(),
        &[trop_long.to_string()],
        &eclats,
        &lignes,
    );
    assert!(
        refus.iter().any(|r| r.starts_with("I20")),
        "une chaîne de 201 caractères est refusée — {refus:?}"
    );

    // La mention de paternité est dérivée, pas saisie : `producteur` et
    // `derniere_mise_a_jour` de l'entrée amont des lignes référencées.
    let mention = manifeste["mention_paternite"].as_str().unwrap();
    // Dérivée des entrées **amont** : le registre d'entités est un fichier du
    // projet, pas la source dont la licence exige la mention au point de
    // réutilisation. Les deux producteurs amont de la fixture y figurent, et la
    // date est la plus récente de leurs `derniere_mise_a_jour`.
    assert_eq!(
        mention,
        "Assemblée nationale — Licence Ouverte v1.0 ; Chapel Hill Expert Survey — aucune licence \
         publiée, citation exigée — données du 2026-08-27"
    );
    assert!(
        !mention.contains("Contrepoint"),
        "le registre du projet n'est pas une source amont"
    );

    // REC-07 — chaque producteur porte **sa** licence. La forme précédente
    // joignait les producteurs par une virgule et suffixait une seule licence :
    // elle attribuait la Licence Ouverte v1.0 à CHES, qui n'en publie aucune, et
    // au ministère, dont le fichier est en v2.0. Une égalité de chaîne seule
    // avait laissé passer ce défaut — d'où la propriété, vérifiée en plus.
    for (producteur, attendue) in licences() {
        if !mention.contains(&producteur) {
            continue;
        }
        let apres = &mention[mention.find(&producteur).unwrap() + producteur.len()..];
        let fragment = apres.split(" ; ").next().unwrap_or(apres);
        assert!(
            fragment.contains(&attendue),
            "REC-07 : « {producteur} » doit porter sa propre licence « {attendue} », \
             obtenu « {fragment} »"
        );
    }
    assert!(
        !mention.contains("Chapel Hill Expert Survey — Licence Ouverte"),
        "REC-07 : CHES ne publie aucune licence — la nommer est une fausse attribution (RG-76)"
    );
    assert!(
        mention.chars().count() <= 200,
        "I20 : `mention_paternite` n'est pas exceptée du plafond"
    );
    assert_eq!(
        manifeste["licence"],
        "Licence Ouverte / Open Licence (Etalab)"
    );

    // Le manifeste ne porte aucune valeur mesurée, et la légende des familles
    // est close, dans l'ordre d'affichage.
    let familles: Vec<&str> = manifeste["familles"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["id"].as_str().unwrap())
        .collect();
    assert_eq!(
        familles,
        FAMILLES.iter().map(|(id, ..)| *id).collect::<Vec<_>>()
    );
    assert_eq!(manifeste["preuves"]["eclats"], 256);
    assert_eq!(
        manifeste["preuves"]["fonction"],
        "deux premiers caractères hexadécimaux de l'id"
    );
}

#[test]
fn chaque_famille_du_manifeste_porte_les_bornes_de_son_echelle() {
    // §4.1 — `familles[]` porte `min`, `max` et `decimales`, **recopiés** de
    // `echelle.*` des lignes de preuve. Le front les lit au lieu de dériver la
    // graduation des valeurs observées : trois échelles étirées sur la même
    // plage de pixels fabriquent des concordances qui n'existent pas, et
    // rendent la moyenne entre familles visuellement dessinable.
    let lignes = lignes();
    let vue = instantane();
    let manifeste_texte = construire_manifeste(
        "0.4.0",
        &[(description(), vue.clone())],
        &lignes,
        &licences(),
    )
    .unwrap();
    let manifeste: Value = serde_json::from_str(&manifeste_texte).unwrap();
    let bornes: Vec<(&str, Value, Value, Value)> = manifeste["familles"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["id"].as_str().unwrap(),
                f["min"].clone(),
                f["max"].clone(),
                f["decimales"].clone(),
            )
        })
        .collect();
    assert_eq!(
        bornes,
        vec![
            ("votes", json!(-1.0), json!(1.0), json!(2)),
            ("experts", json!(0.0), json!(10.0), json!(2)),
            // `administratif` porte un code, pas une position : pas de
            // graduation, donc pas de bornes. Aucune valeur de remplissage.
            ("administratif", json!(null), json!(null), json!(null)),
        ]
    );
    // Les bornes sont celles de l'échelle, pas des valeurs observées : la seule
    // valeur `experts` de la fixture vaut 8,82 et la borne haute reste 10.
    assert_eq!(manifeste["familles"][1]["max"], 10.0);
    assert_ne!(manifeste["familles"][1]["max"], 8.82);

    // L'invariant : des bornes absentes ou fausses sont refusées.
    //
    // Mutants qui survivaient avant ce test : `familles[]` sans bornes, et
    // `min` recopié d'ailleurs.
    let eclats = construire_eclats(&lignes, std::slice::from_ref(&vue)).unwrap();
    for mutant in [
        manifeste_texte.replace(
            r#""min":-1.0,"max":1.0,"decimales":2"#,
            r#""min":-10.0,"max":10.0,"decimales":2"#,
        ),
        manifeste_texte.replace(r#","min":-1.0,"max":1.0,"decimales":2"#, ""),
    ] {
        assert_ne!(mutant, manifeste_texte, "le mutant n'a rien changé");
        let refus = verifier_artefacts(&mutant, std::slice::from_ref(&vue), &eclats, &lignes);
        assert!(
            !refus.is_empty(),
            "un manifeste sans bornes justes est refusé — {refus:?}"
        );
    }

    // Deux lignes d'une même famille sur deux échelles divergentes : erreur
    // bloquante, jamais un arbitrage silencieux.
    let mut divergente: Value =
        serde_json::from_str(&ligne_votes("groupe.an17.soc", json!(-0.5), 0.1, 68)).unwrap();
    divergente["echelle"]["max"] = json!(2.0);
    let mut deux = lignes.clone();
    deux.push(divergente.to_string());
    let erreur = construire_manifeste("0.4.0", &[(description(), vue)], &deux, &licences())
        .expect_err("deux échelles divergentes pour une famille sont un refus");
    assert!(erreur.contains("divergentes"), "{erreur}");
}

#[test]
fn regle_de_construction_des_bandes() {
    // §4.3 — la règle 2 est ce qui empêche une bande de parti de porter deux
    // marqueurs `votes`, et la règle 3 ce qui empêche d'attribuer à ECOLO les
    // votes de 5 députés communistes.
    let mut lignes = lignes();
    // ECOS déclare deux partis : il garde sa propre bande (règle 3).
    lignes.push(ligne_votes("groupe.an17.ecos", json!(-0.5), 0.1, 38));
    // GDR déclare le PCF, qu'ECOS déclare aussi : aucun des deux ne rejoint la
    // bande du PCF.
    lignes.push(ligne_votes("groupe.an17.gdr", json!(-0.6), 0.1, 16));
    // EPR a une composition vide : sa propre bande.
    lignes.push(ligne_votes("groupe.an17.epr", json!(0.3), 0.1, 92));

    let vue: Value = serde_json::from_str(
        &construire_instantane(&description(), "0.4.0", &lignes, &registre()).unwrap(),
    )
    .unwrap();
    let bandes: Vec<&str> = vue["bandes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|b| b["id"].as_str().unwrap())
        .collect();
    assert!(bandes.contains(&"groupe.an17.ecos"), "{bandes:?}");
    assert!(bandes.contains(&"groupe.an17.gdr"), "{bandes:?}");
    assert!(bandes.contains(&"groupe.an17.epr"), "{bandes:?}");
    assert!(
        !bandes.contains(&"parti.pcf"),
        "PCF est déclaré par deux groupes : aucun ne le rejoint"
    );
    assert!(
        bandes.contains(&"parti.lfi"),
        "LFI-NFP est le seul groupe à déclarer parti.lfi"
    );

    // Aucune bande ne porte deux marqueurs de la même famille.
    for bande in vue["bandes"].as_array().unwrap() {
        let mut familles = std::collections::BTreeSet::new();
        for marqueur in bande["marqueurs"].as_array().unwrap() {
            assert!(
                familles.insert(marqueur["famille"].as_str().unwrap()),
                "la bande {} porte deux marqueurs de la même famille",
                bande["id"]
            );
        }
    }

    // §7 — les bandes sont triées par valeur du marqueur `votes` puis par `id`.
    let positions: Vec<f64> = vue["bandes"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|b| {
            b["marqueurs"]
                .as_array()?
                .iter()
                .find(|m| m["famille"] == "votes")?["valeur"]
                .as_f64()
        })
        .collect();
    assert!(
        positions.windows(2).all(|p| p[0] <= p[1]),
        "bandes triées par valeur du marqueur votes : {positions:?}"
    );
}

#[test]
fn aucun_identifiant_dacteur_dans_un_artefact() {
    // I13 — `grep -E '\bPA[0-9]{4,}\b'` sur les artefacts publiés est vide.
    let vue = instantane();
    let manifeste = construire_manifeste(
        "0.4.0",
        &[(description(), vue.clone())],
        &lignes(),
        &licences(),
    )
    .unwrap();
    let eclats = construire_eclats(&lignes(), std::slice::from_ref(&vue)).unwrap();
    for artefact in [&vue, &manifeste].into_iter().chain(eclats.values()) {
        for fenetre in artefact.as_bytes().windows(6) {
            let est_acteur = fenetre[0] == b'P'
                && fenetre[1] == b'A'
                && fenetre[2..].iter().all(u8::is_ascii_digit);
            assert!(!est_acteur, "identifiant d'acteur dans un artefact publié");
        }
    }
}

#[test]
fn artefact_fautif_refuse_bruyamment() {
    // EXP-06, second volet. Les tests qui précèdent inspectent l'artefact
    // **juste** ; celui-ci nourrit `verifier_artefacts` d'artefacts **faux** et
    // exige un refus. Sans lui, rendre la fonction entièrement muette ne casse
    // aucun test — mutants observés vivants avant ce test : I11, I13, I16 et la
    // liste blanche des clés, tous les quatre neutralisés sans conséquence.
    let lignes = lignes();
    let vue = instantane();
    let manifeste = construire_manifeste(
        "0.4.0",
        &[(description(), vue.clone())],
        &lignes,
        &licences(),
    )
    .unwrap();
    let eclats = construire_eclats(&lignes, std::slice::from_ref(&vue)).unwrap();
    assert!(
        verifier_artefacts(&manifeste, std::slice::from_ref(&vue), &eclats, &lignes).is_empty()
    );

    let muter = |mutation: &dyn Fn(&mut Value)| -> Value {
        let mut fautif: Value = serde_json::from_str(&vue).unwrap();
        mutation(&mut fautif);
        fautif
    };

    // I16 — un marqueur qui cite une preuve absente du registre. « Un marqueur
    // sans ligne ne s'affiche pas » : c'est la valeur affichée à la place d'une
    // mesure, interdite par la règle permanente de la roadmap.
    let pendant = muter(&|v| v["bandes"][0]["marqueurs"][0]["preuve"] = json!("0".repeat(64)));
    assert!(
        refus_de(&manifeste, &pendant, &eclats, &lignes, "I16"),
        "une preuve pendante doit être refusée"
    );

    // Le même défaut, mais sous un préfixe qui **existe** : sans ce cas, un
    // contrôle qui se contente de vérifier la présence de l'éclat passe, et la
    // ligne citée n'existe nulle part. Mutant qui survivait : la recherche de la
    // ligne dans le registre remplacée par une chaîne vide, que tout éclat
    // contient.
    let mut vivant: Value = serde_json::from_str(&vue).unwrap();
    let reel = vivant["bandes"][0]["marqueurs"][0]["preuve"]
        .as_str()
        .unwrap()
        .to_owned();
    let falsifie = format!(
        "{}{}{}",
        &reel[..2],
        if &reel[2..3] == "0" { "1" } else { "0" },
        &reel[3..]
    );
    vivant["bandes"][0]["marqueurs"][0]["preuve"] = json!(falsifie);
    // Le refus attendu nomme **ce** défaut : le marqueur cite une ligne qui
    // n'existe pas. Un `starts_with("I16")` suffisait à passer, l'orphelin
    // symétrique produisant lui aussi un I16 — et le mutant survivait.
    let refus = verifier_artefacts(
        &manifeste,
        std::slice::from_ref(&vivant.to_string()),
        &eclats,
        &lignes,
    );
    assert!(
        refus.iter().any(|r| r.contains("absente du registre")),
        "une preuve absente du registre doit être refusée même quand son éclat existe : {refus:?}"
    );

    // I11 — un nombre atteignable depuis `bandes[]` hors d'un `marqueurs[]` :
    // c'est l'emplacement où une moyenne entre familles s'écrirait.
    let agregee = muter(&|v| v["bandes"][0]["position"] = json!(0.5));
    assert!(
        refus_de(&manifeste, &agregee, &eclats, &lignes, "I11"),
        "un nombre hors d'un marqueur doit être refusé"
    );

    // La liste blanche des clés : le producteur refuse d'écrire ce que le
    // schéma publié ne déclare pas.
    let intruse = muter(&|v| v["bandes"][0]["couleur"] = json!("bleu"));
    assert!(
        !verifier_artefacts(&manifeste, &[intruse.to_string()], &eclats, &lignes).is_empty(),
        "une clé absente du schéma publié doit être refusée"
    );

    // I13 — un identifiant d'acteur glissé dans un libellé. Aucune coordonnée
    // individuelle dans un artefact publié (ADR 0000 §2, RG-41).
    let acteur = muter(&|v| v["bandes"][0]["libelle"] = json!("PA793290"));
    assert!(
        refus_de(&manifeste, &acteur, &eclats, &lignes, "I13"),
        "un identifiant d'acteur doit être refusé"
    );

    // I16, sens inverse — un éclat publié que plus aucun marqueur ne référence.
    let mut orphelins = eclats.clone();
    let orpheline = &lignes[3];
    let id = serde_json::from_str::<Value>(orpheline).unwrap()["id"]
        .as_str()
        .unwrap()
        .to_owned();
    orphelins.insert(id[..2].to_owned(), format!("[{orpheline}]"));
    let sans_experts = muter(&|v| {
        for bande in v["bandes"].as_array_mut().unwrap() {
            let marqueurs = bande["marqueurs"].as_array_mut().unwrap();
            marqueurs.retain(|m| m["famille"] != "experts");
        }
        v["bandes"]
            .as_array_mut()
            .unwrap()
            .retain(|b| !b["marqueurs"].as_array().unwrap().is_empty());
    });
    assert!(
        verifier_artefacts(&manifeste, &[sans_experts.to_string()], &orphelins, &lignes)
            .iter()
            .any(|r| r.starts_with("I16")),
        "un éclat publié sans marqueur qui le référence doit être refusé"
    );
}

fn refus_de(
    manifeste: &str,
    instantane: &Value,
    eclats: &std::collections::BTreeMap<String, String>,
    lignes: &[String],
    invariant: &str,
) -> bool {
    let refus = verifier_artefacts(
        manifeste,
        std::slice::from_ref(&instantane.to_string()),
        eclats,
        lignes,
    );
    let trouve = refus.iter().any(|r| r.starts_with(invariant));
    if !trouve {
        eprintln!("{invariant} attendu ; refus observés : {refus:?}");
    }
    trouve
}
