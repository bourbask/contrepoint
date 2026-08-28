//! Suite AGR — du député au groupe (plan-de-tests.md §8).
//!
//! Hors ligne. Les groupes sont construits ici ; le registre vient de
//! `docs/brique0/echantillons/registre-l17.json`.
//!
//! Aucune coordonnée individuelle n'entre dans une sortie : ni valeur, ni
//! minimum, ni maximum, ni rang.

mod commun;

use commun::T2;
use contrepoint::agregation::{
    DISPERSION_INTERNE, DISPERSION_REECHANTILLONNAGE, ECART_TYPE_MAXIMAL, EFFECTIF_INSUFFISANT,
    EFFECTIF_MINIMAL, IQR_MAXIMAL, Membre, Publication, VOTES_MINIMAUX, agreger, groupes_valides,
    rendre,
};
use serde_json::Value;

const LFI: &str = "PO845413";
const RN: &str = "PO845401";
const NI: &str = "PO840056";
const AD: &str = "PO845520";
const UDR: &str = "PO847173";
const UDDPLR: &str = "PO872880";
const DATE: &str = "2026-07-21";

fn registre() -> Value {
    let chemin = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../docs/brique0/echantillons/registre-l17.json");
    serde_json::from_str(&std::fs::read_to_string(chemin).expect("registre lisible"))
        .expect("registre conforme")
}

fn membres(groupe: &str, positions: &[f64], votes: usize) -> Vec<Membre> {
    positions
        .iter()
        .enumerate()
        .map(|(n, position)| Membre {
            acteur: format!("PA{:05}", 10_000 + n + groupe.len() * 1_000),
            groupe: groupe.to_owned(),
            votes_exprimes: votes,
            position: *position,
        })
        .collect()
}

/// Douze positions serrées autour de `centre`, écartées de `demi_ecart`.
fn serre(centre: f64, demi_ecart: f64) -> Vec<f64> {
    (0..12)
        .map(|n| centre + demi_ecart * (n as f64 - 5.5) / 5.5)
        .collect()
}

/// Rééchantillons : chaque tirage décale toutes les positions d'un pas fixe.
/// Aucun générateur — les décalages sont écrits.
fn reechantillons(membres: &[Membre], decalages: &[f64]) -> Vec<Vec<f64>> {
    decalages
        .iter()
        .map(|d| membres.iter().map(|m| m.position + d).collect())
        .collect()
}

fn mesure(publication: &Publication) -> (f64, f64, f64) {
    match publication {
        Publication::Mesuree {
            mediane,
            iqr,
            ecart_type_reechantillonnage,
        } => (*mediane, *iqr, *ecart_type_reechantillonnage),
        Publication::NonMesuree { motif } => panic!("groupe non mesuré : {motif}"),
    }
}

fn motif(publication: &Publication) -> &str {
    match publication {
        Publication::NonMesuree { motif } => motif,
        Publication::Mesuree { mediane, .. } => panic!("groupe mesuré à {mediane}"),
    }
}

/// AGR-01 [C] — aucun `PA…` accompagné d'une valeur de position dans la sortie.
/// Se casse en ajoutant un champ « pour déboguer ».
#[test]
fn aucune_coordonnee_individuelle_en_sortie() {
    let mut liste = membres(LFI, &serre(-1.0, 0.05), 900);
    liste.extend(membres(RN, &serre(1.0, 0.05), 900));
    let tirages = reechantillons(&liste, &[0.002, -0.001, 0.0, 0.001]);
    let sortie = rendre(&agreger(
        &liste,
        &tirages,
        &[LFI.to_owned(), RN.to_owned()],
        DATE,
    ));
    assert!(
        !sortie.contains("PA"),
        "AGR-01 : un identifiant d'acteur dans la sortie (ADR 0000 §2, RG-41)\n{sortie}"
    );
    for interdit in ["min", "max", "etendue", "rang", "variance"] {
        assert!(
            !sortie.contains(interdit),
            "AGR-01 : « {interdit} » dans la sortie — une borne est la coordonnée d'un \
             membre identifiable\n{sortie}"
        );
    }
}

