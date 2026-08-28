//! Le registre de preuves : `data/preuves/positions.jsonl`, JSONL en **ajout
//! seul**, une ligne = une position mesurée.
//!
//! Spécification : `docs/brique0/contrats.md` §2, §3, §6 et §7. Schéma formel :
//! `schemas/preuve-1.schema.json`.
//!
//! **Il n'existe pas de champ de position hors d'une ligne.** [`CLES`] est la
//! liste close des clés admises, et le producteur refuse d'en écrire une autre
//! (§5.1). C'est la forme structurelle de l'interdiction de moyenner les
//! familles de mesure : ce n'est pas une règle de revue, c'est l'absence du
//! champ où l'écrire.
//!
//! Une ligne circule ici sous la forme d'une [`Value`] et pas d'une structure :
//! l'ordre des clés du §7 est rendu par [`rendre`] à partir de [`CLES`], donc
//! une seule liste gouverne le contrôle et l'écriture. Une structure aurait
//! porté le même ordre une seconde fois, et deux copies divergent.

use crate::registre::date_reelle;
use crate::sha256::empreinte;
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::Path;

/// `contrepoint/preuve/1`. Le `schema` est **sur la ligne** et pas dans un
/// en-tête de fichier : un JSONL en ajout seul n'a pas d'en-tête à mettre à
/// jour, et deux majeurs cohabitent dans le même fichier (§5).
pub const SCHEMA: &str = "contrepoint/preuve/1";

/// L'ordre des clés du §2.1, qui est aussi l'ordre d'écriture du §7.
pub const CLES: &[&str] = &[
    "schema",
    "id",
    "contrat",
    "famille",
    "entite",
    "valeur",
    "valeur_code",
    "echelle",
    "motif_code",
    "motif",
    "dispersion",
    "observation",
    "date_source",
    "date_calcul",
    "methode",
    "epingles",
    "entrees",
    "logiciel",
];

pub const CLES_ENTREE: &[&str] = &[
    "source",
    "url",
    "producteur",
    "derniere_mise_a_jour",
    "citation",
    "empreinte_sha256",
    "empreinte_contenu_sha256",
    "recupere_le",
];

const CLES_ECHELLE: &[&str] = &["id", "min", "max", "decimales", "libelle"];
const CLES_DISPERSION: &[&str] = &["effectif", "iqr", "ecart_type_reechantillonnage"];
const CLES_OBSERVATION: &[&str] = &["debut", "fin"];
const CLES_METHODE: &[&str] = &["id", "version", "parametres"];
const CLES_EPINGLE: &[&str] = &["nom", "version"];
const CLES_LOGICIEL: &[&str] = &["version", "commit"];

const FAMILLES: [&str; 3] = ["votes", "experts", "administratif"];
const METHODES: [&str; 3] = ["votes_rang1_ancre", "ches_lrgen", "nuance_constatee"];
const MOTIFS: [&str; 5] = [
    "hors_source",
    // Distinct de `hors_source` : l'entité **est** dans les sources, aucune
    // famille n'a simplement produit de ligne pour elle. Les confondre faisait
    // dire à l'artefact que la source ne porte pas l'entité, ce qu'un lecteur
    // recoupant le registre voyait faux.
    "aucune_mesure",
    "sous_seuil_de_publication",
    "source_indeterminee",
    "source_non_recuperable",
];
const SOURCES: [&str; 11] = [
    "an_scrutins_17",
    "an_organe",
    "an_parpol_mandats",
    "ches_2024",
    "ches_trend",
    "manifesto",
    "nuance_leg2024",
    "parlgov",
    "wikidata",
    "declaration_publique",
    "registre_partis",
];

/// Les sources qui exigent d'être citées (I23, docs/sources.md). Une entrée
/// d'une de ces sources porte `citation` non nulle ; toute autre porte `null`.
pub const SOURCES_A_CITATION: [&str; 2] = ["ches_2024", "ches_trend"];

/// Les sources dont la ressource est un **fichier unique** : les deux empreintes
/// y coïncident par définition, et une inégalité est un refus (I22).
pub const SOURCES_FICHIER_UNIQUE: [&str; 5] = [
    "ches_2024",
    "ches_trend",
    "nuance_leg2024",
    "parlgov",
    "registre_partis",
];

/// Les trois échelles, closes : identifiant, bornes de graduation, décimales,
/// famille. Deux marqueurs de familles différentes ne partagent jamais un
/// identifiant d'échelle (I7).
pub type Echelle = (
    &'static str,
    Option<f64>,
    Option<f64>,
    Option<u32>,
    &'static str,
);

pub const ECHELLES: [Echelle; 3] = [
    (
        "votes_an17_ancre_v1",
        Some(-1.0),
        Some(1.0),
        // Deux décimales, et non quatre. À effectif impair la médiane d'un
        // groupe **est** la coordonnée ancrée d'un député : à quatre décimales,
        // la valeur publiée était celle d'une personne, au chiffre près. L'ADR
        // 0003 §3 avait écarté l'étendue pour ce motif exact — « un minimum et
        // un maximum sont les coordonnées de deux membres identifiables » — sans
        // voir que Q2 tombe sous le même raisonnement quand l'effectif est
        // impair. Arrondir ne cache rien de mesurable : nul ne lit un axe
        // gauche-droite au dix-millième.
        Some(2),
        "votes",
    ),
    ("ches_lrgen_0_10", Some(0.0), Some(10.0), Some(2), "experts"),
    ("nuance_leg2024", None, None, None, "administratif"),
];

/// Règle de non-publication de positionnement.md §6, reprise par I10.
pub const IQR_MAXIMAL: f64 = 0.25;
pub const ECART_TYPE_MAXIMAL: f64 = 0.05;
pub const EFFECTIF_MINIMAL: u64 = 10;

/// L'écart admis sur l'ancrage avant arrondi (I9).
pub const TOLERANCE_ANCRAGE: f64 = 1e-12;

// ------------------------------------------------------------------- clé ----

