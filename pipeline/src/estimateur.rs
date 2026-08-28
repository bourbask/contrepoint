//! Composant [3] d'`architecture.md` — moindres carrés alternés de rang 1 sur
//! les **seules cellules observées**, avec une constante par scrutin, puis
//! l'ancrage affine du signe et de l'échelle.
//!
//! Spécification : `docs/brique0/positionnement.md` §4 et §5. L'ACP et
//! l'analyse des correspondances sont écartées — elles exigent une matrice
//! complète, et 77 % des cellules sont manquantes (ADR 0001 §1.2).
//!
//! Trois points durs, tous mesurés :
//!
//! - **Initialisation déterministe, sans générateur.** `x[i]` part de la
//!   moyenne des résidus observés de sa ligne : une valeur de la donnée, pas
//!   d'un indice. Une initialisation par indice rend la permutation des lignes
//!   signifiante.
//! - **Le signe de l'axe est indéterminé.** Sur six amorces, trois renvoient
//!   l'axe inversé pour une corrélation absolue de 1,000000 (ADR 0001 §1.3).
//!   Il est fixé, avec l'échelle, par l'unique transformation affine
//!   d'[`ancrer`], ancrée sur deux médianes de groupe déclarées dans le
//!   registre d'entités.
//! - **Aucune réduction flottante parallèle.** La somme parallèle d'un même
//!   vecteur produit trois représentations binaires distinctes sur quarante
//!   exécutions, aucune égale au séquentiel (ADR 0001 §1.6). Toutes les sommes
//!   d'ici sont séquentielles.

use crate::agregation::VOTES_MINIMAUX;
use serde_json::Value;
use std::collections::BTreeSet;

/// Itérations des moindres carrés alternés (positionnement.md §4).
pub const ITERATIONS: usize = 300;

/// Bande d'acceptation de `s2/s1` (positionnement.md §5). En dessous, le second
/// axe s'est effondré ; au-dessus, les deux axes sont quasi dégénérés et leur
/// ordre n'est plus stable.
pub const SEPARATION_MINIMALE: f64 = 0.10;
pub const SEPARATION_MAXIMALE: f64 = 0.90;

/// Gain minimal du rang 1 sur le résidu après constante par scrutin. Mesuré à
/// 0,608 sur le corpus (verification-2026-08-27.md §3).
pub const GAIN_MINIMAL: f64 = 0.40;

/// Motifs d'alarme. Quand l'un tombe, la famille « votes » passe en non
/// mesuré : aucune valeur n'est publiée, et la raison l'est.
pub const MOTIF_SEPARATION: &str = "separation_des_axes";
pub const MOTIF_POUVOIR_EXPLICATIF: &str = "pouvoir_explicatif";

/// L'ajustement complet. Les positions sont **brutes** : elles n'ont ni signe
/// ni échelle tant qu'[`ancrer`] n'est pas appliqué, et ne sortent jamais du
/// pipeline sous cette forme.
#[derive(Debug, Clone)]
pub struct Ajustement {
    /// Acteurs en ordre d'octets — l'ordre canonique de la matrice.
    pub acteurs: Vec<String>,
    /// Scrutins en ordre d'octets.
    pub scrutins: Vec<String>,
    /// `b[j]`, moyenne des valeurs **observées** du scrutin `j`.
    pub constantes: Vec<f64>,
    /// `x[i]` de départ : la moyenne des résidus observés de la ligne.
    pub initialisation: Vec<f64>,
    /// Premier axe, norme 1.
    pub positions: Vec<f64>,
    /// Second axe, norme 1. Il sert aux alarmes et n'est jamais publié comme
    /// position : il oppose la majorité relative à ses oppositions, pas la
    /// gauche à la droite.
    pub second_axe: Vec<f64>,
    /// `s1` et `s2` : l'échelle est portée par `y`, donc `s = ‖y‖`.
    pub norme_axe1: f64,
    pub norme_axe2: f64,
    /// Sommes des carrés résiduels, dans l'ordre des trois modèles emboîtés.
    pub scr_moyenne: f64,
    pub scr_constante: f64,
    pub scr_rang1: f64,
    pub scr_rang2: f64,
}

impl Ajustement {
    /// Gain du rang 1 sur le résidu après constante par scrutin. 0,608 sur le
    /// corpus réel — l'ADR 0001 a porté 2,1 % à travers cinq documents.
    pub fn gain_rang1(&self) -> f64 {
        if self.scr_constante == 0.0 {
            return 0.0;
        }
        (self.scr_constante - self.scr_rang1) / self.scr_constante
    }

