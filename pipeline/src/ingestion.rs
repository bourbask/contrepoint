//! Composant [1] d'`architecture.md` — lecture des scrutins et du référentiel
//! des mandats, jusqu'aux triplets `(acteur, scrutin, valeur)`.
//!
//! Le codage est celui d'`ingestion-votes.md` §5 : `pour` → +1, `contre` → −1,
//! `abstention` → 0, non-votant et absent → **rien**. Une cellule manquante
//! n'est pas écrite ; il n'existe donc aucune valeur à mal interpréter.

use crate::{nombre, un_ou_plusieurs};
use serde_json::Value;
use std::collections::BTreeMap;

/// Les trois causes de non-participation relevées sur le corpus, et elles
/// seules (§3). Le codage n'en dépend pas : elles sont reconnues pour qu'une
/// quatrième, apparue dans la source, ne passe pas pour une position.
/// `organeRef` pendante : aucun `organe/PO0.json` n'existe dans AMO30. 146
/// blocs sur 14 scrutins la portent (§8).
pub const REFERENCE_PENDANTE: &str = "PO0";

pub const CAUSES_DE_NON_VOTANT: [&str; 3] = ["MG", "PAN", "PSE"];

/// Les trois seules positions que la source enregistre (§3).
const POSITIONS: [(&str, &str, i8); 3] = [
    ("pours", "pour", 1),
    ("contres", "contre", -1),
    ("abstentions", "abstentions", 0),
];

/// Un triplet, augmenté du groupe daté auquel il se rattache.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cellule {
    pub acteur: String,
    pub groupe: String,
    pub valeur: i8,
}

/// Un scrutin lu, avec ce que le registre de preuves doit en publier (§9b).
#[derive(Debug, Clone)]
pub struct Scrutin {
    pub uid: String,
    pub legislature: String,
    pub date: String,
    pub code_type_vote: String,
    pub nombre_votants: u64,
    pub suffrages_exprimes: u64,
    pub pour: u64,
    pub contre: u64,
    pub abstentions: u64,
    pub non_votants: u64,
    /// Comptes par cause de non-participation, dans l'ordre de
    /// [`CAUSES_DE_NON_VOTANT`]. Relevés, jamais interprétés.
    pub causes: [usize; 3],
    /// Cellules où le groupe de la ventilation diffère du mandat AMO30 valide
    /// ce jour-là. La ventilation tranche (ADR 0003 §1) ; le désaccord est
    /// compté et exposé, jamais tu.
    pub desaccords: usize,
    /// Entrées nominatives de mise au point : ingérées et comptées, jamais
    /// appliquées (§3). Le déclaratif relève de la deuxième famille de mesure.
    pub mises_au_point: usize,
    /// Cellules des seules positions exprimées, triées par acteur.
    pub cellules: Vec<Cellule>,
}

fn champ<'a>(valeur: &'a Value, chemin: &[&str]) -> Result<&'a Value, String> {
    let mut courant = valeur;
    for clef in chemin {
        courant = courant
            .get(clef)
            .ok_or_else(|| format!("champ absent : {}", chemin.join(".")))?;
    }
    Ok(courant)
}

fn texte(valeur: &Value, chemin: &[&str]) -> Result<String, String> {
    champ(valeur, chemin)?
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("champ non textuel : {}", chemin.join(".")))
}

fn compte(valeur: &Value, chemin: &[&str]) -> Result<u64, String> {
    nombre(champ(valeur, chemin)?).ok_or_else(|| format!("nombre illisible : {}", chemin.join(".")))
}