/// La clé de déduplication du §3, avant empreinte.
///
/// `␟` est U+001F, un caractère de contrôle qui ne peut apparaître dans aucun
/// champ : deux clés différentes ne peuvent pas produire la même chaîne.
///
/// Ce qui y est, et pourquoi : tout ce qui **détermine la valeur**. Ce qui n'y
/// est pas — `valeur`, `date_calcul`, `contrat`, `logiciel`, `entrees[].url`,
/// `entrees[].empreinte_sha256`, `producteur`, `derniere_mise_a_jour`,
/// `citation` — ne détermine aucune valeur, et l'y mettre ré-émettrait des
/// lignes sans cause.
pub fn cle(ligne: &Value) -> Result<String, String> {
    let texte = |chemin: &[&str]| -> Result<String, String> {
        let mut noeud = ligne;
        for pas in chemin {
            noeud = &noeud[*pas];
        }
        noeud.as_str().map(str::to_owned).ok_or_else(|| {
            format!(
                "clé du §3 : champ {} absent ou non textuel",
                chemin.join(".")
            )
        })
    };
    let mut empreintes: Vec<String> = ligne["entrees"]
        .as_array()
        .ok_or_else(|| "clé du §3 : `entrees` absent".to_owned())?
        .iter()
        .map(|e| {
            e["empreinte_contenu_sha256"]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| "clé du §3 : entrée sans `empreinte_contenu_sha256`".to_owned())
        })
        .collect::<Result<_, _>>()?;
    empreintes.sort();

    Ok([
        texte(&["famille"])?,
        texte(&["entite"])?,
        texte(&["observation", "debut"])?,
        texte(&["observation", "fin"])?,
        texte(&["methode", "id"])?,
        texte(&["methode", "version"])?,
        canonique(&ligne["methode"]["parametres"]),
        empreintes.join(","),
    ]
    .join("\u{1f}"))
}

/// `canonique()` du §3 : JSON compact, clés triées par point de code, sans
/// espace. `serde_json::Map` est un arbre trié : l'ordre est déjà celui-là.
fn canonique(parametres: &Value) -> String {
    serde_json::to_string(parametres).unwrap_or_else(|_| "{}".to_owned())
}

/// `id = sha256(cle en UTF-8)`, 64 hexadécimaux minuscules.
pub fn identifiant(ligne: &Value) -> Result<String, String> {
    Ok(empreinte(cle(ligne)?.as_bytes()))
}

// ------------------------------------------------------- forme canonique ----

/// La ligne, sérialisée selon le §7 : une ligne, aucune espace après `:` ni
/// `,`, clés dans l'ordre du §2.1, `entrees` triée par empreinte d'archive
/// croissante, décimales fixées par `echelle.decimales`, zéros terminaux
/// compris.
///
/// L'arrondi est appliqué **une fois**, ici : les projections du front
/// recopient la ligne au lieu de rearrondir (§8.4).
pub fn rendre(ligne: &Value) -> Result<String, String> {
    let decimales = ligne["echelle"]["decimales"].as_u64().map(|d| d as usize);
    let mut sortie = String::new();
    ecrire_objet(&mut sortie, ligne, CLES, decimales, "");
    if sortie.contains('\n') {
        return Err("forme canonique : une ligne est une ligne".to_owned());
    }
    Ok(sortie)
}

/// Les champs dont le nombre de décimales est celui de l'échelle. Ailleurs, un
/// nombre entier s'écrit sans point décimal (§7).
const AUX_DECIMALES_DE_LECHELLE: [&str; 5] = [
    "valeur",
    "min",
    "max",
    "iqr",
    "ecart_type_reechantillonnage",
];

fn ecrire_objet(
    sortie: &mut String,
    objet: &Value,
    ordre: &[&str],
    decimales: Option<usize>,
    _chemin: &str,
) {
    let Some(map) = objet.as_object() else {
        sortie.push_str(&serde_json::to_string(objet).unwrap_or_default());
        return;
    };
    sortie.push('{');
    let cles: Vec<&str> = ordre
        .iter()
        .copied()
        .filter(|c| map.contains_key(*c))
        .chain(
            map.keys()
                .map(String::as_str)
                .filter(|c| !ordre.contains(c)),
        )
        .collect();
    for (n, cle) in cles.iter().enumerate() {
        if n > 0 {
            sortie.push(',');
        }
        sortie.push_str(&serde_json::to_string(cle).unwrap_or_default());
        sortie.push(':');
        ecrire_valeur(sortie, &map[*cle], cle, decimales);
    }
    sortie.push('}');
}

fn ordre_de(cle: &str) -> &'static [&'static str] {
    match cle {
        "echelle" => CLES_ECHELLE,
        "dispersion" => CLES_DISPERSION,
        "observation" => CLES_OBSERVATION,
        "methode" => CLES_METHODE,
        "epingles" => CLES_EPINGLE,
        "entrees" => CLES_ENTREE,
        "logiciel" => CLES_LOGICIEL,
        _ => &[],
    }
}

fn ecrire_valeur(sortie: &mut String, valeur: &Value, cle: &str, decimales: Option<usize>) {
    match valeur {
        Value::Array(elements) => {
            let mut ordonnes: Vec<&Value> = elements.iter().collect();
            if cle == "entrees" {
                // L'empreinte d'archive est unique par téléchargement, donc
                // l'ordre est total — ce que l'empreinte de contenu ne garantit
                // pas : deux entrées de contenu identique ne s'ordonneraient
                // plus (§7).
                ordonnes.sort_by(|a, b| {
                    a["empreinte_sha256"]
                        .as_str()
                        .cmp(&b["empreinte_sha256"].as_str())
                });
            } else if cle == "epingles" {
                ordonnes.sort_by(|a, b| a["nom"].as_str().cmp(&b["nom"].as_str()));
            }
            sortie.push('[');
            for (n, element) in ordonnes.iter().enumerate() {
                if n > 0 {
                    sortie.push(',');
                }
                ecrire_objet(sortie, element, ordre_de(cle), decimales, cle);
            }
            sortie.push(']');
        }
        Value::Object(_) => ecrire_objet(sortie, valeur, ordre_de(cle), decimales, cle),
        Value::Number(nombre) => {
            sortie.push_str(&nombre_canonique(nombre, cle, decimales));
        }
        autre => sortie.push_str(&serde_json::to_string(autre).unwrap_or_default()),
    }
}

