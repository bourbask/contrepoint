//! FAM-01 à FAM-10 — les deux familles de mesure qui ne viennent pas des
//! votes : `experts` (Chapel Hill Expert Survey) et `administratif` (codes de
//! nuance du ministère de l'intérieur).
//!
//! Spécification : `docs/brique0/contrats.md` §2.2, §2.3, §2.4 et §6,
//! `docs/brique0/registre-entites.md` §2.3, §2.5 et §3.6.
//!
//! **Aucune fixture n'est un fichier du répertoire `echantillons/`**, et c'est
//! délibéré. L'ADR 0000 §4 interdit de commiter un extrait de CHES — aucune
//! licence n'y est publiée. Le fichier de nuances, lui, porte des colonnes
//! nominatives que RG-111 interdit d'ingérer, y compris dans un fichier
//! intermédiaire du projet. Les deux dialectes de CSV sont donc **fabriqués
//! ici**, avec les colonnes réelles et des valeurs inventées : le test exerce
//! la forme de la source sans en republier la donnée.

use contrepoint::familles::{
    appariements, codes_constates, lignes_administratif, lignes_experts, lire_csv,
    lrgen_par_party_id, motif_code,
};
use contrepoint::preuves::{citation_exigee, confronter_registre, construire, verifier};
use serde_json::{Value, json};

const CITATION: &str = "Rovny, Jan, Jonathan Polk, Ryan Bakker, Liesbet Hooghe, Seth Jolly, Gary Marks, Marco Steenbergen, and Milada Anna Vachudova. 2025. \"The 2024 Chapel Hill Expert Survey on political party positioning in Europe: Twenty-five years of party positional data.\" Electoral Studies 97 (October). doi:10.1016/j.electstud.2025.102981";

const URL_CHES: &str = "https://github.com/chesdata/chesdata.github.io/releases/download/ches-europe/CHES_2024_final_v2.csv";
const URL_NUANCE: &str = "https://static.data.gouv.fr/resources/x.csv";
const URL_REGISTRE: &str = "https://example.invalid/partis.json";

/// Un registre d'entités réduit : quatre entités, deux appariées, une absente
/// de CHES et une que le nuancier ne tranche pas. Les identifiants et les
/// motifs sont fabriqués ; leur **forme** est celle du §3.6.
fn registre() -> Value {
    json!({
        "date_registre": "2026-08-27",
        "sources": [
            {"id": "ches_2024", "url": URL_CHES, "empreinte_sha256": "a".repeat(64)},
            {"id": "nuance_leg2024", "url": URL_NUANCE, "empreinte_sha256": "b".repeat(64)},
            {"id": "registre_partis", "url": URL_REGISTRE, "empreinte_sha256": "c".repeat(64)}
        ],
        "entites": [
            {"id": "parti.alpha", "nature": "parti", "nom": "Alpha", "debut": "2000-01-01", "fin": null,
             "identifiants": [
                {"source": "ches_2024", "valeur": "901", "motif": null},
                {"source": "nuance_leg2024", "valeur": "AAA", "motif": null}]},
            {"id": "parti.beta", "nature": "parti", "nom": "Beta", "debut": "2000-01-01", "fin": null,
             "identifiants": [
                {"source": "ches_2024", "valeur": null, "motif": "Absent du terrain de la vague."},
                {"source": "nuance_leg2024", "valeur": null, "motif": "Aucun code propre dans la grille constatée."}]},
            {"id": "parti.ecologistes", "nature": "parti", "nom": "Écologistes", "debut": "2000-01-01", "fin": null,
             "identifiants": [
                {"source": "ches_2024", "valeur": "903", "motif": null},
                {"source": "nuance_leg2024", "valeur": null, "motif": "Deux codes constatés, l'annexe qui les départage n'est pas récupérable."}]},
            {"id": "coalition.gamma", "nature": "coalition", "nom": "Gamma", "debut": null, "fin": null,
             "identifiants": [
                {"source": "nuance_leg2024", "valeur": "GGG", "motif": null}]}
        ],
        "groupes": []
    })
}

