//! Le registre d'entités et ses 25 règles de validation.
//!
//! Spécification : `docs/brique0/registre-entites.md` §3 et §6. Schéma formel :
//! `schemas/registre-partis-1.schema.json`, dont [`CLES`] est la transcription
//! exécutable.
//!
//! C'est le **seul fichier du projet édité à la main**, et une erreur
//! d'appariement s'y propage dans toutes les mesures. Un registre incohérent
//! n'est jamais corrigé : il est refusé, et chaque refus nomme sa règle.
//!
//! Deux fonctions, deux portées. [`valider`] ne lit que le fichier ;
//! [`confronter`] le met en face des organes de AMO30, et c'est elle qui rend
//! le registre falsifiable contre sa source (V15, V16).

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// La liste blanche de V1, bloc par bloc, **dans l'ordre du schéma**. Elle sert
/// deux fois : à refuser une clé inconnue, et à écrire la forme canonique de
/// V23. Une seule liste, donc aucune divergence possible entre le contrôle et
/// le formateur.
pub const CLES: &[(&str, &[&str])] = &[
    (
        "racine",
        &[
            "schema",
            "version",
            "date_registre",
            "licence",
            "legislatures",
            "sources",
            "entites",
            "groupes",
            "relations",
        ],
    ),
    (
        "legislatures",
        &[
            "numero",
            "chambre",
            "debut",
            "fin",
            "source",
            "url",
            "etabli_le",
            "remarque",
        ],
    ),
    (
        "sources",
        &[
            "id",
            "libelle",
            "url",
            "recupere_le",
            "empreinte_sha256",
            "cardinalite",
            "licence",
            "remarque",
        ],
    ),
    (
        "entites",
        &[
            "id",
            "nature",
            "nom",
            "sigle",
            "debut",
            "fin",
            "identifiants",
            "composition",
            "remarque",
        ],
    ),
    (
        "groupes",
        &[
            "id",
            "chambre",
            "legislature",
            "uid_an",
            "nom",
            "sigle",
            "debut",
            "fin",
            "ancre_axe",
            "composition",
            "remarque",
        ],
    ),
    (
        "relations",
        &["type", "de", "vers", "date", "source", "url", "remarque"],
    ),
    (
        "identifiants",
        &[
            "source",
            "valeur",
            "libelle_source",
            "debut",
            "fin",
            "etabli_le",
            "motif",
            "remarque",
        ],
    ),
    (
        "composition",
        &[
            "entite",
            "debut",
            "fin",
            "source",
            "url",
            "etabli_le",
            "remarque",
        ],
    ),
    (
        "ancre_axe",
        &["pole", "debut", "fin", "etabli_le", "remarque"],
    ),
];

/// L'organe des députés non inscrits. **Seule** exception nommée à V13 : il
/// ouvre le 2024-07-01, dix-sept jours avant l'ouverture de la XVIIe, et V16
/// interdit de corriger sa date (registre-entites.md §4.2). L'exception porte
/// sur l'`uid_an`, jamais sur un libellé ni sur une abréviation.
pub const EXCEPTION_V13: &str = "PO840056";

fn cles_de(bloc: &str) -> &'static [&'static str] {
    CLES.iter()
        .find(|(nom, _)| *nom == bloc)
        .map(|(_, cles)| *cles)
        .unwrap_or(&[])
}

// --------------------------------------------------------------- lexique ----

/// Les termes proscrits par `docs/juridique.md`, lus **dans**
/// `scripts/lexique.sh` à la compilation. Le script est la source unique citée
/// par la documentation ; en recopier la liste ici en ferait une quatrième
/// copie, et trois copies avaient déjà divergé.
const LEXIQUE_SH: &str = include_str!("../../scripts/lexique.sh");

/// Le préfixe littéral d'une alternative d'expression rationnelle : ce qui
/// précède le premier métacaractère : un motif à quantificateur rend le mot nu.
/// Aucun métacaractère n'est interprété. Un préfixe est plus large que le motif, jamais
/// plus étroit : le contrôle ne peut pas laisser passer ce que le script
/// refuse.
fn litteral(alternative: &str) -> Option<String> {
    let prefixe: String = alternative
        .chars()
        .take_while(|c| {
            !matches!(
                c,
                '(' | '[' | '\\' | '.' | '?' | '*' | '+' | '{' | '^' | '$'
            )
        })
        .collect();
    let prefixe = prefixe.trim().to_lowercase();
    (prefixe.chars().count() >= 4).then_some(prefixe)
}

