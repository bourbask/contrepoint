//! Suite EST — rang 1 sur cellules observées, ancrage (plan-de-tests.md §7).
//!
//! Hors ligne. Les fixtures de l'Assemblée ne conviennent pas — trois scrutins
//! retenus sur cinq ne définissent pas un axe — donc l'entrée est une **matrice
//! synthétique construite ici**, dont la solution est connue analytiquement.
//! Les valeurs du corpus réel sont au niveau 2 (`examples/positions-corpus.rs`).

mod commun;

use commun::T2;
use contrepoint::estimateur::{
    GAIN_MINIMAL, MOTIF_POUVOIR_EXPLICATIF, MOTIF_SEPARATION, SEPARATION_MAXIMALE,
    SEPARATION_MINIMALE, ajuster, ancrer, ancres, mediane,
};
use serde_json::Value;

const LFI: &str = "PO845413";
const RN: &str = "PO845401";
const DEM: &str = "PO845454";

/// Les 14 acteurs, leur groupe et leur position de construction. La médiane
/// basse de LFI vaut −0,80 et celle de RN +0,80 : après ancrage, chaque
/// position de construction est multipliée par 1,25 exactement.
fn acteurs() -> Vec<(String, &'static str, f64)> {
    let mut liste = Vec::new();
    for (n, x) in [-0.90, -0.85, -0.80, -0.75, -0.70].iter().enumerate() {
        liste.push((format!("PA{:04}", 100 + n), LFI, *x));
    }
    for (n, x) in [0.00, 0.05, -0.05, 0.10].iter().enumerate() {
        liste.push((format!("PA{:04}", 200 + n), DEM, *x));
    }
    for (n, x) in [0.70, 0.75, 0.80, 0.85, 0.90].iter().enumerate() {
        liste.push((format!("PA{:04}", 300 + n), RN, *x));
    }
    liste
}

/// `y` de moyenne non nulle : l'initialisation de positionnement.md §4 est la
/// moyenne des résidus de la ligne, qui s'annulerait si `ȳ = 0`.
const Y: [f64; 6] = [1.0, 0.8, -0.6, 1.2, 0.5, -0.9];
const B: [f64; 6] = [0.1, -0.2, 0.3, 0.0, 0.4, -0.1];

fn scrutin(j: usize) -> String {
    format!("VT{j:02}")
}

/// `v[i,j] = b[j] + x[i]·y[j]`, observée si `garder(i, j)`.
fn synthetique(garder: impl Fn(usize, usize) -> bool) -> Vec<(String, String, f64)> {
    let mut triplets = Vec::new();
    for (i, (acteur, _, x)) in acteurs().iter().enumerate() {
        for j in 0..Y.len() {
            if garder(i, j) {
                triplets.push((scrutin(j), acteur.clone(), B[j] + x * Y[j]));
            }
        }
    }
    triplets
}

fn complete() -> Vec<(String, String, f64)> {
    synthetique(|_, _| true)
}

/// 77 % de cellules manquantes est le cas réel ; ici un motif fixe qui laisse
/// chaque ligne et chaque colonne suffisamment peuplées.
fn creuse() -> Vec<(String, String, f64)> {
    synthetique(|i, j| (i + 2 * j) % 3 != 0)
}

fn vue(triplets: &[(String, String, f64)]) -> impl Iterator<Item = (&str, &str, f64)> {
    triplets
        .iter()
        .map(|(s, a, v)| (s.as_str(), a.as_str(), *v))
}

/// Les positions ancrées, dans l'ordre des acteurs de l'ajustement.
fn ancrees(triplets: &[(String, String, f64)], depart: Option<&[f64]>) -> Vec<(String, f64)> {
    let ajustement = ajuster(vue(triplets), depart).expect("ajustement");
    let du_groupe = |groupe: &str| {
        let valeurs: Vec<f64> = acteurs()
            .iter()
            .filter(|(_, g, _)| *g == groupe)
            .filter_map(|(a, _, _)| ajustement.position(a))
            .collect();
        mediane(&valeurs).expect("groupe non vide")
    };
    let ancrees = ancrer(&ajustement.positions, du_groupe(LFI), du_groupe(RN)).expect("ancrage");
    ajustement.acteurs.iter().cloned().zip(ancrees).collect()
}

fn ecart_maximal(a: &[(String, f64)], b: &[(String, f64)]) -> f64 {
    assert_eq!(a.len(), b.len(), "mêmes acteurs des deux côtés");
    a.iter()
        .zip(b)
        .map(|((na, va), (nb, vb))| {
            assert_eq!(na, nb, "ordre canonique des acteurs");
            (va - vb).abs()
        })
        .fold(0.0, f64::max)
}