fn entrees_experts() -> Value {
    json!([
        {"source": "ches_2024", "url": URL_CHES, "producteur": "Chapel Hill Expert Survey",
         "derniere_mise_a_jour": "2026-08-04", "citation": CITATION,
         "empreinte_sha256": "a".repeat(64), "empreinte_contenu_sha256": "a".repeat(64),
         "recupere_le": "2026-08-27"},
        {"source": "registre_partis", "url": URL_REGISTRE, "producteur": "Contrepoint",
         "derniere_mise_a_jour": "2026-08-27", "citation": null,
         "empreinte_sha256": "c".repeat(64), "empreinte_contenu_sha256": "c".repeat(64),
         "recupere_le": "2026-08-27"}
    ])
}

fn entrees_administratif() -> Value {
    json!([
        {"source": "nuance_leg2024", "url": URL_NUANCE, "producteur": "Ministère de l'intérieur",
         "derniere_mise_a_jour": "2024-07-10", "citation": null,
         "empreinte_sha256": "b".repeat(64), "empreinte_contenu_sha256": "b".repeat(64),
         "recupere_le": "2026-08-27"},
        {"source": "registre_partis", "url": URL_REGISTRE, "producteur": "Contrepoint",
         "derniere_mise_a_jour": "2026-08-27", "citation": null,
         "empreinte_sha256": "c".repeat(64), "empreinte_contenu_sha256": "c".repeat(64),
         "recupere_le": "2026-08-27"}
    ])
}

/// Le dialecte de CHES : virgule, sans guillemets, décimales à point.
const CSV_CHES: &str = "country,party_id,party,lrgen,galtan\n\
                        6,901,ALPHA,8.818182,7.1\n\
                        6,903,GAMMA,2.3,4.0\n\
                        6,904,SANS,,4.0\n\
                        11,901,HOMONYME,0.5,1.0\n";

/// Le dialecte du ministère : point-virgule, champs guillemetés, colonnes
/// nominatives présentes dans la source et **jamais lues**.
const CSV_NUANCE: &str = "\"Code département\";Inscrits;\"Nuance candidat 1\";\"Nom candidat 1\";\"Elu 1\";\"Nuance candidat 2\";\"Nom candidat 2\";\"Elu 2\"\n\
                          \"01\";86854;AAA;\"MARTIN\";;GGG;\"DUPOND;LE JEUNE\";\"élu\"\n\
                          \"02\";7000;GGG;\"ZZZ\";;AAA;\"O\"\"BRIEN\";\n";

fn experts() -> Vec<Value> {
    lignes_experts(
        &registre(),
        &lrgen_par_party_id(CSV_CHES).expect("CSV CHES lisible"),
        &entrees_experts(),
        "2026-08-04",
        "2026-08-27T00:00:00Z",
        "0.4.0",
        "0.1.0",
    )
    .expect("lignes experts")
}

fn administratif() -> Vec<Value> {
    lignes_administratif(
        &registre(),
        &codes_constates(CSV_NUANCE).expect("CSV nuances lisible"),
        &entrees_administratif(),
        "2024-07-10",
        "2026-08-27T00:00:00Z",
        "0.4.0",
        "0.1.0",
    )
    .expect("lignes administratif")
}

// ------------------------------------------------------------- FAM-01 ------

/// Les deux sources ne parlent pas le même CSV. Un lecteur qui coupe sur le
/// séparateur sans connaître les guillemets casse `"DUPOND;LE JEUNE"` en deux
/// champs et décale toutes les colonnes suivantes — la nuance lue devient celle
/// du candidat d'à côté, silencieusement.
#[test]
fn fam_01_lecture_csv_deux_dialectes() {
    let ches = lire_csv(CSV_CHES, ',');
    assert_eq!(ches.len(), 5, "en-tête et quatre lignes");
    assert_eq!(ches[1], vec!["6", "901", "ALPHA", "8.818182", "7.1"]);
    assert_eq!(ches[3][3], "", "une valeur absente reste vide, jamais zéro");

    let nuance = lire_csv(CSV_NUANCE, ';');
    assert_eq!(
        nuance[0][0], "Code département",
        "les guillemets sont retirés"
    );
    assert_eq!(
        nuance[1][6], "DUPOND;LE JEUNE",
        "un séparateur entre guillemets ne coupe pas le champ"
    );
    assert_eq!(nuance[2][6], "O\"BRIEN", "un guillemet doublé en rend un");
    assert_eq!(
        lire_csv("a;b\r\nc;d\r\n", ';')[1],
        vec!["c", "d"],
        "les fins de ligne CRLF ne laissent pas de retour chariot en fin de champ"
    );
}