fn nombre_canonique(nombre: &serde_json::Number, cle: &str, decimales: Option<usize>) -> String {
    if let (true, Some(n), Some(x)) = (
        AUX_DECIMALES_DE_LECHELLE.contains(&cle),
        decimales,
        nombre.as_f64(),
    ) {
        return format!("{x:.n$}");
    }
    if let Some(entier) = nombre.as_i64() {
        return entier.to_string();
    }
    // Notation décimale, jamais d'exposant, jamais de `+`.
    let brut = nombre.to_string();
    if brut.contains(['e', 'E']) {
        return format!("{:.12}", nombre.as_f64().unwrap_or(0.0));
    }
    brut
}

/// Insère l'`id` calculé, vérifie la ligne et la rend. C'est le seul chemin
/// d'écriture : une ligne qui viole un invariant n'est pas corrigée, elle est
/// refusée.
pub fn construire(mut ligne: Value) -> Result<String, String> {
    if let Some(map) = ligne.as_object_mut() {
        map.insert("schema".to_owned(), Value::String(SCHEMA.to_owned()));
        map.remove("id");
    }
    let id = identifiant(&ligne)?;
    if let Some(map) = ligne.as_object_mut() {
        map.insert("id".to_owned(), Value::String(id));
    }
    let refus = verifier(&ligne);
    if !refus.is_empty() {
        return Err(refus.join(" ; "));
    }
    rendre(&ligne)
}

/// Les sources qui exigent d'être citées (I23, docs/sources.md §4), avec la
/// citation qu'elles publient, **mot pour mot**. Une entrée d'une de ces
/// sources porte cette chaîne caractère pour caractère ; toute autre porte
/// `null`.
///
/// Elle est par jeu de données parce que c'est ainsi que les sources la
/// formulent : chaque vague CHES publie sa propre ligne, et une mention globale
/// du projet attribuerait à un jeu la citation d'un autre (§2.1).
///
/// `citation` est une **exigence de la source, pas une cession de droits** :
/// CHES ne publie aucune licence, et la condition obtenue par échange écrit le
/// 2026-08-27 n'autorise aucune republication du fichier (ADR 0000 §8).
pub const CITATIONS: [(&str, &str); 2] = [
    ("ches_2024", CITATION_CHES_2024),
    ("ches_trend", CITATION_CHES_2024),
];

/// Relevée le 2026-08-27 sur `chesdata.eu/ches-europe/`, « When using the 2024
/// survey, please cite ». 322 caractères, sous le plafond de 400 de I20.
const CITATION_CHES_2024: &str = "Rovny, Jan, Jonathan Polk, Ryan Bakker, Liesbet Hooghe, Seth Jolly, Gary Marks, Marco Steenbergen, and Milada Anna Vachudova. 2025. \"The 2024 Chapel Hill Expert Survey on political party positioning in Europe: Twenty-five years of party positional data.\" Electoral Studies 97 (October). doi:10.1016/j.electstud.2025.102981";

/// La citation exigée par une source, ou `None` si elle n'en exige aucune.
#[must_use]
pub fn citation_exigee(source: &str) -> Option<&'static str> {
    CITATIONS
        .iter()
        .find(|(s, _)| *s == source)
        .map(|(_, citation)| *citation)
}

/// Le motif d'une non-publication, en une phrase de 140 caractères au plus.
///
/// « Absence de donnée dite, jamais comblée » (ADR 0000 §5) : une case non
/// mesurée porte **sa** raison, jamais une raison générique et jamais une
/// valeur accompagnée d'un avertissement, qui serait citée sans l'avertissement.
/// Le `motif_code` qui l'accompagne est toujours `sous_seuil_de_publication` :
/// la mesure existe, elle n'est pas publiable (§2.4).
pub fn motif_de_non_publication(
    motif_agregation: &str,
    effectif: usize,
    iqr: Option<f64>,
    ecart_type: Option<f64>,
) -> String {
    match motif_agregation {
        crate::agregation::EFFECTIF_INSUFFISANT => {
            format!("Effectif retenu de {effectif} membres pour un minimum de {EFFECTIF_MINIMAL}.")
        }
        crate::agregation::DISPERSION_INTERNE => match iqr {
            Some(mesure) => format!(
                "Dispersion interne au-delà du seuil publié : IQR {} pour un maximum de {}.",
                decimal_francais(mesure),
                decimal_francais(IQR_MAXIMAL)
            ),
            None => "Dispersion interne non calculable : le groupe compte trop peu de membres pour un écart interquartile.".to_owned(),
        },
        crate::agregation::DISPERSION_REECHANTILLONNAGE => match ecart_type {
            Some(mesure) => format!(
                "Dispersion de rééchantillonnage au-delà du seuil publié : écart-type {} pour un maximum de {}.",
                decimal_francais(mesure),
                decimal_francais(ECART_TYPE_MAXIMAL)
            ),
            None => "Dispersion de rééchantillonnage non mesurable : moins de deux tirages comparables.".to_owned(),
        },
        autre => format!("Règle de non-publication appliquée : {autre}, sur {effectif} membres."),
    }
}

/// Un nombre tel que le contrat l'écrit **en prose** : virgule décimale, zéros
/// terminaux retirés — « IQR 0,687 pour un maximum de 0,25 » (§2.4). La forme
/// canonique du §7 ne gouverne que le JSON ; une phrase française n'y écrit pas
/// un point décimal.
fn decimal_francais(valeur: f64) -> String {
    let brut = format!("{valeur:.4}");
    let taille = brut.trim_end_matches('0').trim_end_matches('.');
    taille.replace('.', ",")
}

// ------------------------------------------------------------ invariants ----

/// Les invariants du §6 vérifiables sur la ligne seule : I1, I2, I6 à I10, I12,
/// I13, I18 à I23. I3, I4 et I5 exigent le registre d'entités
/// ([`confronter_registre`]) ; I11, I14, I15, I16 et I17 portent sur les
/// artefacts ou sur le fichier, pas sur la ligne.
///
/// Rend la liste **complète** des refus : un artefact fautif se corrige en une
/// passe, pas en vingt-trois exécutions.
pub fn verifier(ligne: &Value) -> Vec<String> {
    let mut refus = Vec::new();
    i1_structure(ligne, &mut refus);
    i2_dates(ligne, &mut refus);
    i6_coherence(ligne, &mut refus);
    i7_echelle(ligne, &mut refus);
    i4_prefixe_de_famille(ligne, &mut refus);
    i8_identifiant(ligne, &mut refus);
    i9_ancrage(ligne, &mut refus);
    i10_non_publication(ligne, &mut refus);
    i12_i13_i18_i19_noms_et_valeurs(ligne, &mut refus);
    i20_i21_i22_i23_entrees(ligne, &mut refus);
    refus.sort();
    refus.dedup();
    refus
}