/// AGR-02 — la valeur publiée est la médiane, pas la moyenne. Sur un groupe
/// hétérogène l'écart atteint le tiers de la valeur.
#[test]
fn mediane_de_groupe_et_pas_moyenne() {
    let positions = [
        0.10, 0.11, 0.12, 0.13, 0.14, 0.15, 0.16, 0.17, 0.18, 0.19, 0.90, 0.95,
    ];
    let liste = membres(LFI, &positions, 900);
    let tirages = reechantillons(&liste, &[0.001, -0.001, 0.0]);
    let resultat = agreger(&liste, &tirages, &[LFI.to_owned()], DATE);
    let (mediane, _, _) = mesure(&resultat[0].publication);
    let moyenne = positions.iter().sum::<f64>() / positions.len() as f64;
    assert!(
        (mediane - 0.15).abs() <= T2,
        "AGR-02 : médiane basse des douze valeurs, obtenu {mediane}"
    );
    assert!(
        (mediane - moyenne).abs() > 0.05,
        "AGR-02 : le cas de test doit séparer médiane et moyenne"
    );
}

/// AGR-03 — la dispersion publiée est l'écart interquartile et l'écart-type de
/// rééchantillonnage. **Jamais la variance, jamais l'étendue** (ADR 0003 §3).
#[test]
fn dispersion_publiee_est_iqr_et_reechantillonnage() {
    let positions: Vec<f64> = (0..12).map(|n| n as f64 / 100.0).collect();
    let liste = membres(LFI, &positions, 900);
    let tirages = reechantillons(&liste, &[0.010, -0.010, 0.0, 0.020]);
    let resultat = agreger(&liste, &tirages, &[LFI.to_owned()], DATE);
    let (_, iqr, ecart_type) = mesure(&resultat[0].publication);
    // Charnières de Tukey sur douze valeurs 0,00 … 0,11 : Q1 = 0,02, Q3 = 0,08.
    assert!(
        (iqr - 0.06).abs() <= T2,
        "AGR-03 : écart interquartile attendu 0,06, obtenu {iqr}"
    );
    assert!(
        ecart_type > 0.0,
        "AGR-03 : l'écart-type de rééchantillonnage est publié avec la médiane"
    );
    let variance: f64 = {
        let moyenne = positions.iter().sum::<f64>() / positions.len() as f64;
        positions.iter().map(|v| (v - moyenne).powi(2)).sum::<f64>() / positions.len() as f64
    };
    assert!(
        (iqr - variance).abs() > 1e-6,
        "AGR-03 : la valeur publiée ne doit pas être une variance"
    );
    let sortie = rendre(&resultat);
    assert!(
        sortie.contains("iqr") && sortie.contains("ecart-type-reechantillonnage"),
        "AGR-03 : les deux mesures de dispersion sont nommées dans la sortie\n{sortie}"
    );
}