    /// Part de la variance totale prise par la constante par scrutin seule.
    pub fn part_de_la_constante(&self) -> f64 {
        if self.scr_moyenne == 0.0 {
            return 0.0;
        }
        (self.scr_moyenne - self.scr_constante) / self.scr_moyenne
    }

    /// `s2/s1`.
    pub fn separation_des_axes(&self) -> f64 {
        if self.norme_axe1 == 0.0 {
            return 0.0;
        }
        self.norme_axe2 / self.norme_axe1
    }

    /// Les alarmes déclenchées. Non vide : la famille « votes » s'affiche non
    /// mesurée, avec la raison.
    pub fn alarmes(&self) -> Vec<&'static str> {
        let mut motifs = Vec::new();
        let separation = self.separation_des_axes();
        if !(SEPARATION_MINIMALE..=SEPARATION_MAXIMALE).contains(&separation) {
            motifs.push(MOTIF_SEPARATION);
        }
        if self.gain_rang1() < GAIN_MINIMAL {
            motifs.push(MOTIF_POUVOIR_EXPLICATIF);
        }
        motifs
    }

    /// La position brute d'un acteur, ou `None`. Sert à composer les médianes
    /// de groupe ; aucune coordonnée individuelle ne sort du pipeline.
    pub fn position(&self, acteur: &str) -> Option<f64> {
        self.acteurs
            .binary_search_by(|a| a.as_str().cmp(acteur))
            .ok()
            .map(|i| self.positions[i])
    }
}

/// Un axe : `x` de norme 1, l'échelle portée par `‖y‖`, et le résidu qu'il
/// laisse cellule par cellule. `None` quand l'ajustement dégénère — plus aucune
/// direction à extraire.
struct Axe {
    x: Vec<f64>,
    residu: Vec<f64>,
    norme: f64,
    scr: f64,
}

