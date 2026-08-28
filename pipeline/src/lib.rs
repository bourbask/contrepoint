//! Les trois adaptateurs de désérialisation de la source de l'Assemblée
//! nationale, mesurés par l'ADR 0001 §1.4 et détaillés par
//! `docs/brique0/ingestion-votes.md` §4.
//!
//! Le JSON publié est une transcription automatique de XML : l'arité et le type
//! y sont perdus. Ces trois fonctions les rendent, et rien d'autre.

pub mod agregation;
pub mod estimateur;
pub mod export;
pub mod familles;
pub mod ingestion;
pub mod matrice;
pub mod preuves;
pub mod registre;
pub mod sha256;

use serde_json::Value;

/// Adaptateur « un-ou-plusieurs » (§4a et §4b).
///
/// Un tableau à un élément est sérialisé en objet nu — 27,3 % des blocs
/// `decompteNominatif.*.votant`. Une liste vide est tantôt `null`, tantôt un
/// tableau dont les éléments sont nuls. Rendu dans tous les cas : la liste des
/// éléments réellement présents.
pub fn un_ou_plusieurs(valeur: &Value) -> Vec<&Value> {
    match valeur {
        Value::Null => Vec::new(),
        Value::Array(elements) => elements.iter().filter(|e| !e.is_null()).collect(),
        autre => vec![autre],
    }
}

/// Adaptateur « chaîne ou objet enveloppé xsi » (§4c).
///
/// `organe.uid` est `"PO845401"` ; `acteur.uid` est
/// `{"@xsi:type": "IdActeur_type", "#text": "PA304016"}`. Les deux rendent
/// l'identifiant. Toute autre forme n'en porte pas.
pub fn uid(valeur: &Value) -> Option<&str> {
    match valeur {
        Value::String(texte) => Some(texte),
        Value::Object(_) => valeur["#text"].as_str(),
        _ => None,
    }
}

/// Adaptateur « nombre sérialisé en chaîne » (§2).
///
/// Toutes les valeurs numériques de la source sont des chaînes. Une chaîne qui
/// n'est pas un entier n'a pas de valeur : l'appelant traite le `None` en
/// erreur bloquante, jamais en `0` — un décompte à zéro se distingue d'un champ
/// illisible.
pub fn nombre(valeur: &Value) -> Option<u64> {
    valeur.as_str()?.parse().ok()
}
