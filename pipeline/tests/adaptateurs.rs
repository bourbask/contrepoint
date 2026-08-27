//! Suite ADA — les trois adaptateurs de désérialisation de l'ADR 0001 §1.4.
//!
//! Hors ligne : toutes les entrées viennent de `docs/brique0/echantillons/`,
//! sauf la forme enveloppée de `uid`, absente des échantillons et reprise
//! verbatim de la mesure 10 de l'ADR 0001 §1.

use contrepoint::{nombre, uid, un_ou_plusieurs};
use serde_json::{Value, json};

fn echantillon(nom: &str) -> Value {
    let chemin = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../docs/brique0/echantillons")
        .join(nom);
    let texte = std::fs::read_to_string(&chemin)
        .unwrap_or_else(|e| panic!("échantillon {} illisible : {e}", chemin.display()));
    serde_json::from_str(&texte).unwrap_or_else(|e| {
        panic!(
            "échantillon {} non conforme au JSON : {e}",
            chemin.display()
        )
    })
}

/// ADA-01 — `votant` est un objet nu dans 27,3 % des blocs du corpus.
/// Comptes de l'échantillon : 5 blocs en objet nu, 7 en tableau, 38 votants au
/// total — soit `decompte.pour + contre + abstentions + nonVotants`
/// = 11 + 24 + 0 + 3 de `syntheseVote`.
#[test]
fn votant_tableau_ou_objet_nu() {
    let scrutin = echantillon("VTANR5L17V5268.json");
    let groupes = scrutin["scrutin"]["ventilationVotes"]["organe"]["groupes"]["groupe"]
        .as_array()
        .expect("ADA-01 : les blocs de groupe de l'échantillon sont un tableau");

    let (mut objets_nus, mut tableaux, mut votants) = (0usize, 0usize, 0usize);
    for bloc in groupes {
        let organe = &bloc["organeRef"];
        for position in ["pours", "contres", "abstentions", "nonVotants"] {
            let brut = &bloc["vote"]["decompteNominatif"][position]["votant"];
            let liste = un_ou_plusieurs(brut);
            match brut {
                Value::Object(_) => {
                    objets_nus += 1;
                    assert_eq!(
                        liste.len(),
                        1,
                        "ADA-01 : un votant sérialisé en objet nu rend une liste d'un élément \
                         ({position} du bloc {organe})"
                    );
                }
                Value::Array(elements) => {
                    tableaux += 1;
                    assert_eq!(
                        liste.len(),
                        elements.len(),
                        "ADA-01 : aucun votant n'est perdu quand la source sérialise un tableau \
                         ({position} du bloc {organe})"
                    );
                }
                _ => assert!(
                    liste.is_empty(),
                    "ADA-01 : une position sans votant ne fabrique aucun votant \
                     ({position} du bloc {organe})"
                ),
            }
            votants += liste.len();
        }
    }

    assert_eq!(
        objets_nus, 5,
        "ADA-01 : l'échantillon doit porter 5 blocs où `votant` est un objet nu — \
         sans eux le test ne prouve rien"
    );
    assert_eq!(
        tableaux, 7,
        "ADA-01 : l'échantillon doit porter 7 blocs où `votant` est un tableau"
    );
    assert_eq!(
        votants, 38,
        "ADA-01 : les deux formes réunies rendent les 38 votants du scrutin \
         (11 pour + 24 contre + 0 abstention + 3 non-votants)"
    );
}

/// ADA-02 — `organe.uid` est une chaîne, `acteur.uid` est un objet enveloppé.
#[test]
fn uid_chaine_ou_objet_xsi() {
    let organes = echantillon("organes-groupes-l17.json");
    let organes = organes["organes"]
        .as_array()
        .expect("ADA-02 : les organes de l'échantillon sont un tableau");
    assert_eq!(
        organes.len(),
        14,
        "ADA-02 : l'échantillon porte les 14 groupes de la législature 17"
    );
    for organe in organes {
        let lu = uid(&organe["uid"]);
        assert!(
            lu.is_some_and(|u| u.starts_with("PO")),
            "ADA-02 : un uid déjà sérialisé en chaîne est rendu tel quel, ici {:?}",
            organe["uid"]
        );
    }
    assert_eq!(
        uid(&organes[0]["uid"]),
        Some("PO840056"),
        "ADA-02 : la forme chaîne rend la chaîne elle-même"
    );

    let enveloppe = json!({
        "@xmlns:xsi": "http://www.w3.org/2001/XMLSchema-instance",
        "@xsi:type": "IdActeur_type",
        "#text": "PA304016"
    });
    assert_eq!(
        uid(&enveloppe),
        Some("PA304016"),
        "ADA-02 : la forme enveloppée rend le même identifiant que la forme chaîne"
    );
    assert_eq!(
        uid(&json!({"@xsi:type": "IdActeur_type"})),
        None,
        "ADA-02 : un objet sans clé `#text` n'a pas d'uid, et n'en invente pas"
    );
}

/// ADA-03 — dans `miseAuPoint`, la forme vide est tantôt `null`, tantôt un
/// tableau dont les éléments sont nuls.
#[test]
fn mise_au_point_tableau_de_nuls() {
    let scrutin = echantillon("VTANR5L17V2767.json");
    let mise_au_point = &scrutin["scrutin"]["miseAuPoint"];

    for champ in ["contres", "abstentions", "nonVotantsVolontaires"] {
        assert!(
            un_ou_plusieurs(&mise_au_point[champ]).is_empty(),
            "ADA-03 : `{champ}` est une forme vide (`null` ou tableau de nuls) et ne produit \
             aucune entrée"
        );
    }

    assert_eq!(
        un_ou_plusieurs(&mise_au_point["pours"]).len(),
        1,
        "ADA-03 : `pours` sérialisé en objet nu rend une liste d'un élément"
    );

    let non_votants = un_ou_plusieurs(&mise_au_point["nonVotants"]);
    assert_eq!(
        non_votants.len(),
        1,
        "ADA-03 : les éléments nuls d'un tableau ne sont pas des mises au point"
    );
    assert_eq!(
        non_votants[0]["votant"]["acteurRef"],
        json!("PA795164"),
        "ADA-03 : l'unique entrée conservée est celle qui porte un votant"
    );
}

/// ADA-04 — toutes les valeurs numériques de la source sont des chaînes.
#[test]
fn valeurs_numeriques_serialisees_en_chaine() {
    let scrutin = echantillon("VTANR5L17V156.json");
    let synthese = &scrutin["scrutin"]["syntheseVote"];

    assert_eq!(
        nombre(&synthese["nombreVotants"]),
        Some(163),
        "ADA-04 : « \"163\" » est lu comme le nombre 163"
    );
    assert_eq!(
        nombre(&synthese["decompte"]["pour"]),
        Some(157),
        "ADA-04 : les décomptes sont lus comme des nombres"
    );
    assert_eq!(
        nombre(&scrutin["scrutin"]["titre"]),
        None,
        "ADA-04 : une chaîne non numérique est une absence de nombre, jamais un 0"
    );
    assert_eq!(
        nombre(&synthese["champInexistant"]),
        None,
        "ADA-04 : un champ absent est une absence de nombre, jamais un 0"
    );
    assert_eq!(
        nombre(&json!("-4")),
        None,
        "ADA-04 : un décompte négatif n'existe pas dans la source et n'est pas un nombre lu"
    );
}