// ------------------------------------------------------------- FAM-02 ------

/// `party_id` n'est unique **que dans un pays** : la ligne `country = 11` porte
/// le même 901 qu'Alpha. Sans le filtre, l'entité française reçoit la position
/// d'un parti étranger, et rien dans la ligne publiée ne le dirait.
#[test]
fn fam_02_lrgen_filtre_le_pays_et_ne_comble_pas() {
    let positions = lrgen_par_party_id(CSV_CHES).expect("CSV lisible");
    assert_eq!(positions.len(), 2, "deux partis français exploitables");
    assert!((positions["901"] - 8.818_182).abs() < 1e-9);
    assert!(
        !positions.contains_key("904"),
        "une valeur absente est absente : jamais comblée par 0"
    );
    assert!(
        lrgen_par_party_id("country,party_id\n6,901\n").is_err(),
        "une colonne attendue et absente est un refus, pas une valeur par défaut"
    );
}

// ------------------------------------------------------------- FAM-03 ------

/// RG-111 : aucune colonne nominative n'est ingérée. Le test le prouve par un
/// nom fabriqué qui **est** un code de nuance valide : s'il apparaît dans
/// l'ensemble, c'est que le lecteur a balayé toutes les colonnes.
#[test]
fn fam_03_aucune_colonne_nominative_ingeree() {
    let csv = "\"Nuance candidat 1\";\"Nom candidat 1\";\"Prénom candidat 1\";\"Elu 1\"\n\
               AAA;BBB;CCC;DDD\n";
    let codes = codes_constates(csv).expect("CSV lisible");
    assert_eq!(codes.len(), 1, "une seule colonne est lue");
    assert!(codes.contains("AAA"));
    for nominatif in ["BBB", "CCC", "DDD"] {
        assert!(
            !codes.contains(nominatif),
            "{nominatif} vient d'une colonne nominative et n'entre jamais"
        );
    }
    assert!(
        codes_constates("Inscrits;Votants\n1;2\n").is_err(),
        "un fichier sans colonne de nuance est un refus, pas un ensemble vide"
    );
}

// ------------------------------------------------------------- FAM-04 ------

/// L'appariement est une **déclaration du registre**, jamais une lecture de la
/// source : une entité sans ligne pour cette source ne produit aucune ligne de
/// preuve. Court-circuiter le registre — apparier par le libellé, par exemple —
/// donnerait ici une ligne `experts` à la coalition, que CHES ne mesure pas.
#[test]
fn fam_04_appariement_par_le_registre_seul() {
    let registre = registre();
    let ches = appariements(&registre, "ches_2024").expect("appariements CHES");
    assert_eq!(ches.len(), 3, "trois entités déclarent un appariement CHES");
    assert!(
        !ches.iter().any(|a| a.entite == "coalition.gamma"),
        "la coalition n'a aucune ligne CHES au registre : aucune ligne de preuve"
    );
    assert_eq!(
        experts().len(),
        3,
        "une ligne experts par appariement déclaré, ni plus ni moins"
    );
    assert_eq!(
        administratif().len(),
        4,
        "les quatre entités déclarent un appariement au nuancier"
    );
}

// ------------------------------------------------------------- FAM-05 ------