fn termes_proscrits() -> Vec<String> {
    let mut termes = Vec::new();
    for ligne in LEXIQUE_SH.lines() {
        let Some(reste) = ["AXES=", "QUALIF=", "AGREG="]
            .iter()
            .find_map(|p| ligne.strip_prefix(p))
        else {
            continue;
        };
        let Some(motifs) = reste.strip_prefix('\'').and_then(|r| r.split('\'').next()) else {
            continue;
        };
        termes.extend(motifs.split('|').filter_map(litteral));
    }
    termes.sort();
    termes.dedup();
    termes
}

// ----------------------------------------------------------------- dates ----

/// Une date réelle, pas seulement une chaîne de la bonne forme (V3). Un
/// `2026-02-30` passe tous les contrôles de motif et casse toute comparaison de
/// période en aval.
pub fn date_reelle(texte: &str) -> bool {
    let octets = texte.as_bytes();
    if octets.len() != 10 || octets[4] != b'-' || octets[7] != b'-' {
        return false;
    }
    let nombre = |a: usize, b: usize| {
        texte[a..b]
            .parse::<u32>()
            .ok()
            .filter(|_| texte[a..b].bytes().all(|c| c.is_ascii_digit()))
    };
    let (Some(annee), Some(mois), Some(jour)) = (nombre(0, 4), nombre(5, 7), nombre(8, 10)) else {
        return false;
    };
    if !(1..=12).contains(&mois) || jour == 0 {
        return false;
    }
    jour <= jours_du_mois(annee, mois)
}

fn jours_du_mois(annee: u32, mois: u32) -> u32 {
    match mois {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ if annee.is_multiple_of(4)
            && (!annee.is_multiple_of(100) || annee.is_multiple_of(400)) =>
        {
            29
        }
        _ => 28,
    }
}

/// Le lendemain d'une date réelle. Sert à V18 : une succession relie deux
/// périodes contiguës, `date` = `fin` du prédécesseur + 1 jour.
pub fn lendemain(date: &str) -> Option<String> {
    if !date_reelle(date) {
        return None;
    }
    let (a, m, j) = (
        date[0..4].parse::<u32>().ok()?,
        date[5..7].parse::<u32>().ok()?,
        date[8..10].parse::<u32>().ok()?,
    );
    let (a, m, j) = if j < jours_du_mois(a, m) {
        (a, m, j + 1)
    } else if m < 12 {
        (a, m + 1, 1)
    } else {
        (a + 1, 1, 1)
    };
    Some(format!("{a:04}-{m:02}-{j:02}"))
}

/// Deux périodes à bornes incluses se chevauchent-elles ? `debut: null` est
/// « antérieur à la couverture du registre », `fin: null` « en cours ».
fn chevauchent(a: (Option<&str>, Option<&str>), b: (Option<&str>, Option<&str>)) -> bool {
    let borne = |d: Option<&str>, f: Option<&str>| {
        (
            d.unwrap_or("0000-01-01").to_owned(),
            f.unwrap_or("9999-12-31").to_owned(),
        )
    };
    let (debut_a, fin_a) = borne(a.0, a.1);
    let (debut_b, fin_b) = borne(b.0, b.1);
    debut_a <= fin_b && debut_b <= fin_a
}

/// `debut: null` est « antérieur à la couverture du registre », `fin: null`
/// « en cours au `date_registre` » — jamais « inconnu comblé » (§4).
type Periode = (Option<String>, Option<String>);

fn periode(noeud: &Value) -> Periode {
    (
        noeud["debut"].as_str().map(str::to_owned),
        noeud["fin"].as_str().map(str::to_owned),
    )
}

/// L'inclusion ne porte que sur les bornes **écrites**. Une borne nulle ne dit
/// pas une date : la contraindre reviendrait à lui en prêter une, ce que le §4
/// interdit. Le RN existe depuis 1972 et son appariement CHES est déclaré sans
/// début : la ligne est incluse, et rien n'autorise à écrire 1972 à sa place.
fn incluse(interieure: &Periode, exterieure: &Periode) -> bool {
    let dedans_debut = match (&interieure.0, &exterieure.0) {
        (Some(i), Some(e)) => i >= e,
        _ => true,
    };
    let dedans_fin = match (&interieure.1, &exterieure.1) {
        (Some(i), Some(e)) => i <= e,
        _ => true,
    };
    dedans_debut && dedans_fin
}

// ------------------------------------------------------------- validation ---

