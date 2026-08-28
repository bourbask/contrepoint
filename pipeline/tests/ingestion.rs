//! Suite ING — lecture d'un scrutin, ses cohérences, et le rattachement au
//! groupe (plan-de-tests.md §5).
//!
//! Hors ligne : toutes les entrées viennent de `docs/brique0/echantillons/`,
//! ou sont construites dans le test quand l'échantillon ne porte pas le cas.

mod commun;

use commun::{CINQ_SCRUTINS, VOTANTS_PO0, echantillon, index_complet};
use contrepoint::ingestion::{Mandat, Scrutin, index_mandats, lire_scrutin};
use serde_json::{Value, json};

/// Les cinq scrutins verbatim, lus avec l'index de mandats complet.
fn cinq_lus() -> Vec<Scrutin> {
    let mandats = index_complet();
    CINQ_SCRUTINS
        .iter()
        .map(|nom| {
            lire_scrutin(&echantillon(nom), &mandats)
                .unwrap_or_else(|e| panic!("{nom} devrait se lire : {e}"))
        })
        .collect()
}

/// ING-05 — la législature est lue dans `scrutin.legislature`, jamais dérivée
/// d'un libellé de fiche source ni du chemin de l'archive.
#[test]
fn legislature_lue_dans_la_donnee() {
    for scrutin in cinq_lus() {
        assert_eq!(
            scrutin.legislature, "17",
            "ING-05 : la législature des cinq échantillons est celle du champ, « 17 » ({})",
            scrutin.uid
        );
    }

    // Le champ, et rien d'autre : une valeur différente ressort différente.
    let mut altere = echantillon("VTANR5L17V156.json");
    altere["scrutin"]["legislature"] = Value::String("16".into());
    let mandats = index_complet();
    assert_eq!(
        lire_scrutin(&altere, &mandats)
            .expect("ING-05 : une autre législature reste un scrutin lisible")
            .legislature,
        "16",
        "ING-05 : la législature rendue est celle du champ, jamais une constante du pipeline"
    );
}

/// ING-06 — les deux cohérences de `syntheseVote`, en erreur bloquante.
#[test]
fn coherences_de_synthese() {
    let mandats = index_complet();
    let scrutin = lire_scrutin(&echantillon("VTANR5L17V156.json"), &mandats)
        .expect("ING-06 : l'échantillon nominal respecte les deux cohérences");
    assert_eq!(
        scrutin.nombre_votants,
        scrutin.pour + scrutin.contre + scrutin.abstentions,
        "ING-06 : nombreVotants = pour + contre + abstentions"
    );
    assert_eq!(
        scrutin.suffrages_exprimes,
        scrutin.pour + scrutin.contre,
        "ING-06 : suffragesExprimes = pour + contre"
    );

    for champ in ["nombreVotants", "suffragesExprimes"] {
        let mut altere = echantillon("VTANR5L17V156.json");
        altere["scrutin"]["syntheseVote"][champ] = Value::String("999".into());
        assert!(
            lire_scrutin(&altere, &mandats).is_err(),
            "ING-06 : un `{champ}` incohérent avec les décomptes est une erreur bloquante, \
             jamais un scrutin lu de travers"
        );
    }
}

/// ING-07 — la liste nominative d'une position a exactement la longueur du
/// `decompteVoix` correspondant. C'est le garde-fou de l'adaptateur ADA-01.
#[test]
fn longueur_nominative_egale_decompte() {
    // Les cinq se lisent : l'invariant tient sur les quatre positions.
    let lus = cinq_lus();
    assert_eq!(lus.len(), 5, "ING-07 : les cinq échantillons sont lus");

    // Un votant retiré d'un bloc, décompte inchangé : la perte est détectée.
    let mut altere = echantillon("VTANR5L17V156.json");
    let bloc = &mut altere["scrutin"]["ventilationVotes"]["organe"]["groupes"]["groupe"][0]["vote"]
        ["decompteNominatif"]["pours"]["votant"];
    let votants = bloc
        .as_array()
        .expect("ING-07 : le premier bloc de l'échantillon porte un tableau de votants");
    *bloc = Value::Array(votants[1..].to_vec());

    let mandats = index_complet();
    assert!(
        lire_scrutin(&altere, &mandats).is_err(),
        "ING-07 : un votant perdu sans que `decompteVoix` bouge est une erreur bloquante"
    );
}

