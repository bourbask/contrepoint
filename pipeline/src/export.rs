//! Les trois artefacts statiques du front : manifeste, instantané, éclats de
//! preuves. Spécification : `docs/brique0/contrats.md` §4, §6 et §7.
//!
//! Les trois sont des **projections** du registre de preuves et ne portent
//! **aucune valeur qui ne soit d'abord une ligne du registre** : un marqueur
//! sans ligne ne s'affiche pas. Une valeur n'est jamais recalculée ici, elle est
//! recopiée — l'arrondi a été appliqué une fois, à l'écriture de la ligne
//! (§8.4).
//!
//! Ce que le contrat interdit par construction, et que ce module rend
//! impossible plutôt qu'interdit : il n'y a **aucun emplacement** où écrire une
//! valeur agrégeant deux familles de mesure. Un nombre n'est atteignable que
//! dans un marqueur, hors `effectif` (I11) ; un marqueur porte une famille et
//! une échelle nommée ; deux familles ne partagent jamais une échelle. Il n'y a
//! pas de champ à supprimer, il n'y en a jamais eu.

use crate::preuves::{ECHELLES, date_arretee};
use crate::sha256::empreinte;
use serde_json::Value;
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

/// La légende, close, dans l'ordre d'affichage. Le front la lit au lieu de
/// coder les familles en dur : c'est ce qui rend l'ajout d'une famille une
/// mineure — et un front qui ne connaît pas la liste ne peut pas les moyenner,
/// puisqu'il ne sait pas ce qu'il additionnerait.
pub const FAMILLES: [(&str, &str, &str); 3] = [
    ("votes", "Votes nominatifs", "votes_an17_ancre_v1"),
    ("experts", "Enquête d'experts", "ches_lrgen_0_10"),
    ("administratif", "Nuance administrative", "nuance_leg2024"),
];

/// Le libellé d'un marqueur, par famille. Quarante caractères au plus
/// (ADR 0000 §5). Celui de la famille `votes` nomme le **groupe mesuré**, ce
/// qui est la seule façon honnête de poser un marqueur de groupe sur la bande
/// d'un parti (§4.3, règle 2).
const LIBELLES: [(&str, &str); 2] = [
    ("experts", "CHES 2024, lrgen"),
    ("administratif", "Nuance 2024"),
];

pub const SCHEMA_MANIFESTE: &str = "contrepoint/manifeste/1";
pub const SCHEMA_INSTANTANE: &str = "contrepoint/instantane/1";
pub const SCHEMA_ECLAT: &str = "contrepoint/eclat-preuves/1";

const CLES_MANIFESTE: &[&str] = &[
    "schema",
    "contrat",
    "schemas",
    "date_arretee",
    "licence",
    "mention_paternite",
    "familles",
    "instantanes",
    "preuves",
];
const CLES_INSTANTANE: &[&str] = &[
    "schema",
    "contrat",
    "id",
    "chambre",
    "legislature",
    "date",
    "date_arretee",
    "ancrage",
    "bandes",
    "sans_mesure",
];
const CLES_MARQUEUR: &[&str] = &[
    "famille",
    "echelle",
    "valeur",
    "valeur_code",
    "libelle",
    "motif_code",
    "motif",
    "dispersion",
    "preuve",
];
const CLES_BANDE: &[&str] = &["id", "libelle", "marqueurs"];
const CLES_ANCRAGE: &[&str] = &["famille", "ancre_gauche", "ancre_droite", "note"];
const CLES_SANS_MESURE: &[&str] = &["entite", "libelle", "motif_code", "motif"];
const CLES_FAMILLE: &[&str] = &["id", "libelle", "echelle", "min", "max", "decimales"];
const CLES_INSTANTANE_LISTE: &[&str] = &[
    "id",
    "chambre",
    "legislature",
    "date",
    "url",
    "empreinte_sha256",
    "octets",
    "bandes",
];
const CLES_PREUVES: &[&str] = &["racine", "eclats", "fonction"];
const CLES_DISPERSION_REDUITE: &[&str] = &["effectif", "iqr"];

pub const LICENCE: &str = "Licence Ouverte / Open Licence (Etalab)";
pub const RACINE_DES_PREUVES: &str = "preuves/";
pub const ECLATS: u64 = 256;
pub const FONCTION_DECLAT: &str = "deux premiers caractères hexadécimaux de l'id";

/// Ce qu'un instantané est, avant d'être rempli. `date_arretee` n'y figure
/// pas : elle est **dérivée** des lignes référencées, jamais saisie (I17).
#[derive(Debug, Clone)]
pub struct Description {
    pub id: String,
    pub chambre: String,
    pub legislature: String,
    pub date: String,
    /// La légende de l'ancrage, 140 caractères au plus. Elle dit pourquoi deux
    /// instantanés ne se superposent pas ; elle ne nomme pas les ancres, qui
    /// sont de la donnée et vivent dans `ancrage`.
    pub note_ancrage: String,
}