/// Lit un fichier de scrutin. Toute incohérence est une erreur bloquante :
/// un scrutin lu de travers vaut moins qu'un scrutin non lu.
pub fn lire_scrutin(fichier: &Value, mandats: &IndexMandats) -> Result<Scrutin, String> {
    let s = champ(fichier, &["scrutin"])?;
    let uid = texte(s, &["uid"])?;
    let date = texte(s, &["dateScrutin"])?;
    let synthese = champ(s, &["syntheseVote"])?;
    let decompte = champ(synthese, &["decompte"])?;

    let nombre_votants = compte(synthese, &["nombreVotants"])?;
    let suffrages_exprimes = compte(synthese, &["suffragesExprimes"])?;
    let pour = compte(decompte, &["pour"])?;
    let contre = compte(decompte, &["contre"])?;
    let abstentions = compte(decompte, &["abstentions"])?;
    let non_votants = compte(decompte, &["nonVotants"])?;

    if nombre_votants != pour + contre + abstentions {
        return Err(format!(
            "{uid} : nombreVotants {nombre_votants} ≠ pour {pour} + contre {contre} \
             + abstentions {abstentions}"
        ));
    }
    if suffrages_exprimes != pour + contre {
        return Err(format!(
            "{uid} : suffragesExprimes {suffrages_exprimes} ≠ pour {pour} + contre {contre}"
        ));
    }

    let mut cellules = Vec::new();
    let mut desaccords = 0usize;
    let mut causes = [0usize; 3];
    for bloc in un_ou_plusieurs(champ(
        s,
        &["ventilationVotes", "organe", "groupes", "groupe"],
    )?) {
        let ventilation = texte(bloc, &["organeRef"])?;
        let nominatif = champ(bloc, &["vote", "decompteNominatif"])?;
        let voix = champ(bloc, &["vote", "decompteVoix"])?;

        // Les non-votants ne produisent aucune cellule ; leur cause est
        // seulement contrôlée.
        let non_votants = un_ou_plusieurs(&nominatif["nonVotants"]["votant"]);
        let annonce = compte(voix, &["nonVotants"])?;
        if non_votants.len() as u64 != annonce {
            return Err(format!(
                "{uid}, bloc {ventilation} : {} non-votants nominatifs pour un decompteVoix de \
                 {annonce}",
                non_votants.len()
            ));
        }
        for votant in non_votants {
            let cause = votant["causePositionVote"].as_str().unwrap_or_default();
            match CAUSES_DE_NON_VOTANT.iter().position(|c| *c == cause) {
                Some(rang) => causes[rang] += 1,
                None => {
                    return Err(format!(
                        "{uid}, bloc {ventilation} : cause de non-participation inconnue « {cause} » — \
                     les seules relevées sur le corpus sont {CAUSES_DE_NON_VOTANT:?}"
                    ));
                }
            }
        }

        for (liste, champ_voix, valeur) in POSITIONS {
            let votants = un_ou_plusieurs(&nominatif[liste]["votant"]);
            let annonce = compte(voix, &[champ_voix])?;
            if votants.len() as u64 != annonce {
                return Err(format!(
                    "{uid}, bloc {ventilation} : {} votants nominatifs en `{liste}` pour un \
                     decompteVoix de {annonce}",
                    votants.len()
                ));
            }
            for votant in votants {
                cellules.push(Cellule {
                    acteur: votant["acteurRef"]
                        .as_str()
                        .ok_or_else(|| {
                            format!("{uid}, bloc {ventilation} : votant sans acteurRef")
                        })?
                        .to_owned(),
                    groupe: ventilation.clone(),
                    valeur,
                });
            }
        }
    }

    // Le groupe du bloc de ventilation est la source retenue (ADR 0003 §1).
    // AMO30 n'intervient que pour les blocs `PO0` et pour le contrôle croisé.
    for cellule in &mut cellules {
        let amo30 = groupe_a_la_date(mandats, &cellule.acteur, &date);
        if cellule.groupe == REFERENCE_PENDANTE {
            cellule.groupe = amo30
                .ok_or_else(|| {
                    format!(
                        "{uid} : bloc {REFERENCE_PENDANTE} — l'acteur {} n'a aucun mandat de \
                         groupe le {date}, la référence pendante reste non résolue",
                        cellule.acteur
                    )
                })?
                .to_owned();
        } else if amo30.is_some_and(|g| g != cellule.groupe) {
            desaccords += 1;
        }
    }
    // La résolution d'un bloc `PO0` doit être unanime : sur les 146 blocs du
    // corpus, elle l'est. Une divergence est une ambiguïté, pas un arbitrage.
    for bloc in un_ou_plusieurs(champ(
        s,
        &["ventilationVotes", "organe", "groupes", "groupe"],
    )?) {
        if bloc["organeRef"] != REFERENCE_PENDANTE {
            continue;
        }
        let membres: Vec<&str> = un_ou_plusieurs(champ(bloc, &["vote", "decompteNominatif"])?)
            .iter()
            .flat_map(|n| {
                POSITIONS
                    .iter()
                    .flat_map(|(liste, _, _)| un_ou_plusieurs(&n[liste]["votant"]))
            })
            .filter_map(|v| v["acteurRef"].as_str())
            .collect();
        let mut groupes: Vec<&str> = cellules
            .iter()
            .filter(|c| membres.contains(&c.acteur.as_str()))
            .map(|c| c.groupe.as_str())
            .collect();
        groupes.sort_unstable();
        groupes.dedup();
        if groupes.len() > 1 {
            return Err(format!(
                "{uid} : bloc {REFERENCE_PENDANTE} résolu en {groupes:?} — la résolution \
                 doit être unanime dans le bloc"
            ));
        }
    }

    cellules.sort_by(|a, b| a.acteur.cmp(&b.acteur));
    if let Some(paire) = cellules.windows(2).find(|p| p[0].acteur == p[1].acteur) {
        return Err(format!(
            "{uid} : l'acteur {} porte deux positions dans le même scrutin",
            paire[0].acteur
        ));
    }

    Ok(Scrutin {
        uid,
        legislature: texte(s, &["legislature"])?,
        date,
        code_type_vote: texte(s, &["typeVote", "codeTypeVote"])?,
        nombre_votants,
        suffrages_exprimes,
        pour,
        contre,
        abstentions,
        non_votants,
        causes,
        desaccords,
        mises_au_point: compter_mises_au_point(champ(s, &["miseAuPoint"])?),
        cellules,
    })
}