/// ING-08 — un acteur apparaît au plus une fois par scrutin, toutes positions
/// confondues. Un doublon fabrique deux cellules contradictoires.
#[test]
fn un_acteur_au_plus_une_fois_par_scrutin() {
    for scrutin in cinq_lus() {
        let mut vus: Vec<&str> = scrutin.cellules.iter().map(|c| c.acteur.as_str()).collect();
        let total = vus.len();
        vus.sort_unstable();
        vus.dedup();
        assert_eq!(
            vus.len(),
            total,
            "ING-08 : aucun acteur n'a deux cellules dans {}",
            scrutin.uid
        );
    }

    // Le même acteur dans deux blocs : refusé.
    let mut altere = echantillon("VTANR5L17V156.json");
    let groupes = altere["scrutin"]["ventilationVotes"]["organe"]["groupes"]["groupe"]
        .as_array_mut()
        .expect("ING-08 : les blocs de l'échantillon sont un tableau");
    let premier = groupes[0]["vote"]["decompteNominatif"]["pours"]["votant"][0].clone();
    let cible = &mut groupes[2]["vote"]["decompteNominatif"]["pours"]["votant"];
    cible
        .as_array_mut()
        .expect("ING-08 : le troisième bloc porte un tableau de `pours`")
        .push(premier);
    groupes[2]["vote"]["decompteVoix"]["pour"] = Value::String("41".into());
    altere["scrutin"]["syntheseVote"]["decompte"]["pour"] = Value::String("158".into());
    altere["scrutin"]["syntheseVote"]["nombreVotants"] = Value::String("164".into());
    altere["scrutin"]["syntheseVote"]["suffragesExprimes"] = Value::String("160".into());

    let mandats = index_complet();
    assert!(
        lire_scrutin(&altere, &mandats).is_err(),
        "ING-08 : un acteur présent deux fois dans le même scrutin est une erreur bloquante"
    );
}

/// ING-09 [C] — `decompteVoix.nonVotantsVolontaires` vaut exactement
/// `abstentions` dans les 101 208 blocs du corpus. Le lire comme une catégorie
/// double-compte les abstentions et fabrique une position absente de la source.
#[test]
fn non_votants_volontaires_jamais_lu_comme_categorie() {
    // Le champ est renseigné dans l'échantillon — deux blocs le portent à 1 et
    // à 3 — et n'ajoute aucune cellule.
    let scrutin = &cinq_lus()[1];
    assert_eq!(
        scrutin.uid, "VTANR5L17V156",
        "ING-09 : l'échantillon nominal"
    );
    assert_eq!(
        scrutin.cellules.len() as u64,
        scrutin.pour + scrutin.contre + scrutin.abstentions,
        "ING-09 : les cellules sont les seules positions exprimées ; \
         `nonVotantsVolontaires` n'en ajoute aucune"
    );

    // Et le pipeline n'a aucun chemin pour le lire : le nom du champ n'apparaît
    // nulle part dans le code d'ingestion ni de matrice.
    for module in ["ingestion.rs", "matrice.rs"] {
        let source = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join(module),
        )
        .unwrap_or_else(|e| panic!("ING-09 : src/{module} illisible : {e}"));
        assert!(
            !source.contains("nonVotantsVolontaires"),
            "ING-09 : src/{module} ne doit jamais nommer `nonVotantsVolontaires` — \
             le champ est un doublon du compte d'abstentions, pas une catégorie"
        );
    }
}