// ---------------------------------------------------------------- rendu -----

fn ordre_de(cle: &str) -> &'static [&'static str] {
    match cle {
        "ancrage" => CLES_ANCRAGE,
        "bandes" => CLES_BANDE,
        "marqueurs" => CLES_MARQUEUR,
        "sans_mesure" => CLES_SANS_MESURE,
        "familles" => CLES_FAMILLE,
        "instantanes" => CLES_INSTANTANE_LISTE,
        "preuves" => CLES_PREUVES,
        "dispersion" => CLES_DISPERSION_REDUITE,
        _ => &[],
    }
}

/// Même compacité et mêmes ordres de clés que le registre de preuves (§7) : la
/// lisibilité passe par l'outillage, pas par l'indentation.
fn rendre(valeur: &Value, ordre: &[&str]) -> String {
    let mut sortie = String::new();
    ecrire(&mut sortie, valeur, ordre, "", None);
    sortie
}

/// Les champs dont le nombre de décimales est celui de l'échelle du marqueur
/// qui les porte. Ailleurs, un entier s'écrit sans point décimal (§7).
const AUX_DECIMALES_DE_LECHELLE: [&str; 2] = ["valeur", "iqr"];

fn ecrire(
    sortie: &mut String,
    valeur: &Value,
    ordre: &[&str],
    cle: &str,
    decimales: Option<usize>,
) {
    match valeur {
        Value::Object(map) => {
            // Un marqueur porte son échelle : c'est elle qui fixe le nombre de
            // décimales de sa valeur et de sa dispersion, et il se propage aux
            // objets qu'il contient. Les zéros terminaux ne survivent pas à un
            // aller-retour par un analyseur JSON générique (§7) : ils sont
            // réécrits ici, à la seule écriture qui engage le producteur.
            let heritees = map
                .get("echelle")
                .and_then(Value::as_str)
                .map(nombre_de_decimales)
                .unwrap_or(decimales);
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
            for (n, sous) in cles.iter().enumerate() {
                if n > 0 {
                    sortie.push(',');
                }
                sortie.push_str(&serde_json::to_string(sous).unwrap_or_default());
                sortie.push(':');
                ecrire(sortie, &map[*sous], ordre_de(sous), sous, heritees);
            }
            sortie.push('}');
        }
        Value::Array(elements) => {
            sortie.push('[');
            for (n, element) in elements.iter().enumerate() {
                if n > 0 {
                    sortie.push(',');
                }
                ecrire(sortie, element, ordre, cle, decimales);
            }
            sortie.push(']');
        }
        Value::Number(brut) => {
            match (
                AUX_DECIMALES_DE_LECHELLE.contains(&cle),
                decimales,
                brut.as_f64(),
            ) {
                (true, Some(n), Some(x)) => sortie.push_str(&format!("{x:.n$}")),
                _ => sortie.push_str(&brut.to_string()),
            }
        }
        autre => sortie.push_str(&serde_json::to_string(autre).unwrap_or_default()),
    }
}

/// Les bornes déclarées d'une échelle nommée : `min`, `max`, `decimales`, tels
/// que la table close du §2.3 les fixe. `None` quand l'identifiant n'est pas de
/// la table ; `null` en JSON quand l'échelle n'est pas graduée — la famille
/// `administratif` porte un code, pas une position, et une valeur de
/// remplissage y dessinerait une graduation qui n'existe pas.
fn bornes_declarees(echelle: &str) -> Option<(Value, Value, Value)> {
    ECHELLES
        .iter()
        .find(|(id, ..)| *id == echelle)
        .map(|(_, min, max, decimales, _)| (json!(min), json!(max), json!(decimales)))
}

/// Le nombre de décimales d'une échelle nommée. Les valeurs sont **recopiées**
/// des lignes de preuve, y compris leurs zéros terminaux : le front reformate à
/// l'affichage depuis ce champ, il ne rearrondit pas.
fn nombre_de_decimales(echelle: &str) -> Option<usize> {
    ECHELLES
        .iter()
        .find(|(id, ..)| *id == echelle)
        .and_then(|(_, _, _, d, _)| d.map(|n| n as usize))
}

// ------------------------------------------------------------ instantané ----

/// Une ligne de preuve désérialisée, avec ce dont l'export a besoin.
struct Mesure {
    famille: String,
    entite: String,
    echelle: String,
    ligne: Value,
}