fn source(module: &str) -> String {
    let chemin = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(module);
    std::fs::read_to_string(&chemin).unwrap_or_else(|e| panic!("{} : {e}", chemin.display()))
}

fn registre() -> Value {
    let chemin = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../docs/brique0/echantillons/registre-l17.json");
    serde_json::from_str(&std::fs::read_to_string(chemin).expect("registre lisible"))
        .expect("registre conforme")
}

/// EST-01 — `b[j]` est la moyenne des valeurs **observées** de la colonne, pas
/// de la colonne complétée par des zéros.
#[test]
fn constante_par_scrutin_est_la_moyenne_observee() {
    let triplets = creuse();
    let ajustement = ajuster(vue(&triplets), None).expect("ajustement");
    for (j, nom) in ajustement.scrutins.iter().enumerate() {
        let observees: Vec<f64> = triplets
            .iter()
            .filter(|(s, _, _)| s == nom)
            .map(|(_, _, v)| *v)
            .collect();
        let attendue = observees.iter().sum::<f64>() / observees.len() as f64;
        assert!(
            (ajustement.constantes[j] - attendue).abs() <= T2,
            "EST-01 : b[{nom}] doit valoir la moyenne des {} cellules observées",
            observees.len()
        );
        let completee = observees.iter().sum::<f64>() / acteurs().len() as f64;
        assert!(
            (attendue - completee).abs() > 1e-6,
            "EST-01 : le cas de test doit distinguer la moyenne observée de la moyenne complétée"
        );
    }
}

/// EST-02 [C] — `x[i]` initial est la moyenne des résidus observés de la ligne,
/// et ne dépend donc d'aucun indice.
#[test]
fn initialisation_ne_depend_pas_de_lindice_de_ligne() {
    let triplets = creuse();
    let ajustement = ajuster(vue(&triplets), None).expect("ajustement");
    for (i, acteur) in ajustement.acteurs.iter().enumerate() {
        let residus: Vec<f64> = triplets
            .iter()
            .filter(|(_, a, _)| a == acteur)
            .map(|(s, _, v)| {
                let j = ajustement.scrutins.iter().position(|n| n == s).unwrap();
                v - ajustement.constantes[j]
            })
            .collect();
        let attendue = residus.iter().sum::<f64>() / residus.len() as f64;
        assert!(
            (ajustement.initialisation[i] - attendue).abs() <= T2,
            "EST-02 : x[{acteur}] initial est la moyenne des résidus observés de sa ligne"
        );
    }
    let mut renverse = triplets.clone();
    renverse.reverse();
    let autre = ajuster(vue(&renverse), None).expect("ajustement");
    assert_eq!(
        ajustement.acteurs, autre.acteurs,
        "EST-02 : ordre canonique des acteurs indépendant de l'ordre d'entrée"
    );
    for (a, b) in ajustement.initialisation.iter().zip(&autre.initialisation) {
        assert!(
            (a - b).abs() <= T2,
            "EST-02 : l'initialisation ne dépend pas de l'ordre d'arrivée des triplets"
        );
    }
}

/// EST-03 [C] — revue outillée : aucun générateur, aucune graine, aucune
/// horloge dans les deux modules de calcul.
#[test]
fn aucun_generateur_pseudo_aleatoire() {
    for module in ["src/estimateur.rs", "src/agregation.rs"] {
        let texte = source(module);
        for interdit in [
            "rand",
            "Rng",
            "graine",
            "seed",
            "SystemTime",
            "Instant",
            "now(",
        ] {
            assert!(
                !texte.contains(interdit),
                "EST-03 : « {interdit} » dans {module} — une graine non fixée est un \
                 non-déterminisme (definition-of-done.md §6)"
            );
        }
    }
}

/// EST-04 — sur une matrice exactement de rang 1, les positions ancrées sont
/// celles de la construction. Sans ce test, un estimateur qui rend une
/// constante passerait toutes les invariances.
#[test]
fn solution_exacte_recuperee_sur_matrice_de_rang_1() {
    let obtenues = ancrees(&complete(), None);
    for (acteur, _, x) in acteurs() {
        let obtenue = obtenues
            .iter()
            .find(|(a, _)| *a == acteur)
            .expect("acteur présent")
            .1;
        assert!(
            (obtenue - 1.25 * x).abs() <= T2,
            "EST-04 : {acteur} attendu à {}, obtenu {obtenue}",
            1.25 * x
        );
    }
}

