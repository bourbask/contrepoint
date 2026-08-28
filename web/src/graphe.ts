// Disposition du graphe : une fonction pure, du contrat vers des coordonnees.
// Aucun rendu ici, aucun DOM — ce qui rend la geometrie testable hors ligne.
//
// Le vocabulaire est celui de la partition, et il n'est pas decoratif : un
// SYSTEME reunit les voix d'un meme sujet sans les fondre, une VOIX porte sa
// PORTEE et sa propre graduation. Trois graduations distinctes sur un meme
// systeme sont ce qui rend l'addition entre familles impossible a dessiner.
//
// Deux proprietes de geometrie portent cette impossibilite, et les tests les
// tiennent :
//
//   1. Chaque echelle occupe une PLAGE D'ABSCISSES QUI LUI EST PROPRE. Sur une
//      plage commune, deux tetes ont un milieu visible — l'addition redevient
//      dessinable — et un decalage horizontal se lit comme un ecart mesure
//      entre familles. Les origines et les bornes hautes sont donc decalees.
//   2. Une tete qui ne porte pas de valeur tombe dans une GOUTTIERE, hors de
//      toute plage graduee. Une pause posee a l'origine de l'axe enoncerait une
//      position sur l'entite nommee, ce que la donnee refuse explicitement de
//      faire en ne publiant pas la mesure.
//
// Les bornes de graduation sont DECLAREES par le manifeste (contrats.md §4.1),
// jamais derivees des valeurs affichees.

import { scaleLinear } from 'd3-scale'
import type { Instantane, Manifeste, Marqueur } from './contrat.ts'
import { ContratRefuse } from './contrat.ts'

/** Tetes de note. La forme porte l'information au meme titre que la couleur. */
export const FORMES = ['losange', 'cercle', 'carre', 'triangle', 'croix'] as const
export type Forme = (typeof FORMES)[number]

export type EtatVoix = 'mesuree' | 'non_mesuree' | 'sans_graduation'

export type Voix = {
  famille: string
  libelleFamille: string
  rang: number
  forme: Forme
  marqueur: Marqueur
  etat: EtatVoix
  /** Abscisse de la tete de note, dans les unites de `largeur`. Hors de toute
   *  plage graduee des que l'etat n'est pas « mesuree ». */
  x: number
  /** Valeur telle qu'elle s'ecrit, ou le code de nuance. Jamais vide en silence. */
  valeur: string | null
  dispersion: string | null
  etiquette: string
  y: number
  hauteur: number
  /** Portee tracee sous cette voix : celle de son echelle, et rien d'autre.
   *  `null` quand la voix ne porte pas de valeur — aucune portee graduee ne se
   *  dessine sous une pause ni sous un code de nuance. */
  portee: { debut: number; fin: number } | null
  /** Ordonnee de la dispersion. Sur la ligne du libelle si la largeur le permet,
   *  sur une ligne propre sinon — deux textes ne se chevauchent jamais. */
  yDispersion: number | null
}

export type Systeme = { id: string; libelle: string; voix: Voix[]; y: number; hauteur: number }

export type Echelle = {
  id: string
  familles: string[]
  min: number
  max: number
  decimales: number
  /** Plage d'abscisses propre a cette echelle. Deux echelles ne la partagent
   *  jamais : ni la meme origine, ni le meme pole haut. */
  debut: number
  fin: number
  bornes: { x: number; libelle: string }[]
}

export type Disposition = {
  systemes: Systeme[]
  echelles: Echelle[]
  largeur: number
  hauteur: number
  /** Sous ce seuil, deux textes ne tiennent pas sur une meme ligne. */
  etroit: boolean
  /** Abscisse de la colonne de texte : titres, libelles de famille, motifs. */
  marge: number
  /** Bord droit de la zone de dessin. */
  bord: number
  /** Enveloppe des plages graduees. Aucune voix n'y est placee sans valeur, et
   *  aucune echelle ne l'occupe entierement : c'est une enveloppe, pas un axe. */
  portee: { debut: number; fin: number }
  /** Abscisse des tetes sans valeur, hors de `portee`. */
  gouttiere: number
}

const MARGE = 26
/** Largeur reservee, a gauche des graduations, aux tetes qui ne portent pas de
 *  valeur. Aucune plage graduee n'y entre. */
const GOUTTIERE = 40
/** Decalage entre deux graduations. Sans lui, deux echelles se superposeraient
 *  sur le meme segment de pixels et leur milieu deviendrait lisible. */