fn mesures(lignes: &[String]) -> Result<Vec<Mesure>, String> {
    lignes
        .iter()
        .map(|texte| {
            let ligne: Value = serde_json::from_str(texte)
                .map_err(|e| format!("ligne de preuve illisible : {e}"))?;
            Ok(Mesure {
                famille: ligne["famille"].as_str().unwrap_or_default().to_owned(),
                entite: ligne["entite"].as_str().unwrap_or_default().to_owned(),
                echelle: ligne["echelle"]["id"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                ligne,
            })
        })
        .collect()
}

fn cherche<'a>(registre: &'a Value, id: &str) -> Option<&'a Value> {
    ["entites", "groupes"].iter().find_map(|bloc| {
        registre[*bloc]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .find(|e| e["id"] == id)
    })
}

/// Le libellé d'une bande : le `nom` du registre s'il tient en 40 caractères,
/// sinon le `sigle` — cas de LIOT, dont le nom en compte 48 (ADR 0000 §5).
fn libelle_de_bande(registre: &Value, id: &str) -> String {
    let Some(objet) = cherche(registre, id) else {
        return id.to_owned();
    };
    let nom = objet["nom"].as_str().unwrap_or(id);
    if nom.chars().count() <= 40 {
        nom.to_owned()
    } else {
        objet["sigle"].as_str().unwrap_or(nom).to_owned()
    }
}

/// Les ancres déclarées dans le registre à la date donnée, rendues par leur
/// `id` de groupe. Recopiées du champ `ancre_axe` : elles **ne se dérivent pas**
/// de `echelle.id`, qui n'encode que la convention (§2.3), et le pipeline n'en
/// choisit jamais une lui-même — une ancre absente à cette date l'arrête.
pub fn ancres_du_registre(registre: &Value, date: &str) -> Result<(String, String), String> {
    let mut trouvees: BTreeMap<String, String> = BTreeMap::new();
    for groupe in registre["groupes"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        let axe = &groupe["ancre_axe"];
        if !axe.is_object() {
            continue;
        }
        let (Some(pole), Some(debut), Some(id)) = (
            axe["pole"].as_str(),
            axe["debut"].as_str(),
            groupe["id"].as_str(),
        ) else {
            return Err("registre : ancre incomplète".to_owned());
        };
        if debut > date || axe["fin"].as_str().is_some_and(|f| f < date) {
            continue;
        }
        if trouvees.insert(pole.to_owned(), id.to_owned()).is_some() {
            return Err(format!(
                "V24 : deux ancres déclarées pour le pôle {pole} au {date}"
            ));
        }
    }
    let cherche = |pole: &str| {
        trouvees.get(pole).cloned().ok_or_else(|| {
            format!(
                "V25 : aucune ancre du pôle {pole} valide au {date} — le calcul s'arrête, \
                 et aucune ancre de remplacement n'est choisie (RG-31)"
            )
        })
    };
    Ok((cherche("gauche")?, cherche("droite")?))
}

