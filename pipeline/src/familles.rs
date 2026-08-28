//! Les deux familles de mesure qui ne viennent pas des votes : `experts`, lue
//! dans le Chapel Hill Expert Survey, et `administratif`, lue dans les codes de
//! nuance attribués par le ministère de l'intérieur.
//!
//! Spécification : `docs/brique0/contrats.md` §2.2 à §2.5,
//! `docs/brique0/registre-entites.md` §2.3 et §2.5.
//!
//! **Les trois familles ne sont jamais moyennées** (règle non négociable n° 6).
//! Ce module ne porte aucune fonction qui prenne deux familles en entrée : il
//! produit des lignes, chacune avec son échelle nommée, et il n'existe nulle
//! part où écrire une valeur qui n'appartienne à aucune famille (§2.1).
//!
//! **Aucun des deux fichiers n'est redistribué.** CHES ne publie aucune licence
//! et la condition obtenue le 2026-08-27 est une exigence de citation, pas une
//! cession de droits (ADR 0000 §8, RG-118) : la citation est portée par
//! `entrees[].citation`, et `scripts/archives.sh` refuse le dépôt. Le nuancier
//! est en Licence Ouverte v2.0, mais c'est une **classification administrative**
//! — révisée par circulaire, contestée devant le Conseil d'État — publiée comme
//! un point de mesure daté et sourcé, jamais comme une vérité de référence.

use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

// -------------------------------------------------------- famille experts ---

pub const SOURCE_EXPERTS: &str = "ches_2024";
pub const FAMILLE_EXPERTS: &str = "experts";
pub const METHODE_EXPERTS: &str = "ches_lrgen";
pub const VERSION_METHODE_EXPERTS: &str = "1.0.0";
pub const ECHELLE_EXPERTS: &str = "ches_lrgen_0_10";
pub const LIBELLE_ECHELLE_EXPERTS: &str = "CHES 2024, variable lrgen, échelle 0 à 10";
pub const COLONNE_EXPERTS: &str = "lrgen";
pub const VAGUE_EXPERTS: &str = "2024";
/// `country = 6` est la France (registre-entites.md §2.3). `party_id` n'est
/// unique que dans un pays : sans ce filtre, une entité française reçoit la
/// position d'un parti étranger portant le même numéro.
pub const PAYS_EXPERTS: u64 = 6;
/// La vague décrit une année de terrain, et c'est elle que la ligne observe.
const OBSERVATION_EXPERTS: (&str, &str) = ("2024-01-01", "2024-12-31");

// -------------------------------------------------- famille administratif ---

pub const SOURCE_ADMINISTRATIF: &str = "nuance_leg2024";
pub const FAMILLE_ADMINISTRATIF: &str = "administratif";
pub const METHODE_ADMINISTRATIF: &str = "nuance_constatee";
pub const VERSION_METHODE_ADMINISTRATIF: &str = "1.0.0";
pub const ECHELLE_ADMINISTRATIF: &str = "nuance_leg2024";
pub const LIBELLE_ECHELLE_ADMINISTRATIF: &str =
    "Ministère de l'intérieur — code de nuance, législatives 2024";
/// Le préfixe des seules colonnes lues. `Nom candidat n`, `Prénom candidat n`,
/// `Sexe candidat n` et `Elu n` existent dans la source et ne sont **jamais**
/// atteintes : RG-111 interdit d'ingérer une colonne nominative, et une nuance
/// administrative n'est jamais rattachée à une personne physique.
pub const COLONNE_ADMINISTRATIF: &str = "Nuance candidat";
const REFERENCE_GRILLE: &str = "IOMA2415630C du 2024-06-11";
const TOUR_ADMINISTRATIF: &str = "2";
/// Les deux tours des législatives de 2024, bornes incluses.
const OBSERVATION_ADMINISTRATIF: (&str, &str) = ("2024-06-30", "2024-07-07");

/// Le code d'absence retenu quand le registre n'apparie pas une entité à une
/// source : « l'entité n'existe pas dans cette source » (contrats.md §2.4).
const MOTIF_PAR_DEFAUT: &str = "hors_source";

/// L'unique exception, déclarée et datée. Le §2.4 la nomme lui-même : deux codes
/// écologistes sont constatés en 2024 — `ECO` et `VEC` — et l'annexe de
/// l'instruction qui les départage répond 403 à toute requête non navigateur
/// (sources.md §1.4). La source ne tranche pas ; ce n'est pas une absence.
///
/// C'est une **décision de méthode**, pas une lecture de source, au même titre
/// que le champ `ancre_axe` du registre : elle est écrite ici, une seule fois,
/// et tout autre cas retombe sur `hors_source`.
const MOTIFS_PARTICULIERS: [(&str, &str, &str); 1] =
    [("nuance_leg2024", "parti.ecologistes", "source_indeterminee")];

