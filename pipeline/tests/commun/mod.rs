// Chaque binaire de test n'utilise qu'une partie de ce module ; ce qu'il
// n'utilise pas n'est pas du code mort du point de vue de la suite.
#![allow(dead_code)]

//! Fixtures partagées par les suites ING et MAT. Hors ligne : tout vient de
//! `docs/brique0/echantillons/`, sauf les mandats notés ci-dessous.

use contrepoint::ingestion::{IndexMandats, Mandat, Scrutin, index_mandats, lire_scrutin};
use contrepoint::matrice::Entete;
use serde_json::Value;

pub const CINQ_SCRUTINS: [&str; 5] = [
    "VTANR5L17V1.json",
    "VTANR5L17V156.json",
    "VTANR5L17V2767.json",
    "VTANR5L17V5268.json",
    "VTANR5L17V6256.json",
];

/// Les 17 votants du bloc `PO0` de `VTANR5L17V6256`. `mandats-gp-l17.json` ne
/// porte que cinq députés, aucun d'eux : l'échantillon dérivé ne couvre pas les
/// acteurs de son propre cas `PO0`. Leurs mandats GP sont relevés verbatim dans
/// AMO30 — `acteur/PA*.json`, `mandats.mandat[]` de `typeOrgane = "GP"` et
/// `legislature = "17"` — où les dix-sept portent `PO845401` du 2024-07-19 à
/// `null`, après un mandat `PO840056` clos le 2024-07-18.
pub const VOTANTS_PO0: [&str; 17] = [
    "PA793290", "PA793362", "PA793616", "PA793656", "PA793672", "PA794598", "PA794954", "PA795900",
    "PA840075", "PA840757", "PA840837", "PA840869", "PA840915", "PA841563", "PA841749", "PA841837",
    "PA842073",
];

pub fn echantillon(nom: &str) -> Value {
    let chemin = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../docs/brique0/echantillons")
        .join(nom);
    let texte = std::fs::read_to_string(&chemin)
        .unwrap_or_else(|e| panic!("échantillon {} illisible : {e}", chemin.display()));
    serde_json::from_str(&texte).unwrap_or_else(|e| {
        panic!(
            "échantillon {} non conforme au JSON : {e}",
            chemin.display()
        )
    })
}

/// L'index des échantillons, complété des mandats des votants `PO0`. Sans eux
/// le bloc `PO0` de `VTANR5L17V6256` ne se résout pas, et ING-17 impose que
/// cela bloque : les cinq scrutins ne se lisent qu'avec cet index.
pub fn index_complet() -> IndexMandats {
    let mut index = index_mandats(&echantillon("mandats-gp-l17.json"));
    for acteur in VOTANTS_PO0 {
        index.insert(
            acteur.to_owned(),
            vec![
                Mandat {
                    groupe: "PO840056".into(),
                    debut: "2024-07-08".into(),
                    fin: Some("2024-07-18".into()),
                },
                Mandat {
                    groupe: "PO845401".into(),
                    debut: "2024-07-19".into(),
                    fin: None,
                },
            ],
        );
    }
    index
}

pub fn lu(nom: &str) -> Scrutin {
    lire_scrutin(&echantillon(nom), &index_complet()).unwrap_or_else(|e| panic!("{nom} : {e}"))
}

/// Empreintes factices : l'en-tête n'a pas à connaître les vraies (MAT-08).
pub fn entete() -> Entete {
    Entete {
        empreinte_scrutins: "a".repeat(64),
        empreinte_amo30: "b".repeat(64),
    }
}

/// Les tolérances de `plan-de-tests.md` §4, dans **un seul** module. T1 est
/// l'égalité d'octet du fichier arrondi : elle n'a pas de constante, et n'est
/// jamais appliquée à un flottant intermédiaire.
///
/// T2 — invariance (permutations, initialisations, idempotence de l'ancrage).
/// Mesuré à 1,6·10⁻¹⁵ : trois ordres de grandeur de marge.
pub const T2: f64 = 1e-12;

/// T3 — non-régression de valeur sur une médiane ancrée, amplitude 2,0.
pub const T3: f64 = 0.02;