/// EST-05 [P] — six permutations fixes des lignes. Aucun générateur : les
/// permutations sont écrites.
#[test]
fn invariance_par_permutation_des_lignes() {
    let reference = ancrees(&creuse(), None);
    let mut reference_triee = reference.clone();
    reference_triee.sort_by(|a, b| a.0.cmp(&b.0));
    // Pas premiers avec 14 : chaque table est une permutation, sans générateur.
    for (n, decalage) in [1usize, 3, 5, 9, 11, 13].into_iter().enumerate() {
        let base = creuse();
        let liste = acteurs();
        let mut permutes: Vec<(String, String, f64)> = Vec::new();
        for k in 0..liste.len() {
            let acteur = &liste[(k * decalage + n) % liste.len()].0;
            permutes.extend(base.iter().filter(|(_, a, _)| a == acteur).cloned());
        }
        assert_eq!(permutes.len(), base.len(), "permutation sans perte");
        let mut obtenues = ancrees(&permutes, None);
        obtenues.sort_by(|a, b| a.0.cmp(&b.0));
        assert!(
            ecart_maximal(&reference_triee, &obtenues) <= T2,
            "EST-05 : permutation {decalage} des lignes déplace les positions ancrées"
        );
    }
}

/// EST-06 [P] — six permutations fixes des colonnes.
#[test]
fn invariance_par_permutation_des_colonnes() {
    let reference = ancrees(&creuse(), None);
    // Seuls 1 et 5 sont premiers avec 6 : six permutations par trois décalages.
    for (decalage, depart) in [(1usize, 0usize), (1, 1), (1, 2), (5, 0), (5, 1), (5, 2)] {
        let base = creuse();
        let mut permutes: Vec<(String, String, f64)> = Vec::new();
        for k in 0..Y.len() {
            let nom = scrutin((k * decalage + depart) % Y.len());
            permutes.extend(base.iter().filter(|(s, _, _)| *s == nom).cloned());
        }
        assert_eq!(permutes.len(), base.len(), "permutation sans perte");
        let obtenues = ancrees(&permutes, None);
        assert!(
            ecart_maximal(&reference, &obtenues) <= T2,
            "EST-06 : permutation ({decalage}, {depart}) des colonnes déplace les positions ancrées"
        );
    }
}

/// EST-07 [P] — quatre initialisations, dont deux qui produisent des axes
/// exactement opposés. Sans ancrage testé, « gauche » change de côté d'une
/// exécution à l'autre, et l'ADR 0000 §6 en fait une majeure.
#[test]
fn invariance_a_linitialisation_signe_compris() {
    let triplets = creuse();
    let defaut = ajuster(vue(&triplets), None).expect("ajustement");
    let inverse: Vec<f64> = defaut.initialisation.iter().map(|x| -x).collect();
    let par_indice: Vec<f64> = (0..defaut.acteurs.len()).map(|i| i as f64 + 1.0).collect();
    let par_indice_inverse: Vec<f64> = par_indice.iter().map(|x| -x).collect();

    let reference = ancrees(&triplets, None);
    let mut opposes = 0;
    for depart in [&inverse, &par_indice, &par_indice_inverse] {
        let brut = ajuster(vue(&triplets), Some(depart)).expect("ajustement");
        let produit: f64 = brut
            .positions
            .iter()
            .zip(&defaut.positions)
            .map(|(a, b)| a * b)
            .sum();
        if produit < 0.0 {
            opposes += 1;
        }
        let obtenues = ancrees(&triplets, Some(depart));
        assert!(
            ecart_maximal(&reference, &obtenues) <= T2,
            "EST-07 : une initialisation déplace les positions ancrées"
        );
    }
    assert!(
        opposes >= 1,
        "EST-07 : le jeu d'initialisations doit contenir au moins un axe inversé, \
         sans quoi il ne teste que la moitié du problème"
    );
}

/// EST-08 — `m(ancre gauche) = −1` et `m(ancre droite) = +1`, exactement.
#[test]
fn ancrage_exact() {
    let obtenues = ancrees(&creuse(), None);
    for (groupe, attendue) in [(LFI, -1.0), (RN, 1.0)] {
        let valeurs: Vec<f64> = acteurs()
            .iter()
            .filter(|(_, g, _)| *g == groupe)
            .map(|(a, _, _)| obtenues.iter().find(|(n, _)| n == a).unwrap().1)
            .collect();
        let m = mediane(&valeurs).expect("groupe non vide");
        assert!(
            (m - attendue).abs() <= T2,
            "EST-08 : médiane ancrée de {groupe} = {m}, attendue {attendue}"
        );
    }
}

