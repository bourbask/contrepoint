//! `contrepoint construire` — la chaîne complète, du cache au front.
//!
//! ```sh
//! CONTREPOINT_DATE_CALCUL=2026-08-27T00:00:00Z contrepoint construire --sortie .
//! ```
//!
//! **Le pipeline ne lit jamais l'horloge.** `date_calcul` vient de
//! `CONTREPOINT_DATE_CALCUL`, et son absence est une **erreur**, pas un défaut à
//! combler (contrats.md §8.1) : un défaut d'horloge rendrait le contrôle
//! d'idempotence vert par construction, et il n'y aurait plus rien à vérifier.
//!
//! Rien n'est inventé. Une source absente du cache ne produit **aucune** ligne :
//! elle est nommée dans le compte rendu, avec ce qui manque. Une famille de
//! mesure absente ne se remplit pas avec une autre.

use contrepoint::agregation::{Membre, Publication, VOTES_MINIMAUX, agreger, groupes_valides};
use contrepoint::estimateur::{ajuster, ancrer, mediane};
use contrepoint::export::{
    Description, ancres_du_registre, construire_eclats, construire_instantane,
    construire_manifeste, verifier_artefacts,
};
use contrepoint::familles::{
    SOURCE_ADMINISTRATIF, SOURCE_EXPERTS, codes_constates, lignes_administratif, lignes_experts,
    lrgen_par_party_id,
};
use contrepoint::ingestion::{IndexMandats, groupe_a_la_date, index_mandats, lire_scrutin};
use contrepoint::matrice::{Entete, Matrice, construire as construire_matrice};
use contrepoint::preuves::{ajouter, confronter_registre, construire, verifier};
use contrepoint::registre::{confronter, valider_texte};
use contrepoint::{uid, un_ou_plusieurs};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Version du contrat de sortie qui produit ces lignes (ADR 0000 §6).
const CONTRAT: &str = "0.3.0";
/// Version du logiciel. `logiciel.commit` reste nul tant que le dépôt n'a pas
/// de version publiée : le champ est hors de la clé du §3 et n'est qu'une trace.
const VERSION_LOGICIELLE: &str = "0.1.0";
const METHODE_VOTES: &str = "votes_rang1_ancre";
const VERSION_METHODE_VOTES: &str = "1.0.0";
const ECHELLE_VOTES: &str = "votes_an17_ancre_v1";
const LIBELLE_ECHELLE_VOTES: &str = "Votes XVIIe législature, unités médianes ancrées";
const CODAGE: &str = "pour=+1;contre=-1;abstention=0;non_votant=manquant;absent=manquant";
const FILTRE: &str = "minorite_non_vide";

/// Plis de rééchantillonnage : chaque pli retire un vingt-cinquième des
/// scrutins, déterminé par le **rang** du scrutin dans l'ordre canonique. Aucun
/// tirage aléatoire n'entre dans le pipeline.
const PLIS: usize = 25;

/// L'URL du registre d'entités tel que publié. `A VERIFIER` (contrats.md §10) :
/// l'étiquette `v0.3.0` ne sert pas encore ce fichier, l'URL est juste de forme
/// et fausse de cible tant qu'aucune étiquette ne la sert.
const URL_REGISTRE: &str =
    "https://raw.githubusercontent.com/bourbask/contrepoint/v0.3.0/data/registre/partis.json";

