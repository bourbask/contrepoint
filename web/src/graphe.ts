// Disposition du graphe : une fonction pure, du contrat vers des coordonnees.
// Aucun rendu ici, aucun DOM — ce qui rend la geometrie testable hors ligne.
//
// Le vocabulaire est celui de la partition, et il n'est pas decoratif : un
// SYSTEME reunit les voix d'un meme sujet sans les fondre, une VOIX porte sa
// PORTEE et sa propre graduation. Trois graduations distinctes sur un meme
// systeme sont ce qui rend l'addition entre familles impossible a dessiner.
//
// La MISE EN PAGE, elle, groupe les voix par famille : un PANNEAU gradue par
// echelle, ou chaque entite occupe une ligne. Les systemes restent le modele —
// c'est par eux que se verifie qu'aucune voix ne partage l'abscisse d'une
// autre — mais ils ne sont plus une boite dessinee : dix systemes dessines
// coutaient dix graduations repetees pour deux echelles.
//
// Deux proprietes de geometrie portent l'impossibilite d'additionner, et les
// tests les tiennent :
//
//   1. Chaque echelle occupe une PLAGE D'ABSCISSES QUI LUI EST PROPRE. Sur une
//      plage commune, deux tetes ont un milieu visible — l'addition redevient
//      dessinable — et un decalage horizontal se lit comme un ecart mesure
//      entre familles. Les origines et les bornes hautes sont donc decalees.
//   2. Une voix qui ne porte pas de valeur n'est PAS DESSINEE : elle sort du
//      SVG et se dit en toutes lettres (voir `Partition.tsx`). Son abscisse
//      reste la GOUTTIERE, hors de toute plage graduee — posee sur l'axe, elle
//      enoncerait une position que la donnee refuse de publier.
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
  /** Entite portant cette voix : le titre de la ligne dans son panneau. */
  entite: string
  libelleEntite: string
  /** Abscisse de la tete de note, dans les unites de `largeur`. Hors de toute
   *  plage graduee des que l'etat n'est pas « mesuree ». */
  x: number
  /** Valeur telle qu'elle s'ecrit, ou le code de nuance. Jamais vide en silence. */
  valeur: string | null
  /** Effectif et IQR, forme breve : la colonne de droite du SVG n'a pas la
   *  place des intitules. */
  dispersion: string | null
  /** La meme paire, nommee : hors du SVG il n'y a pas de colonne pour la tenir,
   *  et « 25 · 0,69 » nu ne se lit pas. */
  dispersionDite: string | null
  etiquette: string
  /** Haut de la ligne dans son panneau. 0 pour une voix non dessinee. */
  y: number
  /** Hauteur de la ligne. 0 pour une voix non dessinee. */
  hauteur: number
  /** Ordonnee de la tete de note et de sa portee. */
  yTete: number
  /** Portee tracee sous cette voix : celle de son echelle, et rien d'autre.
   *  `null` quand la voix ne porte pas de valeur — aucune portee graduee ne se
   *  dessine sous une pause ni sous un code de nuance. */
  portee: { debut: number; fin: number } | null
  /** Ordonnee de la dispersion. Sous la valeur si la largeur le permet, sur une
   *  ligne propre sinon — deux textes ne se chevauchent jamais. */
  yDispersion: number | null
}

export type Systeme = { id: string; libelle: string; voix: Voix[] }

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

/** Un panneau gradue : une echelle, une famille, une ligne par entite mesuree. */
export type Panneau = {
  echelle: Echelle
  famille: string
  libelleFamille: string
  rang: number
  /** Ordonnee du titre de famille, de l'axe, et du pied. */
  yTitre: number
  yAxe: number
  yPied: number
  /** Hauteur du bloc de lignes, pour les deux bornes verticales pales. */
  hautLignes: number
  basLignes: number
  lignes: Voix[]
}