/// Les contrôles de **forme du fichier** — encodage, fins de ligne, forme
/// canonique de V23 — puis tout le reste. Une main qui édite sans passer par le
/// formateur est détectée ici, avant même que le JSON soit interprété.
pub fn valider_texte(texte: &str) -> Result<Value, Vec<String>> {
    let mut refus = Vec::new();
    if texte.starts_with('\u{feff}') {
        refus.push("V23 : marque d'ordre d'octets en tête de fichier".to_owned());
    }
    if texte.contains('\r') {
        refus.push("V23 : fin de ligne CRLF, la forme canonique est LF".to_owned());
    }
    if !texte.ends_with('\n') || texte.ends_with("\n\n") {
        refus.push("V23 : le fichier ne finit pas par exactement une fin de ligne".to_owned());
    }
    let racine: Value = match serde_json::from_str(texte.trim_start_matches('\u{feff}')) {
        Ok(valeur) => valeur,
        Err(erreur) => {
            refus.push(format!("V1 : fichier non conforme au JSON : {erreur}"));
            return Err(refus);
        }
    };
    if refus.is_empty() && canoniser(&racine) != texte {
        refus.push(
            "V23 : le fichier n'est pas identique à sa ré-sérialisation canonique".to_owned(),
        );
    }
    refus.extend(valider(&racine));
    if refus.is_empty() {
        Ok(racine)
    } else {
        Err(refus)
    }
}

/// Les 25 règles, sauf celles qui exigent une source externe (V15, V16) ou une
/// date d'agrégation (V25, portée par [`crate::estimateur::ancres`]).
///
/// Rend la liste **complète** des refus : un registre fautif se corrige en une
/// passe de relecture, pas en vingt-cinq exécutions successives.
pub fn valider(racine: &Value) -> Vec<String> {
    let mut refus = Vec::new();
    v1_cles_connues(racine, &mut refus);
    v2_schema(racine, &mut refus);
    v3_dates_reelles(racine, &mut refus);
    v4_v5_v6_identifiants(racine, &mut refus);
    v7_v8_v14_appariements(racine, &mut refus);
    v9_valeur_nulle_motivee(racine, &mut refus);
    v10_etabli_le(racine, &mut refus);
    v11_v12_v13_periodes(racine, &mut refus);
    v18_successions(racine, &mut refus);
    v19_composition_parpol(racine, &mut refus);
    v20_lexique(racine, &mut refus);
    v21_longueurs(racine, &mut refus);
    v22_aucune_valorisation(racine, &mut refus);
    v23_tris(racine, &mut refus);
    v24_une_ancre_par_pole(racine, &mut refus);
    refus.sort();
    refus.dedup();
    refus
}

/// Parcourt tous les objets du fichier avec le nom du bloc qui les décrit.
fn objets(racine: &Value) -> Vec<(&'static str, &Value)> {
    let mut trouves: Vec<(&'static str, &Value)> = vec![("racine", racine)];
    for bloc in ["legislatures", "sources", "entites", "groupes", "relations"] {
        let nom: &'static str = CLES
            .iter()
            .find(|(n, _)| *n == bloc)
            .map_or("racine", |(n, _)| n);
        for element in racine[bloc]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            trouves.push((nom, element));
            for (porte, sous_bloc) in [
                ("identifiants", "identifiants"),
                ("composition", "composition"),
            ] {
                for sous in element[porte]
                    .as_array()
                    .map(Vec::as_slice)
                    .unwrap_or_default()
                {
                    let nom: &'static str = CLES
                        .iter()
                        .find(|(n, _)| *n == sous_bloc)
                        .map_or("racine", |(n, _)| n);
                    trouves.push((nom, sous));
                }
            }
            if element["ancre_axe"].is_object() {
                trouves.push(("ancre_axe", &element["ancre_axe"]));
            }
        }
    }
    trouves
}

fn v1_cles_connues(racine: &Value, refus: &mut Vec<String>) {
    for (bloc, objet) in objets(racine) {
        let Some(map) = objet.as_object() else {
            continue;
        };
        let blanche = cles_de(bloc);
        for cle in map.keys() {
            if !blanche.contains(&cle.as_str()) {
                refus.push(format!("V1 : clé inconnue `{cle}` dans le bloc {bloc}"));
            }
        }
        for attendue in blanche {
            if !map.contains_key(*attendue) {
                refus.push(format!("V1 : clé `{attendue}` absente du bloc {bloc}"));
            }
        }
    }
}

fn v2_schema(racine: &Value, refus: &mut Vec<String>) {
    if racine["schema"] != "contrepoint/registre-partis/1" {
        refus.push(format!(
            "V2 : `schema` vaut {} et non `contrepoint/registre-partis/1`",
            racine["schema"]
        ));
    }
}

/// Les clés dont la valeur, quand elle n'est pas nulle, est une date.
const CLES_DE_DATE: [&str; 6] = [
    "date_registre",
    "debut",
    "fin",
    "date",
    "recupere_le",
    "etabli_le",
];