/// Le code d'absence d'une entité que le registre n'apparie pas à cette source.
#[must_use]
pub fn motif_code(source: &str, entite: &str) -> &'static str {
    MOTIFS_PARTICULIERS
        .iter()
        .find(|(s, e, _)| *s == source && *e == entite)
        .map_or(MOTIF_PAR_DEFAUT, |(_, _, code)| *code)
}

// -------------------------------------------------------- lecture des CSV ---

/// Lecteur de CSV minimal : séparateur au choix, guillemets doublés, `CRLF`
/// toléré. Les deux sources ne parlent pas le même dialecte — CHES sépare par
/// virgule sans guillemeter, le ministère sépare par point-virgule en
/// guillemetant —, et un lecteur qui découpe sur le séparateur sans connaître
/// les guillemets coupe `"DUPOND;LE JEUNE"` en deux champs : toutes les
/// colonnes suivantes se décalent et la nuance lue devient celle d'à côté.
///
/// Écrit ici plutôt qu'emprunté : trente lignes de bibliothèque standard contre
/// une dépendance de plus dans un pipeline dont la reproductibilité se démontre
/// (ADR 0001).
#[must_use]
pub fn lire_csv(texte: &str, separateur: char) -> Vec<Vec<String>> {
    let mut lignes: Vec<Vec<String>> = Vec::new();
    let mut ligne: Vec<String> = Vec::new();
    let mut champ = String::new();
    let mut entre_guillemets = false;
    let mut caracteres = texte.chars().peekable();
    while let Some(c) = caracteres.next() {
        if entre_guillemets {
            if c == '"' {
                if caracteres.peek() == Some(&'"') {
                    caracteres.next();
                    champ.push('"');
                } else {
                    entre_guillemets = false;
                }
            } else {
                champ.push(c);
            }
        } else if c == '"' && champ.is_empty() {
            entre_guillemets = true;
        } else if c == separateur {
            ligne.push(std::mem::take(&mut champ));
        } else if c == '\n' {
            ligne.push(std::mem::take(&mut champ));
            lignes.push(std::mem::take(&mut ligne));
        } else if c != '\r' {
            champ.push(c);
        }
    }
    if !champ.is_empty() || !ligne.is_empty() {
        ligne.push(champ);
        lignes.push(ligne);
    }
    lignes
}

/// Le rang d'une colonne nommée. Une colonne attendue et absente est un refus,
/// jamais une valeur par défaut : la source aurait changé de forme sans le dire.
fn colonne(entete: &[String], nom: &str) -> Result<usize, String> {
    entete.iter().position(|c| c == nom).ok_or_else(|| {
        format!("colonne `{nom}` absente de l'en-tête — la source a changé de forme")
    })
}

/// La position `lrgen` de chaque parti français de la vague, indexée par
/// `party_id`. Une ligne sans valeur exploitable n'entre pas : elle sera dite
/// absente par son motif, jamais comblée par un zéro.
///
/// # Errors
///
/// Si l'en-tête ne porte pas `country`, `party_id` ou la colonne mesurée.
pub fn lrgen_par_party_id(csv: &str) -> Result<BTreeMap<String, f64>, String> {
    let lignes = lire_csv(csv, ',');
    let entete = lignes
        .first()
        .ok_or_else(|| "CSV CHES vide : aucun en-tête".to_owned())?;
    let pays = colonne(entete, "country")?;
    let identifiant = colonne(entete, "party_id")?;
    let mesure = colonne(entete, COLONNE_EXPERTS)?;
    let mut positions = BTreeMap::new();
    for ligne in lignes.iter().skip(1) {
        let lire = |rang: usize| ligne.get(rang).map(String::as_str).unwrap_or_default();
        if lire(pays).trim().parse::<u64>() != Ok(PAYS_EXPERTS) {
            continue;
        }
        if let Ok(valeur) = lire(mesure).trim().parse::<f64>() {
            positions.insert(lire(identifiant).trim().to_owned(), valeur);
        }
    }
    Ok(positions)
}