export type Disposition = {
  systemes: Systeme[]
  echelles: Echelle[]
  panneaux: Panneau[]
  largeur: number
  hauteur: number
  /** Sous ce seuil, deux textes ne tiennent pas sur une meme ligne. */
  etroit: boolean
  /** Abscisse de la colonne de texte : titres d'entite, libelles de famille. */
  marge: number
  /** Bord droit de la zone de dessin : les valeurs y sont ferrees. */
  bord: number
  /** Enveloppe des plages graduees. Aucune voix n'y est placee sans valeur, et
   *  aucune echelle ne l'occupe entierement : c'est une enveloppe, pas un axe. */
  portee: { debut: number; fin: number }
  /** Abscisse des tetes sans valeur, hors de `portee`. Ces voix ne sont pas
   *  dessinees du tout ; l'abscisse reste definie pour que l'invariant
   *  « hors de toute graduation » se verifie sans exception. */
  gouttiere: number
}

/** Sous ce seuil, la valeur ne tient plus a droite du nom : tout passe en pile. */
const SEUIL_ETROIT = 560
/** Hauteur d'une ligne d'entite. Deux constantes de disposition, pas une echelle. */
const H_LIGNE = 46
const H_LIGNE_ETROIT = 76
/** Part de la largeur reservee a la colonne de texte, et colonne des valeurs. */
const PART_TEXTE = 0.42
const TEXTE_MAX = 300
const COLONNE_VALEUR = 92
const ECART = 12

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
 * Chaque echelle recoit sa PROPRE plage d'abscisses : l'echelle de rang `i`
 * est inseree de 19 % et raccourcie de 32 % par rapport a la precedente. Ni
 * l'origine, ni le pole haut, ni le milieu ne sont partages par deux echelles,
 * et le decalage est delibere (GV-03) : corrige, il fabriquerait une
 * concordance entre deux graduations disjointes.
 */