/// AGR-04 — règle de non-publication : IQR > 0,25, écart-type de
/// rééchantillonnage > 0,05, ou effectif retenu < 10. Case « non mesuré »
/// **avec sa raison**, jamais une médiane accompagnée d'un avertissement.
#[test]
fn regle_de_non_publication() {
    // Dispersion interne : NI et LIOT échouent ainsi sur le corpus réel.
    let disperse = membres(NI, &serre(0.27, 0.35), 900);
    let tirages = reechantillons(&disperse, &[0.001, -0.001, 0.0]);
    let resultat = agreger(&disperse, &tirages, &[NI.to_owned()], DATE);
    assert_eq!(
        motif(&resultat[0].publication),
        DISPERSION_INTERNE,
        "AGR-04 : IQR au-delà de {IQR_MAXIMAL}"
    );

    // Dispersion de rééchantillonnage.
    let flottant = membres(RN, &serre(1.0, 0.05), 900);
    let tirages = reechantillons(&flottant, &[0.2, -0.2, 0.0, 0.1]);
    let resultat = agreger(&flottant, &tirages, &[RN.to_owned()], DATE);
    assert_eq!(
        motif(&resultat[0].publication),
        DISPERSION_REECHANTILLONNAGE,
        "AGR-04 : écart-type de rééchantillonnage au-delà de {ECART_TYPE_MAXIMAL}"
    );

    // Effectif retenu insuffisant.
    let petit = membres(LFI, &[-1.0, -0.99, -0.98, -0.97, -0.96], 900);
    let tirages = reechantillons(&petit, &[0.001, 0.0, -0.001]);
    let resultat = agreger(&petit, &tirages, &[LFI.to_owned()], DATE);
    assert_eq!(
        motif(&resultat[0].publication),
        EFFECTIF_INSUFFISANT,
        "AGR-04 : effectif sous {EFFECTIF_MINIMAL}"
    );

    let sortie = rendre(&resultat);
    assert!(
        sortie.contains(EFFECTIF_INSUFFISANT),
        "AGR-04 : le motif d'absence est publié\n{sortie}"
    );
    assert!(
        !sortie.contains("-1.0") && !sortie.contains("-0.9"),
        "AGR-04 : aucune valeur médiane ne sort d'un groupe non publié\n{sortie}"
    );
}

/// AGR-05 — un groupe éteint à la date de référence est absent, pas présenté
/// avec un effectif résiduel.
#[test]
fn groupe_eteint_a_la_date_de_reference_absent() {
    let valides = groupes_valides(&registre(), DATE);
    assert!(
        !valides.contains(&AD.to_owned()),
        "AGR-05 : AD (dissous le 2024-09-11) est absent au {DATE}"
    );
    assert!(
        !valides.contains(&UDR.to_owned()),
        "AGR-05 : UDR (PO847173, clos le 2025-09-04) est absent au {DATE}"
    );
    assert!(
        valides.contains(&UDDPLR.to_owned()),
        "AGR-05 : UDDPLR (PO872880, ouvert le 2025-09-05) est présent au {DATE}"
    );
    let avant = groupes_valides(&registre(), "2025-01-15");
    assert!(
        avant.contains(&UDR.to_owned()) && !avant.contains(&UDDPLR.to_owned()),
        "AGR-05 : au 2025-01-15 c'est l'inverse"
    );
    // Un membre d'un groupe éteint ne fabrique pas une ligne.
    let liste = membres(AD, &serre(0.5, 0.05), 900);
    let tirages = reechantillons(&liste, &[0.001, 0.0, -0.001]);
    assert!(
        agreger(&liste, &tirages, &valides, DATE).is_empty(),
        "AGR-05 : aucune ligne pour un groupe hors de sa période de validité"
    );
}

/// AGR-06 [C] — la date de référence est une entrée, jamais une lecture
/// d'horloge.
#[test]
fn date_de_reference_est_une_entree() {
    let mut liste = membres(UDR, &serre(0.99, 0.04), 900);
    liste.extend(membres(UDDPLR, &serre(0.99, 0.04), 900));
    let tirages = reechantillons(&liste, &[0.001, 0.0, -0.001]);

    let tard = agreger(&liste, &tirages, &groupes_valides(&registre(), DATE), DATE);
    let tot = agreger(
        &liste,
        &tirages,
        &groupes_valides(&registre(), "2025-01-15"),
        "2025-01-15",
    );
    assert_eq!(
        tard.iter().map(|p| p.groupe.as_str()).collect::<Vec<_>>(),
        [UDDPLR],
        "AGR-06 : au {DATE}, seul PO872880"
    );
    assert_eq!(
        tot.iter().map(|p| p.groupe.as_str()).collect::<Vec<_>>(),
        [UDR],
        "AGR-06 : au 2025-01-15, seul PO847173"
    );
    assert_eq!(tot[0].date_de_reference, "2025-01-15");
    let texte = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/agregation.rs"),
    )
    .expect("module lisible");
    for interdit in ["SystemTime", "Instant", "now("] {
        assert!(
            !texte.contains(interdit),
            "AGR-06 : « {interdit} » dans le module — la date viendrait de l'horloge"
        );
    }
}