fn hexa64(valeur: &Value) -> bool {
    valeur.as_str().is_some_and(|s| {
        s.len() == 64
            && s.bytes()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    })
}

fn semver(valeur: &Value) -> bool {
    valeur.as_str().is_some_and(|s| {
        let parties: Vec<&str> = s.split('.').collect();
        parties.len() == 3
            && parties
                .iter()
                .all(|p| !p.is_empty() && p.bytes().all(|c| c.is_ascii_digit()))
    })
}

fn cles_exactes(objet: &Value, attendues: &[&str], ou: &str, refus: &mut Vec<String>) {
    let Some(map) = objet.as_object() else {
        refus.push(format!("I1 : {ou} n'est pas un objet"));
        return;
    };
    for cle in map.keys() {
        if !attendues.contains(&cle.as_str()) {
            refus.push(format!("I1 : clé `{cle}` absente du schéma dans {ou}"));
        }
    }
    for attendue in attendues {
        if !map.contains_key(*attendue) {
            refus.push(format!("I1 : clé `{attendue}` absente de {ou}"));
        }
    }
}

fn i1_structure(ligne: &Value, refus: &mut Vec<String>) {
    cles_exactes(ligne, CLES, "la ligne", refus);
    if ligne["schema"] != SCHEMA {
        refus.push(format!(
            "I1 : `schema` vaut {} et non {SCHEMA}",
            ligne["schema"]
        ));
    }
    if !semver(&ligne["contrat"]) {
        refus.push("I1 : `contrat` n'est pas une version de la forme X.Y.Z".to_owned());
    }
    if !hexa64(&ligne["id"]) {
        refus.push("I1 : `id` n'est pas 64 hexadécimaux minuscules".to_owned());
    }
    if !FAMILLES.contains(&ligne["famille"].as_str().unwrap_or_default()) {
        refus.push(format!("I1 : famille inconnue {}", ligne["famille"]));
    }
    cles_exactes(&ligne["echelle"], CLES_ECHELLE, "`echelle`", refus);
    cles_exactes(
        &ligne["observation"],
        CLES_OBSERVATION,
        "`observation`",
        refus,
    );
    cles_exactes(&ligne["methode"], CLES_METHODE, "`methode`", refus);
    cles_exactes(&ligne["logiciel"], CLES_LOGICIEL, "`logiciel`", refus);
    if ligne["dispersion"].is_object() {
        cles_exactes(&ligne["dispersion"], CLES_DISPERSION, "`dispersion`", refus);
    }
    if !METHODES.contains(&ligne["methode"]["id"].as_str().unwrap_or_default()) {
        refus.push(format!("I1 : méthode inconnue {}", ligne["methode"]["id"]));
    }
    if !semver(&ligne["methode"]["version"]) {
        refus.push("I1 : `methode.version` n'est pas un semver".to_owned());
    }
    if !semver(&ligne["logiciel"]["version"]) {
        refus.push("I1 : `logiciel.version` n'est pas un semver".to_owned());
    }
    match ligne["methode"]["parametres"].as_object() {
        None => refus.push("I1 : `methode.parametres` n'est pas un objet".to_owned()),
        Some(map) if map.is_empty() => refus.push(
            "I1 : `methode.parametres` vide — ce qui détermine le résultat n'est pas consigné"
                .to_owned(),
        ),
        Some(map) => {
            for cle in map.keys() {
                if !cle
                    .bytes()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
                {
                    refus.push(format!(
                        "I1 : paramètre `{cle}` hors du motif `^[a-z0-9_]+$`"
                    ));
                }
            }
        }
    }
    match ligne["epingles"].as_array() {
        None => refus.push("I1 : `epingles` absent — les fonctions fixes externes sont épinglées en version, même quand la liste est vide".to_owned()),
        Some(epingles) => {
            for epingle in epingles {
                cles_exactes(epingle, CLES_EPINGLE, "une épingle", refus);
            }
        }
    }
    // I1 — `entrees` non vide, chaque entrée complète et empreintée.
    match ligne["entrees"].as_array().map(Vec::as_slice) {
        None | Some([]) => refus
            .push("I1 : aucune position sans source empreintée — `entrees` est vide".to_owned()),
        Some(entrees) => {
            for entree in entrees {
                cles_exactes(entree, CLES_ENTREE, "une entrée", refus);
                if !SOURCES.contains(&entree["source"].as_str().unwrap_or_default()) {
                    refus.push(format!(
                        "I1 : source hors de la liste close : {}",
                        entree["source"]
                    ));
                }
                if !entree["url"]
                    .as_str()
                    .is_some_and(|u| u.starts_with("https://"))
                {
                    refus.push("I1 : entrée sans URL absolue en https".to_owned());
                }
                if !hexa64(&entree["empreinte_sha256"]) {
                    refus.push("I1 : entrée sans `empreinte_sha256` de 64 hexadécimaux".to_owned());
                }
                if !date_reelle(entree["recupere_le"].as_str().unwrap_or_default()) {
                    refus.push("I1 : entrée sans `recupere_le` réel".to_owned());
                }
            }
        }
    }
    match ligne["logiciel"]["commit"] {
        Value::Null => {}
        ref commit if hexa64(commit) || commit.as_str().is_some_and(|c| c.len() == 40) => {}
        _ => refus.push("I1 : `logiciel.commit` n'est ni nul ni 40 hexadécimaux".to_owned()),
    }
}

