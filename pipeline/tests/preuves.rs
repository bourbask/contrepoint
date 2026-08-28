//! PRE-01 à PRE-15 — le registre de preuves : la ligne, sa clé de
//! déduplication, sa forme canonique et l'ajout seul.
//!
//! Spécification : `docs/brique0/contrats.md` §2, §3, §6 et §7.
//!
//! Les cinq lignes de référence du §2.6 ne sont **pas recopiées** ici : elles
//! sont lues dans le document, qui est la spécification. Une copie aurait
//! divergé, et c'est le document qui publie les cinq `id` que ce code doit
//! reproduire.

use contrepoint::preuves::{
    CLES, CLES_ENTREE, SCHEMA, ajouter, cle, construire, identifiant, rendre, verifier,
};
use serde_json::{Value, json};

fn chemin(relatif: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relatif)
}

/// Les cinq lignes du §2.6, lues dans `contrats.md`. Le §2.6 est la fixture du
/// contrôle de schéma : un `id` faux y serait pire qu'absent.
fn lignes_de_reference() -> Vec<String> {
    let document = std::fs::read_to_string(chemin("../docs/brique0/contrats.md"))
        .expect("contrats.md lisible");
    let lignes: Vec<String> = document
        .lines()
        .filter(|l| l.starts_with(r#"{"schema":"contrepoint/preuve/1""#))
        .map(str::to_owned)
        .collect();
    assert_eq!(lignes.len(), 5, "le §2.6 publie cinq lignes de référence");
    lignes
}

/// Une ligne minimale et valide, sur laquelle chaque test retire ou modifie
/// **un** champ. Construire chaque cas fautif de zéro le ferait échouer pour dix
/// raisons et n'en démontrerait aucune.
fn ligne_nominale() -> Value {
    json!({
        "schema": SCHEMA,
        "famille": "votes",
        "entite": "groupe.an17.rn",
        "valeur": 1.0,
        "valeur_code": null,
        "echelle": {
            "id": "votes_an17_ancre_v1",
            "min": -1.0,
            "max": 1.0,
            "decimales": 2,
            "libelle": "Votes XVIIe législature, unités médianes ancrées"
        },
        "motif_code": null,
        "motif": null,
        "dispersion": {"effectif": 129, "iqr": 0.052, "ecart_type_reechantillonnage": 0.0},
        "observation": {"debut": "2024-10-08", "fin": "2026-07-21"},
        "date_source": "2026-08-27",
        "date_calcul": "2026-08-27T00:00:00Z",
        "methode": {
            "id": "votes_rang1_ancre",
            "version": "1.0.0",
            "parametres": {
                "ancre_droite": "groupe.an17.rn",
                "ancre_gauche": "groupe.an17.lfi-nfp",
                "codage": "pour=+1;contre=-1;abstention=0;non_votant=manquant;absent=manquant",
                "filtre_scrutins": "minorite_non_vide",
                "iterations_als": 300,
                "scrutins_ecartes": 455,
                "scrutins_retenus": 7979
            }
        },
        "epingles": [],
        "entrees": [entree("registre_partis", "1", "1")],
        "logiciel": {"version": "0.1.0", "commit": null}
    })
}

fn entree(source: &str, empreinte: &str, contenu: &str) -> Value {
    json!({
        "source": source,
        "url": "https://example.invalid/x",
        "producteur": "Assemblée nationale",
        "derniere_mise_a_jour": "2026-08-27",
        "citation": null,
        "empreinte_sha256": empreinte.repeat(64),
        "empreinte_contenu_sha256": contenu.repeat(64),
        "recupere_le": "2026-08-27"
    })
}

fn refuse_par(ligne: &Value, invariant: &str) {
    let refus = verifier(ligne);
    assert!(
        refus.iter().any(|r| r.starts_with(invariant)),
        "{invariant} devait refuser ; refus observés : {refus:?}"
    );
}

// -------------------------------------------------- les cinq id du §2.6 -----

#[test]
fn cinq_identifiants_du_contrat_reproduits() {
    // La vérification la plus dure : le §2.6 publie cinq lignes réelles avec
    // leurs `id`, recalculés un par un par la commande du §3. Si ce code ne les
    // reproduit pas, c'est ce code qui a tort.
    for texte in lignes_de_reference() {
        let ligne: Value = serde_json::from_str(&texte).expect("ligne du §2.6 conforme au JSON");
        let declare = ligne["id"].as_str().expect("`id` déclaré").to_owned();
        let recalcule = identifiant(&ligne).expect("`id` recalculable");
        assert_eq!(
            recalcule,
            declare,
            "§2.6 — {} / {} : identifiant recalculé différent de l'identifiant publié\nclé = {:?}",
            ligne["famille"],
            ligne["entite"],
            cle(&ligne)
        );
    }
}

#[test]
fn cle_de_deduplication_conforme_au_paragraphe_3() {
    // La clé littérale de la commande du §3, sur la ligne du RN. `␟` est U+001F,
    // et le dernier champ est la liste des empreintes de **contenu**, triée.
    let rn = lignes_de_reference()
        .into_iter()
        .find(|l| l.contains(r#""entite":"groupe.an17.rn""#))
        .expect("la ligne du RN");
    let ligne: Value = serde_json::from_str(&rn).unwrap();
    let attendue = [
        "votes",
        "groupe.an17.rn",
        "2024-10-08",
        "2026-07-21",
        "votes_rang1_ancre",
        "1.0.0",
        concat!(
            r#"{"ancre_droite":"groupe.an17.rn","ancre_gauche":"groupe.an17.lfi-nfp","#,
            r#""codage":"pour=+1;contre=-1;abstention=0;non_votant=manquant;absent=manquant","#,
            r#""filtre_scrutins":"minorite_non_vide","iterations_als":300,"#,
            r#""scrutins_ecartes":455,"scrutins_retenus":7979}"#
        ),
        concat!(
            "0f49c00a8227d6cb8e658d374bacfec35238fe4e2dd6305f7df6ac4f515c5de6,",
            "b7bdb819be8b6773a8af5d2a939a78120e710e6f3cf6e86e87db0443168aaf2b,",
            "c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c"
        ),
    ]
    .join("\u{1f}");
    assert_eq!(cle(&ligne).unwrap(), attendue);
}

#[test]
fn cle_amputee_dun_champ_change_lidentifiant() {
    // Chaque composant de la clé du §3 est porteur. Une clé amputée d'un champ
    // confond deux mesures distinctes, et la déduplication en avale une.
    let ligne = ligne_nominale();
    let reference = identifiant(&ligne).unwrap();
    type Mutation = (&'static str, Box<dyn Fn(&mut Value)>);
    let mutations: [Mutation; 7] = [
        (
            "famille",
            Box::new(|l: &mut Value| l["famille"] = json!("experts")),
        ),
        (
            "entite",
            Box::new(|l: &mut Value| l["entite"] = json!("groupe.an17.soc")),
        ),
        (
            "observation.debut",
            Box::new(|l: &mut Value| l["observation"]["debut"] = json!("2024-10-09")),
        ),
        (
            "observation.fin",
            Box::new(|l: &mut Value| l["observation"]["fin"] = json!("2026-07-22")),
        ),
        (
            "methode.id",
            Box::new(|l: &mut Value| l["methode"]["id"] = json!("ches_lrgen")),
        ),
        (
            "methode.version",
            Box::new(|l: &mut Value| l["methode"]["version"] = json!("1.0.1")),
        ),
        (
            "methode.parametres",
            Box::new(|l: &mut Value| l["methode"]["parametres"]["iterations_als"] = json!(301)),
        ),
    ];
    for (champ, muter) in mutations {
        let mut mutee = ligne.clone();
        muter(&mut mutee);
        assert_ne!(
            identifiant(&mutee).unwrap(),
            reference,
            "{champ} doit entrer dans la clé du §3"
        );
    }

    // L'empreinte de **contenu** entre dans la clé ; celle de l'archive, non.
    let mut contenu = ligne.clone();
    contenu["entrees"][0]["empreinte_contenu_sha256"] = json!("2".repeat(64));
    assert_ne!(identifiant(&contenu).unwrap(), reference);

    // I22 — non-régression : deux lignes identiques par ailleurs, dont les
    // empreintes d'archive diffèrent et les empreintes de contenu coïncident,
    // ont le **même** id. L'archive a été republiée, la donnée non.
    let mut archive = ligne.clone();
    archive["entrees"][0]["empreinte_sha256"] = json!("3".repeat(64));
    assert_eq!(identifiant(&archive).unwrap(), reference);

    // Hors de la clé : valeur, date_calcul, logiciel, url, producteur,
    // date de mise à jour, citation. `contrat` n'y figure plus parce qu'il
    // n'est plus un champ de la ligne (contrat 0.6.0, PRE-15).
    for (nom, mut hors) in [
        ("valeur", ligne.clone()),
        ("date_calcul", ligne.clone()),
        ("logiciel", ligne.clone()),
        ("url", ligne.clone()),
        ("producteur", ligne.clone()),
        ("derniere_mise_a_jour", ligne.clone()),
    ] {
        match nom {
            "valeur" => hors["valeur"] = json!(0.5),
            "date_calcul" => hors["date_calcul"] = json!("2027-01-01T00:00:00Z"),
            "logiciel" => hors["logiciel"]["version"] = json!("9.9.9"),
            "url" => hors["entrees"][0]["url"] = json!("https://example.invalid/y"),
            "producteur" => hors["entrees"][0]["producteur"] = json!("Autre producteur"),
            _ => hors["entrees"][0]["derniere_mise_a_jour"] = json!("2026-08-28"),
        }
        assert_eq!(
            identifiant(&hors).unwrap(),
            reference,
            "{nom} ne doit pas entrer dans la clé du §3"
        );
    }
}

#[test]
fn ordre_des_entrees_ne_change_pas_la_cle() {
    // Le tri de la clé porte sur les empreintes de contenu et ne dépend pas de
    // l'ordre du tableau (§7).
    let mut ligne = ligne_nominale();
    ligne["entrees"] = json!([
        entree("an_organe", "a", "b"),
        entree("an_scrutins_17", "c", "d")
    ]);
    let une = identifiant(&ligne).unwrap();
    ligne["entrees"] = json!([
        entree("an_scrutins_17", "c", "d"),
        entree("an_organe", "a", "b")
    ]);
    assert_eq!(identifiant(&ligne).unwrap(), une);
}

// ---------------------------------------------------------------- PRE-01 ----

#[test]
fn ligne_sans_date_refusee() {
    // PRE-01 [C] — I2. Une position sans date est indatable après coup :
    // l'information est perdue à l'écriture, pas à la lecture.
    let mut sans_source = ligne_nominale();
    sans_source["date_source"] = json!(null);
    refuse_par(&sans_source, "I2");

    let mut sans_calcul = ligne_nominale();
    sans_calcul.as_object_mut().unwrap().remove("date_calcul");
    refuse_par(&sans_calcul, "I2");

    // `date_source` postérieure à la date de calcul, ou à une récupération.
    let mut apres = ligne_nominale();
    apres["date_source"] = json!("2027-01-01");
    refuse_par(&apres, "I2");
}

// ---------------------------------------------------------------- PRE-02 ----

#[test]
fn ligne_sans_preuve_de_source_refusee() {
    // PRE-02 [C] — I1. Les archives de l'Assemblée sont reconstruites chaque
    // nuit et rétroactivement modifiables : sans empreinte, « rejouable » veut
    // dire « recalculable sur des données qui ont changé ».
    let mut vide = ligne_nominale();
    vide["entrees"] = json!([]);
    refuse_par(&vide, "I1");

    let mut sans_archive = ligne_nominale();
    sans_archive["entrees"][0]
        .as_object_mut()
        .unwrap()
        .remove("empreinte_sha256");
    refuse_par(&sans_archive, "I1");

    let mut sans_contenu = ligne_nominale();
    sans_contenu["entrees"][0]["empreinte_contenu_sha256"] = json!(null);
    refuse_par(&sans_contenu, "I22");

    let mut courte = ligne_nominale();
    courte["entrees"][0]["empreinte_sha256"] = json!("abc");
    refuse_par(&courte, "I1");
}

// ---------------------------------------------------------------- PRE-03 ----

#[test]
fn ligne_porte_la_version_qui_la_produite() {
    // PRE-03 — la version que la ligne porte est celle de la **méthode** et
    // celle du **logiciel**, pas celle du contrat : depuis le contrat 0.6.0 la
    // version du contrat décrit le format et vit dans le manifeste et dans
    // l'instantané (PRE-15). Sans `methode.version`, la politique de version
    // est décorative et une preuve publiée devient non interprétable — c'est
    // aussi le levier sémantique que I8 surveille.
    let mut mauvaise = ligne_nominale();
    mauvaise["methode"]["version"] = json!("0.3");
    refuse_par(&mauvaise, "I1");

    let mut logiciel = ligne_nominale();
    logiciel["logiciel"]["version"] = json!("0.3");
    refuse_par(&logiciel, "I1");

    let mut sans_logiciel = ligne_nominale();
    sans_logiciel.as_object_mut().unwrap().remove("logiciel");
    refuse_par(&sans_logiciel, "I1");

    let mut sans_methode = ligne_nominale();
    sans_methode["methode"]
        .as_object_mut()
        .unwrap()
        .remove("version");
    refuse_par(&sans_methode, "I1");
}

// ---------------------------------------------------------------- PRE-04 ----

#[test]
fn fonction_fixe_epinglee() {
    // PRE-04 — `epingles` porte les fonctions fixes externes, `{nom, version}`,
    // et `logiciel` la trace du producteur (contrats.md §2.1). Le plan de tests
    // demande en outre la version de langage et la cible de compilation :
    // AUCUN champ du contrat 0.3.0 ne les porte, et en inventer un serait une
    // majeure non arbitrée. C'est un `A VERIFIER` reporté à la direction
    // technique ; ce qui est vérifiable ici l'est.
    let mut sans_epingles = ligne_nominale();
    sans_epingles.as_object_mut().unwrap().remove("epingles");
    refuse_par(&sans_epingles, "I1");

    let mut epingle_muette = ligne_nominale();
    epingle_muette["epingles"] = json!([{"nom": "embedding"}]);
    refuse_par(&epingle_muette, "I1");

    let mut sans_logiciel = ligne_nominale();
    sans_logiciel["logiciel"]
        .as_object_mut()
        .unwrap()
        .remove("version");
    refuse_par(&sans_logiciel, "I1");
}

// ---------------------------------------------------------------- PRE-05 ----

#[test]
fn ajout_idempotent() {
    // PRE-05 [P] — sans idempotence, chaque exécution du cron hebdomadaire
    // double le fichier et le graphe affiche N marqueurs empilés.
    let dossier = temporaire("pre-05");
    let fichier = dossier.join("positions.jsonl");
    let lignes = vec![construire(ligne_nominale()).unwrap()];

    let premier = ajouter(&fichier, &lignes).expect("premier ajout");
    assert_eq!(premier.ajoutees, 1);
    let apres_un = std::fs::read(&fichier).unwrap();

    let second = ajouter(&fichier, &lignes).expect("second ajout");
    assert_eq!(
        second.ajoutees, 0,
        "une ligne déjà présente n'est pas réécrite"
    );
    assert_eq!(
        std::fs::read(&fichier).unwrap(),
        apres_un,
        "le fichier doit être identique à l'octet après un second passage"
    );
}

// ---------------------------------------------------------------- PRE-06 ----

#[test]
fn ajout_seul_jamais_de_reecriture() {
    // PRE-06 [P] — I15. Le fichier antérieur est un préfixe **octet pour
    // octet** du nouveau. L'ajout seul se démontre, il ne s'affirme pas.
    let dossier = temporaire("pre-06");
    let fichier = dossier.join("positions.jsonl");

    // Un registre quelconque déjà présent : ce qui doit être vérifié est la
    // non-modification de lignes arbitraires déjà écrites.
    let mut ancienne = ligne_nominale();
    ancienne["entite"] = json!("groupe.an17.soc");
    ajouter(&fichier, &[construire(ancienne).unwrap()]).unwrap();
    let avant = std::fs::read(&fichier).unwrap();

    // Une valeur qui bouge est une ligne de plus, jamais une ligne modifiée.
    // Sur un groupe qui n'est pas une ancre : l'ancre droite porte +1,0000 par
    // construction, et I9 refuse qu'elle porte autre chose (test `ancrage_exact`).
    let mut nouvelle = ligne_nominale();
    nouvelle["entite"] = json!("groupe.an17.dr");
    nouvelle["valeur"] = json!(0.5000);
    nouvelle["methode"]["version"] = json!("1.1.0");
    let ajout = ajouter(&fichier, &[construire(nouvelle).unwrap()]).unwrap();
    assert_eq!(ajout.ajoutees, 1);

    let apres = std::fs::read(&fichier).unwrap();
    assert!(apres.len() > avant.len());
    assert_eq!(
        &apres[..avant.len()],
        &avant[..],
        "le fichier antérieur doit être un préfixe octet pour octet du nouveau"
    );

    // Une réécriture est refusée bruyamment : deux lignes de même `id` et de
    // valeurs différentes sont une méthode modifiée sans incrément de version.
    let mut sournoise = ligne_nominale();
    sournoise["entite"] = json!("groupe.an17.soc");
    sournoise["valeur"] = json!(0.9);
    let erreur = ajouter(&fichier, &[construire(sournoise).unwrap()])
        .expect_err("I8 doit refuser une valeur différente sous le même id");
    assert!(erreur.starts_with("I8"), "{erreur}");
    assert_eq!(
        std::fs::read(&fichier).unwrap(),
        apres,
        "un refus ne laisse rien d'écrit"
    );
}

// ---------------------------------------------------------------- PRE-07 ----

#[test]
fn rejouabilite_octet_pour_octet() {
    // PRE-07 [P] [T1] — la reconstruction d'une ligne à empreintes de contenu
    // identiques rend la ligne identique à l'octet. La référence figée est le
    // §2.6 du contrat, lue et contrôlée, jamais recopiée d'une sortie.
    for texte in lignes_de_reference() {
        let ligne: Value = serde_json::from_str(&texte).unwrap();
        let reconstruite = rendre(&ligne).expect("ligne rendue");
        // Les octets diffèrent des lignes du document sur un seul point, connu
        // et documenté : le §7 impose `echelle.decimales` décimales, zéros
        // terminaux compris, et le §2.6 publie des lignes passées par un
        // analyseur générique qui les a réduites (`-1.0000` → `-1.0`). Le §7
        // dit lui-même que la forme canonique n'engage que le producteur.
        let relue: Value = serde_json::from_str(&reconstruite).expect("ligne rendue relisible");

        // `entrees` est comparée hors ordre, et son ordre est vérifié à part.
        // Motif du changement de test (docs/tdd.md §4) : la quatrième ligne du
        // §2.6 publie `entrees` dans l'ordre ches_2024 (`1c1ec053…`) puis
        // registre_partis (`186fc819…`), qui n'est PAS l'ordre croissant des
        // empreintes d'archive exigé par le §7 — `186f` précède `1c1e`. Les
        // quatre autres lignes sont triées. Le producteur suit le §7 ; l'`id`
        // n'en dépend pas, la clé du §3 triant les empreintes de contenu
        // elle-même. Défaut relevé dans le document, pas dans le code.
        let sans_entrees = |mut v: Value| {
            let mut entrees = v["entrees"].as_array().cloned().unwrap_or_default();
            entrees.sort_by_key(|e| {
                e["empreinte_sha256"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned()
            });
            v["entrees"] = Value::Array(entrees);
            v
        };
        assert_eq!(
            sans_entrees(relue.clone()),
            sans_entrees(ligne.clone()),
            "le rendu canonique doit conserver la donnée"
        );
        let empreintes: Vec<&str> = relue["entrees"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| e["empreinte_sha256"].as_str().unwrap())
            .collect();
        assert!(
            empreintes.windows(2).all(|p| p[0] < p[1]),
            "§7 — `entrees` est triée par `empreinte_sha256` croissante : {empreintes:?}"
        );
        assert_eq!(
            identifiant(&relue).unwrap(),
            ligne["id"].as_str().unwrap(),
            "l'identifiant doit survivre au rendu"
        );
        assert_eq!(
            rendre(&relue).unwrap(),
            reconstruite,
            "rendre ∘ rendre = rendre"
        );
    }
}

#[test]
fn forme_canonique_des_lignes() {
    // §7 — une ligne par objet, aucun espace après `:` ni `,`, clés dans
    // l'ordre du §2.1, décimales fixées par `echelle.decimales`.
    let rendue = construire(ligne_nominale()).unwrap();
    assert!(!rendue.contains('\n'), "une ligne est une ligne");
    // Aucune espace de confort entre deux jetons de structure. Le contrôle
    // porte sur les jonctions, pas sur `contains(": ")` : un libellé légitime
    // contient « législature, unités ».
    for jonction in [
        r#"{"schema":"contrepoint/preuve/1","id":""#,
        r#""famille":"votes","entite":""#,
        r#""echelle":{"id":""#,
        r#""dispersion":{"effectif":"#,
        r#""observation":{"debut":""#,
        r#""epingles":[],"entrees":[{"source":""#,
    ] {
        assert!(
            rendue.contains(jonction),
            "jonction absente ou espacée : {jonction}"
        );
    }
    assert!(
        rendue.contains(r#""valeur":1.00"#),
        "décimales de l'échelle : {rendue}"
    );
    assert!(
        rendue.contains(r#""iqr":0.05"#),
        "dispersion aux décimales de l'échelle"
    );
    assert!(
        rendue.contains(r#""effectif":129"#),
        "un entier n'a pas de point décimal"
    );
    assert!(
        rendue.contains(r#""iterations_als":300"#),
        "un décompte est un entier"
    );
    assert!(
        !rendue.contains("\\u"),
        "aucune séquence d'échappement pour un imprimable"
    );

    // L'ordre des clés est celui du §2.1, et il est vérifiable sans analyseur.
    let mut precedent = 0;
    for cle in CLES {
        let motif = format!("\"{cle}\":");
        let position = rendue
            .find(&motif)
            .unwrap_or_else(|| panic!("clé {cle} absente"));
        assert!(position >= precedent, "clé {cle} hors de l'ordre du §2.1");
        precedent = position;
    }
    // `entrees` est triée par empreinte d'archive croissante.
    let mut deux = ligne_nominale();
    deux["entrees"] = json!([
        entree("an_organe", "f", "b"),
        entree("an_scrutins_17", "a", "d")
    ]);
    let rendue = construire(deux).unwrap();
    assert!(
        rendue.find(&"a".repeat(64)).unwrap() < rendue.find(&"f".repeat(64)).unwrap(),
        "`entrees` est triée par `empreinte_sha256` croissante"
    );
}

// ---------------------------------------------------------------- PRE-08 ----

#[test]
fn horloge_absente_des_valeurs_calculees() {
    // PRE-08 [C] — deux dates de calcul injectées, toutes les valeurs
    // identiques, seul `date_calcul` diffère. Un `id` qui bouge quand la date
    // bouge est un bug de la clé du §3 ; c'est le test le plus utile du contrat.
    let une = construire(ligne_nominale()).unwrap();
    let mut autre_date = ligne_nominale();
    autre_date["date_calcul"] = json!("2027-01-01T00:00:00Z");
    let autre = construire(autre_date).unwrap();

    assert_ne!(une, autre);
    assert_eq!(
        une.replace("2026-08-27T00:00:00Z", "2027-01-01T00:00:00Z"),
        autre,
        "seul `date_calcul` doit différer entre deux exécutions"
    );
}

// ---------------------------------------------------------------- PRE-09 ----

#[test]
fn trois_familles_jamais_moyennees() {
    // PRE-09 [C] — trois lignes distinctes, chacune avec sa méthode et son
    // échelle nommée. Aucune ligne dont la valeur dérive de deux familles : il
    // n'existe aucun emplacement où l'écrire.
    let familles = [
        (
            "votes",
            "groupe.an17.rn",
            "votes_an17_ancre_v1",
            "votes_rang1_ancre",
        ),
        ("experts", "parti.rn", "ches_lrgen_0_10", "ches_lrgen"),
        (
            "administratif",
            "parti.rn",
            "nuance_leg2024",
            "nuance_constatee",
        ),
    ];
    let mut echelles = std::collections::BTreeSet::new();
    let mut identifiants = std::collections::BTreeSet::new();
    for (famille, entite, echelle, methode) in familles {
        let mut ligne = ligne_nominale();
        ligne["famille"] = json!(famille);
        ligne["entite"] = json!(entite);
        ligne["echelle"]["id"] = json!(echelle);
        ligne["methode"]["id"] = json!(methode);
        assert!(
            echelles.insert(echelle),
            "deux familles ne partagent jamais une échelle"
        );
        identifiants.insert(identifiant(&ligne).unwrap());
    }
    assert_eq!(
        identifiants.len(),
        3,
        "trois familles, trois lignes distinctes"
    );

    // La ligne n'a pas de champ hors de la liste du §2.1 : un champ de position
    // hors d'une ligne n'existe pas, donc rien n'agrège deux familles.
    let mut consolidee = ligne_nominale();
    consolidee["position_consolidee"] = json!(0.5);
    refuse_par(&consolidee, "I1");
}

// ---------------------------------------------------------------- PRE-10 ----

#[test]
fn aucun_nom_de_champ_de_consolidation() {
    // PRE-10 [C] — I12. Le lexique interdit se contourne en renommant ; la
    // liste noire porte sur la **forme** que prendrait la violation.
    for nom in [
        "moyenne",
        "consolide",
        "synthese",
        "score_global",
        "indice",
        "score",
        "global",
        "consensus",
    ] {
        let mut ligne = ligne_nominale();
        ligne["methode"]["parametres"][nom] = json!("x");
        refuse_par(&ligne, "I12");
    }
    // Sur une valeur d'énumération aussi, pas seulement sur une clé.
    let mut valeur = ligne_nominale();
    valeur["methode"]["parametres"]["filtre_scrutins"] = json!("score_global");
    refuse_par(&valeur, "I12");
}

// ---------------------------------------------------------------- PRE-11 ----

#[test]
fn valeur_publiee_trace_vers_une_ligne() {
    // PRE-11 — chaque valeur affichable pointe une ligne existante du registre.
    // Ici, le versant registre : l'`id` déclaré est celui que la ligne calcule
    // (I8), donc un marqueur qui cite un `id` cite une ligne reproductible.
    let rendue = construire(ligne_nominale()).unwrap();
    let relue: Value = serde_json::from_str(&rendue).unwrap();
    assert_eq!(relue["id"].as_str().unwrap(), identifiant(&relue).unwrap());

    let mut menteuse = relue.clone();
    menteuse["id"] = json!("0".repeat(64));
    refuse_par(&menteuse, "I8");
}

// ---------------------------------------------------------------- PRE-12 ----

#[test]
fn date_darret_derivee_du_registre() {
    // PRE-12 — I17. Le bandeau ne vaut que s'il est dérivé : saisi à la main,
    // il reste juste tant que quelqu'un y pense.
    let mut tot = ligne_nominale();
    tot["date_source"] = json!("2026-07-31");
    tot["date_calcul"] = json!("2026-08-01T00:00:00Z");
    let mut tard = ligne_nominale();
    tard["entite"] = json!("groupe.an17.soc");
    tard["date_calcul"] = json!("2026-08-27T00:00:00Z");
    let lignes = [construire(tot).unwrap(), construire(tard).unwrap()];
    assert_eq!(
        contrepoint::preuves::date_arretee(&lignes).unwrap(),
        "2026-08-27T00:00:00Z"
    );
}

// ---------------------------------------------------------------- PRE-13 ----

#[test]
fn codage_consigne_dans_la_ligne() {
    // PRE-13 — le codage `+1/0/−1` et le choix `abstention = 0` sont une
    // décision, pas une évidence. Non consignée, elle devient indiscernable
    // d'un défaut. Et elle est dans la clé du §3 : la changer ré-émet.
    let ligne = ligne_nominale();
    let codage = ligne["methode"]["parametres"]["codage"].as_str().unwrap();
    assert!(codage.contains("abstention=0") && codage.contains("pour=+1"));

    let mut sans = ligne_nominale();
    sans["methode"]["parametres"] = json!({});
    refuse_par(&sans, "I1");

    let mut autre = ligne.clone();
    autre["methode"]["parametres"]["codage"] = json!("pour=+1;contre=-1;abstention=manquant");
    assert_ne!(identifiant(&autre).unwrap(), identifiant(&ligne).unwrap());
}

// -------------------------------------------- invariants du §6 (suite) ------

#[test]
fn invariants_de_coherence_de_la_ligne() {
    // I6 — valeur et code jamais tous deux présents ; tous deux nuls exige un
    // motif codé.
    let mut deux = ligne_nominale();
    deux["valeur_code"] = json!("UG");
    refuse_par(&deux, "I6");

    let mut muette = ligne_nominale();
    muette["valeur"] = json!(null);
    refuse_par(&muette, "I6");

    let mut motif_de_trop = ligne_nominale();
    motif_de_trop["motif_code"] = json!("hors_source");
    motif_de_trop["motif"] = json!("Un motif sur une valeur publiée.");
    refuse_par(&motif_de_trop, "I6");

    // I7 — hors des bornes de graduation : refus bloquant, pas dépassement
    // toléré.
    let mut hors = ligne_nominale();
    hors["valeur"] = json!(1.5);
    refuse_par(&hors, "I7");

    let mut inconnue = ligne_nominale();
    inconnue["echelle"]["id"] = json!("axe_maison");
    refuse_par(&inconnue, "I7");

    // §2.2 — l'objet mesuré dépend de la famille (I4).
    let mut parti_qui_vote = ligne_nominale();
    parti_qui_vote["entite"] = json!("parti.rn");
    refuse_par(&parti_qui_vote, "I4");

    // I10 — la règle de non-publication est appliquée, pas seulement énoncée.
    let mut disperse = ligne_nominale();
    disperse["dispersion"]["iqr"] = json!(0.687);
    refuse_par(&disperse, "I10");

    let mut maigre = ligne_nominale();
    maigre["dispersion"]["effectif"] = json!(9);
    refuse_par(&maigre, "I10");
}

#[test]
fn aucune_coordonnee_individuelle_ni_borne_detendue() {
    // I13 et I19 — aucune coordonnée individuelle, et aucune borne d'étendue :
    // un minimum ou un maximum **est** la coordonnée d'un membre identifiable,
    // et I13 ne l'attrape pas, un nombre n'ayant pas de préfixe `PA`.
    let mut acteur = ligne_nominale();
    acteur["entite"] = json!("groupe.an17.PA793290");
    refuse_par(&acteur, "I13");

    let mut trace = ligne_nominale();
    trace["methode"]["parametres"]["ancre_gauche"] = json!("PA793290");
    refuse_par(&trace, "I13");

    for borne in [
        "minimum",
        "maximum",
        "etendue",
        "rang",
        "valeur_extreme",
        "percentile",
    ] {
        let mut ligne = ligne_nominale();
        ligne["dispersion"][borne] = json!(0.9);
        refuse_par(&ligne, "I19");
    }
}

#[test]
fn aucun_ecart_entre_deux_dates() {
    // I18 — aucun champ ne porte un écart, un ratio ni une flèche entre deux
    // instantanés, deux dates ou deux législatures.
    for nom in [
        "ecart",
        "variation",
        "evolution",
        "delta",
        "tendance",
        "progression",
    ] {
        let mut ligne = ligne_nominale();
        ligne["methode"]["parametres"][nom] = json!("0.15");
        refuse_par(&ligne, "I18");
    }
}

#[test]
fn paternite_empreintes_et_citation() {
    // I20, I21, I22, I23.
    let mut anonyme = ligne_nominale();
    anonyme["entrees"][0]["producteur"] = json!("");
    refuse_par(&anonyme, "I21");

    let mut code = ligne_nominale();
    code["entrees"][0]["producteur"] = json!("registre_partis");
    refuse_par(&code, "I21");

    let mut apres = ligne_nominale();
    apres["entrees"][0]["derniere_mise_a_jour"] = json!("2026-09-01");
    refuse_par(&apres, "I21");

    // I20 — aucune chaîne au-delà de 200 caractères, `citation` exceptée et
    // plafonnée à 400.
    let mut longue = ligne_nominale();
    longue["motif"] = json!("m".repeat(201));
    refuse_par(&longue, "I20");

    let mut citation = ligne_nominale();
    citation["entrees"][0]["citation"] = json!("c".repeat(401));
    refuse_par(&citation, "I20");

    let mut acceptable = ligne_nominale();
    acceptable["entrees"][0]["source"] = json!("ches_2024");
    acceptable["entrees"][0]["citation"] = json!("c".repeat(322));
    assert!(
        !verifier(&acceptable).iter().any(|r| r.starts_with("I20")),
        "une citation de 322 caractères est admise"
    );

    // I23 — une source à citation la porte ; toute autre porte `null`.
    let mut sans_citation = ligne_nominale();
    sans_citation["entrees"][0]["source"] = json!("ches_2024");
    refuse_par(&sans_citation, "I23");

    let mut citation_de_trop = ligne_nominale();
    citation_de_trop["entrees"][0]["citation"] = json!("Une citation qu'aucune source n'exige.");
    refuse_par(&citation_de_trop, "I23");

    // I22 — pour une source d'un seul fichier, les deux empreintes sont égales.
    let mut fichier_unique = ligne_nominale();
    fichier_unique["entrees"][0]["source"] = json!("registre_partis");
    fichier_unique["entrees"][0]["empreinte_sha256"] = json!("a".repeat(64));
    fichier_unique["entrees"][0]["empreinte_contenu_sha256"] = json!("b".repeat(64));
    refuse_par(&fichier_unique, "I22");
}

#[test]
fn ancrage_exact() {
    // I9 — la ligne du groupe `ancre_gauche` porte −1,0000 et celle de
    // `ancre_droite` +1,0000, à 10⁻¹² près avant arrondi.
    let mut derive = ligne_nominale();
    derive["valeur"] = json!(0.9998);
    refuse_par(&derive, "I9");

    let mut gauche = ligne_nominale();
    gauche["entite"] = json!("groupe.an17.lfi-nfp");
    gauche["valeur"] = json!(-1.0);
    assert!(!verifier(&gauche).iter().any(|r| r.starts_with("I9")));

    let mut inversee = ligne_nominale();
    inversee["entite"] = json!("groupe.an17.lfi-nfp");
    inversee["valeur"] = json!(1.0);
    refuse_par(&inversee, "I9");
}

#[test]
fn liste_blanche_transcrit_le_schema_publie() {
    // Le producteur est strict : il refuse d'écrire une clé absente du schéma.
    // Si la liste blanche et le schéma divergent, l'un des deux ment.
    let schema: Value = serde_json::from_str(
        &std::fs::read_to_string(chemin("../schemas/preuve-1.schema.json")).expect("schéma"),
    )
    .unwrap();
    let requises = |noeud: &Value| -> Vec<String> {
        noeud["required"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|c| c.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    };
    assert_eq!(CLES.to_vec(), requises(&schema));
    assert_eq!(
        CLES_ENTREE.to_vec(),
        requises(&schema["properties"]["entrees"]["items"])
    );
}

// ------------------------------------------------------------------ outils --

fn temporaire(nom: &str) -> std::path::PathBuf {
    let dossier = std::env::temp_dir().join(format!("contrepoint-{nom}"));
    let _ = std::fs::remove_dir_all(&dossier);
    std::fs::create_dir_all(&dossier).expect("dossier temporaire");
    dossier
}

#[test]
fn confrontation_au_registre_dentites() {
    // I3 et I4 — ce que la ligne seule ne peut pas dire. Une `source` déclarée
    // dans le registre y porte la **même** URL et la **même** empreinte :
    // divergence = refus, soit la source a bougé, soit un fichier a été édité à
    // la main. Et `entite` existe dans le registre, sa période couvrant
    // `observation.fin`.
    let registre: Value = serde_json::from_str(
        &std::fs::read_to_string(chemin("../data/registre/partis.json")).unwrap(),
    )
    .unwrap();

    let mut juste = ligne_nominale();
    juste["entrees"] = json!([{
        "source": "an_organe",
        "url": "https://data.assemblee-nationale.fr/static/openData/repository/17/amo/tous_acteurs_mandats_organes_xi_legislature/AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip",
        "producteur": "Assemblée nationale",
        "derniere_mise_a_jour": "2026-08-27",
        "citation": null,
        "empreinte_sha256": "bbecd01274d2bc9f46fcaa276b06868862ae7680131da3162e35b5cbef663061",
        "empreinte_contenu_sha256": "0f49c00a8227d6cb8e658d374bacfec35238fe4e2dd6305f7df6ac4f515c5de6",
        "recupere_le": "2026-08-27"
    }]);
    assert!(
        contrepoint::preuves::confronter_registre(&juste, &registre).is_empty(),
        "{:?}",
        contrepoint::preuves::confronter_registre(&juste, &registre)
    );

    let mut url_qui_derive = juste.clone();
    url_qui_derive["entrees"][0]["url"] = json!("https://data.assemblee-nationale.fr/autre");
    assert!(
        contrepoint::preuves::confronter_registre(&url_qui_derive, &registre)
            .iter()
            .any(|r| r.starts_with("I3"))
    );

    let mut empreinte_qui_derive = juste.clone();
    empreinte_qui_derive["entrees"][0]["empreinte_sha256"] = json!("0".repeat(64));
    assert!(
        contrepoint::preuves::confronter_registre(&empreinte_qui_derive, &registre)
            .iter()
            .any(|r| r.starts_with("I3"))
    );

    let mut entite_inconnue = juste.clone();
    entite_inconnue["entite"] = json!("groupe.an17.fantome");
    assert!(
        contrepoint::preuves::confronter_registre(&entite_inconnue, &registre)
            .iter()
            .any(|r| r.starts_with("I4"))
    );

    // La période de l'entité couvre `observation.fin` : le groupe UDR est clos
    // au 2025-09-04, une mesure arrêtée au 2026-07-21 ne lui appartient pas.
    let mut hors_periode = juste.clone();
    hors_periode["entite"] = json!("groupe.an17.udr");
    assert!(
        contrepoint::preuves::confronter_registre(&hors_periode, &registre)
            .iter()
            .any(|r| r.starts_with("I4"))
    );

    // I5 — les deux ancres existent dans le registre et **y sont déclarées**
    // comme ancres. Un groupe valide mais non déclaré ancre est un refus.
    let mut ancre_non_declaree = juste.clone();
    ancre_non_declaree["methode"]["parametres"]["ancre_gauche"] = json!("groupe.an17.soc");
    assert!(
        contrepoint::preuves::confronter_registre(&ancre_non_declaree, &registre)
            .iter()
            .any(|r| r.starts_with("I5"))
    );
}

#[test]
fn non_publication_motivee_est_une_ligne_publiee() {
    // §2.4 et §4.3 règle 4 — une mesure retenue puis non publiée **occupe une
    // bande** : la mesure existe, sa non-publication est le résultat, et il
    // s'affiche. Une entité absente d'une source n'en occupe pas. Le code
    // distingue les deux, et le motif nomme la règle qui a mordu.
    use contrepoint::agregation::{
        DISPERSION_INTERNE, DISPERSION_REECHANTILLONNAGE, EFFECTIF_INSUFFISANT,
    };
    for motif_agregation in [
        EFFECTIF_INSUFFISANT,
        DISPERSION_INTERNE,
        DISPERSION_REECHANTILLONNAGE,
    ] {
        let motif = contrepoint::preuves::motif_de_non_publication(
            motif_agregation,
            25,
            Some(0.687),
            Some(0.0176),
        );
        assert!(motif.chars().count() <= 140, "I20 et V21 : {motif}");
        assert!(!motif.is_empty());

        // Sur un groupe qui n'est pas une ancre : I9 exige d'une ancre la
        // valeur exacte ±1,0000, donc une ancre non publiée est un refus — et
        // c'est juste, l'axe n'existerait plus.
        //
        // §2.4 — `sous_seuil_de_publication` est la seule occurrence où
        // `dispersion` est renseignée sans `valeur` : les chiffres qui
        // justifient la non-publication sont publiés, la valeur non.
        let mut ligne = ligne_nominale();
        ligne["entite"] = json!("groupe.an17.liot");
        ligne["valeur"] = json!(null);
        ligne["dispersion"] =
            json!({"effectif": 25, "iqr": 0.687, "ecart_type_reechantillonnage": 0.0176});
        ligne["motif_code"] = json!("sous_seuil_de_publication");
        ligne["motif"] = json!(motif);
        construire(ligne.clone())
            .unwrap_or_else(|e| panic!("une non-publication motivée est une ligne valide : {e}"));

        // I10 — le motif d'absence n'est jamais comblé par un vide : une
        // non-publication sans ses chiffres est refusée.
        //
        // Mutant qui survivait avant ce test : `dispersion: null` sur les deux
        // lignes non publiées du pipeline, sortie sans qu'aucune porte ne bouge.
        ligne["dispersion"] = json!(null);
        let refus = contrepoint::preuves::verifier(&ligne);
        assert!(
            refus.iter().any(|r| r.starts_with("I10")),
            "une non-publication sans `dispersion` est refusée : {refus:?}"
        );
    }

    // Le motif porte la **mesure**, pas seulement le seuil : un lecteur qui voit
    // « 0,25 » sans la valeur mesurée lit le seuil comme l'IQR du groupe. §2.4 :
    // « IQR 0,687 pour un maximum de 0,25 ».
    let interne =
        contrepoint::preuves::motif_de_non_publication(DISPERSION_INTERNE, 25, Some(0.687), None);
    assert_eq!(
        interne,
        "Dispersion interne au-delà du seuil publié : IQR 0,687 pour un maximum de 0,25."
    );
    let reechantillonnage = contrepoint::preuves::motif_de_non_publication(
        DISPERSION_REECHANTILLONNAGE,
        25,
        Some(0.1),
        Some(0.0621),
    );
    assert!(
        reechantillonnage.contains("écart-type 0,0621 pour un maximum de 0,05"),
        "{reechantillonnage}"
    );

    // Les trois motifs sont distincts : une case non mesurée porte **sa**
    // raison, jamais une raison générique.
    let distincts: std::collections::BTreeSet<String> = [
        EFFECTIF_INSUFFISANT,
        DISPERSION_INTERNE,
        DISPERSION_REECHANTILLONNAGE,
    ]
    .iter()
    .map(|m| contrepoint::preuves::motif_de_non_publication(m, 25, Some(0.687), Some(0.0176)))
    .collect();
    assert_eq!(distincts.len(), 3);
}

#[test]
fn ajout_ordonne_par_famille_entite_methode_id() {
    // §2.7 — une exécution ajoute les lignes dont l'`id` n'est pas déjà présent,
    // **dans l'ordre `(famille, entite, methode.id, id)`**. Sans cet ordre,
    // deux exécutions du même calcul écrivent les mêmes lignes dans deux ordres
    // et le contrôle 1 du §8.2 tombe : l'ordre de l'itération sur un ensemble
    // n'est pas un ordre.
    //
    // Mutant qui survivait avant ce test : le tri retiré d'`ajouter`.
    let dossier = temporaire("pre-05-ordre");
    let fichier = dossier.join("positions.jsonl");
    let mut lignes = Vec::new();
    for entite in ["groupe.an17.soc", "groupe.an17.dr", "groupe.an17.ecos"] {
        let mut ligne = ligne_nominale();
        ligne["entite"] = json!(entite);
        ligne["valeur"] = json!(0.5);
        lignes.push(construire(ligne).unwrap());
    }
    lignes.reverse();
    ajouter(&fichier, &lignes).unwrap();
    let ecrites: Vec<String> = std::fs::read_to_string(&fichier)
        .unwrap()
        .lines()
        .map(|l| {
            serde_json::from_str::<Value>(l).unwrap()["entite"]
                .as_str()
                .unwrap()
                .to_owned()
        })
        .collect();
    assert_eq!(
        ecrites,
        ["groupe.an17.dr", "groupe.an17.ecos", "groupe.an17.soc"],
        "l'ordre d'écriture est (famille, entite, methode.id, id), jamais l'ordre d'arrivée"
    );

    // Et l'ordre ne dépend pas de celui de l'entrée : la même liste permutée
    // donne le même fichier, octet pour octet.
    let temoin = std::fs::read(&fichier).unwrap();
    let autre = temporaire("pre-05-ordre-bis").join("positions.jsonl");
    lignes.rotate_left(1);
    ajouter(&autre, &lignes).unwrap();
    assert_eq!(std::fs::read(&autre).unwrap(), temoin);
}

#[test]
fn sha256_conforme_aux_vecteurs_publies() {
    // La clé du §3 est un SHA-256, et le §2.8 en fait le témoin de toute la
    // chaîne. Une implémentation juste sur les cinq lignes du contrat et fausse
    // ailleurs est une implémentation fausse : les vecteurs sont ceux de
    // FIPS 180-4 et de la RFC 6234, plus un message de plus d'un bloc, qui est
    // le seul cas où la planification du message intervient.
    use contrepoint::sha256::empreinte;
    for (message, attendu) in [
        (
            "",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            "abc",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
        (
            "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        ),
    ] {
        assert_eq!(
            empreinte(message.as_bytes()),
            attendu,
            "message {message:?}"
        );
    }
    // Un million de « a » : le cas de bourrage sur bloc plein.
    assert_eq!(
        empreinte(&vec![b'a'; 1_000_000]),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
}

/// PRE-14 [C] — le motif de non-publication et le champ `dispersion` portent
/// la **même** précision : celle de l'échelle.
///
/// Ce qui casse si le test disparaît : le motif recevait la valeur brute quand
/// `dispersion` portait l'arrondie. L'artefact publié disait « IQR 0,6417 » à
/// côté de `"iqr": 0.64` — deux précisions pour une même mesure, dans la même
/// ligne. C'est la fausse précision que le contrat `0.4.0` a écartée en passant
/// l'échelle des votes de quatre à deux décimales, et elle survivait dans la
/// prose. Trouvée en regardant le rendu, par deux relectures indépendantes.
#[test]
fn motif_et_dispersion_a_la_meme_precision() {
    let decimales = contrepoint::preuves::ECHELLES
        .iter()
        .find(|(_, _, _, _, famille)| *famille == "votes")
        .and_then(|(_, _, _, d, _)| *d)
        .expect("échelle `votes` déclarée");

    // Une valeur dont l'écriture brute et l'écriture arrondie diffèrent : sans
    // cela le test passerait sur n'importe quelle implémentation.
    let brut = 0.641_7_f64;
    let facteur = 10f64.powi(i32::try_from(decimales).expect("décimales raisonnables"));
    let arrondi = (brut * facteur).round() / facteur;
    assert_ne!(
        brut, arrondi,
        "le cas de test doit distinguer les deux écritures"
    );

    let motif = contrepoint::preuves::motif_de_non_publication(
        contrepoint::agregation::DISPERSION_INTERNE,
        20,
        Some(arrondi),
        None,
    );
    let ecrit_brut = format!("{brut}").replace('.', ",");
    assert!(
        !motif.contains(&ecrit_brut),
        "PRE-14 : le motif porte la valeur brute « {ecrit_brut} » — {motif}"
    );
    let ecrit_arrondi = format!("{arrondi}").replace('.', ",");
    assert!(
        motif.contains(&ecrit_arrondi),
        "PRE-14 : le motif doit porter la valeur publiée « {ecrit_arrondi} » — {motif}"
    );
}

// ---------------------------------------------------------------- PRE-15 ----

/// PRE-15 [C] — une ligne de preuve **ne porte pas** `contrat`.
///
/// Le champ y figurait sans entrer dans la clé du §3 : chaque bascule de
/// version produisait 34 lignes de même `id` et de contenu différent, I8
/// arrêtait le pipeline, et il fallait réécrire un registre en ajout seul —
/// donc enfreindre I15. C'est arrivé en `0.4.0` puis en `0.5.0` ; ce n'était pas
/// un accident, c'était structurel. La version du contrat décrit le **format**,
/// pas la **mesure** : elle vit dans le manifeste et dans l'instantané (EXP-12).
///
/// Sans ce test, quelqu'un la remet : le champ était plausible, et rien d'autre
/// que la liste close [`CLES`] ne l'interdit.
#[test]
fn contrat_absent_de_la_ligne_de_preuve() {
    assert!(
        !CLES.contains(&"contrat"),
        "PRE-15 : `contrat` est revenu dans la liste close des clés de la ligne"
    );

    // Le producteur est strict (§5.1) : une clé hors de la liste est refusée,
    // pas ignorée. Sans ce refus, la remise du champ passerait en silence.
    let mut remise = ligne_nominale();
    remise
        .as_object_mut()
        .expect("ligne objet")
        .insert("contrat".to_owned(), json!("0.6.0"));
    refuse_par(&remise, "I1");

    // Et la ligne nominale, elle, passe : le refus vient bien de la clé
    // ajoutée, pas d'une autre faute de la fixture.
    let mut nominale = ligne_nominale();
    nominale["id"] = json!(identifiant(&nominale).expect("`id` calculable"));
    assert_eq!(
        verifier(&nominale),
        Vec::<String>::new(),
        "la ligne nominale doit être valide"
    );

    // Le producteur n'écrit pas non plus la ligne : `construire` passe par
    // `verifier` avant de rendre. `rendre` seul, lui, recopie les clés
    // étrangères en fin d'objet — c'est voulu, le refus est le rôle de
    // `verifier` — donc c'est bien `construire` qu'il faut interroger ici.
    let erreur = construire(remise).expect_err("PRE-15 : le producteur doit refuser d'écrire");
    assert!(
        erreur.contains("contrat"),
        "PRE-15 : le refus doit nommer la clé — {erreur}"
    );

    // Les cinq lignes réelles du §2.6, qui sont la spécification, ne le portent
    // pas non plus.
    for texte in lignes_de_reference() {
        let ligne: Value = serde_json::from_str(&texte).expect("ligne du §2.6 conforme au JSON");
        assert!(
            ligne.get("contrat").is_none(),
            "PRE-15 : le §2.6 publie encore une ligne portant `contrat` — {}",
            ligne["entite"]
        );
    }
}