const PAS = 28
const SEUIL_ETROIT = 460
const H_TITRE = 30
const H_VOIX = 40
const H_MOTIF = 16
const H_PIED = 16

/**
 * Ecriture francaise des nombres : virgule decimale, signe moins U+2212.
 * Le signe `+` n'apparait que sur une echelle qui porte des valeurs negatives,
 * ou il distingue les deux cotes ; ailleurs il serait du bruit.
 */
export function formaterValeur(v: number, decimales: number, signe = false): string {
  const texte = Math.abs(v).toFixed(decimales).replace('.', ',')
  return (v < 0 ? '−' : signe && v > 0 ? '+' : '') + texte
}

/** Un pixel n'a pas quinze decimales : arrondi stable d'un rendu a l'autre. */
const px = (v: number): number => Math.round(v * 100) / 100

/**
 * Graduations de l'instantane, une par echelle presente et graduee.
 *
 * Les bornes viennent de `manifeste.familles[]`, ou le pipeline les recopie
 * depuis `echelle.min` / `echelle.max` des lignes de preuve. Elles ne sont pas
 * deduites des valeurs affichees : deduites, la plus petite valeur publiee
 * tomberait au pole de l'echelle — LFI, a 0,82 sur une echelle 0 a 10, se
 * retrouvait au pole haut d'une graduation ou sa valeur est a 8,2 % de la
 * course.
 *
 * Chaque echelle recoit sa PROPRE plage d'abscisses : `PAS` pixels de decalage
 * a chaque rang, pris a gauche sur l'origine et a droite sur le pole haut. Aucun
 * de ces trois points, ni le milieu, n'est partage entre deux echelles.
 */
function echelles(
  manifeste: Manifeste,
  instantane: Instantane,
  debut: number,
  fin: number,
): Echelle[] {
  const bornes = new Map(manifeste.familles.map((f) => [f.echelle, f]))
  const rangs = new Map(manifeste.familles.map((f, i) => [f.id, i]))
  const vues = new Map<string, Set<string>>()
  for (const b of instantane.bandes) {
    for (const m of b.marqueurs) {
      const e = vues.get(m.echelle) ?? new Set<string>()
      e.add(m.famille)
      vues.set(m.echelle, e)
    }
  }
  // Ordre d'affichage : celui des familles du manifeste, jamais celui des ids.
  const graduees = [...vues]
    .filter(([id]) => {
      const f = bornes.get(id)
      return f !== undefined && f.min !== null && f.max !== null && f.decimales !== null
    })
    .sort(
      (a, b) =>
        Math.min(...[...a[1]].map((f) => rangs.get(f) ?? 99)) -
        Math.min(...[...b[1]].map((f) => rangs.get(f) ?? 99)),
    )
  const dernier = graduees.length - 1
  return graduees.map(([id, familles], i) => {
    const f = bornes.get(id) as { min: number; max: number; decimales: number }
    const { min, max, decimales } = f
    if (!(max > min)) {
      throw new ContratRefuse(
        `Échelle « ${id} » déclarée de ${min} à ${max} : une graduation exige deux bornes distinctes.`,
      )
    }
    const d = px(debut + i * PAS)
    const g = px(fin - (dernier - i) * PAS)
    return {
      id,
      familles: [...familles].sort(),
      min,
      max,
      decimales,
      debut: d,
      fin: g,
      bornes: [
        { x: d, libelle: formaterValeur(min, decimales, min < 0) },
        { x: g, libelle: formaterValeur(max, decimales, min < 0) },
      ],
    }
  })
}

/**
 * Modele de dessin d'un instantane. Ne calcule aucune valeur : place celles
 * que l'artefact porte, chacune sur la graduation de sa propre echelle.
 */