/// EST-09 [P] — réappliquer l'ancrage à des positions déjà ancrées ne change
/// rien. Un double ancrage accidentel ne se voit sur aucune sortie.
#[test]
fn ancrage_idempotent() {
    let une_fois: Vec<f64> = ancrees(&creuse(), None)
        .into_iter()
        .map(|(_, v)| v)
        .collect();
    let noms: Vec<String> = ancrees(&creuse(), None)
        .into_iter()
        .map(|(n, _)| n)
        .collect();
    let du_groupe = |groupe: &str| {
        let valeurs: Vec<f64> = acteurs()
            .iter()
            .filter(|(_, g, _)| *g == groupe)
            .map(|(a, _, _)| une_fois[noms.iter().position(|n| n == a).unwrap()])
            .collect();
        mediane(&valeurs).expect("groupe non vide")
    };
    let deux_fois = ancrer(&une_fois, du_groupe(LFI), du_groupe(RN)).expect("ancrage");
    for (a, b) in une_fois.iter().zip(&deux_fois) {
        assert!(
            (a - b).abs() <= T2,
            "EST-09 : le second ancrage a déplacé une position"
        );
    }
}

/// EST-10 — sur un effectif pair, la médiane est la **moitié inférieure** des
/// deux valeurs centrales. La moyenne des deux centrales serait un troisième
/// estimateur dans la chaîne.
#[test]
fn mediane_deterministe_sur_effectif_pair() {
    assert_eq!(
        mediane(&[4.0, 1.0, 3.0, 2.0]),
        Some(2.0),
        "EST-10 : moitié inférieure"
    );
    assert_eq!(
        mediane(&[1.0, 2.0, 3.0]),
        Some(2.0),
        "EST-10 : effectif impair"
    );
    assert_eq!(mediane(&[]), None, "EST-10 : aucune médiane sans valeur");
}

/// EST-11 [C] — une ancre absente à la date de calcul arrête le pipeline.
/// Aucune ancre de remplacement n'est choisie.
#[test]
fn ancre_absente_arrete_le_pipeline() {
    let mut sans_droite = registre();
    for groupe in sans_droite["groupes"].as_array_mut().unwrap() {
        if groupe["uid_an"] == RN {
            groupe["ancre_axe"] = Value::Null;
        }
    }
    let erreur = ancres(&sans_droite, "2026-07-21").expect_err("EST-11 : doit refuser");
    assert!(
        erreur.contains("droite"),
        "EST-11 : l'erreur nomme le pôle manquant, obtenu « {erreur} »"
    );
    let erreur = ancres(&registre(), "2024-07-01").expect_err("EST-11 : hors période de validité");
    assert!(
        erreur.contains("gauche") || erreur.contains("droite"),
        "EST-11 : une ancre hors de sa période de validité n'ancre rien, obtenu « {erreur} »"
    );
}

/// EST-12 [C] — l'ancre est lue dans le registre, jamais écrite dans le code.
#[test]
fn ancre_lue_dans_le_registre_pas_dans_le_code() {
    assert_eq!(
        ancres(&registre(), "2026-07-21"),
        Ok((LFI.to_owned(), RN.to_owned())),
        "EST-12 : les deux ancres du registre livré"
    );
    let mut deplacee = registre();
    for groupe in deplacee["groupes"].as_array_mut().unwrap() {
        if groupe["uid_an"] == LFI {
            let ancre = groupe["ancre_axe"].take();
            deplacee_pose(&mut deplacee, ancre);
            break;
        }
    }
    assert_eq!(
        ancres(&deplacee, "2026-07-21"),
        Ok((DEM.to_owned(), RN.to_owned())),
        "EST-12 : déplacer l'ancre dans le registre change la sortie"
    );
    let texte = source("src/estimateur.rs");
    for interdit in [LFI, RN, DEM] {
        assert!(
            !texte.contains(interdit),
            "EST-12 : {interdit} en dur dans le module (definition-of-done.md §16)"
        );
    }
}

fn deplacee_pose(registre: &mut Value, ancre: Value) {
    for groupe in registre["groupes"].as_array_mut().unwrap() {
        if groupe["uid_an"] == DEM {
            groupe["ancre_axe"] = ancre;
            return;
        }
    }
    panic!("groupe d'accueil absent");
}

