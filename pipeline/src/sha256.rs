//! SHA-256, FIPS 180-4, en bibliothèque standard seule.
//!
//! **Justification de dépendance** (definition-of-done.md §21) : la clé de
//! déduplication du contrat (`contrats.md` §3) et les empreintes des artefacts
//! exigent SHA-256 ; la bibliothèque standard ne le fournit pas, et la seule
//! alternative serait une dépendance nouvelle pour soixante lignes d'un
//! algorithme figé depuis 2001, sans surface d'évolution. Une dépendance de
//! moins est une empreinte de moins à épingler et à vérifier.
//!
//! Aucune ambition de performance : le pipeline empreinte des fichiers, pas des
//! flux. Aucune ambition cryptographique non plus — l'usage est la
//! déduplication et la traçabilité, pas un secret.

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// État d'un calcul en cours. Sert au cas où l'entrée ne tient pas en mémoire —
/// une archive de 26 Mo décompressée en 8 434 fichiers concaténés (§2.8).
pub struct Sha256 {
    etat: [u32; 8],
    tampon: [u8; 64],
    remplis: usize,
    longueur: u64,
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256 {
    pub fn new() -> Self {
        Self {
            etat: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            tampon: [0; 64],
            remplis: 0,
            longueur: 0,
        }
    }

    pub fn absorber(&mut self, octets: &[u8]) {
        self.longueur = self.longueur.wrapping_add(octets.len() as u64);
        let mut reste = octets;
        while !reste.is_empty() {
            let place = 64 - self.remplis;
            let pris = place.min(reste.len());
            self.tampon[self.remplis..self.remplis + pris].copy_from_slice(&reste[..pris]);
            self.remplis += pris;
            reste = &reste[pris..];
            if self.remplis == 64 {
                let bloc = self.tampon;
                self.comprimer(&bloc);
                self.remplis = 0;
            }
        }
    }

    fn comprimer(&mut self, bloc: &[u8; 64]) {
        let mut mots = [0u32; 64];
        for (n, mot) in mots.iter_mut().take(16).enumerate() {
            *mot = u32::from_be_bytes([
                bloc[n * 4],
                bloc[n * 4 + 1],
                bloc[n * 4 + 2],
                bloc[n * 4 + 3],
            ]);
        }
        for n in 16..64 {
            let s0 =
                mots[n - 15].rotate_right(7) ^ mots[n - 15].rotate_right(18) ^ (mots[n - 15] >> 3);
            let s1 =
                mots[n - 2].rotate_right(17) ^ mots[n - 2].rotate_right(19) ^ (mots[n - 2] >> 10);
            mots[n] = mots[n - 16]
                .wrapping_add(s0)
                .wrapping_add(mots[n - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.etat;
        for n in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choix = (e & f) ^ ((!e) & g);
            let t1 = h
                .wrapping_add(s1)
                .wrapping_add(choix)
                .wrapping_add(K[n])
                .wrapping_add(mots[n]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majorite = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(majorite);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (place, valeur) in self.etat.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *place = place.wrapping_add(valeur);
        }
    }

    /// Les 64 hexadécimaux **minuscules** exigés par le contrat.
    pub fn terminer(mut self) -> String {
        let bits = self.longueur.wrapping_mul(8);
        self.absorber(&[0x80]);
        while self.remplis != 56 {
            self.absorber(&[0]);
        }
        self.absorber(&bits.to_be_bytes());
        let mut hexa = String::with_capacity(64);
        for mot in self.etat {
            for octet in mot.to_be_bytes() {
                hexa.push_str(&format!("{octet:02x}"));
            }
        }
        hexa
    }
}

/// SHA-256 d'une suite d'octets.
pub fn empreinte(octets: &[u8]) -> String {
    let mut calcul = Sha256::new();
    calcul.absorber(octets);
    calcul.terminer()
}