/// L'instantané du §4.2, bandes construites selon la règle du §4.3.
pub fn construire_instantane(
    description: &Description,
    contrat: &str,
    lignes: &[String],
    registre: &Value,
) -> Result<String, String> {
    let mesures = mesures(lignes)?;
    let (ancre_gauche, ancre_droite) = ancres_du_registre(registre, &description.date)?;

    // Règle 2 : le marqueur `votes` d'un groupe rejoint la bande du parti que sa
    // `composition` désigne, **si et seulement si** elle en désigne exactement
    // un et qu'aucun autre groupe valide à la même date ne désigne ce parti.
    // C'est ce qui empêche d'attribuer à ECOLO les votes de 5 députés
    // communistes.
    let mut declarants: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for groupe in registre["groupes"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        let Some(id) = groupe["id"].as_str() else {
            continue;
        };
        let debut = groupe["debut"].as_str().unwrap_or("9999-12-31");
        if debut > description.date.as_str()
            || groupe["fin"]
                .as_str()
                .is_some_and(|f| f < description.date.as_str())
        {
            continue;
        }
        // Tous les partis désignés comptent, pas seulement ceux des groupes à
        // composition unique : ECOS déclare ECOLO **et** le PCF, et c'est
        // précisément ce qui doit empêcher GDR de rejoindre la bande du PCF.
        for parti in groupe["composition"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .filter_map(|c| c["entite"].as_str())
        {
            declarants
                .entry(parti.to_owned())
                .or_default()
                .push(id.to_owned());
        }
    }
    let bande_de = |entite: &str| -> String {
        let Some(groupe) = cherche(registre, entite).filter(|_| entite.starts_with("groupe."))
        else {
            return entite.to_owned();
        };
        let partis: Vec<&str> = groupe["composition"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .filter_map(|c| c["entite"].as_str())
            .collect();
        match partis.as_slice() {
            [seul] if declarants.get(*seul).is_some_and(|g| g.len() == 1) => (*seul).to_owned(),
            _ => entite.to_owned(),
        }
    };

    // Une bande par entité portant au moins un marqueur avec une valeur, ou dont
    // un marqueur porte une non-publication motivée (règles 1 et 4). Une entité
    // dont aucun marqueur ne porte rien de tout cela est dite dans
    // `sans_mesure` (règle 5) — jamais dessinée, jamais laissée vide.
    let mut par_bande: BTreeMap<String, Vec<&Mesure>> = BTreeMap::new();
    for mesure in &mesures {
        par_bande
            .entry(bande_de(&mesure.entite))
            .or_default()
            .push(mesure);
    }

    let mut bandes: Vec<(Option<f64>, String, Value)> = Vec::new();
    let mut sans_mesure: Vec<Value> = Vec::new();
    for (id, groupe) in &par_bande {
        let visibles: Vec<&&Mesure> = groupe
            .iter()
            .filter(|m| {
                !m.ligne["valeur"].is_null()
                    || !m.ligne["valeur_code"].is_null()
                    || m.ligne["motif_code"] == "sous_seuil_de_publication"
            })
            .collect();
        if visibles.is_empty() {
            let premiere = groupe[0];
            sans_mesure.push(serde_json::json!({
                "entite": id,
                "libelle": libelle_de_bande(registre, id),
                "motif_code": premiere.ligne["motif_code"].clone(),
                "motif": premiere.ligne["motif"].clone(),
            }));
            continue;
        }
        // Les marqueurs sont dans l'ordre de `familles` du manifeste (§7).
        let mut marqueurs: Vec<(usize, Value)> = Vec::new();
        for mesure in &visibles {
            let rang = FAMILLES
                .iter()
                .position(|(f, ..)| *f == mesure.famille)
                .ok_or_else(|| format!("famille inconnue : {}", mesure.famille))?;
            marqueurs.push((rang, marqueur(mesure, registre)));
        }
        marqueurs.sort_by_key(|(rang, _)| *rang);
        let position = marqueurs
            .iter()
            .find(|(_, m)| m["famille"] == "votes")
            .and_then(|(_, m)| m["valeur"].as_f64());
        bandes.push((
            position,
            id.clone(),
            serde_json::json!({
                "id": id,
                "libelle": libelle_de_bande(registre, id),
                "marqueurs": marqueurs.into_iter().map(|(_, m)| m).collect::<Vec<_>>(),
            }),
        ));
    }
    // §4.3 règle 5, appliquée au **registre** et pas aux seules entités mesurées :
    // une entité qui ne porte aucune ligne de preuve n'a pas de marqueur, donc
    // aucun marqueur ne porte de valeur — elle est dite dans `sans_mesure`. Sans
    // ce passage elle disparaît de l'instantané sans motif, ce qui est une
    // absence comblée par un vide.
    let dites: BTreeSet<String> = bandes
        .iter()
        .map(|(_, id, _)| id.clone())
        .chain(
            sans_mesure
                .iter()
                .filter_map(|e| e["entite"].as_str().map(str::to_owned)),
        )
        .collect();
    for entite in registre["entites"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        let Some(id) = entite["id"].as_str() else {
            continue;
        };
        if !(id.starts_with("parti.") || id.starts_with("coalition.")) || dites.contains(id) {
            continue;
        }
        if entite["debut"]
            .as_str()
            .is_some_and(|d| d > description.date.as_str())
            || entite["fin"]
                .as_str()
                .is_some_and(|f| f < description.date.as_str())
        {
            continue;
        }
        sans_mesure.push(serde_json::json!({
            "entite": id,
            "libelle": libelle_de_bande(registre, id),
            "motif_code": "aucune_mesure",
            "motif": "Aucune ligne de preuve ne porte cette entité : aucune famille de mesure ne l'a produite.",
        }));
    }
    sans_mesure.sort_by(|a, b| a["entite"].as_str().cmp(&b["entite"].as_str()));

    // §7 — `bandes` par valeur du marqueur `votes` puis par `id`. Une bande sans
    // marqueur `votes` n'a pas de position : elle vient après, par `id`.
    bandes.sort_by(|a, b| match (a.0, b.0) {
        (Some(x), Some(y)) => x.total_cmp(&y).then_with(|| a.1.cmp(&b.1)),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.1.cmp(&b.1),
    });

    let vue = serde_json::json!({
        "schema": SCHEMA_INSTANTANE,
        "contrat": contrat,
        "id": description.id,
        "chambre": description.chambre,
        "legislature": description.legislature,
        "date": description.date,
        "date_arretee": date_arretee(lignes)?,
        "ancrage": {
            "famille": "votes",
            "ancre_gauche": ancre_gauche,
            "ancre_droite": ancre_droite,
            "note": description.note_ancrage,
        },
        "bandes": bandes.into_iter().map(|(.., b)| b).collect::<Vec<_>>(),
        "sans_mesure": sans_mesure,
    });
    Ok(rendre(&vue, CLES_INSTANTANE))
}

fn marqueur(mesure: &Mesure, registre: &Value) -> Value {
    let ligne = &mesure.ligne;
    let echelle = mesure.echelle.as_str();
    let libelle = match mesure.famille.as_str() {
        "votes" => {
            let sigle = cherche(registre, &mesure.entite)
                .and_then(|g| g["sigle"].as_str())
                .unwrap_or(&mesure.entite);
            format!("Votes du groupe {sigle}")
        }
        famille => LIBELLES
            .iter()
            .find(|(f, _)| *f == famille)
            .map_or_else(|| famille.to_owned(), |(_, l)| (*l).to_owned()),
    };
    let dispersion = if ligne["dispersion"].is_object() {
        serde_json::json!({
            "effectif": ligne["dispersion"]["effectif"].clone(),
            "iqr": ligne["dispersion"]["iqr"].clone(),
        })
    } else {
        Value::Null
    };
    serde_json::json!({
        "famille": mesure.famille,
        "echelle": echelle,
        "valeur": ligne["valeur"].clone(),
        "valeur_code": ligne["valeur_code"].clone(),
        "libelle": libelle,
        "motif_code": ligne["motif_code"].clone(),
        "motif": ligne["motif"].clone(),
        "dispersion": dispersion,
        "preuve": ligne["id"].clone(),
    })
}

// -------------------------------------------------------------- manifeste ---

/// Le manifeste du §4.1. Il ne porte **aucune valeur mesurée** : de quoi
/// choisir un fichier, la légende des familles, et la date d'arrêt dérivée.
///
/// `date_arretee` et `mention_paternite` sont **dérivées, jamais saisies** : un
/// bandeau saisi à la main reste juste tant que quelqu'un y pense.
pub fn construire_manifeste(
    contrat: &str,
    instantanes: &[(Description, String)],
    lignes: &[String],
    licences: &BTreeMap<String, String>,
) -> Result<String, String> {
    let mesures = mesures(lignes)?;

    // La mention de paternité vient de l'entrée **amont** des lignes
    // référencées : le registre d'entités est un fichier du projet, pas la
    // source dont la licence exige la mention au point de réutilisation.
    let mut producteurs: BTreeSet<String> = BTreeSet::new();
    let mut derniere = String::new();
    for mesure in &mesures {
        for entree in mesure.ligne["entrees"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            if entree["source"] == "registre_partis"
                && mesures.iter().any(|m| {
                    m.ligne["entrees"]
                        .as_array()
                        .map(Vec::as_slice)
                        .unwrap_or_default()
                        .iter()
                        .any(|e| e["source"] != "registre_partis")
                })
            {
                continue;
            }
            if let Some(producteur) = entree["producteur"].as_str() {
                producteurs.insert(producteur.to_owned());
            }
            if let Some(date) = entree["derniere_mise_a_jour"].as_str()
                && date > derniere.as_str()
            {
                derniere = date.to_owned();
            }
        }
    }
    if producteurs.is_empty() || derniere.is_empty() {
        return Err(
            "mention de paternité : aucune entrée amont dans les lignes référencées".to_owned(),
        );
    }
    // La licence est celle **de chaque producteur**, jamais une licence
    // recopiée de l'un sur l'autre. Deux des quatre sources ne sont pas de
    // l'Assemblée : CHES ne publie aucune licence, et le fichier du ministère
    // est en v2.0. Attribuer une Licence Ouverte v1.0 aux trois est une fausse
    // attribution, pas une approximation (RG-76, REC-07).
    //
    // Une licence absente de la table est une **erreur bloquante** : mieux vaut
    // un pipeline qui s'arrête qu'un manifeste qui revendique au hasard.
    let mut fragments = Vec::new();
    for producteur in &producteurs {
        let licence = licences.get(producteur).ok_or_else(|| {
            format!(
                "mention de paternité : aucune licence connue pour « {producteur} » — \
                 une licence ne se devine pas"
            )
        })?;
        fragments.push(format!("{producteur} — {licence}"));
    }
    let mention = format!("{} — données du {derniere}", fragments.join(" ; "));

    let mut liste = Vec::new();
    for (description, vue) in instantanes {
        let fichier = format!("{vue}\n");
        let racine: Value =
            serde_json::from_str(vue).map_err(|e| format!("instantané illisible : {e}"))?;
        liste.push(serde_json::json!({
            "id": description.id,
            "chambre": description.chambre,
            "legislature": description.legislature,
            "date": description.date,
            "url": format!("instantanes/{}.json", description.id),
            "empreinte_sha256": empreinte(fichier.as_bytes()),
            "octets": fichier.len(),
            "bandes": racine["bandes"].as_array().map_or(0, Vec::len),
        }));
    }

    // Chaque famille du manifeste porte les bornes de **son** échelle, recopiées
    // de `echelle.*` des lignes de preuve. Le front les lit au lieu de les
    // dériver des valeurs observées : trois échelles étirées sur la même plage
    // de pixels fabriquent des concordances qui n'existent pas, et rendent la
    // moyenne entre familles visuellement dessinable.
    let mut declarees: BTreeMap<String, (String, Value, Value, Value)> = BTreeMap::new();
    for mesure in &mesures {
        let echelle = &mesure.ligne["echelle"];
        let declaree = (
            mesure.echelle.clone(),
            echelle["min"].clone(),
            echelle["max"].clone(),
            echelle["decimales"].clone(),
        );
        if let Some(deja) = declarees.get(&mesure.famille)
            && *deja != declaree
        {
            // Deux jeux de bornes pour une famille n'est pas un arbitrage à
            // rendre : c'est une erreur bloquante. Publier l'un des deux
            // publierait une graduation qu'une partie des lignes contredit.
            return Err(format!(
                "la famille {} déclare deux échelles divergentes : {:?} et {:?}",
                mesure.famille, deja, declaree
            ));
        }
        declarees.insert(mesure.famille.clone(), declaree);
    }
    let mut familles = Vec::new();
    for (id, libelle, echelle) in FAMILLES {
        let (min, max, decimales) = match declarees.get(id) {
            Some((declaree, min, max, decimales)) => {
                if declaree != echelle {
                    return Err(format!(
                        "la famille {id} est déclarée sur l'échelle {declaree} et le manifeste la nomme {echelle}"
                    ));
                }
                (min.clone(), max.clone(), decimales.clone())
            }
            // Aucune ligne de cette famille : les bornes viennent de la table
            // close des échelles (§2.3), jamais d'une valeur de remplissage.
            None => bornes_declarees(echelle)
                .ok_or_else(|| format!("échelle inconnue au manifeste : {echelle}"))?,
        };
        familles.push(serde_json::json!({
            "id": id,
            "libelle": libelle,
            "echelle": echelle,
            "min": min,
            "max": max,
            "decimales": decimales,
        }));
    }

    let manifeste = serde_json::json!({
        "schema": SCHEMA_MANIFESTE,
        "contrat": contrat,
        "schemas": ["contrepoint/preuve/1", SCHEMA_INSTANTANE, SCHEMA_ECLAT],
        "date_arretee": date_arretee(lignes)?,
        "licence": LICENCE,
        "mention_paternite": mention,
        "familles": familles,
        "instantanes": liste,
        "preuves": {"racine": RACINE_DES_PREUVES, "eclats": ECLATS, "fonction": FONCTION_DECLAT},
    });
    Ok(rendre(&manifeste, CLES_MANIFESTE))
}

// ------------------------------------------------------------------ éclats --

/// Les éclats du §4.4 : `<xx>` = les **deux premiers caractères** de l'`id`. Le
/// chemin se dérive de l'identifiant du marqueur — aucun index, aucune requête
/// préalable, un clic télécharge un fichier.
///
/// Les lignes y sont **identiques octet pour octet** à celles du registre : le
/// front n'affiche jamais une preuve reformatée. Un éclat ne contient que les
/// lignes référencées par au moins un instantané ; l'historique complet vit dans
/// le dépôt, pas dans le navigateur.
pub fn construire_eclats(
    lignes: &[String],
    instantanes: &[String],
) -> Result<BTreeMap<String, String>, String> {
    let mut references: BTreeSet<String> = BTreeSet::new();
    for vue in instantanes {
        let racine: Value =
            serde_json::from_str(vue).map_err(|e| format!("instantané illisible : {e}"))?;
        for bande in racine["bandes"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            for marqueur in bande["marqueurs"]
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or_default()
            {
                if let Some(preuve) = marqueur["preuve"].as_str() {
                    references.insert(preuve.to_owned());
                }
            }
        }
    }

    let mut par_prefixe: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for texte in lignes {
        let ligne: Value =
            serde_json::from_str(texte).map_err(|e| format!("ligne de preuve illisible : {e}"))?;
        let Some(id) = ligne["id"].as_str() else {
            return Err("ligne de preuve sans `id`".to_owned());
        };
        if !references.contains(id) {
            continue;
        }
        par_prefixe
            .entry(id[..2].to_owned())
            .or_default()
            .push((id.to_owned(), texte.clone()));
    }
    for contenu in par_prefixe.values_mut() {
        contenu.sort();
        contenu.dedup();
    }
    Ok(par_prefixe
        .into_iter()
        .map(|(prefixe, contenu)| {
            let corps: Vec<String> = contenu.into_iter().map(|(_, ligne)| ligne).collect();
            (prefixe, format!("[{}]", corps.join(",")))
        })
        .collect())
}

// -------------------------------------------------------- invariants -------

/// Les invariants du §6 qui portent sur les **artefacts** : I11, I12, I13, I16,
/// I17, I18, I19, I20, et la liste blanche des clés de chaque schéma publié.
///
/// Un artefact qui viole une règle n'est pas corrigé, il est refusé.
pub fn verifier_artefacts(
    manifeste: &str,
    instantanes: &[String],
    eclats: &BTreeMap<String, String>,
    lignes: &[String],
) -> Vec<String> {
    let mut refus = Vec::new();
    let Ok(manifeste_json) = serde_json::from_str::<Value>(manifeste) else {
        return vec!["manifeste illisible".to_owned()];
    };
    cles_exactes(&manifeste_json, CLES_MANIFESTE, "le manifeste", &mut refus);
    // I7 sur le manifeste — toute famille porte les bornes de son échelle, et
    // ce sont celles de la table close du §2.3. Sans elles, le front dérive la
    // graduation des valeurs observées et étire trois échelles sur la même
    // plage ; avec des bornes fausses, il dessine une graduation qui n'existe
    // pas. Les deux se refusent ici.
    for famille in manifeste_json["familles"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        cles_exactes(
            famille,
            CLES_FAMILLE,
            "une famille du manifeste",
            &mut refus,
        );
        let echelle = famille["echelle"].as_str().unwrap_or_default();
        match bornes_declarees(echelle) {
            None => refus.push(format!(
                "I7 : la famille {} du manifeste nomme l'échelle {echelle}, hors de la liste close",
                famille["id"]
            )),
            Some((min, max, decimales)) => {
                if famille["min"] != min
                    || famille["max"] != max
                    || famille["decimales"] != decimales
                {
                    refus.push(format!(
                        "I7 : la famille {} du manifeste déclare {}..{} à {} décimales, l'échelle {echelle} porte {min}..{max} à {decimales}",
                        famille["id"], famille["min"], famille["max"], famille["decimales"]
                    ));
                }
            }
        }
    }
    for artefact in [manifeste]
        .iter()
        .copied()
        .chain(instantanes.iter().map(String::as_str))
    {
        controles_de_texte(artefact, &mut refus);
    }
    for eclat in eclats.values() {
        controles_de_texte(eclat, &mut refus);
    }

    let index: BTreeMap<&str, &str> = lignes
        .iter()
        .filter_map(|l| {
            let debut = l.find(r#""id":""#)? + 6;
            Some((&l[debut..debut + 64], l.as_str()))
        })
        .collect();

    let mut references: BTreeSet<String> = BTreeSet::new();
    for vue in instantanes {
        let Ok(racine) = serde_json::from_str::<Value>(vue) else {
            refus.push("instantané illisible".to_owned());
            continue;
        };
        cles_exactes(&racine, CLES_INSTANTANE, "un instantané", &mut refus);
        cles_exactes(&racine["ancrage"], CLES_ANCRAGE, "`ancrage`", &mut refus);
        let mut citees: BTreeSet<String> = BTreeSet::new();
        for bande in racine["bandes"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            cles_exactes(bande, CLES_BANDE, "une bande", &mut refus);
            // I11 — aucun nombre atteignable depuis `bandes[]` hors d'un
            // `marqueurs[]`, hors `effectif`.
            for (cle, valeur) in bande.as_object().into_iter().flatten() {
                if cle != "marqueurs" && valeur.is_number() {
                    refus.push(format!("I11 : nombre `{cle}` hors d'un marqueur"));
                }
            }
            for marqueur in bande["marqueurs"]
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or_default()
            {
                cles_exactes(marqueur, CLES_MARQUEUR, "un marqueur", &mut refus);
                let Some(preuve) = marqueur["preuve"].as_str() else {
                    refus.push("I16 : marqueur sans `preuve`".to_owned());
                    continue;
                };
                references.insert(preuve.to_owned());
                citees.insert(preuve.to_owned());
                let Some(ligne) = index.get(preuve) else {
                    refus.push(format!(
                        "I16 : le marqueur cite la preuve {preuve}, absente du registre — un marqueur sans ligne ne s'affiche pas"
                    ));
                    continue;
                };
                match eclats.get(&preuve[..2]) {
                    None => refus.push(format!("I16 : aucun éclat publié pour le préfixe {}", &preuve[..2])),
                    Some(eclat) if !eclat.contains(*ligne) => refus.push(format!(
                        "I16 : la ligne {preuve} servie au front n'est pas identique octet pour octet à celle du registre"
                    )),
                    Some(_) => {}
                }
            }
        }
        // I17 — `date_arretee` dérivée, jamais saisie. Elle porte sur les lignes
        // que **cet** instantané référence, pas sur l'union : deux instantanés
        // n'ont pas la même date d'arrêt.
        i17_date_arretee(&racine, "de l'instantané", &citees, &index, &mut refus);
    }

    // I17 porte sur « le manifeste **et** chaque instantané » (§6). Le manifeste
    // arrête la même date sur l'union des lignes référencées ; sans ce contrôle,
    // une date saisie à la main y passait.
    i17_date_arretee(
        &manifeste_json,
        "du manifeste",
        &references,
        &index,
        &mut refus,
    );

    // I16, sens inverse — aucun éclat orphelin.
    for (prefixe, eclat) in eclats {
        let Ok(contenu) = serde_json::from_str::<Value>(eclat) else {
            refus.push(format!("éclat {prefixe} illisible"));
            continue;
        };
        for ligne in contenu.as_array().map(Vec::as_slice).unwrap_or_default() {
            let Some(id) = ligne["id"].as_str() else {
                continue;
            };
            if !references.contains(id) {
                refus.push(format!(
                    "I16 : ligne {id} publiée dans un éclat sans marqueur qui la référence"
                ));
            }
        }
    }
    refus.sort();
    refus.dedup();
    refus
}

fn i17_date_arretee(
    artefact: &Value,
    ou: &str,
    references: &BTreeSet<String>,
    index: &BTreeMap<&str, &str>,
    refus: &mut Vec<String>,
) {
    let referencees: Vec<String> = references
        .iter()
        .filter_map(|id| index.get(id.as_str()).map(|l| (*l).to_owned()))
        .collect();
    match date_arretee(&referencees) {
        Ok(attendue) if artefact["date_arretee"] == attendue.as_str() => {}
        Ok(attendue) => refus.push(format!(
            "I17 : `date_arretee` {ou} vaut {} et le maximum des `date_calcul` référencées est {attendue}",
            artefact["date_arretee"]
        )),
        Err(erreur) => refus.push(format!("I17 : {erreur}")),
    }
}

fn cles_exactes(objet: &Value, attendues: &[&str], ou: &str, refus: &mut Vec<String>) {
    let Some(map) = objet.as_object() else {
        refus.push(format!("{ou} n'est pas un objet"));
        return;
    };
    for cle in map.keys() {
        if !attendues.contains(&cle.as_str()) {
            refus.push(format!(
                "clé `{cle}` absente du schéma publié dans {ou} — le producteur refuse d'écrire ce que le schéma ne déclare pas"
            ));
        }
    }
    for attendue in attendues {
        if !map.contains_key(*attendue) {
            refus.push(format!("clé `{attendue}` absente de {ou}"));
        }
    }
}

/// I12, I13, I18, I19 et I20 sur le **texte** de l'artefact : ce sont des
/// contrôles mécaniques, et une règle vérifiée par expression rationnelle coûte
/// moins cher qu'une règle vérifiée par relecture.
fn controles_de_texte(artefact: &str, refus: &mut Vec<String>) {
    let Ok(racine) = serde_json::from_str::<Value>(artefact) else {
        refus.push("artefact illisible — la porte ne peut rien affirmer".to_owned());
        return;
    };
    let mut ligne = racine.clone();
    if let Some(map) = ligne.as_object_mut() {
        map.remove("id");
    }
    for erreur in crate::preuves::verifier(&ligne) {
        if erreur.starts_with("I12")
            || erreur.starts_with("I18")
            || erreur.starts_with("I19")
            || erreur.starts_with("I20")
        {
            refus.push(erreur);
        }
    }
    // I13 — `grep -E '\bPA[0-9]{4,}\b'` sur les artefacts publiés est vide.
    let octets = artefact.as_bytes();
    for n in 0..octets.len().saturating_sub(5) {
        if octets[n] == b'P'
            && octets[n + 1] == b'A'
            && octets[n + 2..n + 6].iter().all(u8::is_ascii_digit)
            && (n == 0 || !octets[n - 1].is_ascii_alphanumeric())
        {
            refus.push("I13 : identifiant d'acteur dans un artefact publié (RG-41)".to_owned());
            break;
        }
    }
}
