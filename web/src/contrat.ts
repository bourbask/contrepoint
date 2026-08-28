// Contrat de sortie consomme par le front — docs/brique0/contrats.md §4 et §5.
//
// Consommateur tolerant (§5.1) : aucune liste de familles, d'echelles ni de
// motifs n'est ecrite ici. Un front qui ne connait pas la liste des familles ne
// peut pas les moyenner, puisqu'il ne sait pas ce qu'il additionnerait.
// La seule liste close est celle des MAJEURS de schema que ce front sait lire.

export type Marqueur = {
  famille: string
  echelle: string
  valeur: number | null
  valeur_code: string | null
  libelle: string
  motif_code: string | null
  motif: string | null
  dispersion: { effectif: number; iqr: number } | null
  preuve: string
}

/** Un parti que le registre retient pour un groupe. Deux champs, aucun
 *  effectif : le nombre de membres qui le declarent n'existe qu'en prose dans
 *  le registre, et un nombre tire d'une phrase serait fabrique. */
export type PartiDeclare = { entite: string; libelle: string }

export type Bande = {
  id: string
  libelle: string
  /** Partis abrites par ce groupe, ABSENT sur une bande de parti et sur un
   *  groupe dont le registre ne retient aucun parti. Le nom dit ce que le
   *  front doit redire au lecteur : la liste est un extrait, jamais la
   *  composition complete d'un groupe. contrats.md §4.2. */
  composition_partielle?: PartiDeclare[]
  marqueurs: Marqueur[]
}

export type SansMesure = {
  entite: string
  libelle: string
  motif_code: string | null
  motif: string
}

export type Instantane = {
  schema: string
  contrat: string
  id: string
  chambre: string
  legislature: string
  date: string
  date_arretee: string
  ancrage: { famille: string; ancre_gauche: string; ancre_droite: string; note: string }
  bandes: Bande[]
  sans_mesure: SansMesure[]
}

/** Legende d'une famille. `min`, `max` et `decimales` sont les bornes DECLAREES
 *  de sa graduation, recopiees par le pipeline depuis les lignes de preuve, et
 *  `null` pour une famille sans echelle graduee. Le front ne derive jamais une
 *  borne des valeurs qu'il affiche : deriver ferait tomber la plus petite
 *  valeur publiee au pole de l'echelle. contrats.md §4.1. */
export type Famille = {
  id: string
  libelle: string
  echelle: string
  min: number | null
  max: number | null
  decimales: number | null
}

export type EntreeInstantane = {
  id: string
  chambre: string
  legislature: string
  date: string
  url: string
  empreinte_sha256: string
  octets: number
  bandes: number
}

export type Manifeste = {
  schema: string
  contrat: string
  schemas: string[]
  date_arretee: string
  licence: string
  mention_paternite: string
  familles: Famille[]
  instantanes: EntreeInstantane[]
  preuves: { racine: string; eclats: number; fonction: string }
}

export type Entree = {
  source: string
  url: string
  producteur: string
  derniere_mise_a_jour: string
  citation: string | null
  empreinte_sha256: string
  empreinte_contenu_sha256: string
  recupere_le: string
}

/** Une ligne de preuve. Elle ne porte PAS `contrat` : la version du contrat
 *  decrit le format, pas la mesure, et elle vit dans le manifeste et dans
 *  l'instantane (contrats.md, contrat 0.6.0). */
export type Preuve = {
  schema: string
  id: string
  famille: string
  entite: string
  valeur: number | null
  valeur_code: string | null
  echelle: { id: string; min: number | null; max: number | null; decimales: number | null; libelle: string }
  motif_code: string | null
  motif: string | null
  dispersion: { effectif: number; iqr: number; ecart_type_reechantillonnage: number } | null
  observation: { debut: string; fin: string }
  date_source: string
  date_calcul: string
  methode: { id: string; version: string; parametres: Record<string, unknown> }
  epingles: { nom: string; version: string }[]
  entrees: Entree[]
  logiciel: { version: string; commit: string | null }
}

/** Majeurs de schema que ce front sait rendre. contrats.md §5.2. */
export const SCHEMAS_CONNUS: readonly string[] = [
  'contrepoint/preuve/2',
  'contrepoint/instantane/1',
  'contrepoint/eclat-preuves/1',
]

/** Refus du contrat : le front ne rend rien plutot que de rendre a moitie. */
export class ContratRefuse extends Error {
  constructor(message: string) {
    super(message)
    this.name = 'ContratRefuse'
  }
}

/**
 * contrats.md §5.2 — un majeur inconnu fait refuser l'artefact entier, et le
 * message dit qu'une version plus recente est exigee. Jamais « non mesuré » :
 * une incompatibilite de version n'est pas une absence de donnee, et les
 * confondre ferait passer un defaut d'outil pour un resultat.
 */
export function verifierSchemas(declares: string[]): void {
  const inconnus = declares.filter((s) => !SCHEMAS_CONNUS.includes(s))
  if (inconnus.length > 0) {
    throw new ContratRefuse(
      `Ces données exigent une version plus récente de l'affichage : ${inconnus.join(', ')}.`,
    )
  }
}

/** contrats.md §4.4 — le chemin d'un eclat se derive de l'id, sans index. */
export function cheminEclat(racine: string, id: string): string {
  return `${racine}${id.slice(0, 2)}.json`
}

/**
 * Extrait d'un eclat le TEXTE exact de la ligne portant cet identifiant.
 *
 * Une ligne de preuve est publiee identique octet pour octet a celle du
 * registre (contrats.md §4.4, I16) : la relire puis la reserialiser en
 * produirait une reformulation, et c'est justement ce que le contrat interdit.
 * Le decoupage se fait donc sur le texte, par profondeur d'accolades.
 */
export function extraireLigne(texte: string, id: string): string | null {
  let profondeur = 0
  let debut = -1
  let dansChaine = false
  let echappe = false
  for (let i = 0; i < texte.length; i += 1) {
    const c = texte[i] as string
    if (dansChaine) {
      if (echappe) echappe = false
      else if (c === '\\') echappe = true
      else if (c === '"') dansChaine = false
      continue
    }
    if (c === '"') dansChaine = true
    else if (c === '{') {
      if (profondeur === 0) debut = i
      profondeur += 1
    } else if (c === '}') {
      profondeur -= 1
      if (profondeur === 0 && debut >= 0) {
        const objet = texte.slice(debut, i + 1)
        if (objet.includes(`"id":"${id}"`)) return objet
        debut = -1
      }
    }
  }
  return null
}