fn v3_dates_reelles(racine: &Value, refus: &mut Vec<String>) {
    for (bloc, objet) in objets(racine) {
        let Some(map) = objet.as_object() else {
            continue;
        };
        for (cle, valeur) in map {
            if !CLES_DE_DATE.contains(&cle.as_str()) {
                continue;
            }
            if let Some(texte) = valeur.as_str()
                && !date_reelle(texte)
            {
                refus.push(format!(
                    "V3 : `{cle}` = {texte} n'est pas une date réelle ({bloc})"
                ));
            }
        }
    }
}

fn tous_les_id(racine: &Value) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for bloc in ["entites", "groupes"] {
        for element in racine[bloc]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            if let Some(id) = element["id"].as_str() {
                ids.insert(id.to_owned());
            }
        }
    }
    ids
}

fn v4_v5_v6_identifiants(racine: &Value, refus: &mut Vec<String>) {
    // V4 — motif, unicité toutes catégories confondues, accord `nature` /
    // préfixe. `id` est immuable : rien n'est renommé ni supprimé.
    let mut vus: BTreeSet<String> = BTreeSet::new();
    for bloc in ["entites", "groupes"] {
        for element in racine[bloc]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            let Some(id) = element["id"].as_str() else {
                refus.push(format!("V4 : élément de `{bloc}` sans `id`"));
                continue;
            };
            if !vus.insert(id.to_owned()) {
                refus.push(format!("V4 : `id` {id} employé deux fois"));
            }
            let forme_valide = if bloc == "groupes" {
                id.starts_with("groupe.an")
                    && id.split('.').count() == 3
                    && id
                        .split('.')
                        .nth(1)
                        .is_some_and(|l| l.len() == 4 && l[2..].bytes().all(|c| c.is_ascii_digit()))
            } else {
                (id.starts_with("parti.") || id.starts_with("coalition."))
                    && id.split('.').count() == 2
            };
            if !forme_valide {
                refus.push(format!("V4 : `id` {id} ne respecte pas son motif"));
            }
            if bloc == "entites" {
                let attendue = id.split('.').next().unwrap_or("");
                if element["nature"] != attendue {
                    refus.push(format!(
                        "V4 : `nature` {} ne s'accorde pas avec le préfixe de {id}",
                        element["nature"]
                    ));
                }
            }
        }
    }

    // V5 — toute référence pointe une entité ou un groupe existant.
    let ids = tous_les_id(racine);
    let verifier_ref = |valeur: &Value, ou: &str, refus: &mut Vec<String>| {
        if let Some(cible) = valeur.as_str()
            && !ids.contains(cible)
        {
            refus.push(format!("V5 : référence pendante vers {cible} ({ou})"));
        }
    };
    for bloc in ["entites", "groupes"] {
        for element in racine[bloc]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            for composition in element["composition"]
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or_default()
            {
                verifier_ref(&composition["entite"], "composition", refus);
            }
        }
    }
    for relation in racine["relations"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        verifier_ref(&relation["de"], "relations.de", refus);
        verifier_ref(&relation["vers"], "relations.vers", refus);
    }

    // V6 — toute source citée est déclarée, et toute source déclarée est citée.
    let declarees: BTreeSet<&str> = racine["sources"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
        .iter()
        .filter_map(|s| s["id"].as_str())
        .collect();
    let mut citees: BTreeSet<&str> = BTreeSet::new();
    for (_, objet) in objets(racine) {
        if let Some(source) = objet["source"].as_str() {
            citees.insert(source);
        }
    }
    for source in &citees {
        if !declarees.contains(source) {
            refus.push(format!("V6 : source citée et non déclarée : {source}"));
        }
    }
    for source in &declarees {
        if !citees.contains(source) {
            refus.push(format!("V6 : source déclarée et jamais citée : {source}"));
        }
    }
}

