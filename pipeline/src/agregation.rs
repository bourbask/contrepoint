//! Composant [4] d'`architecture.md` — du député au groupe.
//!
//! Spécification : `docs/brique0/positionnement.md` §6, arbitré par l'ADR 0003
//! §3.
//!
//! **La position publiée est celle d'un groupe, jamais d'un député.** La
//! magnitude de la position individuelle croît avec la seule assiduité
//! (corrélation de rang +0,336) et 15,37 % des positions enregistrées sont
//! exprimées par délégation : aucune coordonnée individuelle n'est défendable,
//! donc aucune n'est calculée ici au-delà de ce que la médiane exige.
//!
//! **La dispersion publiée est l'écart interquartile et l'écart-type de
//! rééchantillonnage.** Jamais la variance — illisible sur un axe sans unité —
//! et jamais l'étendue : un minimum et un maximum **sont** les coordonnées de
//! deux membres identifiables du groupe, réidentifiables en une exécution sur
//! un groupe de neuf membres. Ils ne sont ni publiés ni calculés, et ce module
//! n'a aucun emplacement d'où ils pourraient fuir.

use crate::estimateur::mediane;
use serde_json::Value;
use std::fmt::Write as _;

/// Règle de non-publication (positionnement.md §6). Les trois conditions
/// tiennent ensemble ; une seule qui tombe suffit à retirer la valeur.
pub const IQR_MAXIMAL: f64 = 0.25;
pub const ECART_TYPE_MAXIMAL: f64 = 0.05;
pub const EFFECTIF_MINIMAL: usize = 10;

/// En deçà de ce nombre de membres, Q1 est le minimum du groupe : l'écart
/// interquartile y porte une borne d'étendue, que le projet ne calcule ni ne
/// publie (positionnement.md §6, I19).
///
/// Le seuil vaut **six** et non quatre. Avec la médiane basse de
/// [`crate::estimateur::mediane`], la moitié inférieure d'un groupe de quatre
/// ou cinq membres compte deux éléments, dont la médiane basse est le premier
/// — c'est-à-dire le minimum du groupe. Six est le premier effectif où ni Q1
/// ni Q3 n'est un extrême. Recompté sur n = 4 à 10, pas déduit du nom
/// « charnières » : la méthode ne moyenne rien.
pub const MEMBRES_MINIMAUX_POUR_LIQR: usize = 6;

/// Un député n'entre dans la médiane de son groupe qu'au-delà de ce nombre de
/// votes exprimés. Le seuil ne porte **que** là : il n'est jamais un filtre
/// d'entrée dans la matrice, où il appauvrirait le corpus sans corriger le
/// mécanisme d'absence.
pub const VOTES_MINIMAUX: usize = 200;

/// Motifs d'absence. Une case non mesurée porte sa raison ; elle ne porte
/// jamais une valeur accompagnée d'un avertissement, qui serait citée sans
/// l'avertissement.
pub const EFFECTIF_INSUFFISANT: &str = "effectif_insuffisant";
pub const DISPERSION_INTERNE: &str = "dispersion_interne";
pub const DISPERSION_REECHANTILLONNAGE: &str = "dispersion_de_reechantillonnage";

/// Un député rattaché à son groupe daté, avec sa position sur l'axe ancré.
/// Cette structure est une **entrée de calcul** : elle ne sort d'aucune
/// fonction de rendu.
#[derive(Debug, Clone)]
pub struct Membre {
    pub acteur: String,
    pub groupe: String,
    pub votes_exprimes: usize,
    pub position: f64,
}

/// Ce qui s'affiche pour un groupe : une mesure, ou une absence dite.
#[derive(Debug, Clone, PartialEq)]
pub enum Publication {
    Mesuree {
        mediane: f64,
        iqr: f64,
        ecart_type_reechantillonnage: f64,
    },
    /// Le motif, **et les chiffres qui l'ont déclenché**. Le §2.4 les veut
    /// publiés : « les chiffres qui justifient la non-publication sont publiés,
    /// la valeur non ». Un motif qui annonce le seuil sans la mesure se lit
    /// comme la mesure. `None` quand la statistique n'est pas calculable —
    /// jamais un zéro de remplissage.
    NonMesuree {
        motif: &'static str,
        iqr: Option<f64>,
        ecart_type_reechantillonnage: Option<f64>,
    },
}

