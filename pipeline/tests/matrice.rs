//! Suite MAT — du scrutin aux triplets (plan-de-tests.md §6).
//!
//! Hors ligne : les entrées viennent de `docs/brique0/echantillons/`.

mod commun;

use commun::{CINQ_SCRUTINS, entete, lu};
use contrepoint::ingestion::Scrutin;
use contrepoint::matrice::{Entete, Matrice, construire};

fn matrice_des_cinq() -> Matrice {
    construire(entete(), CINQ_SCRUTINS.iter().map(|n| lu(n)).collect())
}

/// MAT-01 [C] — **le test central du projet.** Le nombre de triplets émis égale
/// le nombre de votants des trois blocs exprimés. Ni l'absent, ni le non-votant
/// n'ont de cellule.
#[test]
fn absence_nest_pas_une_position() {
    let scrutin = lu("VTANR5L17V5268.json");
    assert_eq!(
        scrutin.nombre_votants, 35,
        "MAT-01 : 35 votants sur 577 sièges — le scrutin le plus creux des échantillons"
    );
    assert_eq!(
        scrutin.cellules.len(),
        35,
        "MAT-01 : exactement une cellule par position exprimée (11 pour + 24 contre \
         + 0 abstention), et aucune autre"
    );
    assert_eq!(
        scrutin.non_votants, 3,
        "MAT-01 : les trois non-votants existent dans la source"
    );

    let matrice = construire(entete(), vec![scrutin]);
    // Les trois non-votants : aucune cellule.
    for acteur in ["PA721908", "PA795310", "PA720066"] {
        assert_eq!(
            matrice.valeur(acteur, "VTANR5L17V5268"),
            None,
            "MAT-01 : un non-votant n'a pas de cellule ({acteur}) — un 0 le rapprocherait \
             du centre de l'axe"
        );
    }
    // Un député absent de tous les blocs : aucune cellule non plus.
    assert_eq!(
        matrice.valeur("PA793314", "VTANR5L17V5268"),
        None,
        "MAT-01 : un député absent n'a pas de cellule"
    );
    assert_eq!(
        matrice.cellules().count(),
        35,
        "MAT-01 : la matrice ne porte que les 35 cellules observées"
    );
}

/// MAT-02 — `pours` → +1, `contres` → −1, `abstentions` → 0. Une inversion de
/// signe retourne l'axe entier et l'ancrage la masque.
#[test]
fn codage_des_trois_positions() {
    let matrice = construire(entete(), vec![lu("VTANR5L17V156.json")]);
    for (acteur, attendu, bloc) in [
        ("PA795228", 1i8, "pours"),
        ("PA793314", -1, "contres"),
        ("PA793158", 0, "abstentions"),
    ] {
        assert_eq!(
            matrice.valeur(acteur, "VTANR5L17V156"),
            Some(attendu),
            "MAT-02 : un votant du bloc `{bloc}` est codé {attendu}"
        );
    }
}

/// MAT-03 [C] — l'abstention et l'absence ne sont pas le même objet : la
/// première est une cellule de valeur 0, la seconde n'est pas une cellule.
#[test]
fn abstention_et_absence_ne_sont_pas_le_meme_objet() {
    let matrice = construire(entete(), vec![lu("VTANR5L17V156.json")]);
    assert_eq!(
        matrice.valeur("PA793158", "VTANR5L17V156"),
        Some(0),
        "MAT-03 : l'abstention observée est une cellule, de valeur 0"
    );
    assert_eq!(
        matrice.valeur("PA795164", "VTANR5L17V156"),
        None,
        "MAT-03 : l'absence n'est pas une cellule — pas un 0, pas un masque"
    );
    // Aucune structure de masque : ce que la matrice porte, ce sont des cellules.
    assert_eq!(
        matrice.cellules().count(),
        163,
        "MAT-03 : la matrice porte 163 cellules pour 577 sièges, et rien pour les 414 autres"
    );
}