fn i2_dates(ligne: &Value, refus: &mut Vec<String>) {
    let Some(source) = ligne["date_source"].as_str().filter(|d| date_reelle(d)) else {
        refus.push(
            "I2 : aucune position sans source datée — `date_source` absente ou irréelle".to_owned(),
        );
        return;
    };
    let Some(calcul) = ligne["date_calcul"].as_str() else {
        refus.push("I2 : `date_calcul` absente".to_owned());
        return;
    };
    if calcul.len() != 20 || !calcul.ends_with('Z') || !date_reelle(&calcul[..10]) {
        refus.push(format!(
            "I2 : `date_calcul` {calcul} n'est pas un RFC 3339 en UTC"
        ));
        return;
    }
    if source > &calcul[..10] {
        refus.push(format!(
            "I2 : `date_source` {source} postérieure à `date_calcul` {}",
            &calcul[..10]
        ));
    }
    for entree in ligne["entrees"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        if let Some(recupere) = entree["recupere_le"].as_str()
            && source > recupere
        {
            refus.push(format!(
                "I2 : `date_source` {source} postérieure au `recupere_le` {recupere} de {}",
                entree["source"]
            ));
        }
    }
    for champ in ["debut", "fin"] {
        if !date_reelle(ligne["observation"][champ].as_str().unwrap_or_default()) {
            refus.push(format!(
                "I2 : `observation.{champ}` n'est pas une date réelle"
            ));
        }
    }
    let (debut, fin) = (
        ligne["observation"]["debut"].as_str().unwrap_or_default(),
        ligne["observation"]["fin"].as_str().unwrap_or_default(),
    );
    if debut > fin {
        refus.push(format!("I2 : `observation` {debut} → {fin} est à l'envers"));
    }
}

fn i6_coherence(ligne: &Value, refus: &mut Vec<String>) {
    let valeur = !ligne["valeur"].is_null();
    let code = !ligne["valeur_code"].is_null();
    let motif_code = ligne["motif_code"].as_str();
    let motif = ligne["motif"].as_str();
    if valeur && code {
        refus.push("I6 : `valeur` et `valeur_code` sont tous deux non nuls".to_owned());
    }
    if !valeur && !code {
        if motif_code.is_none() || motif.is_none_or(str::is_empty) {
            refus.push(
                "I6 : ni valeur ni code, et l'absence n'est pas dite — `motif_code` et `motif` sont exigés"
                    .to_owned(),
            );
        }
    } else if motif_code.is_some() || motif.is_some() {
        refus.push("I6 : une valeur publiée ne porte ni `motif_code` ni `motif`".to_owned());
    }
    if let Some(code) = motif_code
        && !MOTIFS.contains(&code)
    {
        refus.push(format!("I6 : `motif_code` inconnu : {code}"));
    }
    // §2.4 — la dispersion sans valeur n'est admise qu'en non-publication
    // motivée : les chiffres qui justifient la non-publication sont publiés.
    if !valeur && ligne["dispersion"].is_object() && motif_code != Some("sous_seuil_de_publication")
    {
        refus.push(
            "I6 : dispersion renseignée sans valeur hors d'une non-publication motivée".to_owned(),
        );
    }
}

fn i7_echelle(ligne: &Value, refus: &mut Vec<String>) {
    let id = ligne["echelle"]["id"].as_str().unwrap_or_default();
    let famille = ligne["famille"].as_str().unwrap_or_default();
    let Some((_, min, max, decimales, sienne)) = ECHELLES.iter().find(|(e, ..)| *e == id) else {
        refus.push(format!("I7 : `echelle.id` hors de la liste close : {id}"));
        return;
    };
    if *sienne != famille {
        refus.push(format!(
            "I7 : l'échelle {id} appartient à la famille {sienne} et non à {famille} — deux familles ne partagent jamais une échelle"
        ));
    }
    if ligne["echelle"]["min"].as_f64() != *min || ligne["echelle"]["max"].as_f64() != *max {
        refus.push(format!("I7 : bornes de graduation de {id} altérées"));
    }
    if ligne["echelle"]["decimales"].as_u64() != decimales.map(u64::from) {
        refus.push(format!("I7 : `decimales` de {id} altérées"));
    }
    if let Some(valeur) = ligne["valeur"].as_f64() {
        if min.is_some_and(|m| valeur < m) || max.is_some_and(|m| valeur > m) {
            refus.push(format!(
                "I7 : valeur {valeur} hors des bornes de graduation de {id} — refus bloquant, pas dépassement toléré"
            ));
        }
        if let Some(n) = decimales {
            let arrondie = format!("{valeur:.*}", *n as usize);
            if arrondie.parse::<f64>().unwrap_or(f64::NAN) != valeur {
                refus.push(format!(
                    "I7 : valeur {valeur} n'est pas écrite avec exactement {n} décimales"
                ));
            }
        }
    }
}

/// Le versant de I4 qui se lit sur la ligne : le préfixe de `entite`
/// correspond à la famille (§2.2). L'existence dans le registre est vérifiée
/// par [`confronter_registre`].
fn i4_prefixe_de_famille(ligne: &Value, refus: &mut Vec<String>) {
    let entite = ligne["entite"].as_str().unwrap_or_default();
    let attendu: &[&str] = match ligne["famille"].as_str().unwrap_or_default() {
        "votes" => &["groupe."],
        "experts" | "administratif" => &["parti.", "coalition."],
        _ => return,
    };
    if !attendu.iter().any(|p| entite.starts_with(p)) {
        refus.push(format!(
            "I4 : {entite} ne peut pas porter une mesure de la famille {} — l'objet mesuré n'est pas le même selon la famille (§2.2)",
            ligne["famille"]
        ));
    }
}

fn i8_identifiant(ligne: &Value, refus: &mut Vec<String>) {
    match identifiant(ligne) {
        Err(erreur) => refus.push(format!("I8 : {erreur}")),
        Ok(recalcule) => {
            if ligne["id"].as_str() != Some(recalcule.as_str()) {
                refus.push(format!(
                    "I8 : `id` déclaré {} ≠ `id` recalculé {recalcule}",
                    ligne["id"]
                ));
            }
        }
    }
}

fn i9_ancrage(ligne: &Value, refus: &mut Vec<String>) {
    if ligne["famille"] != "votes" {
        return;
    }
    let entite = ligne["entite"].as_str().unwrap_or_default();
    let parametres = &ligne["methode"]["parametres"];
    for (pole, attendue) in [("ancre_gauche", -1.0), ("ancre_droite", 1.0)] {
        if parametres[pole].as_str() != Some(entite) {
            continue;
        }
        match ligne["valeur"].as_f64() {
            Some(valeur) if (valeur - attendue).abs() <= TOLERANCE_ANCRAGE => {}
            autre => refus.push(format!(
                "I9 : {entite} est l'{pole} et porte {autre:?} au lieu de {attendue} — l'ancrage n'est pas exact"
            )),
        }
    }
}