/// L'ensemble des codes de nuance **constatés** dans le fichier de résultats.
///
/// Seules les colonnes dont l'en-tête commence par `Nuance candidat` sont lues.
/// C'est la forme mécanique de RG-111 : les colonnes nominatives de la même
/// source ne sont pas filtrées après coup, elles ne sont jamais atteintes.
///
/// # Errors
///
/// Si aucune colonne de nuance ne figure dans l'en-tête.
pub fn codes_constates(csv: &str) -> Result<BTreeSet<String>, String> {
    let lignes = lire_csv(csv, ';');
    let entete = lignes
        .first()
        .ok_or_else(|| "CSV de nuances vide : aucun en-tête".to_owned())?;
    let rangs: Vec<usize> = entete
        .iter()
        .enumerate()
        .filter(|(_, c)| c.starts_with(COLONNE_ADMINISTRATIF))
        .map(|(rang, _)| rang)
        .collect();
    if rangs.is_empty() {
        return Err(format!(
            "aucune colonne `{COLONNE_ADMINISTRATIF} n` dans l'en-tête — la source a changé de forme"
        ));
    }
    let mut codes = BTreeSet::new();
    for ligne in lignes.iter().skip(1) {
        for rang in &rangs {
            let code = ligne
                .get(*rang)
                .map(String::as_str)
                .unwrap_or_default()
                .trim();
            if !code.is_empty() {
                codes.insert(code.to_owned());
            }
        }
    }
    Ok(codes)
}

// -------------------------------------------------- appariement au registre --

/// Ce que le **registre d'entités** déclare de l'appariement d'une entité à une
/// source : une valeur, ou son absence avec son motif (registre-entites.md
/// §3.6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Appariement {
    pub entite: String,
    pub valeur: Option<String>,
    pub motif: Option<String>,
}

