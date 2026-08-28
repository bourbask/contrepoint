// Ce qui relève de la présentation seule : nature d'une entité, couleur
// d'identité, phrases qui disent ce qu'une famille mesure.
//
// Rien ici n'entre dans une valeur, une abscisse ni une graduation. Le module
// existe pour que ces trois choses soient au même endroit et qu'on voie d'un
// coup d'œil qu'aucune n'est mesurée.

import registre from '../../data/identite/couleurs.json'

/** Nature d'une entité, lue sur le préfixe de son identifiant.
 *
 *  Les trois préfixes sont contractuels (docs/ton.md §3, « Entités ») : un
 *  parti est une personne morale durable, une coalition une alliance datée, un
 *  groupe parlementaire l'organe d'une chambre pour une législature. La
 *  distinction est portée par le modèle (methode.md §4) et se perdait à
 *  l'affichage : « Gauche Démocrate et Républicaine » est un groupe, pas un
 *  parti, et rien à l'écran ne le disait.
 *
 *  Un identifiant hors des trois préfixes ne reçoit rien : inventer une nature
 *  serait affirmer ce que la donnée ne dit pas. */
export function natureEntite(id: string, court = false): string | null {
  if (id.startsWith('parti.')) return 'parti'
  if (id.startsWith('coalition.')) return 'coalition'
  if (id.startsWith('groupe.')) return court ? 'groupe' : 'groupe parlementaire'
  return null
}

/** Les partis qu'un groupe abrite, en une ligne.
 *
 *  Le site n'affichait que le nom du groupe : quelqu'un qui cherche les
 *  communistes ou les écologistes ne les trouvait pas, alors que le PCF siège
 *  à « Gauche Démocrate et Républicaine » et les écologistes à « Écologiste et
 *  Social ». La composition vivait dans le registre, lu par le seul pipeline.
 *
 *  « extrait » n'est pas une précaution de style. La composition du registre
 *  EST un extrait : GDR n'y porte que le PCF quand ses membres déclarent aussi
 *  sept partis ultramarins hors périmètre. Publier la liste sans le dire
 *  serait faux par omission, et le champ publié porte ce mot dans son nom.
 *
 *  Aucun effectif : le nombre de membres qui déclarent un parti n'existe que
 *  dans une phrase du registre, et un nombre tiré d'une phrase est fabriqué. */
export function direComposition(partis: { libelle: string }[]): string | null {
  if (partis.length === 0) return null
  return `abrite (extrait) : ${partis.map((p) => p.libelle).join(', ')}`
}

/** Couleur d'identité déclarée par l'organisation, récupérée de Wikidata
 *  (propriété P465, CC0) par `scripts/couleurs-identite.sh`. Aucune n'est
 *  choisie ici.
 *
 *  Le fichier est délibérément hors du registre d'entités et hors de la chaîne
 *  de preuve (voir l'en-tête du script) : il est de la présentation, et le
 *  front l'incorpore à la construction. Ce n'est pas un appel réseau, et il ne
 *  transite pas par `public/api/`, qui ne porte que des artefacts mesurés.
 *
 *  Deux règles d'emploi, tenues par `styles.css` et par les appelants :
 *
 *    1. Jamais sur le marqueur. La teinte du marqueur encode la FAMILLE de
 *       mesure (EXP-08) ; deux systèmes de couleur sur le même glyphe se
 *       détruisent. L'identité ne va que sur l'étiquette de l'entité.
 *    2. Jamais seule porteuse. Le nom de l'entité est toujours à côté, et le
 *       carré ne porte aucun texte : sa lisibilité ne dépend donc pas de son
 *       contraste avec l'encre. Le seul contraste exigé est celui de son
 *       CONTOUR avec le papier, et il est constant — un filet `--regle` fait
 *       le tour des onze, du #FFF100 de Place publique, illisible sur papier
 *       clair, au #00205B de Renaissance, illisible sur papier sombre. */
export function couleurIdentite(entite: string): string | null {
  return registre.couleurs.find((c) => c.entite === entite)?.srgb ?? null
}

/** Ce qu'une famille mesure, en une phrase, et le nom de son échelle.
 *
 *  Le site affichait `votes_an17_ancre_v1` et `ches_lrgen_0_10` : des
 *  identifiants de contrat, qui ne disent ni ce qui a été mesuré, ni sur quoi.
 *
 *  Ce n'est PAS la liste close de familles que `contrat.ts` refuse d'écrire
 *  (contrats.md §5.1) : une famille absente de cette table s'affiche avec son
 *  libellé et ses bornes, sans phrase, et rien d'autre ne change. Aucun code
 *  ne branche sur ces clés — elles ne servent qu'à choisir un texte.
 *
 *  Les faits sont recopiés de docs/methode.md §1 et §3 et de
 *  docs/brique0/positionnement.md §1, et de nulle part ailleurs. */
export type Phrases = { quoi: string; echelle: string }

export const PHRASES: Record<string, Phrases> = {
  votes: {
    quoi:
      "Position tirée de l'ensemble des 7 979 scrutins publics retenus sur 8 434, " +
      'du 2024-10-08 au 2026-07-21 — aucun scrutin en particulier.',
    echelle: 'Unités ancrées : médiane LFI-NFP à −1,00, médiane RN à +1,00',
  },
  experts: {
    quoi:
      'Enquête Chapel Hill, vague 2024 : des politologues situent chaque parti ' +
      'sur une échelle gauche-droite de 0 à 10.',
    echelle: 'CHES, variable lrgen, échelle 0 à 10',
  },
  administratif: {
    quoi:
      "Code de dépouillement attribué par le ministère de l'Intérieur aux " +
      'législatives 2024 : une étiquette de comptage, sans position ni ordre.',
    echelle: "Ministère de l'intérieur — code de nuance, législatives 2024",
  },
}