/// AGR-07 — chaque valeur porte son effectif retenu et sa date. ADR 0000 §5 :
/// « Chiffres : jamais seuls ».
#[test]
fn effectif_retenu_publie_avec_la_valeur() {
    let liste = membres(LFI, &serre(-1.0, 0.05), 900);
    let tirages = reechantillons(&liste, &[0.001, 0.0, -0.001]);
    let resultat = agreger(&liste, &tirages, &[LFI.to_owned()], DATE);
    assert_eq!(resultat[0].effectif_retenu, 12, "AGR-07 : effectif retenu");
    assert_eq!(resultat[0].date_de_reference, DATE, "AGR-07 : date");
    let sortie = rendre(&resultat);
    assert!(
        sortie.contains(DATE) && sortie.contains("\t12\t"),
        "AGR-07 : effectif et date accompagnent la valeur\n{sortie}"
    );
}

/// AGR-08 — le seuil de 200 votes exprimés ne porte que sur l'entrée dans la
/// médiane du groupe, jamais sur l'entrée dans la matrice.
#[test]
fn seuil_de_200_votes_ne_sapplique_quau_calcul_de_la_mediane() {
    let mut liste = membres(LFI, &serre(-1.0, 0.05), 900);
    let mut rare = Membre {
        acteur: "PA99999".to_owned(),
        groupe: LFI.to_owned(),
        votes_exprimes: 12,
        position: 4.0,
    };
    rare.position = 4.0;
    liste.push(rare);
    let tirages = reechantillons(&liste, &[0.001, 0.0, -0.001]);
    let resultat = agreger(&liste, &tirages, &[LFI.to_owned()], DATE);
    assert_eq!(
        resultat[0].effectif_retenu, 12,
        "AGR-08 : le membre à 12 votes n'entre pas dans l'effectif retenu (seuil \
         {VOTES_MINIMAUX})"
    );
    let (mediane, iqr, _) = mesure(&resultat[0].publication);
    assert!(
        (mediane + 1.0).abs() <= 0.03 && iqr < 0.1,
        "AGR-08 : sa position de 4,0 n'entre ni dans la médiane ni dans l'IQR — \
         obtenu médiane {mediane}, iqr {iqr}"
    );
    assert_eq!(
        liste.len(),
        13,
        "AGR-08 : il reste dans la matrice et dans l'estimation"
    );
}

/// AGR-09 — deux `uid` d'une même entité ne sont pas fusionnés par
/// l'estimateur. La réconciliation appartient au registre d'entités.
#[test]
fn deux_uid_pour_une_meme_entite_ne_sont_pas_fusionnes_par_lestimateur() {
    let mut liste = membres(UDR, &serre(0.98, 0.04), 900);
    liste.extend(membres(UDDPLR, &serre(0.99, 0.04), 900));
    let tirages = reechantillons(&liste, &[0.001, 0.0, -0.001]);
    let resultat = agreger(&liste, &tirages, &[UDR.to_owned(), UDDPLR.to_owned()], DATE);
    assert_eq!(
        resultat
            .iter()
            .map(|p| p.groupe.as_str())
            .collect::<Vec<_>>(),
        [UDR, UDDPLR],
        "AGR-09 : deux lignes distinctes, jamais une fusion par sigle"
    );
    for ligne in &resultat {
        assert_eq!(ligne.effectif_retenu, 12, "AGR-09 : effectifs non cumulés");
    }
}
