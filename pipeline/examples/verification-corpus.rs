//! Vérification de fin de cycle, pas un test : rejoue l'ingestion et la matrice
//! sur l'archive complète du cache et affiche les grandeurs de
//! `docs/brique0/verification-2026-08-27.md`. Exige le cache, donc ne peut pas
//! vivre dans la suite hors ligne.
//!
//!   cargo run --release --example verification-corpus
//!
//! L'adaptation de forme d'AMO30 vers l'index de mandats est faite ici et non
//! dans la bibliothèque : la suite hors ligne ne porte aucun échantillon
//! d'acteur AMO30 brut, donc aucun test ne la contraindrait encore.

use contrepoint::ingestion::{CAUSES_DE_NON_VOTANT, index_mandats, lire_scrutin};
use contrepoint::matrice::{Entete, construire};
use contrepoint::{uid, un_ou_plusieurs};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

fn fichiers(racine: &Path) -> Vec<PathBuf> {
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
    trouves.sort();
    trouves
}

fn lire(chemin: &Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(chemin).expect("fichier lisible"))
        .expect("JSON conforme")
}

/// Trouve l'entrée de cache dont l'extrait porte le chemin repère donné.
fn cache(repere: &str) -> (String, PathBuf) {
    let racine = Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/cache");
    let entrees = std::fs::read_dir(&racine)
        .expect("data/cache absent — lancer scripts/recuperer-sources.sh");
    for entree in entrees.flatten() {
        let extrait = entree.path().join("extrait/json");
        if extrait.join(repere).exists() {
            return (entree.file_name().to_string_lossy().into_owned(), extrait);
        }
    }
    panic!("aucune entrée de cache ne porte {repere}");
}

fn main() {
    let (empreinte_amo30, amo30) = cache("acteur");
    let acteurs: Vec<Value> = fichiers(&amo30.join("acteur"))
        .iter()
        .map(|chemin| {
            let fichier = lire(chemin);
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
    let mandats = index_mandats(&json!({"acteurs": acteurs}));
    println!(
        "acteurs AMO30 indexés : {} dont {} avec au moins un mandat GP",
        mandats.len(),
        mandats.values().filter(|m| !m.is_empty()).count()
    );

    let (empreinte_scrutins, scrutins) = cache("VTANR5L17V1.json");
    let chemins = fichiers(&scrutins);
    println!("fichiers de scrutin : {}", chemins.len());

    let mut po0: Vec<String> = Vec::new();
    let lus: Vec<_> = chemins
        .iter()
        .map(|chemin| {
            let brut = lire(chemin);
            let blocs = &brut["scrutin"]["ventilationVotes"]["organe"]["groupes"]["groupe"];
            if un_ou_plusieurs(blocs)
                .iter()
                .any(|b| b["organeRef"] == "PO0")
            {
                po0.push(
                    brut["scrutin"]["dateScrutin"]
                        .as_str()
                        .unwrap_or("?")
                        .to_owned(),
                );
            }
            lire_scrutin(&brut, &mandats).unwrap_or_else(|e| panic!("{} : {e}", chemin.display()))
        })
        .collect();

    let cellules_completes: usize = lus.iter().map(|s| s.cellules.len()).sum();
    let mut acteurs_complets: Vec<&str> = lus
        .iter()
        .flat_map(|s| s.cellules.iter().map(|c| c.acteur.as_str()))
        .collect();
    acteurs_complets.sort_unstable();
    acteurs_complets.dedup();
    let mut causes = [0usize; 3];
    for scrutin in &lus {
        for (total, n) in causes.iter_mut().zip(scrutin.causes) {
            *total += n;
        }
    }
    let desaccords: usize = lus.iter().map(|s| s.desaccords).sum();
    let mises_au_point: usize = lus.iter().map(|s| s.mises_au_point).sum();

    println!(
        "corpus complet : {cellules_completes} cellules, {} acteurs",
        acteurs_complets.len()
    );
    for (cause, n) in CAUSES_DE_NON_VOTANT.iter().zip(causes) {
        println!("  cause {cause} : {n}");
    }
    println!("  désaccords ventilation / AMO30 : {desaccords}");
    println!("  entrées de mise au point : {mises_au_point}");

    po0.sort();
    let mut dates_po0: Vec<(String, usize)> = Vec::new();
    for date in &po0 {
        match dates_po0.last_mut() {
            Some((d, n)) if d == date => *n += 1,
            _ => dates_po0.push((date.clone(), 1)),
        }
    }
    println!(
        "scrutins portant organeRef PO0 : {} — {dates_po0:?}",
        po0.len()
    );

    let matrice = construire(
        Entete {
            empreinte_scrutins,
            empreinte_amo30,
        },
        lus,
    );
    let cellules = matrice.cellules().count();
    let mut acteurs_retenus: Vec<&str> = matrice.cellules().map(|(_, a, _)| a).collect();
    acteurs_retenus.sort_unstable();
    acteurs_retenus.dedup();
    let retenus = matrice.retenus().len();
    println!(
        "après filtre : {retenus} retenus, {} écartés {:?}, {cellules} cellules, {} acteurs, densité {:.4}",
        matrice.ecartes().len(),
        matrice.ecartes_par_motif(),
        acteurs_retenus.len(),
        cellules as f64 / (acteurs_retenus.len() * retenus) as f64
    );
}
