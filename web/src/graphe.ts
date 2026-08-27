// Disposition du graphe : une fonction pure, du contrat vers des coordonnees.
// Aucun rendu ici, aucun DOM — ce qui rend la geometrie testable hors ligne.
//
// Le vocabulaire est celui de la partition, et il n'est pas decoratif : un
// SYSTEME reunit les voix d'un meme sujet sans les fondre, une VOIX porte sa
// PORTEE et sa propre graduation. Trois graduations distinctes sur un meme
// systeme sont ce qui rend l'addition entre familles impossible a dessiner.

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
  /** Abscisse de la tete de note, dans les unites de `largeur`. */
  x: number
  /** Valeur telle qu'elle s'ecrit, ou le code de nuance. Jamais vide en silence. */
  valeur: string | null
  dispersion: string | null
  etiquette: string
  y: number
  hauteur: number
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
  bornes: { x: number; libelle: string }[]
}

export type Disposition = {
  systemes: Systeme[]
  echelles: Echelle[]
  largeur: number
  hauteur: number
  /** Sous ce seuil, deux textes ne tiennent pas sur une meme ligne. */
  etroit: boolean
  /** Abscisses de debut et de fin de portee, communes a toutes les voix. */
  portee: { debut: number; fin: number }
}

const MARGE = 26
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

/** Nombre de decimales ecrites, lu sur la valeur et jamais suppose. */
function decimalesDe(v: number): number {
  const [, apres] = String(v).split('.')
  return apres?.length ?? 0
}

/**
 * Graduation d'une echelle, deduite des valeurs que l'instantane porte sur
 * elle. Le contrat ne publie ni `min`, ni `max`, ni `decimales` au niveau du
 * manifeste ou du marqueur — ils vivent dans la ligne de preuve, qui n'est
 * chargee qu'au clic. Les bornes affichees sont donc des valeurs observees,
 * jamais des bornes supposees, et elles portent leur libelle.
 */
function echelles(instantane: Instantane, largeur: number, debut: number, fin: number): Echelle[] {
  const vues = new Map<string, { familles: Set<string>; valeurs: number[] }>()
  for (const b of instantane.bandes) {
    for (const m of b.marqueurs) {
      const e = vues.get(m.echelle) ?? { familles: new Set<string>(), valeurs: [] }
      e.familles.add(m.famille)
      if (m.valeur !== null) e.valeurs.push(m.valeur)
      vues.set(m.echelle, e)
    }
  }
  const sortie: Echelle[] = []
  for (const [id, e] of [...vues].sort((a, b) => (a[0] < b[0] ? -1 : 1))) {
    if (e.valeurs.length === 0) continue // echelle sans graduation : rien a tracer
    const min = Math.min(...e.valeurs)
    const max = Math.max(...e.valeurs)
    const decimales = Math.max(...e.valeurs.map(decimalesDe))
    const echelle = scaleLinear().domain([min, max]).range([debut, fin])
    sortie.push({
      id,
      familles: [...e.familles].sort(),
      min,
      max,
      decimales,
      bornes:
        min === max
          ? [{ x: px((debut + fin) / 2), libelle: formaterValeur(min, decimales, min < 0) }]
          : [
              { x: px(echelle(min)), libelle: formaterValeur(min, decimales, min < 0) },
              { x: px(echelle(max)), libelle: formaterValeur(max, decimales, min < 0) },
            ],
    })
  }
  void largeur
  return sortie
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
  const debut = MARGE
  const fin = Math.max(debut + 40, largeur - MARGE)
  const etroit = largeur < SEUIL_ETROIT
  const rangs = new Map(manifeste.familles.map((f, i) => [f.id, i]))
  const grilles = echelles(instantane, largeur, debut, fin)
  // Ordre d'affichage : celui des familles du manifeste, jamais celui des ids.
  const premierRang = (e: Echelle): number =>
    Math.min(...e.familles.map((f) => rangs.get(f) ?? 99))
  grilles.sort((a, b) => premierRang(a) - premierRang(b))
  const parEchelle = new Map(grilles.map((e) => [e.id, e]))

  let y = 0
  const systemes: Systeme[] = []
  for (const bande of instantane.bandes) {
    const debutSysteme = y
    y += H_TITRE
    const voix: Voix[] = []
    const ordonnees = [...bande.marqueurs].sort(
      (a, b) => (rangs.get(a.famille) ?? 99) - (rangs.get(b.famille) ?? 99),
    )
    for (const m of ordonnees) {
      const rang = rangs.get(m.famille) ?? manifeste.familles.length
      const grille = parEchelle.get(m.echelle)
      const libelleFamille = manifeste.familles[rang]?.libelle ?? m.famille
      const etat: EtatVoix =
        m.valeur !== null ? 'mesuree' : m.valeur_code !== null ? 'sans_graduation' : 'non_mesuree'

      let x = debut
      let valeur: string | null = null
      if (etat === 'mesuree' && grille) {
        x = px(
          grille.min === grille.max
            ? (debut + fin) / 2
            : scaleLinear().domain([grille.min, grille.max]).range([debut, fin])(m.valeur as number),
        )
        valeur = formaterValeur(m.valeur as number, grille.decimales, grille.min < 0)
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
        forme: FORMES[rang % FORMES.length] as Forme,
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

  return { systemes, echelles: grilles, largeur, hauteur: y, etroit, portee: { debut, fin } }
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