/// §2.4 : l'absence est dite, avec son code. Beta est hors de CHES, les
/// Écologistes ont deux codes de nuance que la source ne départage pas — deux
/// absences, deux codes différents, jamais une case vide.
#[test]
fn fam_05_absence_dite_avec_son_code() {
    let beta = experts()
        .into_iter()
        .find(|l| l["entite"] == "parti.beta")
        .expect("ligne beta");
    assert!(beta["valeur"].is_null() && beta["valeur_code"].is_null());
    assert_eq!(beta["motif_code"], "hors_source");
    assert_eq!(beta["motif"], "Absent du terrain de la vague.");

    let eco = administratif()
        .into_iter()
        .find(|l| l["entite"] == "parti.ecologistes")
        .expect("ligne écologistes");
    assert_eq!(eco["motif_code"], "source_indeterminee");
    assert_eq!(
        motif_code("nuance_leg2024", "parti.beta"),
        "hors_source",
        "le défaut est `hors_source` ; l'exception est nommée, pas générale"
    );
}

// ------------------------------------------------------------- FAM-06 ------

/// I23 : CHES n'accorde aucun droit de republication et exige d'être cité. La
/// citation est portée **par l'entrée**, mot pour mot. Une citation abrégée,
/// reformulée ou remontée en mention globale est un refus.
#[test]
fn fam_06_citation_ches_mot_pour_mot() {
    assert_eq!(
        citation_exigee("ches_2024"),
        Some(CITATION),
        "la citation que le pipeline oppose est celle que la source publie"
    );
    assert_eq!(
        citation_exigee("nuance_leg2024"),
        None,
        "le nuancier n'exige aucune citation : lui en attribuer une serait faux"
    );
    for ligne in experts() {
        let entree = ligne["entrees"]
            .as_array()
            .and_then(|e| e.iter().find(|e| e["source"] == "ches_2024"))
            .expect("entrée CHES");
        assert_eq!(entree["citation"], CITATION);
    }
    let mut ligne = experts().remove(0);
    ligne["schema"] = json!("contrepoint/preuve/1");
    ligne["entrees"][0]["citation"] = json!("Rovny et al. 2025.");
    // `id` recalculé après la mutation : un `id` faux ferait tomber le refus sur
    // I8, et le test passerait au vert sans jamais exercer I23.
    ligne["id"] = json!(contrepoint::preuves::identifiant(&ligne).expect("`id` recalculable"));
    assert!(
        verifier(&ligne).iter().any(|r| r.starts_with("I23")),
        "une citation abrégée est refusée : {:?}",
        verifier(&ligne)
    );
}

// ------------------------------------------------------------- FAM-07 ------

/// Le nuancier ne publie pas un nombre : il publie un **code**, attribué par une
/// administration, révisé par circulaire et contesté au contentieux. Le publier
/// comme une valeur numérique le ferait entrer sur un axe et rendrait une
/// moyenne représentable.
#[test]
fn fam_07_nuance_publiee_en_code_jamais_en_nombre() {
    for ligne in administratif() {
        assert!(
            ligne["valeur"].is_null(),
            "la famille administratif ne porte jamais de valeur numérique"
        );
        assert!(ligne["echelle"]["min"].is_null() && ligne["echelle"]["max"].is_null());
        assert!(ligne["echelle"]["decimales"].is_null());
        if let Some(code) = ligne["valeur_code"].as_str() {
            assert!(code.chars().all(|c| c.is_ascii_uppercase()));
        }
    }
    let alpha = administratif()
        .into_iter()
        .find(|l| l["entite"] == "parti.alpha")
        .expect("ligne alpha");
    assert_eq!(alpha["valeur_code"], "AAA");
}

// ------------------------------------------------------------- FAM-08 ------

/// Règle non négociable n° 6 : les trois familles ne sont jamais moyennées. La
/// forme de l'interdit est l'échelle : trois identifiants distincts, trois
/// domaines qui ne se superposent pas, et aucune ligne qui n'appartienne à une
/// famille.
#[test]
fn fam_08_echelles_propres_jamais_partagees() {
    let mut vues = std::collections::BTreeSet::new();
    for ligne in experts().into_iter().chain(administratif()) {
        let famille = ligne["famille"].as_str().expect("famille").to_owned();
        let echelle = ligne["echelle"]["id"].as_str().expect("échelle").to_owned();
        vues.insert((famille, echelle));
    }
    assert_eq!(
        vues,
        [
            ("experts".to_owned(), "ches_lrgen_0_10".to_owned()),
            ("administratif".to_owned(), "nuance_leg2024".to_owned())
        ]
        .into_iter()
        .collect(),
        "une famille, une échelle, et jamais celle d'une autre"
    );
    let echelles: Vec<&str> = vues.iter().map(|(_, e)| e.as_str()).collect();
    assert!(
        !echelles.contains(&"votes_an17_ancre_v1"),
        "aucune des deux familles n'emprunte l'échelle des votes"
    );
}