/// Ajuste `v[i,j] ≈ b[j] + x[i]·y[j]` sur les cellules fournies, et rien
/// d'autre : une cellule absente n'entre dans aucune somme, et n'est jamais
/// remplacée par un zéro.
///
/// `depart` force l'initialisation de `x` — les tests d'invariance à
/// l'initialisation en ont besoin, et eux seuls. `None` applique celle de
/// positionnement.md §4.
pub fn ajuster<'a>(
    cellules: impl Iterator<Item = (&'a str, &'a str, f64)>,
    depart: Option<&[f64]>,
) -> Result<Ajustement, String> {
    let brut: Vec<(&str, &str, f64)> = cellules.collect();
    let acteurs: Vec<String> = brut
        .iter()
        .map(|(_, a, _)| (*a).to_owned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let scrutins: Vec<String> = brut
        .iter()
        .map(|(s, _, _)| (*s).to_owned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if acteurs.is_empty() || scrutins.is_empty() {
        return Err("aucune cellule observée".to_owned());
    }

    // Ordre canonique des sommes : par acteur puis par scrutin, en ordre
    // d'octets. Sans lui, l'ordre d'arrivée des triplets entre dans les
    // derniers bits du résultat.
    let mut observees: Vec<(usize, usize, f64)> = brut
        .iter()
        .map(|(s, a, v)| {
            let i = acteurs.binary_search_by(|x| x.as_str().cmp(a)).unwrap();
            let j = scrutins.binary_search_by(|x| x.as_str().cmp(s)).unwrap();
            (i, j, *v)
        })
        .collect();
    observees.sort_by_key(|(i, j, _)| (*i, *j));

    // 1. Constante par scrutin : la moyenne des valeurs OBSERVÉES de la
    //    colonne. C'est le modèle « constante seule », celui dont la somme des
    //    carrés résiduels est publiée, et le départ de l'ajustement conjoint.
    let brutes_initiales: Vec<f64> = observees.iter().map(|(_, _, v)| *v).collect();
    let constantes = moyennes_par_colonne(&observees, &brutes_initiales, scrutins.len());

    // 2. Résidu de ce seul modèle.
    let residus: Vec<f64> = ecarts(&observees, &brutes_initiales, &constantes);

    let total: f64 = observees.iter().map(|(_, _, v)| *v).sum();
    let moyenne = total / observees.len() as f64;
    let scr_moyenne: f64 = observees
        .iter()
        .map(|(_, _, v)| (v - moyenne).powi(2))
        .sum();
    let scr_constante: f64 = residus.iter().map(|r| r * r).sum();

    // 3. Initialisation déterministe, sans générateur pseudo-aléatoire.
    let initialisation = match depart {
        Some(fourni) if fourni.len() == acteurs.len() => fourni.to_vec(),
        Some(_) => return Err("initialisation fournie de longueur incorrecte".to_owned()),
        None => moyennes_par_ligne(&observees, &residus, acteurs.len()),
    };

    // 4. Premier axe, la constante par scrutin réajustée à chaque itération.
    let brutes: Vec<f64> = observees.iter().map(|(_, _, v)| *v).collect();
    let premier = extraire(
        &observees,
        &brutes,
        &initialisation,
        acteurs.len(),
        scrutins.len(),
    )
    .ok_or_else(|| "ajustement dégénéré : aucun axe extractible".to_owned())?;

    // 5. Second axe sur le résidu du premier. Calculé pour les alarmes, jamais
    //    publié comme position.
    let depart2 = moyennes_par_ligne(&observees, &premier.residu, acteurs.len());
    let second = extraire(
        &observees,
        &premier.residu,
        &depart2,
        acteurs.len(),
        scrutins.len(),
    );

    Ok(Ajustement {
        scr_rang1: premier.scr,
        scr_rang2: second.as_ref().map_or(premier.scr, |a| a.scr),
        norme_axe1: premier.norme,
        norme_axe2: second.as_ref().map_or(0.0, |a| a.norme),
        second_axe: second.map_or_else(|| vec![0.0; acteurs.len()], |a| a.x),
        positions: premier.x,
        acteurs,
        scrutins,
        constantes,
        initialisation,
        scr_moyenne,
        scr_constante,
    })
}

/// Moyenne des résidus observés de chaque ligne. Ne dépend d'aucun indice de
/// ligne : c'est la condition pour qu'une permutation des lignes ne change rien.
fn moyennes_par_ligne(
    observees: &[(usize, usize, f64)],
    residus: &[f64],
    lignes: usize,
) -> Vec<f64> {
    let mut sommes = vec![0.0; lignes];
    let mut effectifs = vec![0usize; lignes];
    for ((i, _, _), r) in observees.iter().zip(residus) {
        sommes[*i] += r;
        effectifs[*i] += 1;
    }
    sommes
        .iter()
        .zip(&effectifs)
        .map(|(somme, n)| if *n == 0 { 0.0 } else { somme / *n as f64 })
        .collect()
}

/// Moindres carrés alternés de rang 1 sur les cellules observées, avec la
/// constante par scrutin **réajustée à chaque itération** : `y`, puis `x`
/// renormalisé, puis `b`. L'échelle est portée par `y`. Sommes séquentielles,
/// sans exception.
///
/// Le réajustement conjoint de `b` n'est pas un raffinement : il vaut 1,6 point
/// de gain sur le corpus réel — 383 381,1 de somme des carrés résiduels quand
/// `b` est calculé une seule fois, **368 791,6** quand il est réajusté, pour la
/// même initialisation et les mêmes 300 itérations. Ce sont les 368 791,6 et
/// les 60,8 % de `verification-2026-08-27.md` §3 ; positionnement.md §4 point 2
/// décrit l'autre modèle, celui qui rend 59,2 %. Écart mesuré ici le
/// 2026-08-28, il est de modèle et non de convergence.
fn extraire(
    observees: &[(usize, usize, f64)],
    valeurs: &[f64],
    depart: &[f64],
    lignes: usize,
    colonnes: usize,
) -> Option<Axe> {
    let mut constantes = moyennes_par_colonne(observees, valeurs, colonnes);
    let mut residus: Vec<f64> = ecarts(observees, valeurs, &constantes);
    let mut x = normaliser(depart.to_vec())?;
    let mut y = vec![0.0; colonnes];
    for _ in 0..ITERATIONS {
        y = ajuster_un_cote(
            observees,
            &residus,
            &x,
            colonnes,
            |(_, j, _)| *j,
            |(i, _, _)| *i,
        );
        let suivant = ajuster_un_cote(
            observees,
            &residus,
            &y,
            lignes,
            |(i, _, _)| *i,
            |(_, j, _)| *j,
        );
        x = normaliser(suivant)?;
        let sans_axe: Vec<f64> = observees
            .iter()
            .zip(valeurs)
            .map(|((i, j, _), v)| v - x[*i] * y[*j])
            .collect();
        constantes = moyennes_par_colonne(observees, &sans_axe, colonnes);
        residus = ecarts(observees, valeurs, &constantes);
    }
    y = ajuster_un_cote(
        observees,
        &residus,
        &x,
        colonnes,
        |(_, j, _)| *j,
        |(i, _, _)| *i,
    );

    let residu: Vec<f64> = observees
        .iter()
        .zip(&residus)
        .map(|((i, j, _), r)| r - x[*i] * y[*j])
        .collect();
    let scr = residu.iter().map(|r| r * r).sum();
    let norme = y.iter().map(|v| v * v).sum::<f64>().sqrt();
    Some(Axe {
        x,
        residu,
        norme,
        scr,
    })
}

/// `v[i,j] − b[j]` cellule par cellule.
fn ecarts(observees: &[(usize, usize, f64)], valeurs: &[f64], constantes: &[f64]) -> Vec<f64> {
    observees
        .iter()
        .zip(valeurs)
        .map(|((_, j, _), v)| v - constantes[*j])
        .collect()
}

/// Moyenne des valeurs **observées** de chaque colonne. Calculée sur une
/// colonne complétée de zéros, elle absorberait la composition des présents.
fn moyennes_par_colonne(
    observees: &[(usize, usize, f64)],
    valeurs: &[f64],
    colonnes: usize,
) -> Vec<f64> {
    let mut sommes = vec![0.0; colonnes];
    let mut effectifs = vec![0usize; colonnes];
    for ((_, j, _), v) in observees.iter().zip(valeurs) {
        sommes[*j] += v;
        effectifs[*j] += 1;
    }
    sommes
        .iter()
        .zip(&effectifs)
        .map(|(somme, n)| if *n == 0 { 0.0 } else { somme / *n as f64 })
        .collect()
}

/// Un demi-pas des moindres carrés alternés : pour chaque case `cible`, la
/// pente qui minimise le carré du résidu sachant l'autre côté `connu`.
fn ajuster_un_cote(
    observees: &[(usize, usize, f64)],
    residus: &[f64],
    connu: &[f64],
    taille: usize,
    cible: impl Fn(&(usize, usize, f64)) -> usize,
    autre: impl Fn(&(usize, usize, f64)) -> usize,
) -> Vec<f64> {
    let mut produits = vec![0.0; taille];
    let mut carres = vec![0.0; taille];
    for (cellule, r) in observees.iter().zip(residus) {
        let k = cible(cellule);
        let valeur = connu[autre(cellule)];
        produits[k] += r * valeur;
        carres[k] += valeur * valeur;
    }
    produits
        .iter()
        .zip(&carres)
        .map(|(p, c)| if *c == 0.0 { 0.0 } else { p / c })
        .collect()
}

fn normaliser(mut valeurs: Vec<f64>) -> Option<Vec<f64>> {
    let norme = valeurs.iter().map(|v| v * v).sum::<f64>().sqrt();
    if norme == 0.0 || !norme.is_finite() {
        return None;
    }
    for v in &mut valeurs {
        *v /= norme;
    }
    Some(valeurs)
}

/// Médiane déterministe : sur un effectif pair, la **moitié inférieure** des
/// deux valeurs centrales. Une moyenne des deux centrales serait un troisième
/// estimateur dans la chaîne, et rendrait l'ancrage dépendant de l'arrondi.
pub fn mediane(valeurs: &[f64]) -> Option<f64> {
    if valeurs.is_empty() {
        return None;
    }
    let mut triees = valeurs.to_vec();
    triees.sort_by(f64::total_cmp);
    Some(triees[(triees.len() - 1) / 2])
}

/// L'unique transformation affine qui fixe **le signe et l'échelle** :
/// `x' = (2x − m(gauche) − m(droite)) / (m(droite) − m(gauche))`.
///
/// Par construction la médiane de l'ancre gauche vaut −1 et celle de l'ancre
/// droite +1. Deux initialisations produisant des axes exactement opposés
/// donnent alors les mêmes positions à 1,6·10⁻¹⁵.
pub fn ancrer(
    positions: &[f64],
    mediane_gauche: f64,
    mediane_droite: f64,
) -> Result<Vec<f64>, String> {
    let amplitude = mediane_droite - mediane_gauche;
    if amplitude == 0.0 || !amplitude.is_finite() {
        return Err(format!(
            "les deux ancres ont la même médiane ({mediane_gauche}) : l'échelle n'est pas définie"
        ));
    }
    Ok(positions
        .iter()
        .map(|x| (2.0 * x - mediane_gauche - mediane_droite) / amplitude)
        .collect())
}

/// Ancrage affine sur les **médianes des deux groupes ancres**, l'unique
/// chemin par lequel une position brute devient une position publiable.
///
/// `groupes[n]` et `votes_exprimes[n]` décrivent l'acteur de `positions[n]` :
/// les trois tranches sont parallèles, dans l'ordre canonique de la matrice.
///
/// La médiane d'ancrage porte sur **le même ensemble que la médiane publiée** :
/// les membres au-delà de `VOTES_MINIMAUX`. Sans cette coïncidence, la valeur
/// publiée pour une ancre n'est plus exactement ±1 — mesuré à +1,0002 pour
/// l'ancre droite le 2026-08-28. positionnement.md §5 ne dit pas sur quel
/// ensemble la médiane d'ancrage se calcule : c'est l'`A VERIFIER` de ce cycle.
///
/// Cette fonction vivait dans `examples/verification-corpus.rs`, hors de
/// portée de la CI. Elle est ici pour qu'un test la retienne, et pour que
/// l'agrégation n'ait qu'une seule source de positions ancrées.
///
/// Un groupe d'ancrage sans aucun membre au-delà du seuil est une **erreur
/// bloquante**, comme une ancre absente du registre.
pub fn ancrer_sur_groupes(
    positions: &[f64],
    groupes: &[&str],
    votes_exprimes: &[usize],
    gauche: &str,
    droite: &str,
) -> Result<Vec<f64>, String> {
    if groupes.len() != positions.len() || votes_exprimes.len() != positions.len() {
        return Err(format!(
            "tranches désalignées : {} positions, {} groupes, {} décomptes de votes",
            positions.len(),
            groupes.len(),
            votes_exprimes.len()
        ));
    }
    let mediane_de = |cherche: &str| -> Result<f64, String> {
        let valeurs: Vec<f64> = positions
            .iter()
            .enumerate()
            .filter(|(n, _)| groupes[*n] == cherche && votes_exprimes[*n] >= VOTES_MINIMAUX)
            .map(|(_, x)| *x)
            .collect();
        mediane(&valeurs).ok_or_else(|| {
            format!(
                "groupe d'ancrage {cherche} sans membre au-delà de {VOTES_MINIMAUX} votes \
                 exprimés : l'axe ne peut pas être ancré"
            )
        })
    };
    ancrer(positions, mediane_de(gauche)?, mediane_de(droite)?)
}

/// Les deux ancres déclarées dans le registre d'entités à la date donnée :
/// `(uid du pôle gauche, uid du pôle droite)`.
///
/// Aucun identifiant de groupe n'est écrit ici : le choix d'une ancre est une
/// décision de méthode datée, elle vit dans la donnée. Une ancre manquante à la
/// date de calcul est une **erreur bloquante** ; aucune ancre de remplacement
/// n'est choisie, un tel choix réétalonnerait l'axe en silence et invaliderait
/// toutes les preuves déjà publiées.
pub fn ancres(registre: &Value, date: &str) -> Result<(String, String), String> {
    let mut trouvees: Vec<(String, String)> = Vec::new();
    let groupes = registre["groupes"]
        .as_array()
        .ok_or_else(|| "registre sans tableau `groupes`".to_owned())?;
    for groupe in groupes {
        let ancre = &groupe["ancre_axe"];
        if ancre.is_null() {
            continue;
        }
        let uid = groupe["uid_an"]
            .as_str()
            .ok_or_else(|| "groupe sans `uid_an`".to_owned())?;
        let pole = ancre["pole"]
            .as_str()
            .ok_or_else(|| format!("ancre de {uid} sans pôle"))?;
        let debut = ancre["debut"]
            .as_str()
            .ok_or_else(|| format!("ancre de {uid} sans début de validité"))?;
        let fin = ancre["fin"].as_str();
        if debut > date || fin.is_some_and(|f| f < date) {
            continue;
        }
        if trouvees.iter().any(|(p, _)| p == pole) {
            return Err(format!(
                "deux ancres déclarées pour le pôle {pole} au {date} : le registre \
                 n'en admet qu'une (V24)"
            ));
        }
        trouvees.push((pole.to_owned(), uid.to_owned()));
    }
    let cherche = |pole: &str| {
        trouvees
            .iter()
            .find(|(p, _)| p == pole)
            .map(|(_, uid)| uid.clone())
            .ok_or_else(|| {
                format!(
                    "aucune ancre du pôle {pole} valide au {date} : le calcul s'arrête, \
                     et aucune ancre de remplacement n'est choisie (RG-31)"
                )
            })
    };
    Ok((cherche("gauche")?, cherche("droite")?))
}