export function disposer(
  manifeste: Manifeste,
  instantane: Instantane,
  largeur: number,
): Disposition {
  if (manifeste.familles.length > FORMES.length) {
    throw new ContratRefuse(
      `${manifeste.familles.length} familles déclarées pour ${FORMES.length} formes de marqueur disponibles.`,
    )
  }
  const rangs = new Map(manifeste.familles.map((f, i) => [f.id, i]))
  // Une famille hors manifeste n'a ni forme, ni couleur, ni libelle : lui en
  // preter par un repli modulaire la ferait partager les siens avec une autre
  // famille, ce qui est exactement ce que EXP-08 interdit. Le contrat est refuse.
  for (const bande of instantane.bandes) {
    for (const m of bande.marqueurs) {
      if (!rangs.has(m.famille)) {
        throw new ContratRefuse(
          `Famille « ${m.famille} » absente du manifeste : aucune forme de marqueur ne lui correspond.`,
        )
      }
    }
  }

  const etroit = largeur < SEUIL_ETROIT
  const marge = MARGE
  const bord = Math.max(marge + 40, largeur - MARGE)
  const grilles = echelles(manifeste, instantane, marge + GOUTTIERE, bord)
  const parEchelle = new Map(grilles.map((e) => [e.id, e]))
  // La gouttiere est a gauche de toute graduation : une tete sans valeur ne peut
  // donc pas etre lue sur une echelle, quelle qu'elle soit.
  const gouttiere = marge + 10
  const debut = marge + GOUTTIERE
  const fin = bord

  let y = 0
  const systemes: Systeme[] = []
  for (const bande of instantane.bandes) {
    const debutSysteme = y
    y += H_TITRE
    const voix: Voix[] = []
    const ordonnees = [...bande.marqueurs].sort(
      (a, b) => (rangs.get(a.famille) as number) - (rangs.get(b.famille) as number),
    )
    for (const m of ordonnees) {
      const rang = rangs.get(m.famille) as number
      const grille = parEchelle.get(m.echelle)
      const libelleFamille = manifeste.familles[rang]?.libelle ?? m.famille
      const etat: EtatVoix =
        m.valeur !== null ? 'mesuree' : m.valeur_code !== null ? 'sans_graduation' : 'non_mesuree'
      // Une valeur sans graduation declaree n'est pas une absence : la rendre
      // « non mesuré » ferait passer un defaut d'outil pour un resultat (§5.2).
      if (etat === 'mesuree' && grille === undefined) {
        throw new ContratRefuse(
          `Échelle « ${m.echelle} » sans bornes déclarées au manifeste : la valeur de ${m.famille} n'est pas plaçable.`,
        )
      }

      let x = gouttiere
      let valeur: string | null = null
      let portee: { debut: number; fin: number } | null = null
      if (etat === 'mesuree' && grille) {
        x = px(
          scaleLinear()
            .domain([grille.min, grille.max])
            .range([grille.debut, grille.fin])(m.valeur as number),
        )
        valeur = formaterValeur(m.valeur as number, grille.decimales, grille.min < 0)
        portee = { debut: grille.debut, fin: grille.fin }
      } else if (etat === 'sans_graduation') {
        valeur = m.valeur_code
      }

      const surLigne = m.dispersion !== null && etroit
      const hauteur =
        H_VOIX + (etat === 'non_mesuree' ? H_MOTIF : 0) + (surLigne ? H_MOTIF : 0)
      voix.push({
        famille: m.famille,
        libelleFamille,
        rang,
        forme: FORMES[rang] as Forme,
        marqueur: m,
        etat,
        x,
        valeur,
        dispersion:
          m.dispersion === null
            ? null
            : `effectif ${m.dispersion.effectif} · IQR ${formaterValeur(m.dispersion.iqr, grille?.decimales ?? 4)}`,
        etiquette: etiqueter(bande.libelle, libelleFamille, etat, valeur, m.echelle, instantane.date),
        y,
        hauteur,
        portee,
        yDispersion: m.dispersion === null ? null : surLigne ? y + 46 : y + 13,
      })
      y += hauteur
    }
    y += H_PIED
    systemes.push({
      id: bande.id,
      libelle: bande.libelle,
      voix,
      y: debutSysteme,
      hauteur: y - debutSysteme,
    })
  }

  return {
    systemes,
    echelles: grilles,
    largeur,
    hauteur: y,
    etroit,
    marge,
    bord,
    portee: { debut, fin },
    gouttiere,
  }
}

/** Une phrase, 140 caracteres au plus (ton.md T5). L'absence y est dite. */
function etiqueter(
  bande: string,
  famille: string,
  etat: EtatVoix,
  valeur: string | null,
  echelle: string,
  date: string,
): string {
  const tete = `${bande}, ${famille.toLocaleLowerCase('fr')}`
  const corps =
    etat === 'mesuree'
      ? `${valeur} sur l'échelle ${echelle}`
      : etat === 'sans_graduation'
        ? `code ${valeur}, sans échelle graduée`
        : 'non mesuré'
  const phrase = `${tete} : ${corps}, au ${date}.`
  return phrase.length <= 140 ? phrase : `${tete} : ${corps}.`
}