/// ING-10 — les trois causes de non-participation sont reconnues sans être
/// interprétées : aucune ne change le codage, une quatrième est bloquante.
#[test]
fn trois_causes_de_non_votant_reconnues_sans_etre_interpretees() {
    let mandats = index_complet();
    let scrutin = lire_scrutin(&echantillon("VTANR5L17V5268.json"), &mandats)
        .expect("ING-10 : l'échantillon porte les trois causes `MG`, `PAN`, `PSE`");
    assert_eq!(
        scrutin.non_votants, 3,
        "ING-10 : l'échantillon porte trois non-votants, un par cause"
    );
    assert_eq!(
        scrutin.causes,
        [1, 1, 1],
        "ING-10 : les trois causes sont comptées séparément, dans l'ordre {:?} — \
         comptées, jamais interprétées",
        contrepoint::ingestion::CAUSES_DE_NON_VOTANT
    );
    for acteur in ["PA721908", "PA795310", "PA720066"] {
        assert!(
            !scrutin.cellules.iter().any(|c| c.acteur == acteur),
            "ING-10 : aucune des trois causes ne produit de cellule ({acteur})"
        );
    }

    // Une quatrième valeur : erreur bloquante, jamais une position.
    let mut altere = echantillon("VTANR5L17V5268.json");
    altere["scrutin"]["ventilationVotes"]["organe"]["groupes"]["groupe"][1]["vote"]["decompteNominatif"]
        ["nonVotants"]["votant"]["causePositionVote"] = Value::String("XYZ".into());
    assert!(
        lire_scrutin(&altere, &mandats).is_err(),
        "ING-10 : une cause inconnue est une erreur bloquante — sans quoi une nouvelle \
         catégorie de la source passerait pour une position"
    );
}

/// ING-11 — un vote par délégation est la position du délégant, exprimée en son
/// nom. 15,4 % des cellules exprimées du corpus.
#[test]
fn par_delegation_est_une_position() {
    let mandats = index_complet();
    let scrutin = lire_scrutin(&echantillon("VTANR5L17V5268.json"), &mandats)
        .expect("ING-11 : l'échantillon porte les deux valeurs de `parDelegation`");
    // 7 délégations et 28 votes directs dans l'échantillon : les 35 sont codés.
    assert_eq!(
        scrutin.cellules.len(),
        35,
        "ING-11 : les 7 votes par délégation sont codés comme les 28 autres"
    );

    // Le pipeline n'a aucun chemin pour distinguer les deux.
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/ingestion.rs"),
    )
    .expect("ING-11 : src/ingestion.rs illisible");
    assert!(
        !source.contains("parDelegation"),
        "ING-11 : `parDelegation` ne doit jamais être lu — le distinguer viderait \
         un septième de la matrice"
    );
}

/// ING-12 — la mise au point est ingérée et comptée, jamais appliquée.
#[test]
fn mise_au_point_ingeree_jamais_appliquee() {
    let mandats = index_complet();
    let scrutin = lire_scrutin(&echantillon("VTANR5L17V2767.json"), &mandats)
        .expect("ING-12 : l'échantillon porte trois entrées de mise au point");
    assert_eq!(
        scrutin.mises_au_point, 3,
        "ING-12 : les trois entrées nominatives sont comptées — deux en `pours` \
         (objet nu de deux votants), une en `nonVotants` (tableau à élément nul)"
    );

    // PA795164 a voté « pour » à la machine et déclaré n'avoir pas voté.
    let cellule = scrutin
        .cellules
        .iter()
        .find(|c| c.acteur == "PA795164")
        .expect("ING-12 : la cellule du vote de la machine existe");
    assert_eq!(
        cellule.valeur, 1,
        "ING-12 : la cellule vaut le vote enregistré (+1), jamais la mise au point"
    );

    // PA267780 et PA796078 n'ont pas voté et déclarent « pour » : aucune cellule.
    for acteur in ["PA267780", "PA796078"] {
        assert!(
            !scrutin.cellules.iter().any(|c| c.acteur == acteur),
            "ING-12 : une mise au point ne comble jamais une absence ({acteur})"
        );
    }
}

