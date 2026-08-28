//! Composant [2] d'`architecture.md` — le seul filtre retenu, et les cellules
//! observées.
//!
//! Filtre unique, acté par l'ADR 0003 §2 : un scrutin sans minorité enregistrée
//! n'entre pas, `min(pour, contre) ≥ 1`. **Aucun seuil de participation** — la
//! mesure dit que le seul seuil justifiable est l'absence de seuil.
//!
//! Aucune imputation, aucun masque, aucune densification : la matrice est la
//! liste des cellules observées, et rien d'autre.

use crate::ingestion::Scrutin;
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// Version du code d'ingestion, portée par l'en-tête. Elle change quand la
/// sortie change, jamais autrement.
pub const VERSION_INGESTION: &str = "1";

/// Motif du seul rejet possible.
pub const MINORITE_VIDE: &str = "minorite_vide";

/// Ce qui rend le cache invalidable **sans horloge** : les deux empreintes
/// d'entrée et la version du code, et rien d'autre de variable (§9b).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entete {
    pub empreinte_scrutins: String,
    pub empreinte_amo30: String,
}

/// La matrice creuse : les scrutins retenus avec leurs cellules, et le décompte
/// des écartés avec leur motif.
#[derive(Debug, Clone)]
pub struct Matrice {
    pub entete: Entete,
    retenus: Vec<Scrutin>,
    ecartes: Vec<(String, String)>,
}

/// Applique le filtre et fixe l'ordre canonique : par `uid` de scrutin, puis
/// par `acteurRef`, en ordre d'octets. Sans tri explicite, l'ordre du système
/// de fichiers ou celui d'une table de hachage entre dans la sortie.
pub fn construire(entete: Entete, mut scrutins: Vec<Scrutin>) -> Matrice {
    scrutins.sort_by(|a, b| a.uid.cmp(&b.uid));
    let (retenus, ecartes): (Vec<_>, Vec<_>) = scrutins
        .into_iter()
        .partition(|s| s.pour.min(s.contre) >= 1);
    Matrice {
        entete,
        retenus,
        ecartes: ecartes
            .into_iter()
            .map(|s| (s.uid, MINORITE_VIDE.to_owned()))
            .collect(),
    }
}

impl Matrice {
    /// Les scrutins retenus, en ordre canonique.
    pub fn retenus(&self) -> &[Scrutin] {
        &self.retenus
    }

    /// Les écartés, `(uid, motif)`.
    pub fn ecartes(&self) -> &[(String, String)] {
        &self.ecartes
    }

    /// Décompte des écartés par motif — sortie visible de la v0.1.
    pub fn ecartes_par_motif(&self) -> BTreeMap<&str, usize> {
        let mut par_motif = BTreeMap::new();
        for (_, motif) in &self.ecartes {
            *par_motif.entry(motif.as_str()).or_insert(0) += 1;
        }
        par_motif
    }

    /// Les cellules observées : `(uid_scrutin, acteurRef, valeur)`, en ordre
    /// canonique. C'est toute la matrice — il n'y a rien d'autre à parcourir.
    pub fn cellules(&self) -> impl Iterator<Item = (&str, &str, i8)> {
        self.retenus.iter().flat_map(|scrutin| {
            scrutin
                .cellules
                .iter()
                .map(move |c| (scrutin.uid.as_str(), c.acteur.as_str(), c.valeur))
        })
    }

    /// L'artefact de matrice normalisée (§9b) : l'en-tête, le décompte par
    /// motif, une ligne par scrutin retenu et une par cellule observée. Ordre
    /// canonique, donc identique d'une exécution à l'autre et d'une machine à
    /// l'autre.
    pub fn rendre(&self) -> String {
        let mut sortie = String::new();
        let _ = writeln!(
            sortie,
            "# empreinte-scrutins\t{}\n# empreinte-amo30\t{}\n# version-ingestion\t{}",
            self.entete.empreinte_scrutins, self.entete.empreinte_amo30, VERSION_INGESTION
        );
        let _ = writeln!(sortie, "# retenus\t{}", self.retenus.len());
        for (motif, n) in self.ecartes_par_motif() {
            let _ = writeln!(sortie, "# ecartes\t{motif}\t{n}");
        }
        for scrutin in &self.retenus {
            let _ = writeln!(
                sortie,
                "S\t{}\t{}\t{}\t{}\t{}",
                scrutin.uid,
                scrutin.date,
                scrutin.nombre_votants,
                scrutin.code_type_vote,
                scrutin.mises_au_point
            );
        }
        for (uid, motif) in &self.ecartes {
            let _ = writeln!(sortie, "E\t{uid}\t{motif}");
        }
        for scrutin in &self.retenus {
            for cellule in &scrutin.cellules {
                let _ = writeln!(
                    sortie,
                    "C\t{}\t{}\t{}\t{}",
                    scrutin.uid, cellule.acteur, cellule.valeur, cellule.groupe
                );
            }
        }
        sortie
    }

    /// La valeur d'une case, ou `None` si elle n'a pas été observée. `None`
    /// n'est pas un zéro, et ne le devient nulle part.
    pub fn valeur(&self, acteur: &str, scrutin: &str) -> Option<i8> {
        self.cellules()
            .find(|(s, a, _)| *s == scrutin && *a == acteur)
            .map(|(_, _, v)| v)
    }
}