/// La ligne d'un groupe. La valeur ne voyage jamais sans son effectif et sa
/// date — ADR 0000 §5, « chiffres : jamais seuls ».
#[derive(Debug, Clone)]
pub struct Position {
    pub groupe: String,
    pub effectif_retenu: usize,
    pub date_de_reference: String,
    pub publication: Publication,
}

/// Les `uid_an` des groupes dont la période de validité couvre la date de
/// référence, en ordre d'octets. Un groupe éteint est **absent**, jamais
/// présenté avec un effectif résiduel.
pub fn groupes_valides(registre: &Value, date: &str) -> Vec<String> {
    let mut valides: Vec<String> = registre["groupes"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
        .iter()
        .filter(|groupe| {
            let debut = groupe["debut"].as_str().unwrap_or("9999-99-99");
            let fin = groupe["fin"].as_str();
            debut <= date && !fin.is_some_and(|f| f < date)
        })
        .filter_map(|groupe| groupe["uid_an"].as_str().map(str::to_owned))
        .collect();
    valides.sort();
    valides
}

/// Agrège les positions ancrées par groupe, à une **date de référence
/// explicite** : aucune horloge n'est lue, deux exécutions du même jeu donnent
/// la même sortie.
///
/// `reechantillons[k][n]` est la position du membre `membres[n]` dans le
/// tirage `k`. L'écart-type de rééchantillonnage de la médiane s'en déduit ;
/// sans au moins deux tirages il n'est pas mesurable, et le groupe passe en non
/// mesuré plutôt que de sortir avec une dispersion inventée.
pub fn agreger(
    membres: &[Membre],
    reechantillons: &[Vec<f64>],
    groupes: &[String],
    date_de_reference: &str,
) -> Vec<Position> {
    let mut lignes = Vec::new();
    for groupe in groupes {
        if !membres.iter().any(|m| m.groupe == *groupe) {
            continue;
        }
        // Le seuil de votes ne s'applique qu'ici : les membres écartés sont
        // entrés dans la matrice et dans l'estimation.
        let retenus: Vec<usize> = membres
            .iter()
            .enumerate()
            .filter(|(_, m)| m.groupe == *groupe && m.votes_exprimes >= VOTES_MINIMAUX)
            .map(|(n, _)| n)
            .collect();
        lignes.push(Position {
            groupe: groupe.clone(),
            effectif_retenu: retenus.len(),
            date_de_reference: date_de_reference.to_owned(),
            publication: publier(membres, reechantillons, &retenus),
        });
    }
    lignes
}

fn publier(membres: &[Membre], reechantillons: &[Vec<f64>], retenus: &[usize]) -> Publication {
    // Les deux dispersions sont calculées **avant** la règle de non-publication :
    // ce sont elles qui la justifient, et le §2.4 exige qu'elles soient publiées
    // même quand la valeur ne l'est pas. Le seuil d'effectif ne les supprime
    // pas — positionnement.md §6 publie l'IQR de NI, neuf membres.
    let positions: Vec<f64> = retenus.iter().map(|n| membres[*n].position).collect();
    let centre = centre_et_dispersion(&positions);
    let iqr = centre.map(|(_, iqr)| iqr);
    let ecart_type = ecart_type_de_reechantillonnage(reechantillons, retenus);
    if retenus.len() < EFFECTIF_MINIMAL {
        return Publication::NonMesuree {
            motif: EFFECTIF_INSUFFISANT,
            iqr,
            ecart_type_reechantillonnage: ecart_type,
        };
    }
    let Some((mediane_groupe, iqr_mesure)) = centre else {
        return Publication::NonMesuree {
            motif: EFFECTIF_INSUFFISANT,
            iqr: None,
            ecart_type_reechantillonnage: ecart_type,
        };
    };
    if iqr_mesure > IQR_MAXIMAL {
        return Publication::NonMesuree {
            motif: DISPERSION_INTERNE,
            iqr,
            ecart_type_reechantillonnage: ecart_type,
        };
    }
    let Some(ecart_type_mesure) = ecart_type else {
        return Publication::NonMesuree {
            motif: DISPERSION_REECHANTILLONNAGE,
            iqr,
            ecart_type_reechantillonnage: None,
        };
    };
    if ecart_type_mesure > ECART_TYPE_MAXIMAL {
        return Publication::NonMesuree {
            motif: DISPERSION_REECHANTILLONNAGE,
            iqr,
            ecart_type_reechantillonnage: ecart_type,
        };
    }
    Publication::Mesuree {
        mediane: mediane_groupe,
        iqr: iqr_mesure,
        ecart_type_reechantillonnage: ecart_type_mesure,
    }
}

/// Médiane et écart interquartile. Les quartiles sont les charnières de Tukey,
/// avec la médiane basse de l'estimateur des deux côtés : **un seul estimateur
/// de position dans toute la chaîne**, celui qui définit déjà l'ancrage.
///
/// Aucune autre statistique d'ordre n'est calculée. En deçà de six membres, Q1
/// est le minimum : l'IQR y porterait la coordonnée d'un membre identifiable
/// (I19), et rien n'est calculé. Au-delà, l'IQR est une différence et non une coordonnée — il est
/// calculé même sous le seuil d'effectif, parce que c'est lui qui justifie la
/// non-publication (§2.4).
fn centre_et_dispersion(positions: &[f64]) -> Option<(f64, f64)> {
    if positions.len() < MEMBRES_MINIMAUX_POUR_LIQR {
        return None;
    }
    let mut triees = positions.to_vec();
    triees.sort_by(f64::total_cmp);
    let moitie = triees.len() / 2;
    let basse = &triees[..moitie];
    let haute = &triees[triees.len() - moitie..];
    Some((mediane(&triees)?, mediane(haute)? - mediane(basse)?))
}

/// Écart-type de la médiane du groupe à travers les tirages, par la formule du
/// jackknife : `√((K−1)/K · Σ(θₖ − θ̄)²)`. Les tirages sont des plis déterminés,
/// jamais des tirages aléatoires — aucun générateur n'entre dans le pipeline.
fn ecart_type_de_reechantillonnage(reechantillons: &[Vec<f64>], retenus: &[usize]) -> Option<f64> {
    if reechantillons.len() < 2 {
        return None;
    }
    let mut medianes = Vec::with_capacity(reechantillons.len());
    for tirage in reechantillons {
        let positions: Vec<f64> = retenus
            .iter()
            .map(|n| tirage.get(*n).copied().unwrap_or(f64::NAN))
            .collect();
        if positions.iter().any(|v| v.is_nan()) {
            return None;
        }
        medianes.push(mediane(&positions)?);
    }
    let k = medianes.len() as f64;
    let centre = medianes.iter().sum::<f64>() / k;
    let carres: f64 = medianes.iter().map(|m| (m - centre).powi(2)).sum();
    Some(((k - 1.0) / k * carres).sqrt())
}

/// L'artefact d'agrégation : une ligne par groupe, arrondie à quatre décimales
/// avant écriture — c'est le fichier arrondi qui est identique à l'octet d'une
/// exécution à l'autre, jamais le flottant intermédiaire (ADR 0001 §1.7).
///
/// Une ligne `G` porte une mesure, une ligne `N` porte une absence et son
/// motif. Aucune des deux ne porte d'identifiant d'acteur.
pub fn rendre(positions: &[Position]) -> String {
    let mut sortie = String::new();
    if let Some(premiere) = positions.first() {
        let _ = writeln!(
            sortie,
            "# date-de-reference\t{}",
            premiere.date_de_reference
        );
    }
    let _ = writeln!(
        sortie,
        "# colonnes\tgroupe\teffectif-retenu\tmediane\tiqr\tecart-type-reechantillonnage"
    );
    for ligne in positions {
        match &ligne.publication {
            Publication::Mesuree {
                mediane,
                iqr,
                ecart_type_reechantillonnage,
            } => {
                let _ = writeln!(
                    sortie,
                    "G\t{}\t{}\t{mediane:.4}\t{iqr:.4}\t{ecart_type_reechantillonnage:.4}",
                    ligne.groupe, ligne.effectif_retenu
                );
            }
            // Une ligne `N` porte son motif **et** les chiffres qui l'ont
            // déclenché : un motif seul se lit comme un seuil, pas comme une
            // mesure (§2.4). `-` là où la statistique n'est pas calculable.
            Publication::NonMesuree {
                motif,
                iqr,
                ecart_type_reechantillonnage,
            } => {
                let chiffre = |x: Option<f64>| x.map_or("-".to_owned(), |v| format!("{v:.4}"));
                let _ = writeln!(
                    sortie,
                    "N\t{}\t{}\t{motif}\t{}\t{}",
                    ligne.groupe,
                    ligne.effectif_retenu,
                    chiffre(*iqr),
                    chiffre(*ecart_type_reechantillonnage)
                );
            }
        }
    }
    sortie
}