/// Un scrutin minimal, une seule position exprimée, pour les cas que les cinq
/// échantillons ne portent pas.
fn scrutin_construit(date: &str, organe: &str, acteur: &str) -> Value {
    json!({"scrutin": {
        "uid": "VTANR5L17V0", "legislature": "17", "dateScrutin": date,
        "typeVote": {"codeTypeVote": "SPO"},
        "syntheseVote": {
            "nombreVotants": "2", "suffragesExprimes": "2",
            "decompte": {"pour": "1", "contre": "1", "abstentions": "0", "nonVotants": "0"}
        },
        "miseAuPoint": null,
        "ventilationVotes": {"organe": {"groupes": {"groupe": [
            {"organeRef": organe, "vote": {
                "decompteVoix": {"pour": "1", "contre": "0", "abstentions": "0", "nonVotants": "0"},
                "decompteNominatif": {"pours": {"votant": {"acteurRef": acteur,
                    "mandatRef": "PM000000", "parDelegation": "false"}},
                    "contres": null, "abstentions": null, "nonVotants": null}}},
            {"organeRef": "PO845413", "vote": {
                "decompteVoix": {"pour": "0", "contre": "1", "abstentions": "0", "nonVotants": "0"},
                "decompteNominatif": {"contres": {"votant": {"acteurRef": "PA999999",
                    "mandatRef": "PM000001", "parDelegation": "false"}},
                    "pours": null, "abstentions": null, "nonVotants": null}}}
        ]}}}
    }})
}

/// ING-14 — dédoublonnage des mandats GP : regrouper par `(organeRef,
/// dateDebut)`, retenir la `dateFin` maximale, `null` valant « en cours ».
#[test]
fn dedoublonnage_des_mandats_gp() {
    let index = index_mandats(&echantillon("mandats-gp-l17.json"));

    // PA267285 porte deux fois PO845425 du 2024-07-19 à `null`.
    let wauquiez = &index["PA267285"];
    assert_eq!(
        wauquiez.len(),
        2,
        "ING-14 : les deux mandats PO845425 identiques n'en font qu'un — {wauquiez:?}"
    );

    // PA642868 porte PO845470 du 2025-01-25, avec deux `dateFin` concurrentes.
    let christophe = &index["PA642868"];
    let concurrent = christophe
        .iter()
        .filter(|m| m.groupe == "PO845470" && m.debut == "2025-01-25")
        .collect::<Vec<_>>();
    assert_eq!(
        concurrent.len(),
        1,
        "ING-14 : une seule entrée subsiste pour (PO845470, 2025-01-25)"
    );
    assert_eq!(
        concurrent[0].fin.as_deref(),
        Some("2026-04-06"),
        "ING-14 : la `dateFin` maximale est retenue, 2026-04-06 et non 2026-03-31"
    );

    // La `dateFin` maximale est retenue quel que soit l'ordre de la source :
    // le même cas, les deux mandats présentés dans l'ordre inverse.
    let inverse = index_mandats(&json!({"acteurs": [{"acteurRef": "PA642868", "mandatsGP": [
        {"organeRef": "PO845470", "dateDebut": "2025-01-25", "dateFin": "2026-03-31"},
        {"organeRef": "PO845470", "dateDebut": "2025-01-25", "dateFin": "2026-04-06"}
    ]}]}));
    assert_eq!(
        inverse["PA642868"][0].fin.as_deref(),
        Some("2026-04-06"),
        "ING-14 : la `dateFin` maximale gagne même présentée en second — la règle est un \
         maximum, jamais « la première rencontrée »"
    );

    // Aucun chevauchement résiduel entre deux organeRef différents.
    for (acteur, mandats) in &index {
        for a in mandats {
            for b in mandats {
                if a.groupe == b.groupe || a.debut > b.debut {
                    continue;
                }
                let a_fin = a.fin.as_deref().unwrap_or("9999-12-31");
                assert!(
                    a_fin < b.debut.as_str(),
                    "ING-14 : {acteur} chevauche entre {a:?} et {b:?} — la jointure \
                     choisirait au hasard"
                );
            }
        }
    }
}