fn i10_non_publication(ligne: &Value, refus: &mut Vec<String>) {
    // §2.4 — `sous_seuil_de_publication` est la seule occurrence où `dispersion`
    // est renseignée sans `valeur` : les chiffres qui justifient la
    // non-publication sont publiés, la valeur non. Un motif d'absence comblé par
    // un vide est exactement ce que le projet refuse.
    if ligne["motif_code"] == "sous_seuil_de_publication" && !ligne["dispersion"].is_object() {
        refus.push(
            "I10 : `sous_seuil_de_publication` sans `dispersion` — les chiffres qui justifient la non-publication sont publiés, la valeur non (§2.4)"
                .to_owned(),
        );
    }
    if !ligne["dispersion"].is_object() {
        return;
    }
    let dispersion = &ligne["dispersion"];
    let sous_seuil = dispersion["iqr"].as_f64().is_some_and(|x| x > IQR_MAXIMAL)
        || dispersion["ecart_type_reechantillonnage"]
            .as_f64()
            .is_some_and(|x| x > ECART_TYPE_MAXIMAL)
        || dispersion["effectif"]
            .as_u64()
            .is_some_and(|n| n < EFFECTIF_MINIMAL);
    let publiee = !ligne["valeur"].is_null();
    if sous_seuil && publiee {
        refus.push(format!(
            "I10 : la règle de non-publication s'applique (IQR {} > {IQR_MAXIMAL}, écart-type {} > {ECART_TYPE_MAXIMAL} ou effectif {} < {EFFECTIF_MINIMAL}) et la valeur est publiée",
            dispersion["iqr"], dispersion["ecart_type_reechantillonnage"], dispersion["effectif"]
        ));
    }
    if sous_seuil && !publiee && ligne["motif_code"] != "sous_seuil_de_publication" {
        refus.push(
            "I10 : mesure retenue et non publiée sans `motif_code = sous_seuil_de_publication`"
                .to_owned(),
        );
    }
}

/// I12 — aucun nom d'agrégation entre familles. I13 — aucune coordonnée
/// individuelle. I18 — aucun écart entre deux dates. I19 — aucune borne
/// d'étendue.
///
/// Les trois listes portent sur la **forme** que prendrait la violation, pas sur
/// un vocabulaire d'affichage : le lexique proscrit se contourne en renommant.
const NOMS_DAGREGATION: [&str; 9] = [
    "moyenne",
    "score",
    "synthese",
    "synthèse",
    "global",
    "consensus",
    "indice",
    "consolide",
    "note",
];
const NOMS_DECART: [&str; 8] = [
    "ecart",
    "écart",
    "variation",
    "evolution",
    "évolution",
    "delta",
    "tendance",
    "progression",
];
const NOMS_DE_BORNE: [&str; 8] = [
    "minimum",
    "maximum",
    "etendue",
    "étendue",
    "rang",
    "extreme",
    "extrême",
    "percentile",
];

/// La comparaison porte sur les **jetons** d'un nom en casse serpent, pas sur
/// une sous-chaîne. Mesuré : une recherche par sous-chaîne refuse
/// `ecart_type_reechantillonnage` et `scrutins_ecartes`, qui sont une dispersion
/// et un décompte, et rien de ce que I18 vise. Un contrôle qui refuse le
/// légitime se fait désactiver, pas corriger.
///
/// Une seule exception nommée : `ecart_type`, qui est l'écart-type d'une
/// distribution et non un écart entre deux dates.
fn jeton_proscrit(nom: &str, liste: &[&str]) -> bool {
    let jetons: Vec<&str> = nom.split('_').collect();
    jetons.iter().enumerate().any(|(n, jeton)| {
        let bas = jeton.to_lowercase();
        if !liste.contains(&bas.as_str()) {
            return false;
        }
        !((bas == "ecart" || bas == "écart") && jetons.get(n + 1) == Some(&"type"))
    })
}

fn i12_i13_i18_i19_noms_et_valeurs(ligne: &Value, refus: &mut Vec<String>) {
    parcourir(
        ligne,
        "$",
        &mut |chemin, cle, valeur, refus| {
            if let Some(cle) = cle {
                // `note` désigne une note **chiffrée** attribuée à une entité, pas
                // une note de bas de page : `ancrage.note` du §4.2 est une légende
                // de 140 caractères que le schéma publié exige. Le jeton n'est donc
                // proscrit que sur une valeur numérique — un contrôle qui refuse le
                // champ légitime que le contrat impose se fait désactiver.
                let note_legende = jeton_proscrit(cle, &["note"]) && valeur.is_string();
                if jeton_proscrit(cle, &NOMS_DAGREGATION) && !note_legende {
                    refus.push(format!(
                    "I12 : clé `{cle}` en {chemin} suggère une agrégation entre familles de mesure — il n'existe aucun emplacement pour l'écrire"
                ));
                }
                if jeton_proscrit(cle, &NOMS_DECART) {
                    refus.push(format!(
                    "I18 : clé `{cle}` en {chemin} porte un écart entre deux dates — le contrat ne fournit pas l'emplacement pour l'écrire"
                ));
                }
                if jeton_proscrit(cle, &NOMS_DE_BORNE) {
                    refus.push(format!(
                    "I19 : clé `{cle}` en {chemin} porte une borne d'étendue ou un rang — c'est la coordonnée d'un membre identifiable"
                ));
                }
            }
            if let Some(texte) = valeur.as_str() {
                if texte.len() >= 6
                    && let Some(reste) = texte.rsplit('.').next()
                    && reste.starts_with("PA")
                    && reste.len() >= 6
                    && reste[2..].bytes().all(|c| c.is_ascii_digit())
                {
                    refus.push(format!(
                    "I13 : identifiant d'acteur {texte} en {chemin} — aucune coordonnée individuelle dans un artefact publié"
                ));
                }
                let mot_seul = !texte.is_empty()
                    && texte
                        .bytes()
                        .all(|c| c.is_ascii_alphanumeric() || c == b'_');
                if mot_seul && jeton_proscrit(texte, &NOMS_DAGREGATION) {
                    refus.push(format!(
                    "I12 : valeur d'énumération `{texte}` en {chemin} suggère une agrégation entre familles"
                ));
                }
            }
        },
        refus,
    );
}

