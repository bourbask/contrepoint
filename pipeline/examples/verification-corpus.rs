//! Vérification de fin de cycle, pas un test : rejoue l'ingestion, la matrice,
//! l'estimateur et l'agrégation sur l'archive complète du cache, et affiche les
//! grandeurs de `docs/brique0/verification-2026-08-27.md` §1 à §4. Exige le
//! cache, donc ne peut pas vivre dans la suite hors ligne.
//!
//!   cargo run --release --example verification-corpus
//!
//! L'adaptation de forme d'AMO30 vers l'index de mandats est faite ici et non
//! dans la bibliothèque : la suite hors ligne ne porte aucun échantillon
//! d'acteur AMO30 brut, donc aucun test ne la contraindrait encore.

use contrepoint::agregation::{Membre, Publication, agreger, groupes_valides, rendre};
use contrepoint::estimateur::{ajuster, ancrer_sur_groupes, ancres};
use contrepoint::ingestion::{CAUSES_DE_NON_VOTANT, index_mandats, lire_scrutin};
use contrepoint::matrice::{Entete, Matrice, construire};
use contrepoint::{uid, un_ou_plusieurs};
use serde_json::{Value, json};
use std::collections::BTreeMap;
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

    positions(&matrice);
}

/// Date de référence de l'agrégation. Portée par la ligne de preuve, jamais
/// lue sur l'horloge : c'est le dernier jour de scrutin du corpus.
const DATE_DE_REFERENCE: &str = "2026-07-21";

/// Plis de rééchantillonnage. Chaque pli retire un vingt-cinquième des
/// scrutins, déterminé par le rang du scrutin dans l'ordre canonique — aucun
/// tirage aléatoire n'entre dans le pipeline.
const PLIS: usize = 25;

/// §3 et §4 de verification-2026-08-27.md : les trois sommes des carrés
/// résiduels, le gain du rang 1, et les positions de groupe après ancrage.
fn positions(matrice: &Matrice) {
    let registre: Value =
        lire(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/registre/partis.json"));
    let (ancre_gauche, ancre_droite) =
        ancres(&registre, DATE_DE_REFERENCE).expect("les deux ancres du registre");

    // Rattachement : dernier groupe observé jusqu'à la date de référence. C'est
    // l'approximation de verification-2026-08-27.md §4 ; la spécification
    // retenue reste positionnement.md §6, portée par le cycle du registre.
    let mut groupe_de: BTreeMap<&str, (&str, &str)> = BTreeMap::new();
    let mut votes: BTreeMap<&str, usize> = BTreeMap::new();
    for scrutin in matrice.retenus() {
        if scrutin.date.as_str() > DATE_DE_REFERENCE {
            continue;
        }
        for cellule in &scrutin.cellules {
            *votes.entry(cellule.acteur.as_str()).or_default() += 1;
            let precedent = groupe_de
                .entry(cellule.acteur.as_str())
                .or_insert((scrutin.date.as_str(), cellule.groupe.as_str()));
            if precedent.0 <= scrutin.date.as_str() {
                *precedent = (scrutin.date.as_str(), cellule.groupe.as_str());
            }
        }
    }

    let triplets: Vec<(&str, &str, f64)> = matrice
        .cellules()
        .map(|(s, a, v)| (s, a, f64::from(v)))
        .collect();
    let ajustement = ajuster(triplets.iter().copied(), None).expect("ajustement");
    println!(
        "\nsomme des carrés résiduels : {:.1} autour de la moyenne, {:.1} après constante \
         par scrutin, {:.1} après constante + rang 1",
        ajustement.scr_moyenne, ajustement.scr_constante, ajustement.scr_rang1
    );
    println!(
        "gain du rang 1 sur le résidu : {:.1} % ; sur la variance totale : {:.1} % ; \
         part de la constante seule : {:.1} %",
        100.0 * ajustement.gain_rang1(),
        100.0 * (ajustement.scr_constante - ajustement.scr_rang1) / ajustement.scr_moyenne,
        100.0 * ajustement.part_de_la_constante()
    );
    println!(
        "séparation des axes s2/s1 : {:.3} — alarmes {:?}",
        ajustement.separation_des_axes(),
        ajustement.alarmes()
    );

    let ancrees = ancrage(
        &ajustement.acteurs,
        &ajustement.positions,
        &groupe_de,
        &votes,
        &ancre_gauche,
        &ancre_droite,
    );
    let membres: Vec<Membre> = ajustement
        .acteurs
        .iter()
        .enumerate()
        .map(|(n, acteur)| Membre {
            acteur: acteur.clone(),
            groupe: groupe_de
                .get(acteur.as_str())
                .map_or_else(String::new, |(_, g)| (*g).to_owned()),
            votes_exprimes: votes.get(acteur.as_str()).copied().unwrap_or(0),
            position: ancrees[n],
        })
        .collect();

    // Rééchantillonnage par plis déterminés : le pli k retire les scrutins dont
    // le rang canonique vaut k modulo PLIS, et l'axe est réajusté puis réancré.
    let mut tirages: Vec<Vec<f64>> = Vec::with_capacity(PLIS);
    for k in 0..PLIS {
        let sous_corpus: Vec<(&str, &str, f64)> = triplets
            .iter()
            .copied()
            .filter(|(s, _, _)| {
                ajustement
                    .scrutins
                    .binary_search_by(|n| n.as_str().cmp(s))
                    .unwrap()
                    % PLIS
                    != k
            })
            .collect();
        let pli = ajuster(sous_corpus.into_iter(), None).expect("ajustement du pli");
        let ancrees_pli = ancrage(
            &pli.acteurs,
            &pli.positions,
            &groupe_de,
            &votes,
            &ancre_gauche,
            &ancre_droite,
        );
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

    let groupes = groupes_valides(&registre, DATE_DE_REFERENCE);
    let lignes = agreger(&membres, &tirages, &groupes, DATE_DE_REFERENCE);
    let mut triees = lignes.clone();
    triees.sort_by(|a, b| match (&a.publication, &b.publication) {
        (Publication::Mesuree { mediane: x, .. }, Publication::Mesuree { mediane: y, .. }) => {
            x.total_cmp(y)
        }
        (Publication::Mesuree { .. }, _) => std::cmp::Ordering::Less,
        (_, Publication::Mesuree { .. }) => std::cmp::Ordering::Greater,
        _ => a.groupe.cmp(&b.groupe),
    });
    println!("\nagrégation au {DATE_DE_REFERENCE}, ordre croissant sur l'axe ancré :");
    print!("{}", rendre(&triees));
}

/// Les deux tranches parallèles qu'exige [`ancrer_sur_groupes`], construites
/// depuis le rattachement « dernier groupe observé ». L'ancrage lui-même vit
/// dans la bibliothèque, sous test : cet exemple ne le réimplémente plus.
fn ancrage(
    acteurs: &[String],
    positions: &[f64],
    groupe_de: &BTreeMap<&str, (&str, &str)>,
    votes: &BTreeMap<&str, usize>,
    gauche: &str,
    droite: &str,
) -> Vec<f64> {
    let groupes: Vec<&str> = acteurs
        .iter()
        .map(|a| groupe_de.get(a.as_str()).map_or("", |(_, g)| *g))
        .collect();
    let exprimes: Vec<usize> = acteurs
        .iter()
        .map(|a| votes.get(a.as_str()).copied().unwrap_or(0))
        .collect();
    ancrer_sur_groupes(positions, &groupes, &exprimes, gauche, droite).expect("ancrage")
}