/// ING-15 [C] — `votant.mandatRef` pointe toujours un mandat `ASSEMBLEE`, sur
/// les 1 270 476 cellules. Il identifie le siège, jamais l'appartenance.
#[test]
fn mandat_ref_ne_donne_pas_le_groupe() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/ingestion.rs"),
    )
    .expect("ING-15 : src/ingestion.rs illisible");
    assert!(
        !source.contains("mandatRef"),
        "ING-15 : le pipeline ne lit jamais `mandatRef` — le piège est de croire y tenir \
         la jointure vers le groupe"
    );

    // Et la résolution se fait bien par `acteurRef` : un `mandatRef` factice
    // n'empêche ni le rattachement par la ventilation, ni la résolution `PO0`.
    let mandats = index_complet();
    let par_ventilation = lire_scrutin(
        &scrutin_construit("2025-05-01", "PO845425", "PA267285"),
        &mandats,
    )
    .expect("ING-15 : un mandatRef inconnu n'empêche pas la lecture");
    assert_eq!(
        par_ventilation.cellules[0].groupe, "PO845425",
        "ING-15 : le groupe vient de la ventilation, pas de `mandatRef`"
    );
}

/// ING-16 — chaque bloc `PO0` non vide est résolu par le mandat GP de ses
/// votants à `dateScrutin`, unanimement. Un bloc vide est sans objet.
#[test]
fn po0_resolu_par_les_mandats() {
    let scrutin = lire_scrutin(&echantillon("VTANR5L17V6256.json"), &index_complet())
        .expect("ING-16 : le bloc PO0 se résout par les mandats de ses votants");
    let resolus: Vec<_> = scrutin
        .cellules
        .iter()
        .filter(|c| VOTANTS_PO0.contains(&c.acteur.as_str()))
        .collect();
    assert_eq!(
        resolus.len(),
        17,
        "ING-16 : les 17 votants du bloc PO0 gardent leur cellule"
    );
    for cellule in resolus {
        assert_eq!(
            cellule.groupe, "PO845401",
            "ING-16 : la résolution est unanime dans le bloc — jamais `PO0`, jamais « inconnu »"
        );
    }
    assert!(
        !scrutin.cellules.iter().any(|c| c.groupe == "PO0"),
        "ING-16 : aucune cellule ne conserve la référence pendante"
    );

    // Un bloc `PO0` sans votant est sans objet : il ne bloque pas.
    let mut vide = scrutin_construit("2025-05-01", "PO0", "PA267285");
    let bloc = &mut vide["scrutin"]["ventilationVotes"]["organe"]["groupes"]["groupe"][0];
    bloc["vote"]["decompteVoix"]["pour"] = json!("0");
    bloc["vote"]["decompteNominatif"]["pours"] = Value::Null;
    vide["scrutin"]["syntheseVote"]["nombreVotants"] = json!("1");
    vide["scrutin"]["syntheseVote"]["suffragesExprimes"] = json!("1");
    vide["scrutin"]["syntheseVote"]["decompte"]["pour"] = json!("0");
    assert!(
        lire_scrutin(&vide, &index_complet()).is_ok(),
        "ING-16 : un bloc PO0 vide de tout votant est sans objet — 10 des 146 blocs du corpus"
    );
}

/// ING-17 [C] — un bloc `PO0` non résolu est une erreur bloquante. Pas de
/// groupe « inconnu » qui remonterait dans une agrégation publiée.
#[test]
fn po0_non_resolu_est_bloquant() {
    // L'index des échantillons ne porte aucun des 17 votants du bloc PO0.
    let partiel = index_mandats(&echantillon("mandats-gp-l17.json"));
    let erreur = lire_scrutin(&echantillon("VTANR5L17V6256.json"), &partiel)
        .expect_err("ING-17 : un bloc PO0 dont les votants n'ont pas de mandat GP bloque");
    assert!(
        erreur.contains("PO0"),
        "ING-17 : l'erreur nomme la référence pendante — {erreur}"
    );

    // Deux votants du même bloc relevant de deux groupes : ambiguïté bloquante.
    let mut index = index_complet();
    index.insert(
        "PA793290".to_owned(),
        vec![Mandat {
            groupe: "PO845413".into(),
            debut: "2024-07-19".into(),
            fin: None,
        }],
    );
    assert!(
        lire_scrutin(&echantillon("VTANR5L17V6256.json"), &index).is_err(),
        "ING-17 : une résolution non unanime dans un bloc est bloquante, jamais arbitrée"
    );
}