fn parcourir(
    noeud: &Value,
    chemin: &str,
    visiter: &mut impl FnMut(&str, Option<&str>, &Value, &mut Vec<String>),
    refus: &mut Vec<String>,
) {
    match noeud {
        Value::Object(map) => {
            for (cle, valeur) in map {
                let sous = format!("{chemin}.{cle}");
                visiter(&sous, Some(cle), valeur, refus);
                parcourir(valeur, &sous, visiter, refus);
            }
        }
        Value::Array(elements) => {
            for (n, valeur) in elements.iter().enumerate() {
                let sous = format!("{chemin}[{n}]");
                visiter(&sous, None, valeur, refus);
                parcourir(valeur, &sous, visiter, refus);
            }
        }
        _ => {}
    }
}

fn i20_i21_i22_i23_entrees(ligne: &Value, refus: &mut Vec<String>) {
    // I20 — aucune chaîne au-delà de 200 caractères, `url`, empreintes et
    // `citation` exceptées, cette dernière plafonnée à 400.
    parcourir(
        ligne,
        "$",
        &mut |chemin, cle, valeur, refus| {
            let Some(texte) = valeur.as_str() else { return };
            let n = texte.chars().count();
            let plafond = match cle {
                Some("url")
                | Some("empreinte_sha256")
                | Some("empreinte_contenu_sha256")
                | Some("id") => return,
                Some("citation") => 400,
                Some("motif") => 140,
                _ => 200,
            };
            if n > plafond {
                refus.push(format!(
                    "I20 : chaîne de {n} caractères en {chemin} pour {plafond} au plus"
                ));
            }
        },
        refus,
    );

    for entree in ligne["entrees"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        let source = entree["source"].as_str().unwrap_or_default();

        // I21 — la mention de paternité est structurelle, pas un texte à
        // maintenir. Un code interne n'est pas un nom de producteur.
        match entree["producteur"].as_str() {
            None | Some("") => refus.push(format!("I21 : entrée {source} sans `producteur`")),
            Some(producteur) if producteur == source => refus.push(format!(
                "I21 : `producteur` {producteur} est le code interne de la source — un code n'est pas un nom"
            )),
            Some(_) => {}
        }
        match entree["derniere_mise_a_jour"].as_str() {
            None => refus.push(format!("I21 : entrée {source} sans `derniere_mise_a_jour`")),
            Some(date) if !date_reelle(date) => refus.push(format!(
                "I21 : `derniere_mise_a_jour` {date} n'est pas une date réelle"
            )),
            Some(date) => {
                if let Some(recupere) = entree["recupere_le"].as_str()
                    && date > recupere
                {
                    refus.push(format!(
                        "I21 : `derniere_mise_a_jour` {date} postérieure au `recupere_le` {recupere}"
                    ));
                }
            }
        }

        // I22 — l'empreinte de contenu est la seule des deux dans la clé, et
        // elle est obligatoire. Pour une source d'un seul fichier, les deux
        // coïncident ; une inégalité y est un refus.
        if !hexa64(&entree["empreinte_contenu_sha256"]) {
            refus.push(format!(
                "I22 : entrée {source} sans `empreinte_contenu_sha256` de 64 hexadécimaux"
            ));
        } else if SOURCES_FICHIER_UNIQUE.contains(&source)
            && entree["empreinte_contenu_sha256"] != entree["empreinte_sha256"]
        {
            refus.push(format!(
                "I22 : {source} est un fichier unique et ses deux empreintes diffèrent"
            ));
        }

        // I23 — une source à citation la porte, toute autre porte `null`.
        let citation = entree["citation"].as_str();
        // La comparaison porte sur le **texte**, pas sur la présence : une
        // citation abrégée ou reformulée n'est pas la citation que la source
        // exige, et la présence seule laissait passer « Rovny et al. 2025. ».
        if let Some(exigee) = citation_exigee(source) {
            if citation.is_some_and(|c| !c.is_empty() && c != exigee) {
                refus.push(format!(
                    "I23 : la citation de {source} n'est pas celle que la source publie, caractère pour caractère"
                ));
            }
            if citation.is_none_or(str::is_empty) {
                refus.push(format!(
                    "I23 : {source} exige une citation et l'entrée n'en porte pas"
                ));
            }
        } else if citation.is_some() {
            refus.push(format!(
                "I23 : {source} n'exige aucune citation et l'entrée en porte une"
            ));
        }
    }
}