// ------------------------------------------------------------- FAM-09 ------

/// Les deux familles n'ont pas le même rapport à leur source, et le confondre
/// serait faux dans les deux sens.
///
/// CHES publie un **référentiel** : les partis de la vague sont énumérés, et un
/// `party_id` apparié au registre mais absent du fichier est une divergence —
/// soit la source a bougé, soit le registre est périmé. C'est V15 et V16
/// transposés, et c'est un arrêt.
///
/// Le nuancier publie une **observation** : les codes constatés sont ceux que
/// des candidatures ont portés à ce tour. Un code de la grille qu'aucune
/// candidature n'a porté n'est pas une divergence, c'est une absence — dite
/// avec son code, jamais comblée (§2.4, ADR 0000 §8 : « case affichée comme non
/// mesurée, jamais interpolée »). Cas réel : `COM` est constaté au 1er tour de
/// 2024, pas au 2nd.
#[test]
fn fam_09_referentiel_et_observation_ne_se_traitent_pas_pareil() {
    let codes = codes_constates("\"Nuance candidat 1\"\nGGG\n").expect("CSV lisible");
    let lignes = lignes_administratif(
        &registre(),
        &codes,
        &entrees_administratif(),
        "2024-07-10",
        "2026-08-27T00:00:00Z",
        "0.4.0",
        "0.1.0",
    )
    .expect("un code non constaté n'arrête pas l'exécution");
    let alpha = lignes
        .iter()
        .find(|l| l["entite"] == "parti.alpha")
        .expect("ligne alpha");
    assert!(
        alpha["valeur_code"].is_null(),
        "AAA n'est pas constaté à ce tour"
    );
    assert_eq!(alpha["motif_code"], "hors_source");
    assert!(
        alpha["motif"]
            .as_str()
            .is_some_and(|m| m.contains("AAA") && m.len() <= 140),
        "le motif nomme le code non constaté : {}",
        alpha["motif"]
    );
    assert_eq!(
        lignes
            .iter()
            .find(|l| l["entite"] == "coalition.gamma")
            .expect("ligne gamma")["valeur_code"],
        "GGG",
        "le code constaté, lui, est publié"
    );

    let refus = lignes_experts(
        &registre(),
        &lrgen_par_party_id("country,party_id,lrgen\n6,999,1.0\n").expect("CSV lisible"),
        &entrees_experts(),
        "2026-08-04",
        "2026-08-27T00:00:00Z",
        "0.4.0",
        "0.1.0",
    );
    assert!(refus.is_err(), "le party_id 901 n'est pas dans la source");
}

// ------------------------------------------------------------- FAM-10 ------

/// Les lignes produites passent les invariants du §6 et la confrontation au
/// registre, et l'identifiant est une fonction de la clé du §3 : deux
/// exécutions rendent le même `id`.
#[test]
fn fam_10_lignes_valides_et_identifiant_stable() {
    let registre = registre();
    for ligne in experts().into_iter().chain(administratif()) {
        let refus = confronter_registre(&ligne, &registre);
        assert!(refus.is_empty(), "confrontation au registre : {refus:?}");
        let rendue = construire(ligne.clone()).expect("ligne construite");
        let relue: Value = serde_json::from_str(&rendue).expect("ligne relue");
        let refus = verifier(&relue);
        assert!(refus.is_empty(), "invariants du §6 : {refus:?}");
        assert_eq!(
            construire(ligne).expect("seconde construction"),
            rendue,
            "même entrée, même octet"
        );
    }
}