/// ING-18 — quand la ventilation et AMO30 divergent, la ventilation tranche ;
/// le désaccord est compté et exposé, jamais tu.
#[test]
fn desaccord_ventilation_amo30_tranche_pour_la_ventilation() {
    // PA642725 : la ventilation dit DR (PO845425) le 2024-10-22, le mandat
    // AMO30 de non-inscrit (PO840056) court encore ce jour-là.
    let scrutin = lire_scrutin(
        &scrutin_construit("2024-10-22", "PO845425", "PA642725"),
        &index_complet(),
    )
    .expect("ING-18 : un désaccord n'est pas une erreur");
    let cellule = scrutin
        .cellules
        .iter()
        .find(|c| c.acteur == "PA642725")
        .expect("ING-18 : la cellule existe");
    assert_eq!(
        cellule.groupe, "PO845425",
        "ING-18 : le groupe retenu est celui de la ventilation, jamais le non-inscrit \
         d'un mandat à `dateFin` en retard"
    );
    assert_eq!(
        scrutin.desaccords, 1,
        "ING-18 : le désaccord est compté — 2 255 cellules sur le corpus"
    );

    // Sans désaccord, le compteur reste à zéro.
    let accord = lire_scrutin(
        &scrutin_construit("2024-10-23", "PO845425", "PA642725"),
        &index_complet(),
    )
    .expect("ING-18 : lendemain, les deux sources s'accordent");
    assert_eq!(
        accord.desaccords, 0,
        "ING-18 : le compteur ne compte que les désaccords réels"
    );
}

/// ING-19 — le groupe est celui valide le jour du vote, jamais le dernier
/// groupe connu. Nul sur la v0, réel dès la XVIe législature.
#[test]
fn periode_de_validite_respectee() {
    let index = index_mandats(&echantillon("mandats-gp-l17.json"));
    // PA330240 : AD (PO845520) → UDR (PO847173) → UDDPLR (PO872880).
    for (date, attendu) in [
        ("2024-08-01", "PO845520"),
        ("2025-05-01", "PO847173"),
        ("2026-01-01", "PO872880"),
    ] {
        assert_eq!(
            contrepoint::ingestion::groupe_a_la_date(&index, "PA330240", date),
            Some(attendu),
            "ING-19 : le {date}, le groupe valide est {attendu}, jamais le dernier connu"
        );
    }
    assert_eq!(
        contrepoint::ingestion::groupe_a_la_date(&index, "PA330240", "2024-07-01"),
        None,
        "ING-19 : avant le premier mandat, il n'y a pas de groupe — pas de valeur par défaut"
    );
}

/// ING-13 [P] — l'ordre de présentation des fichiers n'entre pas dans la
/// sortie. La machine de mesure a livré `VTANR5L17V5646` avant
/// `VTANR5L17V2136` : sans tri explicite, cet ordre-là serait publié.
#[test]
fn ordre_des_fichiers_sans_effet() {
    use contrepoint::matrice::{Entete, construire};

    let entete = || Entete {
        empreinte_scrutins: "a".repeat(64),
        empreinte_amo30: "b".repeat(64),
    };
    let mandats = index_complet();
    let lire = |nom: &str| {
        lire_scrutin(&echantillon(nom), &mandats).unwrap_or_else(|e| panic!("{nom} : {e}"))
    };

    // Cinq ordres fixes, aucun tirage : les cinq rotations de la liste.
    let reference = construire(entete(), CINQ_SCRUTINS.iter().map(|n| lire(n)).collect()).rendre();
    for depart in 0..5 {
        let ordre: Vec<_> = (0..5)
            .map(|i| lire(CINQ_SCRUTINS[(depart + i) % 5]))
            .collect();
        assert_eq!(
            construire(entete(), ordre).rendre(),
            reference,
            "ING-13 : l'ordre de départ {depart} donne une sortie identique à l'octet"
        );
    }
}