/// I3, I4 et I5 — ce que la ligne seule ne peut pas dire.
///
/// I3 : une `source` déclarée dans le registre d'entités y porte la **même**
/// URL et la **même** empreinte. Divergence = refus : soit la source a bougé,
/// soit un fichier a été édité à la main. I4 : `entite` existe dans le registre
/// et sa période couvre `observation.fin`. I5 : les deux ancres y sont
/// **déclarées** comme ancres, valides à cette date.
pub fn confronter_registre(ligne: &Value, registre: &Value) -> Vec<String> {
    let mut refus = Vec::new();
    let fin = ligne["observation"]["fin"].as_str().unwrap_or_default();

    for entree in ligne["entrees"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        let source = entree["source"].as_str().unwrap_or_default();
        let Some(declaree) = registre["sources"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .find(|s| s["id"] == source)
        else {
            continue; // La source n'est pas déclarée dans le registre : I3 ne dit rien.
        };
        if !declaree["url"].is_null() && declaree["url"] != entree["url"] {
            refus.push(format!(
                "I3 : {source} porte l'URL {} dans le registre et {} dans la ligne",
                declaree["url"], entree["url"]
            ));
        }
        if !declaree["empreinte_sha256"].is_null()
            && declaree["empreinte_sha256"] != entree["empreinte_sha256"]
        {
            refus.push(format!(
                "I3 : {source} porte l'empreinte {} dans le registre et {} dans la ligne",
                declaree["empreinte_sha256"], entree["empreinte_sha256"]
            ));
        }
    }

    let entite = ligne["entite"].as_str().unwrap_or_default();
    let cherche = |id: &str| -> Option<&Value> {
        ["entites", "groupes"].iter().find_map(|bloc| {
            registre[*bloc]
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or_default()
                .iter()
                .find(|e| e["id"] == id)
        })
    };
    match cherche(entite) {
        None => refus.push(format!(
            "I4 : {entite} est inconnu du registre d'entités — aucun identifiant d'entité inventé"
        )),
        Some(objet) => {
            let debut = objet["debut"].as_str();
            let close = objet["fin"].as_str();
            if debut.is_some_and(|d| d > fin) || close.is_some_and(|f| f < fin) {
                refus.push(format!(
                    "I4 : la période de {entite} ({debut:?} → {close:?}) ne couvre pas `observation.fin` {fin}"
                ));
            }
        }
    }

    if ligne["famille"] == "votes" {
        for (champ, pole) in [("ancre_gauche", "gauche"), ("ancre_droite", "droite")] {
            let Some(ancre) = ligne["methode"]["parametres"][champ].as_str() else {
                refus.push(format!("I5 : `methode.parametres.{champ}` absent"));
                continue;
            };
            let declaree = cherche(ancre).filter(|groupe| {
                let axe = &groupe["ancre_axe"];
                axe["pole"] == pole
                    && axe["debut"].as_str().is_some_and(|d| d <= fin)
                    && !axe["fin"].as_str().is_some_and(|f| f < fin)
            });
            if declaree.is_none() {
                refus.push(format!(
                    "I5 : {ancre} n'est pas déclaré ancre du pôle {pole} au {fin} dans le registre — arrêt, jamais substitution automatique (RG-31)"
                ));
            }
        }
    }
    refus.sort();
    refus.dedup();
    refus
}

// -------------------------------------------------------------- ajout seul --

/// Ce qu'une exécution a ajouté au registre de preuves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ajout {
    /// Lignes réellement écrites — celles dont l'`id` n'était pas déjà présent.
    pub ajoutees: usize,
    /// Lignes déjà présentes à l'identique, donc non réécrites.
    pub deja_presentes: usize,
    /// Taille du fichier avant l'ajout. Le nouveau fichier la conserve octet
    /// pour octet en tête (I15).
    pub octets_avant: u64,
}

/// **Ajout seul.** Le fichier n'est jamais réécrit : une exécution ajoute les
/// lignes dont l'`id` n'est pas déjà présent, dans l'ordre
/// `(famille, entite, methode.id, id)`. Aucun tri global, aucune réindexation,
/// aucune suppression.
///
/// Une valeur qui bouge est une ligne de plus, jamais une ligne modifiée. Deux
/// lignes de même `id` et de valeurs différentes sont une méthode corrigée sans
/// incrément de `methode.version` : I8 refuse, bruyamment, et rien n'est écrit.
pub fn ajouter(chemin: &Path, lignes: &[String]) -> Result<Ajout, String> {
    let existant = match std::fs::read_to_string(chemin) {
        Ok(texte) => texte,
        Err(erreur) if erreur.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(erreur) => return Err(format!("registre de preuves illisible : {erreur}")),
    };
    let octets_avant = existant.len() as u64;

    let mut connues: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    for ligne in existant.lines().filter(|l| !l.is_empty()) {
        let valeur: Value = serde_json::from_str(ligne)
            .map_err(|e| format!("registre de preuves : ligne illisible ({e})"))?;
        let id = valeur["id"]
            .as_str()
            .ok_or_else(|| "registre de preuves : ligne sans `id`".to_owned())?;
        connues.insert(id.to_owned(), ligne.to_owned());
    }

    let mut a_ecrire: Vec<(String, String, String, String, String)> = Vec::new();
    let mut deja_presentes = 0;
    let mut vues: BTreeSet<String> = BTreeSet::new();
    for ligne in lignes {
        let valeur: Value =
            serde_json::from_str(ligne).map_err(|e| format!("ligne à ajouter illisible : {e}"))?;
        let id = valeur["id"]
            .as_str()
            .ok_or_else(|| "ligne à ajouter sans `id`".to_owned())?
            .to_owned();
        if let Some(ancienne) = connues.get(&id) {
            if ancienne != ligne {
                return Err(format!(
                    "I8 : deux lignes de même `id` {id} portent des contenus différents — \
                     une méthode a été modifiée sans incrémenter `methode.version`"
                ));
            }
            deja_presentes += 1;
            continue;
        }
        if !vues.insert(id.clone()) {
            deja_presentes += 1;
            continue;
        }
        a_ecrire.push((
            valeur["famille"].as_str().unwrap_or_default().to_owned(),
            valeur["entite"].as_str().unwrap_or_default().to_owned(),
            valeur["methode"]["id"]
                .as_str()
                .unwrap_or_default()
                .to_owned(),
            id,
            ligne.clone(),
        ));
    }
    a_ecrire.sort_by(|a, b| (&a.0, &a.1, &a.2, &a.3).cmp(&(&b.0, &b.1, &b.2, &b.3)));

    if !a_ecrire.is_empty() {
        if let Some(parent) = chemin.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("répertoire du registre de preuves : {e}"))?;
        }
        let mut sortie = existant;
        for (.., ligne) in &a_ecrire {
            sortie.push_str(ligne);
            sortie.push('\n');
        }
        std::fs::write(chemin, sortie)
            .map_err(|e| format!("écriture du registre de preuves : {e}"))?;
    }
    Ok(Ajout {
        ajoutees: a_ecrire.len(),
        deja_presentes,
        octets_avant,
    })
}

/// I17 — `date_arretee` = maximum des `date_calcul` des lignes référencées.
/// **Dérivée, jamais saisie.**
pub fn date_arretee(lignes: &[String]) -> Result<String, String> {
    lignes
        .iter()
        .map(|ligne| {
            serde_json::from_str::<Value>(ligne)
                .ok()
                .and_then(|v| v["date_calcul"].as_str().map(str::to_owned))
                .ok_or_else(|| "date_arretee : ligne sans `date_calcul`".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .max()
        .ok_or_else(|| "date_arretee : aucune ligne référencée".to_owned())
}