/// Les appariements déclarés pour une source, dans l'ordre du registre.
///
/// Une entité **sans ligne** pour cette source ne produit aucune ligne de
/// preuve : l'appariement se lit, il ne se devine pas. Apparier par le libellé
/// donnerait une position d'expert à une coalition que l'enquête ne mesure pas.
///
/// # Errors
///
/// Si une entité déclare deux appariements pour la même source — la
/// cardinalité serait ambiguë et le choix, arbitraire —, ou si une ligne nulle
/// ne porte pas son motif (V9).
pub fn appariements(registre: &Value, source: &str) -> Result<Vec<Appariement>, String> {
    let mut trouves = Vec::new();
    for entite in registre["entites"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        let id = entite["id"].as_str().unwrap_or_default();
        let lignes: Vec<&Value> = entite["identifiants"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .filter(|i| i["source"] == source)
            .collect();
        match lignes.as_slice() {
            [] => continue,
            [ligne] => {
                let valeur = ligne["valeur"].as_str().map(str::to_owned);
                let motif = ligne["motif"].as_str().map(str::to_owned);
                if valeur.is_none() && motif.is_none() {
                    return Err(format!(
                        "{id} : appariement nul à {source} sans motif — l'absence est dite, jamais comblée"
                    ));
                }
                trouves.push(Appariement {
                    entite: id.to_owned(),
                    valeur,
                    motif,
                });
            }
            plusieurs => {
                return Err(format!(
                    "{id} : {} appariements à {source} — la cardinalité est ambiguë, aucun n'est choisi",
                    plusieurs.len()
                ));
            }
        }
    }
    Ok(trouves)
}

// ------------------------------------------------------ lignes de preuve ----

/// L'arrondi est appliqué **une fois**, avant l'écriture de la ligne (§8.4).
fn arrondir(valeur: f64, decimales: i32) -> f64 {
    let facteur = 10f64.powi(decimales);
    (valeur * facteur).round() / facteur
}

/// Le squelette commun aux deux familles. Il ne porte **aucun champ de position
/// hors d'une ligne**, et aucune des deux familles ne peut lire la valeur de
/// l'autre : elles ne se rencontrent nulle part.
#[allow(clippy::too_many_arguments)]
fn ligne(
    famille: &str,
    entite: &str,
    valeur: Value,
    valeur_code: Value,
    motif_code: Value,
    motif: Value,
    echelle: Value,
    observation: (&str, &str),
    methode: Value,
    entrees: &Value,
    date_source: &str,
    date_calcul: &str,
    version_logicielle: &str,
) -> Value {
    json!({
        "famille": famille,
        "entite": entite,
        "valeur": valeur,
        "valeur_code": valeur_code,
        "echelle": echelle,
        "motif_code": motif_code,
        "motif": motif,
        "dispersion": Value::Null,
        "observation": {"debut": observation.0, "fin": observation.1},
        "date_source": date_source,
        "date_calcul": date_calcul,
        "methode": methode,
        "epingles": [],
        "entrees": entrees,
        "logiciel": {"version": version_logicielle, "commit": Value::Null},
    })
}

/// Les lignes de la famille `experts`, une par appariement CHES déclaré au
/// registre.
///
/// # Errors
///
/// Si le registre apparie un `party_id` que la vague ne porte pas : le registre
/// est falsifiable par sa source, exactement comme V15 et V16 le font pour les
/// organes de l'Assemblée. Un appariement périmé est un arrêt, pas une valeur.
pub fn lignes_experts(
    registre: &Value,
    lrgen: &BTreeMap<String, f64>,
    entrees: &Value,
    date_source: &str,
    date_calcul: &str,
    version_logicielle: &str,
) -> Result<Vec<Value>, String> {
    let echelle = json!({
        "id": ECHELLE_EXPERTS,
        "min": 0.0,
        "max": 10.0,
        "decimales": 2,
        "libelle": LIBELLE_ECHELLE_EXPERTS,
    });
    let methode = json!({
        "id": METHODE_EXPERTS,
        "version": VERSION_METHODE_EXPERTS,
        "parametres": {"colonne": COLONNE_EXPERTS, "pays": PAYS_EXPERTS, "vague": VAGUE_EXPERTS},
    });
    let mut lignes = Vec::new();
    for appariement in appariements(registre, SOURCE_EXPERTS)? {
        let (valeur, motif_code, motif) = match &appariement.valeur {
            Some(party_id) => {
                let position = lrgen.get(party_id).ok_or_else(|| {
                    format!(
                        "{} : le registre apparie {SOURCE_EXPERTS} = {party_id}, absent de la source — \
                         soit la source a bougé, soit le registre est périmé, et les deux exigent un humain",
                        appariement.entite
                    )
                })?;
                (json!(arrondir(*position, 2)), Value::Null, Value::Null)
            }
            None => (
                Value::Null,
                json!(motif_code(SOURCE_EXPERTS, &appariement.entite)),
                json!(appariement.motif),
            ),
        };
        lignes.push(ligne(
            FAMILLE_EXPERTS,
            &appariement.entite,
            valeur,
            Value::Null,
            motif_code,
            motif,
            echelle.clone(),
            OBSERVATION_EXPERTS,
            methode.clone(),
            entrees,
            date_source,
            date_calcul,
            version_logicielle,
        ));
    }
    Ok(lignes)
}

/// Les lignes de la famille `administratif`, une par appariement au nuancier
/// déclaré au registre.
///
/// La valeur est un **code**, jamais un nombre : `echelle.min`, `max` et
/// `decimales` sont nuls, et il n'existe donc aucune graduation sur laquelle ce
/// code pourrait être moyenné avec une position d'expert ou de vote.
///
/// Le nuancier publie une **observation**, pas un référentiel : les codes
/// constatés sont ceux que des candidatures ont portés à ce tour. Un code de la
/// grille qu'aucune candidature n'a porté n'est donc pas une divergence, c'est
/// une absence — dite avec son code, jamais interpolée ni reportée d'un autre
/// tour (§2.4, ADR 0000 §8). Cas réel : `COM` est constaté au 1er tour de 2024
/// et pas au 2nd, les candidatures communistes s'y présentant sous `UG`.
///
/// # Errors
///
/// Si le registre est incohérent — appariement nul sans motif, ou deux
/// appariements pour la même source.
pub fn lignes_administratif(
    registre: &Value,
    codes: &BTreeSet<String>,
    entrees: &Value,
    date_source: &str,
    date_calcul: &str,
    version_logicielle: &str,
) -> Result<Vec<Value>, String> {
    let echelle = json!({
        "id": ECHELLE_ADMINISTRATIF,
        "min": Value::Null,
        "max": Value::Null,
        "decimales": Value::Null,
        "libelle": LIBELLE_ECHELLE_ADMINISTRATIF,
    });
    let methode = json!({
        "id": METHODE_ADMINISTRATIF,
        "version": VERSION_METHODE_ADMINISTRATIF,
        "parametres": {
            "colonne": COLONNE_ADMINISTRATIF,
            "reference_grille": REFERENCE_GRILLE,
            "tour": TOUR_ADMINISTRATIF,
        },
    });
    let mut lignes = Vec::new();
    for appariement in appariements(registre, SOURCE_ADMINISTRATIF)? {
        let (valeur_code, motif_code, motif) = match &appariement.valeur {
            Some(code) if codes.contains(code) => (json!(code), Value::Null, Value::Null),
            Some(code) => (
                Value::Null,
                json!(MOTIF_PAR_DEFAUT),
                json!(format!(
                    "Code {code} de la grille non constaté au tour {TOUR_ADMINISTRATIF} : aucune candidature ne l'a porté."
                )),
            ),
            None => (
                Value::Null,
                json!(motif_code(SOURCE_ADMINISTRATIF, &appariement.entite)),
                json!(appariement.motif),
            ),
        };
        lignes.push(ligne(
            FAMILLE_ADMINISTRATIF,
            &appariement.entite,
            Value::Null,
            valeur_code,
            motif_code,
            motif,
            echelle.clone(),
            OBSERVATION_ADMINISTRATIF,
            methode.clone(),
            entrees,
            date_source,
            date_calcul,
            version_logicielle,
        ));
    }
    Ok(lignes)
}