/// Le groupe valide le jour donné, ou `None`. Jamais le dernier groupe connu :
/// une jointure « dernier groupe » attribuerait aux non-inscrits tout vote
/// antérieur à la constitution des groupes (§8).
pub fn groupe_a_la_date<'a>(
    mandats: &'a IndexMandats,
    acteur: &str,
    date: &str,
) -> Option<&'a str> {
    mandats.get(acteur)?.iter().find_map(|m| {
        (m.debut.as_str() <= date && m.fin.as_deref().is_none_or(|f| date <= f))
            .then_some(m.groupe.as_str())
    })
}

/// Compte les entrées nominatives de `miseAuPoint`, sans nommer aucun de ses
/// champs : les cinq listes n'ont pas la même sérialisation (§4b) et l'une
/// d'elles porte un nom que le pipeline ne doit lire nulle part comme une
/// catégorie de position (§3). `dysfonctionnement` est un autre objet.
fn compter_mises_au_point(mise_au_point: &Value) -> usize {
    let Some(champs) = mise_au_point.as_object() else {
        return 0;
    };
    champs
        .iter()
        .filter(|(clef, _)| *clef != "dysfonctionnement")
        .flat_map(|(_, liste)| un_ou_plusieurs(liste))
        .map(|entree| un_ou_plusieurs(&entree["votant"]).len())
        .sum()
}

/// Index des mandats de groupe politique, par acteur.
pub type IndexMandats = BTreeMap<String, Vec<Mandat>>;

/// Un mandat de groupe politique, dédoublonné (§8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mandat {
    pub groupe: String,
    pub debut: String,
    /// `None` vaut « en cours ».
    pub fin: Option<String>,
}

/// Construit l'index des mandats GP en appliquant la règle de dédoublonnage
/// d'`ingestion-votes.md` §8 : regrouper par `(organeRef, dateDebut)`, retenir
/// la `dateFin` maximale, `null` valant « en cours ».
pub fn index_mandats(fichier: &Value) -> IndexMandats {
    let mut index = IndexMandats::new();
    for acteur in un_ou_plusieurs(&fichier["acteurs"]) {
        let Some(reference) = acteur["acteurRef"].as_str() else {
            continue;
        };
        // La clef porte le dédoublonnage ; la valeur, la `dateFin` maximale.
        let mut retenus: BTreeMap<(String, String), Option<String>> = BTreeMap::new();
        for mandat in un_ou_plusieurs(&acteur["mandatsGP"]) {
            let (Some(groupe), Some(debut)) =
                (mandat["organeRef"].as_str(), mandat["dateDebut"].as_str())
            else {
                continue;
            };
            let fin = mandat["dateFin"].as_str().map(str::to_owned);
            let clef = (groupe.to_owned(), debut.to_owned());
            match retenus.get(&clef) {
                // `None` est le maximum : « en cours » ne finit jamais.
                Some(None) => {}
                Some(Some(connue)) if fin.as_deref().is_some_and(|f| f <= connue.as_str()) => {}
                _ => {
                    retenus.insert(clef, fin);
                }
            }
        }
        index.insert(
            reference.to_owned(),
            retenus
                .into_iter()
                .map(|((groupe, debut), fin)| Mandat { groupe, debut, fin })
                .collect(),
        );
    }
    index
}