/// EST-13 — le second axe est calculé, sert aux alarmes, et n'est jamais une
/// position publiée.
#[test]
fn second_axe_calcule_jamais_publie_comme_position() {
    // Matrice de rang 2 : au terme de rang 1 s'ajoute un second motif.
    let mut triplets = complete();
    for (index, (_, _, v)) in triplets.iter_mut().enumerate() {
        *v += if index % 2 == 0 { 0.30 } else { -0.30 };
    }
    let ajustement = ajuster(vue(&triplets), None).expect("ajustement");
    assert_eq!(
        ajustement.second_axe.len(),
        ajustement.acteurs.len(),
        "EST-13 : le second axe existe pour chaque acteur"
    );
    assert!(
        ajustement.norme_axe2 > 0.0 && ajustement.separation_des_axes() > 0.0,
        "EST-13 : le second axe porte une norme, sans quoi l'alarme de séparation ne mesure rien"
    );
    let texte = source("src/agregation.rs");
    assert!(
        !texte.contains("second_axe"),
        "EST-13 : le second axe n'entre pas dans le module qui produit la sortie publiée"
    );
}

/// EST-14 [C] — aucune réduction flottante parallèle sur le chemin de calcul.
/// `par_iter().sum()` a produit 3 représentations binaires sur 40 exécutions.
#[test]
fn aucune_reduction_flottante_parallele() {
    let manifeste = source("Cargo.toml");
    assert!(
        !manifeste.contains("rayon"),
        "EST-14 : `rayon` dans Cargo.toml — ADR 0001 §1.6 l'exclut du chemin déterministe"
    );
    for module in ["src/estimateur.rs", "src/agregation.rs", "src/matrice.rs"] {
        let texte = source(module);
        for interdit in ["par_iter", "par_bridge", "into_par_iter"] {
            assert!(
                !texte.contains(interdit),
                "EST-14 : « {interdit} » dans {module}"
            );
        }
    }
}

/// EST-15 — `s2/s1` hors de [0,10 ; 0,90] fait passer la famille « votes » en
/// non mesuré. Un tirage sur 25 a produit `s2/s1 = 0,002`.
#[test]
fn alarme_separation_des_axes() {
    let effondre = ajuster(vue(&complete()), None).expect("ajustement");
    assert!(
        effondre.separation_des_axes() < SEPARATION_MINIMALE,
        "EST-15 : sur une matrice exactement de rang 1, le second axe s'effondre — \
         obtenu {}",
        effondre.separation_des_axes()
    );
    assert!(
        effondre.alarmes().contains(&MOTIF_SEPARATION),
        "EST-15 : l'alarme de séparation doit se déclencher"
    );
    // Les deux bornes de la bande, éprouvées sur un ajustement construit : la
    // borne haute détecte la quasi-dégénérescence, la borne basse
    // l'effondrement. Aucune matrice réelle ne les visite toutes les deux.
    for (separation, alarme) in [
        (SEPARATION_MINIMALE / 2.0, true),
        (SEPARATION_MINIMALE, false),
        ((SEPARATION_MINIMALE + SEPARATION_MAXIMALE) / 2.0, false),
        (SEPARATION_MAXIMALE, false),
        (SEPARATION_MAXIMALE + 0.05, true),
    ] {
        let mut construit = effondre.clone();
        construit.norme_axe1 = 1.0;
        construit.norme_axe2 = separation;
        construit.scr_constante = 1.0;
        construit.scr_rang1 = 0.0;
        assert_eq!(
            construit.alarmes().contains(&MOTIF_SEPARATION),
            alarme,
            "EST-15 : s2/s1 = {separation} hors de la bande \
             [{SEPARATION_MINIMALE} ; {SEPARATION_MAXIMALE}] ?"
        );
    }
}

/// EST-16 — gain du rang 1 sur le résidu inférieur à 0,40 : idem.
#[test]
fn alarme_pouvoir_explicatif() {
    // Aucune structure de rang 1 : chaque acteur ne se distingue que sur un
    // scrutin, la matrice est de rang plein.
    let mut triplets = Vec::new();
    for (i, (acteur, _, _)) in acteurs().iter().enumerate() {
        for j in 0..Y.len() {
            let v = if i % Y.len() == j { 1.0 } else { -1.0 };
            triplets.push((scrutin(j), acteur.clone(), v));
        }
    }
    let ajustement = ajuster(vue(&triplets), None).expect("ajustement");
    assert!(
        ajustement.gain_rang1() < GAIN_MINIMAL,
        "EST-16 : gain obtenu {}, attendu sous {GAIN_MINIMAL}",
        ajustement.gain_rang1()
    );
    assert!(
        ajustement.alarmes().contains(&MOTIF_POUVOIR_EXPLICATIF),
        "EST-16 : l'alarme de pouvoir explicatif doit se déclencher"
    );
}