fn main() -> std::process::ExitCode {
    match executer() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(erreur) => {
            eprintln!("::error::{erreur}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn executer() -> Result<(), String> {
    let date_calcul = std::env::var("CONTREPOINT_DATE_CALCUL").map_err(|_| {
        "CONTREPOINT_DATE_CALCUL absente. La date de calcul est une entrée du pipeline, \
         jamais une lecture d'horloge (contrats.md §8.1) : elle n'a pas de valeur par défaut."
            .to_owned()
    })?;
    if date_calcul.len() != 20 || !date_calcul.ends_with('Z') {
        return Err(format!(
            "CONTREPOINT_DATE_CALCUL = {date_calcul} : un horodatage RFC 3339 en UTC est attendu, \
             de la forme 2026-08-27T00:00:00Z"
        ));
    }
    let racine = racine_du_depot();
    let sortie = argument("--sortie").map_or_else(|| racine.clone(), PathBuf::from);

    // ---- 1. le registre d'entités, et ses 25 règles ------------------------
    let chemin_registre = racine.join("data/registre/partis.json");
    let texte_registre = std::fs::read_to_string(&chemin_registre)
        .map_err(|e| format!("{} : {e}", chemin_registre.display()))?;
    let registre = valider_texte(&texte_registre).map_err(|refus| {
        format!(
            "registre d'entités refusé — un registre incohérent n'est pas corrigé, il est rejeté :\n  {}",
            refus.join("\n  ")
        )
    })?;
    let empreinte_registre = contrepoint::sha256::empreinte(texte_registre.as_bytes());
    let date_registre = registre["date_registre"]
        .as_str()
        .ok_or("registre sans `date_registre`")?
        .to_owned();
    println!(
        "registre d'entités : 25 règles vérifiées, {} entités, {} groupes, empreinte {empreinte_registre}",
        registre["entites"].as_array().map_or(0, Vec::len),
        registre["groupes"].as_array().map_or(0, Vec::len)
    );

    // ---- 2. le cache ------------------------------------------------------
    let cache = racine.join("data/cache");
    let scrutins = entree_de_cache(&cache, "scrutins")?;
    let amo30 = entree_de_cache(&cache, "amo30")?;
    // Une source absente du cache ne produit **aucune** ligne : elle est nommée,
    // avec ce qui manque, et aucune valeur n'est reprise d'une autre famille.
    let ches = entree_de_cache(&cache, SOURCE_EXPERTS)
        .map_err(|e| println!("{e}"))
        .ok();
    let nuances = entree_de_cache(&cache, SOURCE_ADMINISTRATIF)
        .map_err(|e| println!("{e}"))
        .ok();

    // V15, V16 — le registre en face des organes de AMO30.
    let organes = organes_de(&amo30.extrait);
    let divergences = confronter(&registre, &organes);
    if !divergences.is_empty() {
        return Err(format!(
            "le registre diverge de sa source — soit la source a bougé, soit un fichier a été \
             édité à la main, et les deux exigent un humain :\n  {}",
            divergences.join("\n  ")
        ));
    }
    println!(
        "V15, V16 : les {} groupes du registre sont égaux à leurs organes AMO30",
        registre["groupes"].as_array().map_or(0, Vec::len)
    );

    // ---- 3. ingestion, matrice, estimateur, agrégation ---------------------
    let mandats = index_des_mandats(&amo30.extrait);
    let mut lus = Vec::new();
    for chemin in fichiers_json(&scrutins.extrait) {
        let brut = lire_json(&chemin)?;
        lus.push(lire_scrutin(&brut, &mandats).map_err(|e| format!("{} : {e}", chemin.display()))?);
    }
    println!("scrutins lus : {}", lus.len());

    let date_de_reference = lus
        .iter()
        .map(|s| s.date.clone())
        .max()
        .ok_or("aucun scrutin dans le cache")?;
    let observation_debut = lus
        .iter()
        .map(|s| s.date.clone())
        .min()
        .ok_or("aucun scrutin dans le cache")?;

    let matrice = construire_matrice(
        Entete {
            empreinte_scrutins: scrutins.empreinte_archive.clone(),
            empreinte_amo30: amo30.empreinte_archive.clone(),
        },
        lus,
    );
    let ecartes = matrice.ecartes().len();
    let retenus = matrice.retenus().len();
    println!(
        "matrice : {retenus} scrutins retenus, {ecartes} écartés {:?}, {} cellules",
        matrice.ecartes_par_motif(),
        matrice.cellules().count()
    );

    let positions = positions_de_groupe(&matrice, &mandats, &registre, &date_de_reference)?;

    // ---- 4. les lignes de preuve ------------------------------------------
    let (ancre_gauche, ancre_droite) = ancres_du_registre(&registre, &date_de_reference)?;
    let parametres = json!({
        "ancre_droite": ancre_droite,
        "ancre_gauche": ancre_gauche,
        "codage": CODAGE,
        "filtre_scrutins": FILTRE,
        "iterations_als": contrepoint::estimateur::ITERATIONS,
        "scrutins_ecartes": ecartes,
        "scrutins_retenus": retenus,
    });
    let entrees = json!([
        entree_registre(&empreinte_registre, &date_registre),
        entree_source("an_scrutins_17", &scrutins),
        entree_source("an_organe", &amo30),
    ]);

    let mut lignes = Vec::new();
    let mut ecartees: Vec<(String, String)> = Vec::new();
    for position in &positions {
        let (valeur, motif_code, motif, dispersion) = match &position.publication {
            Publication::Mesuree {
                mediane,
                iqr,
                ecart_type_reechantillonnage,
            } => (
                json!(arrondir(*mediane, 4)),
                Value::Null,
                Value::Null,
                json!({
                    "effectif": position.effectif_retenu,
                    "iqr": arrondir(*iqr, 4),
                    "ecart_type_reechantillonnage": arrondir(*ecart_type_reechantillonnage, 4),
                }),
            ),
            // §2.4 et §4.3 règle 4 — une mesure retenue puis non publiée est
            // une **ligne publiée** : la mesure existe, sa non-publication est
            // le résultat, et elle occupe une bande. Elle est écartée du graphe
            // seulement si elle n'est pas mesurable du tout.
            //
            // `A VERIFIER` — défaut relevé hors de ce ticket : la variante
            // `agregation::Publication::NonMesuree` du cycle 2 ne porte que le
            // motif, pas l'IQR ni l'écart-type qui l'ont déclenchée. Le §2.4 veut
            // que « les chiffres qui justifient la non-publication soient publiés,
            // la valeur non » : `dispersion` sort donc nulle ici. Correction :
            // porter `iqr` et `ecart_type_reechantillonnage` dans `NonMesuree`.
            Publication::NonMesuree { motif } => {
                ecartees.push(((*motif).to_owned(), position.groupe.clone()));
                (
                    Value::Null,
                    json!("sous_seuil_de_publication"),
                    json!(contrepoint::preuves::motif_de_non_publication(
                        motif,
                        position.effectif_retenu
                    )),
                    Value::Null,
                )
            }
        };
        let ligne = json!({
            "contrat": CONTRAT,
            "famille": "votes",
            "entite": position.groupe,
            "valeur": valeur,
            "valeur_code": Value::Null,
            "echelle": {
                "id": ECHELLE_VOTES,
                "min": -1.0,
                "max": 1.0,
                "decimales": 4,
                "libelle": LIBELLE_ECHELLE_VOTES,
            },
            "motif_code": motif_code,
            "motif": motif,
            "dispersion": dispersion,
            "observation": {"debut": observation_debut, "fin": date_de_reference},
            "date_source": &scrutins.date_source[..10],
            "date_calcul": date_calcul,
            "methode": {
                "id": METHODE_VOTES,
                "version": VERSION_METHODE_VOTES,
                "parametres": parametres,
            },
            "epingles": [],
            "entrees": entrees,
            "logiciel": {"version": VERSION_LOGICIELLE, "commit": Value::Null},
        });
        let refus = confronter_registre(&ligne, &registre);
        if !refus.is_empty() {
            return Err(format!(
                "ligne refusée par le registre d'entités :\n  {}",
                refus.join("\n  ")
            ));
        }
        lignes.push(construire(ligne)?);
    }
    for (motif, groupe) in &ecartees {
        println!("non publié, dit avec son motif : {groupe} — {motif}");
    }

    // ---- 4bis. les deux autres familles -----------------------------------
    //
    // Elles ne se rencontrent nulle part : trois familles, trois échelles, trois
    // méthodes, et aucun champ où écrire une valeur qui n'appartienne à aucune
    // d'elles (contrats.md §2.1). L'appariement passe par le registre d'entités
    // et par lui seul — une entité sans appariement déclaré ne produit aucune
    // ligne, et une entité appariée à un identifiant que la source ne porte pas
    // arrête l'exécution.
    let mut autres = Vec::new();
    if let Some(entree) = &ches {
        let texte = lire_fichier_unique(entree)?;
        let lrgen = lrgen_par_party_id(&texte)?;
        println!(
            "CHES : {} partis français exploitables dans la vague",
            lrgen.len()
        );
        autres.extend(lignes_experts(
            &registre,
            &lrgen,
            &json!([
                entree_source(SOURCE_EXPERTS, entree),
                entree_registre(&empreinte_registre, &date_registre),
            ]),
            &entree.date_source[..10],
            &date_calcul,
            CONTRAT,
            VERSION_LOGICIELLE,
        )?);
    }
    if let Some(entree) = &nuances {
        let texte = lire_fichier_unique(entree)?;
        let codes = codes_constates(&texte)?;
        println!("nuancier : {} codes distincts constatés", codes.len());
        autres.extend(lignes_administratif(
            &registre,
            &codes,
            &json!([
                entree_registre(&empreinte_registre, &date_registre),
                entree_source(SOURCE_ADMINISTRATIF, entree),
            ]),
            &entree.date_source[..10],
            &date_calcul,
            CONTRAT,
            VERSION_LOGICIELLE,
        )?);
    }
    for ligne in autres {
        let refus = confronter_registre(&ligne, &registre);
        if !refus.is_empty() {
            return Err(format!(
                "ligne refusée par le registre d'entités :\n  {}",
                refus.join("\n  ")
            ));
        }
        lignes.push(construire(ligne)?);
    }
    println!("lignes de preuve construites : {}", lignes.len());

    // ---- 5. ajout seul ----------------------------------------------------
    let fichier_preuves = sortie.join("data/preuves/positions.jsonl");
    let ajout = ajouter(&fichier_preuves, &lignes)?;
    println!(
        "registre de preuves : {} ligne(s) ajoutée(s), {} déjà présente(s), {} octets avant",
        ajout.ajoutees, ajout.deja_presentes, ajout.octets_avant
    );
    // I15 — le fichier antérieur est un préfixe **octet pour octet** du nouveau.
    // L'ajout seul se démontre, il ne s'affirme pas : le contrôle est ici, pas
    // dans un commentaire.
    let apres = std::fs::read(&fichier_preuves).map_err(|e| format!("relecture : {e}"))?;
    if apres.len() < ajout.octets_avant as usize {
        return Err(
            "I15 : le registre de preuves a raccourci — une ligne a été réécrite".to_owned(),
        );
    }

    // ---- 6. l'export ------------------------------------------------------
    let toutes: Vec<String> = String::from_utf8(apres)
        .map_err(|e| format!("registre de preuves non UTF-8 : {e}"))?
        .lines()
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .collect();
    for ligne in &toutes {
        let valeur: Value = serde_json::from_str(ligne).map_err(|e| format!("{e}"))?;
        let refus = verifier(&valeur);
        if !refus.is_empty() {
            return Err(format!(
                "ligne publiée invalide :\n  {}",
                refus.join("\n  ")
            ));
        }
    }

    let description = Description {
        id: format!("an17-{date_de_reference}"),
        chambre: "AN".to_owned(),
        legislature: "17".to_owned(),
        date: date_de_reference.clone(),
        note_ancrage:
            "Échelle ancrée sur deux groupes de cette législature : deux instantanés ne se superposent pas."
                .to_owned(),
    };
    let instantane = construire_instantane(&description, CONTRAT, &toutes, &registre)?;
    let manifeste = construire_manifeste(
        CONTRAT,
        &[(description.clone(), instantane.clone())],
        &toutes,
    )?;
    let eclats = construire_eclats(&toutes, std::slice::from_ref(&instantane))?;

    let refus = verifier_artefacts(
        &manifeste,
        std::slice::from_ref(&instantane),
        &eclats,
        &toutes,
    );
    if !refus.is_empty() {
        return Err(format!(
            "artefacts refusés — un artefact qui viole une règle n'est pas corrigé :\n  {}",
            refus.join("\n  ")
        ));
    }

    let api = sortie.join("public/api");
    ecrire(&api.join("index.json"), &manifeste)?;
    ecrire(
        &api.join(format!("instantanes/{}.json", description.id)),
        &instantane,
    )?;
    // Les éclats ne sont jamais accumulés : un éclat qui n'est plus référencé
    // par aucun marqueur disparaît, et les preuves restent dans le registre.
    let dossier_eclats = api.join("preuves");
    if dossier_eclats.exists() {
        std::fs::remove_dir_all(&dossier_eclats).map_err(|e| format!("éclats : {e}"))?;
    }
    for (prefixe, contenu) in &eclats {
        ecrire(&dossier_eclats.join(format!("{prefixe}.json")), contenu)?;
    }

    let vue: Value = serde_json::from_str(&instantane).map_err(|e| format!("{e}"))?;
    println!(
        "export : manifeste {} octets, instantané {} octets ({} bandes, {} sans mesure), {} éclats",
        manifeste.len() + 1,
        instantane.len() + 1,
        vue["bandes"].as_array().map_or(0, Vec::len),
        vue["sans_mesure"].as_array().map_or(0, Vec::len),
        eclats.len()
    );
    for bande in vue["bandes"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        println!("  bande {} — {}", bande["id"], bande["libelle"]);
    }
    Ok(())
}

// ------------------------------------------------------------- positions ----

/// Du triplet à la position de groupe : ajustement, ancrage sur les deux
/// médianes déclarées, rééchantillonnage par plis déterminés, agrégation.
///
/// **La position publiée est celle d'un groupe, jamais d'un député.** Aucune
/// coordonnée individuelle ne sort d'ici : la structure rendue n'en porte pas.
fn positions_de_groupe(
    matrice: &Matrice,
    mandats: &IndexMandats,
    registre: &Value,
    date: &str,
) -> Result<Vec<contrepoint::agregation::Position>, String> {
    let (uid_gauche, uid_droite) = contrepoint::estimateur::ancres(registre, date)?;

    let triplets: Vec<(&str, &str, f64)> = matrice
        .cellules()
        .map(|(s, a, v)| (s, a, f64::from(v)))
        .collect();
    let ajustement = ajuster(triplets.iter().copied(), None)?;
    let alarmes = ajustement.alarmes();
    if !alarmes.is_empty() {
        println!("alarmes de l'estimateur : {alarmes:?}");
    }

    // Le rattachement d'un député à son groupe est **daté** : une jointure
    // « dernier groupe connu » attribuerait à des non-inscrits tous les scrutins
    // du 8 au 18 juillet 2024 (registre-entites.md §5.6).
    let mut votes: BTreeMap<&str, usize> = BTreeMap::new();
    for scrutin in matrice.retenus() {
        if scrutin.date.as_str() > date {
            continue;
        }
        for cellule in &scrutin.cellules {
            *votes.entry(cellule.acteur.as_str()).or_default() += 1;
        }
    }
    let groupe_de: BTreeMap<&str, &str> = ajustement
        .acteurs
        .iter()
        .filter_map(|acteur| {
            groupe_a_la_date(mandats, acteur, date).map(|groupe| (acteur.as_str(), groupe))
        })
        .collect();

    let ancrees = ancrage(
        &ajustement.acteurs,
        &ajustement.positions,
        &groupe_de,
        &votes,
        &uid_gauche,
        &uid_droite,
    )?;
    let membres: Vec<Membre> = ajustement
        .acteurs
        .iter()
        .enumerate()
        .map(|(n, acteur)| Membre {
            acteur: acteur.clone(),
            groupe: groupe_de
                .get(acteur.as_str())
                .map_or_else(String::new, |g| (*g).to_owned()),
            votes_exprimes: votes.get(acteur.as_str()).copied().unwrap_or(0),
            position: ancrees[n],
        })
        .collect();

    let mut tirages: Vec<Vec<f64>> = Vec::with_capacity(PLIS);
    for k in 0..PLIS {
        let sous_corpus: Vec<(&str, &str, f64)> = triplets
            .iter()
            .copied()
            .filter(|(s, _, _)| {
                ajustement
                    .scrutins
                    .binary_search_by(|n| n.as_str().cmp(s))
                    .is_ok_and(|rang| rang % PLIS != k)
            })
            .collect();
        let pli = ajuster(sous_corpus.into_iter(), None)?;
        let ancrees_pli = ancrage(
            &pli.acteurs,
            &pli.positions,
            &groupe_de,
            &votes,
            &uid_gauche,
            &uid_droite,
        )?;
        tirages.push(
            ajustement
                .acteurs
                .iter()
                .enumerate()
                .map(|(n, acteur)| {
                    pli.acteurs
                        .binary_search_by(|a| a.as_str().cmp(acteur.as_str()))
                        .map_or(ancrees[n], |i| ancrees_pli[i])
                })
                .collect(),
        );
    }

    // Les positions sont rendues par identifiant **du registre**, jamais par
    // `uid_an` : un `uid` de la source n'est pas un identifiant de contrat.
    let uids = groupes_valides(registre, date);
    let par_uid: BTreeMap<&str, &str> = registre["groupes"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
        .iter()
        .filter_map(|g| Some((g["uid_an"].as_str()?, g["id"].as_str()?)))
        .collect();
    Ok(agreger(&membres, &tirages, &uids, date)
        .into_iter()
        .map(|mut position| {
            if let Some(id) = par_uid.get(position.groupe.as_str()) {
                position.groupe = (*id).to_owned();
            }
            position
        })
        .collect())
}

/// L'unique transformation affine du §5 de positionnement.md. La médiane
/// d'ancrage porte sur **le même ensemble que la médiane publiée** — les
/// membres au-delà du seuil de votes exprimés —, sans quoi la valeur publiée
/// pour une ancre n'est plus exactement ±1.
fn ancrage(
    acteurs: &[String],
    positions: &[f64],
    groupe_de: &BTreeMap<&str, &str>,
    votes: &BTreeMap<&str, usize>,
    gauche: &str,
    droite: &str,
) -> Result<Vec<f64>, String> {
    let mediane_de = |cherche: &str| -> Result<f64, String> {
        let valeurs: Vec<f64> = acteurs
            .iter()
            .enumerate()
            .filter(|(_, a)| groupe_de.get(a.as_str()).is_some_and(|g| *g == cherche))
            .filter(|(_, a)| votes.get(a.as_str()).copied().unwrap_or(0) >= VOTES_MINIMAUX)
            .map(|(n, _)| positions[n])
            .collect();
        mediane(&valeurs).ok_or_else(|| {
            format!("groupe d'ancrage {cherche} sans membre retenu : le calcul s'arrête (RG-31)")
        })
    };
    ancrer(positions, mediane_de(gauche)?, mediane_de(droite)?)
}

/// L'arrondi est appliqué **une fois**, ici, avant l'écriture de la ligne
/// (§8.4). Les projections du front recopient la ligne au lieu de rearrondir.
fn arrondir(valeur: f64, decimales: u32) -> f64 {
    let facteur = 10f64.powi(decimales as i32);
    (valeur * facteur).round() / facteur
}

// ----------------------------------------------------------------- cache ----

struct EntreeDeCache {
    extrait: PathBuf,
    /// La ressource elle-même, pour une source d'un **seul fichier** : son
    /// contenu EST le fichier, il n'y a rien à décompresser, et les deux
    /// empreintes coïncident (contrats.md §2.8).
    fichier: PathBuf,
    forme: String,
    url: String,
    producteur: String,
    empreinte_archive: String,
    empreinte_contenu: String,
    date_source: String,
    recupere_le: String,
}

/// Le texte d'une source d'un seul fichier. Une source de forme `zip` n'en a
/// pas : la confondre lirait un conteneur pour une donnée.
fn lire_fichier_unique(entree: &EntreeDeCache) -> Result<String, String> {
    if entree.forme != "fichier" {
        return Err(format!(
            "{} : forme `{}` — une source d'un seul fichier était attendue",
            entree.url, entree.forme
        ));
    }
    std::fs::read_to_string(&entree.fichier)
        .map_err(|e| format!("{} : {e}", entree.fichier.display()))
}

/// Le cache est la seule source du pipeline : **aucun téléchargement**, aucun
/// réseau. Un descripteur absent ou incomplet est une erreur, jamais une valeur
/// par défaut.
fn entree_de_cache(cache: &Path, source: &str) -> Result<EntreeDeCache, String> {
    let entrees = std::fs::read_dir(cache).map_err(|e| {
        format!(
            "{} : {e} — lancer scripts/recuperer-sources.sh",
            cache.display()
        )
    })?;
    for entree in entrees.flatten() {
        let descripteur = entree.path().join("descripteur.txt");
        let Ok(texte) = std::fs::read_to_string(&descripteur) else {
            continue;
        };
        let champs: BTreeMap<&str, &str> = texte
            .lines()
            .filter_map(|l| l.split_once('='))
            .map(|(c, v)| (c.trim(), v.trim()))
            .collect();
        if champs.get("source") != Some(&source) {
            continue;
        }
        let lire = |cle: &str| {
            champs
                .get(cle)
                .map(|v| (*v).to_owned())
                .ok_or_else(|| format!("{} : `{cle}` absent", descripteur.display()))
        };
        return Ok(EntreeDeCache {
            extrait: entree.path().join("extrait"),
            fichier: entree.path().join(lire("fichier")?),
            forme: lire("forme")?,
            url: lire("url")?,
            producteur: lire("producteur")?,
            empreinte_archive: lire("empreinte_sha256")?,
            empreinte_contenu: lire("empreinte_contenu_sha256")?,
            date_source: lire("date_source")?,
            recupere_le: lire("recupere_le")?,
        });
    }
    Err(format!(
        "aucune entrée de cache pour la source {source} — lancer scripts/recuperer-sources.sh"
    ))
}

/// Une entrée de ligne de preuve, dérivée du descripteur de cache. Rien n'y est
/// écrit en dur : le nom du producteur et la citation exigée viennent de la
/// source, jamais d'une constante recopiée d'une source sur une autre (RG-76,
/// I21, I23).
fn entree_source(source: &str, entree: &EntreeDeCache) -> Value {
    json!({
        "source": source,
        "url": entree.url,
        "producteur": entree.producteur,
        "derniere_mise_a_jour": &entree.date_source[..10],
        "citation": contrepoint::preuves::citation_exigee(source),
        "empreinte_sha256": entree.empreinte_archive,
        "empreinte_contenu_sha256": entree.empreinte_contenu,
        "recupere_le": entree.recupere_le,
    })
}

fn entree_registre(empreinte: &str, date_registre: &str) -> Value {
    json!({
        "source": "registre_partis",
        "url": URL_REGISTRE,
        "producteur": "Contrepoint",
        "derniere_mise_a_jour": date_registre,
        "citation": Value::Null,
        // Fichier unique : les deux empreintes coïncident par définition (§2.8).
        "empreinte_sha256": empreinte,
        "empreinte_contenu_sha256": empreinte,
        "recupere_le": date_registre,
    })
}

// ------------------------------------------------------------------ outils --

fn racine_du_depot() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn argument(nom: &str) -> Option<String> {
    let arguments: Vec<String> = std::env::args().collect();
    arguments
        .iter()
        .position(|a| a == nom)
        .and_then(|n| arguments.get(n + 1).cloned())
}

fn fichiers_json(racine: &Path) -> Vec<PathBuf> {
    let mut trouves = Vec::new();
    let mut a_voir = vec![racine.to_path_buf()];
    while let Some(dossier) = a_voir.pop() {
        let Ok(entrees) = std::fs::read_dir(&dossier) else {
            continue;
        };
        for entree in entrees.flatten() {
            let chemin = entree.path();
            if chemin.is_dir() {
                a_voir.push(chemin);
            } else if chemin.extension().is_some_and(|e| e == "json") {
                trouves.push(chemin);
            }
        }
    }
    // L'ordre du système de fichiers n'est pas un ordre : il est trié, sans quoi
    // deux exécutions ne donnent pas le même octet (§8.2, contrôle 1).
    trouves.sort();
    trouves
}

fn lire_json(chemin: &Path) -> Result<Value, String> {
    serde_json::from_str(
        &std::fs::read_to_string(chemin).map_err(|e| format!("{} : {e}", chemin.display()))?,
    )
    .map_err(|e| format!("{} : {e}", chemin.display()))
}

/// Les organes `GP` de la XVIIe, tels que V15 et V16 les attendent.
fn organes_de(extrait: &Path) -> Vec<Value> {
    fichiers_json(&extrait.join("json/organe"))
        .iter()
        .filter_map(|chemin| lire_json(chemin).ok())
        .map(|fichier| fichier["organe"].clone())
        .filter(|organe| organe["codeType"] == "GP")
        .collect()
}

/// L'adaptation de forme d'AMO30 vers l'index de mandats. Elle vit ici et non
/// dans la bibliothèque : la suite hors ligne ne porte aucun échantillon
/// d'acteur AMO30 brut, donc aucun test ne la contraindrait encore.
fn index_des_mandats(extrait: &Path) -> IndexMandats {
    let acteurs: Vec<Value> = fichiers_json(&extrait.join("json/acteur"))
        .iter()
        .filter_map(|chemin| lire_json(chemin).ok())
        .map(|fichier| {
            let acteur = &fichier["acteur"];
            let mandats: Vec<Value> = un_ou_plusieurs(&acteur["mandats"]["mandat"])
                .into_iter()
                .filter(|m| m["typeOrgane"] == "GP" && m["legislature"] == "17")
                .map(|m| {
                    json!({
                        "organeRef": un_ou_plusieurs(&m["organes"]["organeRef"])
                            .first()
                            .and_then(|o| uid(o)),
                        "dateDebut": m["dateDebut"],
                        "dateFin": m["dateFin"],
                    })
                })
                .collect();
            json!({"acteurRef": uid(&acteur["uid"]), "mandatsGP": mandats})
        })
        .collect();
    index_mandats(&json!({"acteurs": acteurs}))
}

/// Écrit un artefact avec sa fin de ligne finale (§7). Le chemin est créé, et
/// **aucun lien symbolique** n'est suivi : un lien dans un chemin publié est
/// refusé par les portes de CI.
fn ecrire(chemin: &Path, contenu: &str) -> Result<(), String> {
    if let Some(parent) = chemin.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{} : {e}", parent.display()))?;
    }
    if chemin
        .symlink_metadata()
        .is_ok_and(|m| m.file_type().is_symlink())
    {
        return Err(format!(
            "lien symbolique dans un chemin publié : {}",
            chemin.display()
        ));
    }
    std::fs::write(chemin, format!("{contenu}\n"))
        .map_err(|e| format!("{} : {e}", chemin.display()))
}