function echelles(
  manifeste: Manifeste,
  instantane: Instantane,
  debut: number,
  course: number,
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
  return graduees.map(([id, familles], i) => {
    const f = bornes.get(id) as { min: number; max: number; decimales: number }
    const { min, max, decimales } = f
    if (!(max > min)) {
      throw new ContratRefuse(
        `Échelle « ${id} » déclarée de ${min} à ${max} : une graduation exige deux bornes distinctes.`,
      )
    }
    // Insertion et raccourcissement croissants : les plages restent disjointes
    // en origine, en pole haut et en milieu, a tout rang.
    const d = px(debut + i * 0.19 * course)
    const g = px(d + Math.max(0.1, 1 - i * 0.32) * course)
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
  const colonneTexte = etroit ? 0 : Math.min(TEXTE_MAX, Math.round(largeur * PART_TEXTE))
  const colonneValeur = etroit ? 0 : COLONNE_VALEUR
  const marge = 0
  const bord = largeur
  const debut = etroit ? 7 : colonneTexte + ECART
  const course = Math.max(
    40,
    etroit ? largeur - 14 : largeur - colonneTexte - colonneValeur - 2 * ECART,
  )
  const hLigne = etroit ? H_LIGNE_ETROIT : H_LIGNE

  const grilles = echelles(manifeste, instantane, debut, course)
  const parEchelle = new Map(grilles.map((e) => [e.id, e]))
  // La gouttiere est a gauche de toute graduation : une voix sans valeur ne peut
  // donc pas etre lue sur une echelle, quelle qu'elle soit. Elle n'est de toute
  // facon pas dessinee.
  const gouttiere = 0

  // ---- Les voix, groupees par entite : le modele, et ce que les tests lisent --
  const systemes: Systeme[] = []
  for (const bande of instantane.bandes) {
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

      voix.push({
        famille: m.famille,
        libelleFamille,
        rang,
        forme: FORMES[rang] as Forme,
        marqueur: m,
        etat,
        entite: bande.id,
        libelleEntite: bande.libelle,
        x,
        valeur,
        // Deux decimales, celles de l'echelle : quatre donneraient a l'IQR une
        // precision que la mesure ne publie pas (GV-11).
        dispersion:
          m.dispersion === null
            ? null
            : `${m.dispersion.effectif} · ${formaterValeur(m.dispersion.iqr, grille?.decimales ?? 2)}`,
        dispersionDite:
          m.dispersion === null
            ? null
            : `effectif ${m.dispersion.effectif} · IQR ${formaterValeur(m.dispersion.iqr, grille?.decimales ?? 2)}`,
        etiquette: etiqueter(bande.libelle, libelleFamille, etat, valeur, m, instantane.date),
        y: 0,
        hauteur: 0,
        yTete: 0,
        portee,
        yDispersion: null,
      })
    }
    systemes.push({ id: bande.id, libelle: bande.libelle, voix })
  }

  // ---- Les panneaux, groupes par echelle : la mise en page ------------------
  // L'ordre des lignes est celui de `instantane.bandes` : le front ne trie
  // jamais, un tri serait un classement (GV-07).
  const toutes = systemes.flatMap((s) => s.voix)
  let y = 0
  const panneaux: Panneau[] = []
  for (const echelle of grilles) {
    const lignes = toutes.filter((v) => v.etat === 'mesuree' && v.marqueur.echelle === echelle.id)
    if (lignes.length === 0) continue
    const premiere = lignes[0] as Voix
    y += 26
    const yTitre = y
    y += etroit ? 40 : 26
    const yAxe = y
    y += 14
    const hautLignes = y
    lignes.forEach((v, i) => {
      v.y = hautLignes + i * hLigne
      v.hauteur = hLigne
      // En etroit, la ligne se lit en pile : nom, puis sigle et valeur, puis
      // la dispersion, puis la portee. Rien ne se chevauche.
      v.yTete = etroit ? v.y + 62 : v.y + hLigne / 2
      v.yDispersion = v.dispersion === null ? null : etroit ? v.y + 48 : v.yTete + 18
    })
    y = hautLignes + lignes.length * hLigne
    const basLignes = y
    y += 20
    const yPied = y
    panneaux.push({
      echelle,
      famille: premiere.famille,
      libelleFamille: premiere.libelleFamille,
      rang: premiere.rang,
      yTitre,
      yAxe,
      yPied,
      hautLignes,
      basLignes,
      lignes,
    })
    y += 14
  }

  return {
    systemes,
    echelles: grilles,
    panneaux,
    largeur,
    hauteur: y + 10,
    etroit,
    marge,
    bord,
    portee: {
      debut: Math.min(...grilles.map((e) => e.debut)),
      fin: Math.max(...grilles.map((e) => e.fin)),
    },
    gouttiere,
  }
}

/**
 * Deux formes d'absence, et deux seulement.
 *
 * « Position non publiée » quand la mesure existe et que ses chiffres sont
 * publies avec le motif — c'est le cas de LIOT et de NI, dont l'IQR est
 * calcule, consigne, et hors des conditions de publication. « Non mesuré »
 * quand aucune valeur n'existe. Confondre les deux fait passer un refus de
 * publier pour une mesure manquante.
 */
export function direAbsence(motifCode: string | null): string {
  return motifCode === 'sous_seuil_de_publication' ? 'Position non publiée' : 'Non mesuré'
}

/** Une phrase, 140 caracteres au plus (ton.md T5). L'absence y est dite. */
function etiqueter(
  bande: string,
  famille: string,
  etat: EtatVoix,
  valeur: string | null,
  m: Marqueur,
  date: string,
): string {
  const tete = `${bande}, ${famille.toLocaleLowerCase('fr')}`
  const corps =
    etat === 'mesuree'
      ? `${valeur} sur l'échelle ${m.echelle}`
      : etat === 'sans_graduation'
        ? `code ${valeur}, sans échelle graduée`
        : direAbsence(m.motif_code).toLocaleLowerCase('fr')
  const phrase = `${tete} : ${corps}, au ${date}.`
  return phrase.length <= 140 ? phrase : `${tete} : ${corps}.`
}