/// MAT-04 — le seul filtre : `min(pour, contre) ≥ 1`. Un scrutin à variance
/// nulle n'apporte rien à l'axe.
#[test]
fn filtre_minorite_vide() {
    let matrice = matrice_des_cinq();
    let ecartes: Vec<_> = matrice
        .ecartes()
        .iter()
        .map(|(uid, motif)| (uid.as_str(), motif.as_str()))
        .collect();
    assert_eq!(
        ecartes,
        vec![
            ("VTANR5L17V1", "minorite_vide"),
            ("VTANR5L17V2767", "minorite_vide"),
        ],
        "MAT-04 : la motion de censure (contre = 0 par construction de l'article 49-2) \
         et le scrutin sans opposant sont écartés, avec leur motif"
    );
    assert!(
        matrice.retenus().iter().any(|s| s.uid == "VTANR5L17V156"),
        "MAT-04 : le scrutin nominal, qui porte 2 contre, est retenu"
    );
}

/// MAT-05 [C] — aucun seuil de participation. `nombreVotants` est publié,
/// jamais une porte : à ≥ 300, RN passe devant DR et LIOT traverse le zéro.
#[test]
fn aucun_seuil_de_participation() {
    let matrice = matrice_des_cinq();
    let creux = matrice
        .retenus()
        .iter()
        .find(|s| s.uid == "VTANR5L17V5268")
        .expect("MAT-05 : le scrutin à 35 votants sur 577 sièges est retenu");
    assert_eq!(
        creux.nombre_votants, 35,
        "MAT-05 : `nombreVotants` est conservé et publié"
    );
    assert!(
        matrice
            .rendre()
            .contains("\tVTANR5L17V5268\t2026-01-29\t35\tSPO\t"),
        "MAT-05 : `nombreVotants` est publié dans la sortie, à côté du scrutin qu'il décrit"
    );

    // Le plus petit corpus concevable — un pour, un contre — passe aussi : le
    // filtre est une définition, pas un seuil.
    let minuscule = Scrutin {
        uid: "VTANR5L17V0".into(),
        legislature: "17".into(),
        date: "2024-10-08".into(),
        code_type_vote: "SPO".into(),
        nombre_votants: 2,
        suffrages_exprimes: 2,
        pour: 1,
        contre: 1,
        abstentions: 0,
        non_votants: 0,
        causes: [0, 0, 0],
        desaccords: 0,
        mises_au_point: 0,
        cellules: vec![
            contrepoint::ingestion::Cellule {
                acteur: "PA000001".into(),
                groupe: "PO845401".into(),
                valeur: 1,
            },
            contrepoint::ingestion::Cellule {
                acteur: "PA000002".into(),
                groupe: "PO845413".into(),
                valeur: -1,
            },
        ],
    };
    assert_eq!(
        construire(entete(), vec![minuscule]).retenus().len(),
        1,
        "MAT-05 : deux votants suffisent — aucune valeur de `nombreVotants` n'écarte un scrutin"
    );
}

/// MAT-06 — le décompte retenus / écartés avec motif est une sortie de la v0.1.
#[test]
fn decompte_retenus_ecartes_expose_avec_motif() {
    let matrice = matrice_des_cinq();
    assert_eq!(
        matrice.retenus().len(),
        3,
        "MAT-06 : trois des cinq échantillons portent une minorité enregistrée"
    );
    assert_eq!(
        matrice.ecartes_par_motif(),
        [("minorite_vide", 2)].into_iter().collect(),
        "MAT-06 : le décompte est ventilé par motif, jamais un total muet"
    );
    let rendu = matrice.rendre();
    assert!(
        rendu.contains("# retenus\t3\n") && rendu.contains("# ecartes\tminorite_vide\t2\n"),
        "MAT-06 : le décompte par motif est écrit dans la sortie — sans cela la case de \
         roadmap est cochée sans exister. Rendu :\n{rendu}"
    );
}