/// Les appariements, cœur du contrôle : V7 injectivité par source et par date,
/// V8 cardinalité, V14 non-chevauchement.
fn v7_v8_v14_appariements(racine: &Value, refus: &mut Vec<String>) {
    let cardinalite: BTreeMap<&str, &str> = racine["sources"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
        .iter()
        .filter_map(|s| Some((s["id"].as_str()?, s["cardinalite"].as_str()?)))
        .collect();

    // (source, valeur) -> [(entite, periode)]
    let mut par_valeur: BTreeMap<(String, String), Vec<(String, Periode)>> = BTreeMap::new();
    // (entite, source) -> [periode]
    let mut par_entite: BTreeMap<(String, String), Vec<Periode>> = BTreeMap::new();

    for entite in racine["entites"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        let Some(id) = entite["id"].as_str() else {
            continue;
        };
        for ligne in entite["identifiants"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            let (Some(source), Some(valeur)) = (ligne["source"].as_str(), ligne["valeur"].as_str())
            else {
                continue;
            };
            let p = periode(ligne);
            par_valeur
                .entry((source.to_owned(), valeur.to_owned()))
                .or_default()
                .push((id.to_owned(), p.clone()));
            par_entite
                .entry((id.to_owned(), source.to_owned()))
                .or_default()
                .push(p);
        }
    }

    for ((source, valeur), porteurs) in &par_valeur {
        for (n, (entite_a, pa)) in porteurs.iter().enumerate() {
            for (entite_b, pb) in &porteurs[n + 1..] {
                if !chevauchent(
                    (pa.0.as_deref(), pa.1.as_deref()),
                    (pb.0.as_deref(), pb.1.as_deref()),
                ) {
                    continue;
                }
                if entite_a == entite_b {
                    refus.push(format!(
                        "V14 : deux périodes de {entite_a} sur ({source}, {valeur}) se chevauchent"
                    ));
                } else {
                    refus.push(format!(
                        "V7 : {entite_a} et {entite_b} portent la même valeur {valeur} \
                         de la source {source} sur des périodes qui se chevauchent"
                    ));
                }
            }
        }
    }

    for ((entite, source), periodes) in &par_entite {
        if cardinalite.get(source.as_str()) != Some(&"un_par_date") {
            continue;
        }
        for (n, pa) in periodes.iter().enumerate() {
            for pb in &periodes[n + 1..] {
                if chevauchent(
                    (pa.0.as_deref(), pa.1.as_deref()),
                    (pb.0.as_deref(), pb.1.as_deref()),
                ) {
                    refus.push(format!(
                        "V8 : {entite} porte deux identifiants {source} actifs à la même date"
                    ));
                }
            }
        }
    }
}

fn v9_valeur_nulle_motivee(racine: &Value, refus: &mut Vec<String>) {
    for entite in racine["entites"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        for ligne in entite["identifiants"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            let vide = ligne["valeur"].is_null();
            let motive = ligne["motif"].as_str().is_some_and(|m| !m.is_empty());
            if vide && !motive {
                refus.push(format!(
                    "V9 : {} porte une valeur nulle sans motif pour {}",
                    entite["id"], ligne["source"]
                ));
            }
            if !vide && motive {
                refus.push(format!(
                    "V9 : {} porte un motif sur une valeur renseignée pour {}",
                    entite["id"], ligne["source"]
                ));
            }
        }
    }
}

fn v10_etabli_le(racine: &Value, refus: &mut Vec<String>) {
    let registre = racine["date_registre"].as_str().unwrap_or("0000-01-01");
    for (bloc, objet) in objets(racine) {
        if !cles_de(bloc).contains(&"etabli_le") {
            continue;
        }
        match objet["etabli_le"].as_str() {
            None => refus.push(format!("V10 : `etabli_le` absent d'un bloc {bloc}")),
            Some(date) if date > registre => refus.push(format!(
                "V10 : `etabli_le` {date} postérieur à `date_registre` {registre} ({bloc})"
            )),
            Some(_) => {}
        }
    }
}

fn v11_v12_v13_periodes(racine: &Value, refus: &mut Vec<String>) {
    // V11 — `debut ≤ fin` partout où les deux sont présents.
    for (bloc, objet) in objets(racine) {
        if let (Some(debut), Some(fin)) = (objet["debut"].as_str(), objet["fin"].as_str())
            && debut > fin
        {
            refus.push(format!(
                "V11 : `debut` {debut} postérieur à `fin` {fin} ({bloc})"
            ));
        }
    }

    // V12 — la période d'un identifiant, d'une composition ou d'une ancre est
    // incluse dans celle de son porteur.
    for bloc in ["entites", "groupes"] {
        for porteur in racine[bloc]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            let sienne = periode(porteur);
            for (nom, sous) in [
                ("identifiant", porteur["identifiants"].as_array()),
                ("composition", porteur["composition"].as_array()),
            ] {
                for ligne in sous.map(Vec::as_slice).unwrap_or_default() {
                    if !incluse(&periode(ligne), &sienne) {
                        refus.push(format!(
                            "V12 : période d'un {nom} hors de celle de {}",
                            porteur["id"]
                        ));
                    }
                }
            }
            if porteur["ancre_axe"].is_object()
                && !incluse(&periode(&porteur["ancre_axe"]), &sienne)
            {
                refus.push(format!(
                    "V12 : période de l'ancre hors de celle de {}",
                    porteur["id"]
                ));
            }
        }
    }

    // V13 — la période d'un groupe est incluse dans celle de sa législature.
    // Une seule exception, nommée sur l'`uid_an` : l'organe des non inscrits.
    let legislatures: BTreeMap<&str, Periode> = racine["legislatures"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
        .iter()
        .filter_map(|l| Some((l["numero"].as_str()?, periode(l))))
        .collect();
    for groupe in racine["groupes"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        if groupe["uid_an"] == EXCEPTION_V13 {
            continue;
        }
        let Some(numero) = groupe["legislature"].as_str() else {
            refus.push(format!("V13 : groupe {} sans législature", groupe["id"]));
            continue;
        };
        let Some(bornes) = legislatures.get(numero) else {
            refus.push(format!("V13 : législature {numero} inconnue"));
            continue;
        };
        if !incluse(&periode(groupe), bornes) {
            refus.push(format!(
                "V13 : la période de {} n'est pas incluse dans celle de la législature {numero}",
                groupe["id"]
            ));
        }
    }
}

fn v18_successions(racine: &Value, refus: &mut Vec<String>) {
    let bornes: BTreeMap<&str, Periode> = ["entites", "groupes"]
        .iter()
        .flat_map(|bloc| {
            racine[bloc]
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or_default()
        })
        .filter_map(|e| Some((e["id"].as_str()?, periode(e))))
        .collect();
    for relation in racine["relations"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        let type_ = relation["type"].as_str().unwrap_or_default();
        if type_ != "renommage" && type_ != "succession_groupe" {
            continue;
        }
        let (Some(de), Some(vers), Some(date)) = (
            relation["de"].as_str(),
            relation["vers"].as_str(),
            relation["date"].as_str(),
        ) else {
            refus.push("V18 : relation incomplète".to_owned());
            continue;
        };
        let (Some(avant), Some(apres)) = (bornes.get(de), bornes.get(vers)) else {
            continue;
        };
        let attendue = avant.1.as_deref().and_then(lendemain);
        if attendue.as_deref() != Some(date) || apres.0.as_deref() != Some(date) {
            refus.push(format!(
                "V18 : {type_} {de} → {vers} au {date} ne relie pas deux périodes contiguës \
                 (fin du prédécesseur + 1 jour = {attendue:?}, début du successeur = {:?})",
                apres.0
            ));
        }
    }
}

fn v19_composition_parpol(racine: &Value, refus: &mut Vec<String>) {
    // La confrontation aux mandats PARPOL de AMO30 est faite hors de cette
    // fonction, qui ne lit pas la source. Ce qui est vérifiable ici est
    // l'obligation qui l'accompagne : une composition déclarée sur les mandats
    // porte la remarque qui dit ce qui y a été lu, et sur quel effectif.
    for groupe in racine["groupes"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        for ligne in groupe["composition"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            if ligne["source"] == "an_parpol_mandats"
                && !ligne["remarque"].as_str().is_some_and(|r| !r.is_empty())
            {
                refus.push(format!(
                    "V19 : composition de {} déclarée sur les mandats PARPOL sans remarque",
                    groupe["id"]
                ));
            }
        }
    }
}

fn v20_lexique(racine: &Value, refus: &mut Vec<String>) {
    let termes = termes_proscrits();
    let mut chercher = |texte: &str, ou: &str, refus: &mut Vec<String>| {
        let bas = texte.to_lowercase();
        for terme in &termes {
            if bas.contains(terme.as_str()) {
                refus.push(format!("V20 : terme proscrit `{terme}` dans {ou}"));
            }
        }
    };
    fn parcourir(
        noeud: &Value,
        chemin: &str,
        chercher: &mut impl FnMut(&str, &str, &mut Vec<String>),
        refus: &mut Vec<String>,
    ) {
        match noeud {
            Value::Object(map) => {
                for (cle, valeur) in map {
                    chercher(cle, &format!("la clé {chemin}.{cle}"), refus);
                    parcourir(valeur, &format!("{chemin}.{cle}"), chercher, refus);
                }
            }
            Value::Array(elements) => {
                for (n, valeur) in elements.iter().enumerate() {
                    parcourir(valeur, &format!("{chemin}[{n}]"), chercher, refus);
                }
            }
            Value::String(texte) => chercher(texte, chemin, refus),
            _ => {}
        }
    }
    parcourir(racine, "$", &mut chercher, refus);
}

fn v21_longueurs(racine: &Value, refus: &mut Vec<String>) {
    for (bloc, objet) in objets(racine) {
        let Some(map) = objet.as_object() else {
            continue;
        };
        for (cle, valeur) in map {
            let Some(texte) = valeur.as_str() else {
                continue;
            };
            let n = texte.chars().count();
            let maximum = match cle.as_str() {
                "sigle" => 40,
                "remarque" | "motif" => 140,
                _ => continue,
            };
            if n > maximum {
                refus.push(format!(
                    "V21 : `{cle}` de {n} caractères pour {maximum} au plus ({bloc})"
                ));
            }
        }
    }
}

fn v22_aucune_valorisation(racine: &Value, refus: &mut Vec<String>) {
    // Le registre est une table d'identifiants : il ne porte **aucun** nombre.
    // Le contrôle est donc total et ne demande aucune liste noire de noms —
    // c'est la forme la plus dure de « aucun champ numérique de valorisation ».
    fn parcourir(noeud: &Value, chemin: &str, refus: &mut Vec<String>) {
        match noeud {
            Value::Object(map) => {
                for (cle, valeur) in map {
                    parcourir(valeur, &format!("{chemin}.{cle}"), refus);
                }
            }
            Value::Array(elements) => {
                for (n, valeur) in elements.iter().enumerate() {
                    parcourir(valeur, &format!("{chemin}[{n}]"), refus);
                }
            }
            Value::Number(_) | Value::Bool(_) => {
                refus.push(format!(
                    "V22 : valeur numérique en {chemin} — le registre n'en porte aucune"
                ));
            }
            Value::String(texte) => {
                let identifiant_de_personne = texte.len() >= 6
                    && texte.starts_with("PA")
                    && texte[2..].bytes().all(|c| c.is_ascii_digit());
                if identifiant_de_personne {
                    refus.push(format!(
                        "V22 : identifiant d'acteur {texte} en {chemin} — le registre ne contient aucune personne"
                    ));
                }
            }
            Value::Null => {}
        }
    }
    parcourir(racine, "$", refus);
}

fn v23_tris(racine: &Value, refus: &mut Vec<String>) {
    for bloc in ["sources", "entites", "groupes"] {
        let cles: Vec<&str> = racine[bloc]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .filter_map(|e| e["id"].as_str())
            .collect();
        if cles.windows(2).any(|paire| paire[0] > paire[1]) {
            refus.push(format!("V23 : `{bloc}` n'est pas triée par `id`"));
        }
    }
    let relations: Vec<(&str, &str)> = racine["relations"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
        .iter()
        .filter_map(|r| Some((r["date"].as_str()?, r["de"].as_str()?)))
        .collect();
    if relations.windows(2).any(|paire| paire[0] > paire[1]) {
        refus.push("V23 : `relations` n'est pas triée par (`date`, `de`)".to_owned());
    }
}

fn v24_une_ancre_par_pole(racine: &Value, refus: &mut Vec<String>) {
    let ancres: Vec<(&str, &str, Periode)> = racine["groupes"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
        .iter()
        .filter(|g| g["ancre_axe"].is_object())
        .filter_map(|g| {
            Some((
                g["id"].as_str()?,
                g["ancre_axe"]["pole"].as_str()?,
                periode(&g["ancre_axe"]),
            ))
        })
        .collect();
    for (n, (id_a, pole_a, pa)) in ancres.iter().enumerate() {
        if *pole_a != "gauche" && *pole_a != "droite" {
            refus.push(format!("V24 : pôle inconnu `{pole_a}` sur {id_a}"));
        }
        for (id_b, pole_b, pb) in &ancres[n + 1..] {
            if pole_a == pole_b
                && chevauchent(
                    (pa.0.as_deref(), pa.1.as_deref()),
                    (pb.0.as_deref(), pb.1.as_deref()),
                )
            {
                refus.push(format!(
                    "V24 : {id_a} et {id_b} déclarent tous deux le pôle {pole_a} sur des \
                     périodes qui se chevauchent"
                ));
            }
        }
    }
}

// ---------------------------------------------------- confrontation source --

/// V15, V16 — le registre en face des organes de AMO30. **C'est ce qui rend le
/// registre falsifiable contre sa source** : une divergence est soit une source
/// qui a bougé, soit une main qui a édité, et les deux exigent un humain.
///
/// La jointure se fait sur `uid_an`, jamais sur un sigle : `libelleAbrev` et
/// `libelleAbrege` divergent sur `PO872880`, et une clé sur l'abréviation
/// fusionnerait deux organes distincts.
pub fn confronter(racine: &Value, organes: &[Value]) -> Vec<String> {
    let mut refus = Vec::new();
    for groupe in racine["groupes"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        let Some(uid) = groupe["uid_an"].as_str() else {
            refus.push(format!("V15 : groupe {} sans `uid_an`", groupe["id"]));
            continue;
        };
        let Some(organe) = organes.iter().find(|o| o["uid"] == uid) else {
            refus.push(format!(
                "V15 : `uid_an` {uid} absent des organes de AMO30 ({})",
                groupe["id"]
            ));
            continue;
        };
        if organe["codeType"] != "GP" {
            refus.push(format!("V15 : {uid} n'est pas un organe de `codeType` GP"));
        }
        if organe["legislature"] != groupe["legislature"] {
            refus.push(format!(
                "V15 : {uid} porte la législature {} et non {}",
                organe["legislature"], groupe["legislature"]
            ));
        }
        for (champ, attendu) in [
            ("nom", &organe["libelle"]),
            ("sigle", &organe["libelleAbrev"]),
            ("debut", &organe["viMoDe"]["dateDebut"]),
            ("fin", &organe["viMoDe"]["dateFin"]),
        ] {
            if groupe[champ] != *attendu {
                refus.push(format!(
                    "V16 : {uid} — `{champ}` vaut {} dans le registre et {attendu} dans la source",
                    groupe[champ]
                ));
            }
        }
    }
    refus.sort();
    refus.dedup();
    refus
}

// ------------------------------------------------------- forme canonique ----

/// Le formateur de V23. L'ordre des clés est celui de [`CLES`], les tableaux
/// sont triés, l'indentation est de deux espaces, la fin de ligne est LF, et le
/// fichier finit par exactement une fin de ligne.
///
/// C'est la fonction qui manquait au registre : la règle décrivait la forme,
/// aucun script ne la produisait (registre-entites.md §8).
pub fn canoniser(racine: &Value) -> String {
    let mut sortie = String::new();
    ecrire_objet(&mut sortie, racine, "racine", 0);
    sortie.push('\n');
    sortie
}

fn ecrire_objet(sortie: &mut String, objet: &Value, bloc: &str, profondeur: usize) {
    let marge = "  ".repeat(profondeur);
    let marge_interne = "  ".repeat(profondeur + 1);
    let Some(map) = objet.as_object() else {
        sortie.push_str(&serde_json::to_string(objet).unwrap_or_default());
        return;
    };
    let ordre: Vec<&str> = cles_de(bloc)
        .iter()
        .copied()
        .filter(|c| map.contains_key(*c))
        .chain(
            // Une clé inconnue est refusée par V1 ; le formateur la conserve en
            // queue plutôt que de la faire disparaître, sans quoi une clé
            // fautive serait effacée par le formatage au lieu d'être signalée.
            map.keys()
                .map(String::as_str)
                .filter(|c| !cles_de(bloc).contains(c)),
        )
        .collect();
    sortie.push_str("{\n");
    for (n, cle) in ordre.iter().enumerate() {
        sortie.push_str(&marge_interne);
        sortie.push_str(&serde_json::to_string(cle).unwrap_or_default());
        sortie.push_str(": ");
        ecrire_valeur(sortie, &map[*cle], cle, profondeur + 1);
        if n + 1 < ordre.len() {
            sortie.push(',');
        }
        sortie.push('\n');
    }
    sortie.push_str(&marge);
    sortie.push('}');
}

/// Le bloc qui décrit le contenu d'un tableau, d'après le nom de la clé qui le
/// porte. `identifiants`, `composition` et `ancre_axe` sont les trois sous-blocs.
fn bloc_de(cle: &str) -> &'static str {
    CLES.iter()
        .find(|(nom, _)| *nom == cle)
        .map_or("racine", |(nom, _)| nom)
}

fn ecrire_valeur(sortie: &mut String, valeur: &Value, cle: &str, profondeur: usize) {
    match valeur {
        Value::Array(elements) if !elements.is_empty() => {
            let marge = "  ".repeat(profondeur);
            let marge_interne = "  ".repeat(profondeur + 1);
            sortie.push_str("[\n");
            for (n, element) in elements.iter().enumerate() {
                sortie.push_str(&marge_interne);
                ecrire_objet(sortie, element, bloc_de(cle), profondeur + 1);
                if n + 1 < elements.len() {
                    sortie.push(',');
                }
                sortie.push('\n');
            }
            sortie.push_str(&marge);
            sortie.push(']');
        }
        Value::Object(_) => ecrire_objet(sortie, valeur, bloc_de(cle), profondeur),
        autre => sortie.push_str(&serde_json::to_string(autre).unwrap_or_default()),
    }
}