/// MAT-07 — tri canonique par `(uid_scrutin, acteurRef)`, en ordre d'octets.
/// Sans tri explicite, l'itération d'une table de hachage rend la sortie non
/// reproductible d'un processus à l'autre.
#[test]
fn tri_canonique_des_triplets() {
    let reference = matrice_des_cinq().rendre();

    // Cinq ordres d'entrée fixes : les cinq rotations.
    for depart in 0..5 {
        let permute: Vec<_> = (0..5)
            .map(|i| lu(CINQ_SCRUTINS[(depart + i) % 5]))
            .collect();
        assert_eq!(
            construire(entete(), permute).rendre(),
            reference,
            "MAT-07 : la rotation de départ {depart} donne la même sortie à l'octet"
        );
    }

    let triplets: Vec<_> = matrice_des_cinq()
        .cellules()
        .map(|(s, a, _)| (s.to_owned(), a.to_owned()))
        .collect();
    let mut attendu = triplets.clone();
    attendu.sort();
    assert_eq!(
        triplets, attendu,
        "MAT-07 : les triplets sortent triés par (uid_scrutin, acteurRef)"
    );
}

/// MAT-08 — l'en-tête porte les deux empreintes d'entrée et la version du code
/// d'ingestion, et rien d'autre de variable. C'est ce qui rend le cache
/// invalidable **sans horloge**.
#[test]
fn entete_porte_les_empreintes_dentree() {
    let matrice = matrice_des_cinq();
    let rendu = matrice.rendre();
    let entete: Vec<&str> = rendu
        .lines()
        .take_while(|l| l.starts_with("# empreinte") || l.starts_with("# version"))
        .collect();
    assert_eq!(
        entete,
        vec![
            format!("# empreinte-scrutins\t{}", "a".repeat(64)),
            format!("# empreinte-amo30\t{}", "b".repeat(64)),
            "# version-ingestion\t1".to_owned(),
        ],
        "MAT-08 : les deux empreintes d'archive et la version du code d'ingestion, \
         dans cet ordre, et rien d'autre"
    );

    // Une empreinte différente change l'en-tête, et elle seule.
    let autre = construire(
        Entete {
            empreinte_scrutins: "c".repeat(64),
            empreinte_amo30: "b".repeat(64),
        },
        CINQ_SCRUTINS.iter().map(|n| lu(n)).collect(),
    );
    assert_ne!(
        autre.rendre().lines().next(),
        matrice.rendre().lines().next(),
        "MAT-08 : l'en-tête suit l'empreinte d'entrée"
    );
    let corps = |rendu: &str| rendu.lines().collect::<Vec<_>>()[3..].join("\n");
    assert_eq!(
        corps(&autre.rendre()),
        corps(&matrice.rendre()),
        "MAT-08 : et rien d'autre ne bouge — aucune date, aucune horloge dans la sortie"
    );
}

/// MAT-09 [C] — aucune imputation. Le nombre de valeurs disponibles au calcul
/// est exactement le nombre de cellules observées.
#[test]
fn aucune_imputation() {
    let matrice = matrice_des_cinq();
    // 163 + 35 + 110 positions exprimées sur les trois scrutins retenus.
    let observees = matrice.cellules().count();
    assert_eq!(
        observees, 308,
        "MAT-09 : 163 + 35 + 110 cellules observées sur les trois scrutins retenus"
    );

    let mut acteurs: Vec<&str> = matrice.cellules().map(|(_, a, _)| a).collect();
    acteurs.sort_unstable();
    acteurs.dedup();
    let plein = acteurs.len() * matrice.retenus().len();
    assert!(
        observees * 4 < plein * 3,
        "MAT-09 : la matrice est creuse — {observees} cellules pour {plein} cases, \
         soit moins de 75 %, et les cases manquantes ne sont nulle part"
    );

    // Ce que la sortie porte est exactement ce qui a été observé.
    assert_eq!(
        matrice
            .rendre()
            .lines()
            .filter(|l| l.starts_with("C\t"))
            .count(),
        observees,
        "MAT-09 : aucune cellule n'est ajoutée à la sérialisation"
    );

    // Et toute case non observée reste sans valeur, pour tout acteur du corpus.
    let mut non_observees = 0usize;
    for acteur in &acteurs {
        for scrutin in matrice.retenus() {
            if matrice.valeur(acteur, &scrutin.uid).is_none() {
                non_observees += 1;
            }
        }
    }
    assert_eq!(
        non_observees,
        plein - observees,
        "MAT-09 : les {} cases non observées rendent `None` — jamais 0, jamais une moyenne \
         de ligne, jamais un masque",
        plein - observees
    );
}
