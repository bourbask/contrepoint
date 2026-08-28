// Le graphe. SVG ecrit a la main, ADR 0001 §3 : aucune bibliotheque de
// graphiques ne sait afficher trois marqueurs que le projet refuse de moyenner,
// et l'ordre du DOM doit rester sous controle pour la lecture au clavier.
//
// Un PANNEAU par echelle graduee, une ligne par entite mesuree, et rien
// d'autre dans le SVG. Ce qui n'a pas de graduation n'entre pas dans le
// dessin :
//
//   - une position retenue (LIOT, NI) se dit en toutes lettres, avec son
//     chiffre et son seuil en regard. Le rendu precedent en dessinait un
//     intervalle ouvert dont la longueur valait l'IQR a l'echelle de l'axe :
//     GV-10 refuse toute longueur geometrique derivee de `dispersion`, et une
//     tete posee dans une gouttiere se lit encore comme un defaut d'alignement
//     qu'on aura envie de « corriger » sur l'axe (GV-08) ;
//   - un code de nuance n'a ni min, ni max, ni decimales, et n'a AUCUN ORDRE :
//     il entre dans un repertoire, jamais dans une abscisse ni dans une rangee
//     (GV-09). Le libelle du marqueur, qui nomme son producteur, precede le
//     code dans l'ordre du DOM (GV-16).
//
// Le decalage des deux graduations est delibere (GV-03) : il est le controle,
// pas un defaut de mise en page.

import type { JSX } from 'react'
import type { Instantane, Manifeste } from './contrat.ts'
import type { Disposition, Forme, Voix } from './graphe.ts'
import { direAbsence } from './graphe.ts'
import { PHRASES, couleurIdentite, natureEntite } from './presentation.ts'

/** Largeur de la colonne du carre d'identite, dans le SVG. Le nom s'y indente
 *  toujours, que le carre soit peint ou non : une entite sans couleur declaree
 *  ne doit pas se distinguer par un alignement. */
const COLONNE_IDENTITE = 16

/** Le carre d'identite, dans le SVG. `style` et non `fill` : la regle
 *  `svg.partition rect { fill: none }` gagnerait contre un attribut de
 *  presentation et effacerait la couleur. Le contour vient de la feuille de
 *  style, et il est le meme pour les onze. */
function CarreSvg({ entite, x, y }: { entite: string; x: number; y: number }): JSX.Element | null {
  const srgb = couleurIdentite(entite)
  if (srgb === null) return null
  return <rect className="identite" x={x} y={y} width={10} height={10} style={{ fill: srgb }} />
}

/** Le meme carre, hors du SVG. */
function Carre({ entite }: { entite: string }): JSX.Element | null {
  const srgb = couleurIdentite(entite)
  if (srgb === null) return null
  return <span className="identite" style={{ background: srgb }} aria-hidden="true" />
}

/** La nature de l'entite, hors du SVG. */
export function Nature({ entite }: { entite: string }): JSX.Element | null {
  const nature = natureEntite(entite)
  if (nature === null) return null
  return <span className="nature"> · {nature}</span>
}

export { Carre }

type Props = {
  manifeste: Manifeste
  instantane: Instantane
  disposition: Disposition
  onPreuve?: (id: string) => void
}

/** Tetes de note. Chaque famille en a une distincte : la couleur ne porte rien seule. */
function TeteDeNote({ forme, x, y }: { forme: Forme; x: number; y: number }): JSX.Element {
  switch (forme) {
    case 'losange':
      return <path className="tete tete--pleine" d={`M${x},${y - 6}L${x + 6},${y}L${x},${y + 6}L${x - 6},${y}Z`} />
    case 'cercle':
      return <circle className="tete tete--pleine" cx={x} cy={y} r={5} />
    case 'carre':
      return <rect className="tete tete--pleine" x={x - 5} y={y - 5} width={10} height={10} />
    case 'triangle':
      return <path className="tete tete--pleine" d={`M${x},${y - 6}L${x + 5.5},${y + 4.5}L${x - 5.5},${y + 4.5}Z`} />
    default:
      return <path className="tete tete--creuse" d={`M${x - 4.5},${y - 4.5}L${x + 4.5},${y + 4.5}M${x - 4.5},${y + 4.5}L${x + 4.5},${y - 4.5}`} />
  }
}

/** Ce qui rend une voix atteignable : un clic, une touche, une phrase lue. */
function proprietesVoix(
  voix: Voix,
  onPreuve: ((id: string) => void) | undefined,
): Record<string, unknown> {
  const activer = (): void => onPreuve?.(voix.marqueur.preuve)
  return {
    tabIndex: 0,
    role: 'button',
    'aria-label': voix.etiquette,
    onClick: activer,
    onKeyDown: (e: { key: string; preventDefault: () => void }) => {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault()
        activer()
      }
    },
  }
}

/** Une ligne d'entite dans un panneau gradue. */
function Ligne({
  voix,
  disposition,
  seul,
  nue,
  onPreuve,
}: {
  voix: Voix
  disposition: Disposition
  /** Le libelle du marqueur est-il le meme sur tout le panneau ? Alors il est
   *  deja au pied, et le repeter dix fois est du bruit. */
  seul: boolean
  /** Une ligne sur deux reste sur le papier : le fond alterne se pose sur
   *  l'autre. Rien n'y est indexe sur la valeur. */
  nue: boolean
  onPreuve: ((id: string) => void) | undefined
}): JSX.Element {
  const { etroit, largeur } = disposition
  const portee = voix.portee as { debut: number; fin: number }
  const cy = voix.yTete
  // Le nom et son sigle tiennent dans la ligne : deposes plus bas, le fond de
  // la ligne suivante, peint apres, leur coupe les jambages.
  const yNom = etroit ? voix.y + 18 : cy - 3
  const yValeur = etroit ? voix.y + 34 : cy + 1
  const nom = disposition.marge + COLONNE_IDENTITE
  // Un groupe parlementaire n'est pas un parti (methode.md §4), et rien a
  // l'ecran ne le disait : « Gauche Democrate et Republicaine » est un groupe,
  // « Parti socialiste » un parti mesure sur les votes du groupe SOC. La nature
  // vient du prefixe de l'identifiant, contractuel (ton.md §3), et se pose sur
  // la seconde ligne — posee en suffixe du nom, elle mordait sur la graduation.
  // Le libelle du marqueur reste apres elle quand il n'est pas deja au pied.
  const nature = natureEntite(voix.entite, true)
  const dessous = [nature, seul ? null : voix.marqueur.libelle].filter((t) => t !== null)
  return (
    <g className={`ligne${nue ? ' ligne--nue' : ''} marqueur--${voix.rang}`} {...proprietesVoix(voix, onPreuve)}>
      <rect className="fond" x={0} y={voix.y} width={largeur} height={voix.hauteur} />
      <CarreSvg entite={voix.entite} x={disposition.marge} y={yNom - 9} />
      <text className="titre" x={nom} y={yNom}>
        {voix.libelleEntite}
      </text>
      {dessous.length > 0 && (
        <text className="sigle" x={nom} y={yNom + (etroit ? 16 : 15)}>
          {dessous.join(' · ')}
        </text>
      )}
      <line className="portee" x1={portee.debut} y1={cy} x2={portee.fin} y2={cy} />
      <TeteDeNote forme={voix.forme} x={voix.x} y={cy} />
      <text className="valeur" x={largeur} y={yValeur} textAnchor="end">
        {voix.valeur}
      </text>
      {voix.dispersion !== null && voix.yDispersion !== null && (
        <text className="dispersion" x={largeur} y={voix.yDispersion} textAnchor="end">
          {voix.dispersion}
        </text>
      )}
    </g>
  )
}

/** Un bloc de texte pour ce qui n'a pas de graduation : nom, etat, chiffres, motif. */
function Retenue({
  voix,
  onPreuve,
}: {
  voix: Voix
  onPreuve: ((id: string) => void) | undefined
}): JSX.Element {
  return (
    <div className="retenue" {...proprietesVoix(voix, onPreuve)}>
      <div className="retenue__tete">
        <p className="retenue__nom">
          <Carre entite={voix.entite} />
          {voix.libelleEntite}
          <Nature entite={voix.entite} />
        </p>
        <span className="retenue__etat">{direAbsence(voix.marqueur.motif_code)}</span>
      </div>
      <p className="retenue__compte">
        {voix.marqueur.libelle}
        {voix.dispersionDite === null ? '' : ` · ${voix.dispersionDite}`}
      </p>
      {voix.marqueur.motif !== null && <p className="retenue__motif">{voix.marqueur.motif}</p>}
    </div>
  )
}

export function Partition({ manifeste, instantane, disposition, onPreuve }: Props): JSX.Element {
  const toutes = disposition.systemes.flatMap((s) => s.voix)
  const retenues = toutes.filter((v) => v.etat === 'non_mesuree')
  const nuances = toutes.filter((v) => v.etat === 'sans_graduation')
  const familleNuance = manifeste.familles.find((f) => f.id === nuances[0]?.famille)

  return (
    <>
      <svg
        className="partition"
        width={disposition.largeur}
        height={disposition.hauteur}
        role="group"
        aria-label={`Positions publiées par famille, au ${instantane.date}.`}
      >
        {disposition.panneaux.map((p) => {
          const seul = new Set(p.lignes.map((v) => v.marqueur.libelle)).size === 1
          return (
            <g className={`panneau marqueur--${p.rang}`} key={p.echelle.id}>
              <text className="famille" x={disposition.marge} y={p.yTitre}>
                {p.libelleFamille}
              </text>
              {/* Exactement deux barres de graduation, aux bornes declarees. */}
              <line
                className="axe"
                x1={p.echelle.debut}
                y1={p.yAxe}
                x2={p.echelle.fin}
                y2={p.yAxe}
              />
              {p.echelle.bornes.map((b, i) => (
                <g key={b.libelle}>
                  <line className="tick" x1={b.x} y1={p.yAxe - 5} x2={b.x} y2={p.yAxe + 1} />
                  <text
                    className="graduation"
                    x={b.x}
                    y={p.yAxe - 10}
                    textAnchor={i === 0 ? 'start' : 'end'}
                  >
                    {b.libelle}
                  </text>
                  <line
                    className="borne"
                    x1={b.x}
                    y1={p.hautLignes}
                    x2={b.x}
                    y2={p.basLignes}
                  />
                </g>
              ))}
              {p.lignes.map((v, i) => (
                <Ligne
                  key={v.entite}
                  voix={v}
                  disposition={disposition}
                  seul={seul}
                  nue={i % 2 === 1}
                  onPreuve={onPreuve}
                />
              ))}
              {/* Le pied nommait l'echelle par son identifiant de contrat —
                  `votes_an17_ancre_v1`. Il la nomme desormais en francais ;
                  l'identifiant reste a la legende, ou il est etiquete comme
                  tel. Aucun nombre ici hors des bornes declarees (GV-01). */}
              <text className="echelle" x={disposition.marge} y={p.yPied}>
                {PHRASES[p.famille]?.echelle ?? p.echelle.id}
              </text>
            </g>
          )
        })}
      </svg>

      <p className="note-axes">
        Les axes ne partagent ni origine ni largeur : les échelles sont disjointes, et le décalage
        est délibéré.
      </p>

      {retenues.length > 0 && (
        <>
          <h3>Mesuré, non publié</h3>
          {retenues.map((v) => (
            <Retenue key={`${v.entite}-${v.famille}`} voix={v} onPreuve={onPreuve} />
          ))}
        </>
      )}

      {nuances.length > 0 && familleNuance !== undefined && (
        <>
          <h3>{familleNuance.libelle}</h3>
          {/* GV-16 : le producteur nomme precede le code dans l'ordre du DOM. */}
          <p className="sous-titre">
            {nuances[0]?.marqueur.libelle} ·{' '}
            {PHRASES[familleNuance.id]?.quoi ?? 'Code sans échelle graduée.'}
          </p>
          {/* Une RANGEE de pastilles se lit comme un axe. Les codes sortaient
              dans l'ordre du contrat — FI · SOC · HOR · LR · RN · ENS · UG —
              qui est, pour les cinq premiers, l'ordre des votes de gauche a
              droite ; alignes, ils enoncaient une gauche et une droite, et
              placaient le Nouveau Front populaire a droite du Rassemblement
              national. La nuance n'a aucun ordre (GV-09).
              La forme retenue est un INDEX : une colonne, le nom d'abord, le
              code ferre a droite. Il n'y a plus de rangee ou lire un axe. */}
          <dl className="nuancier">
            {nuances.map((v) => (
              <div className={`nuancier__entree marqueur--${v.rang}`} key={v.entite}>
                <dt>
                  <Carre entite={v.entite} />
                  {v.libelleEntite}
                  <Nature entite={v.entite} />
                </dt>
                <dd>
                  <span className="nuancier__code" {...proprietesVoix(v, onPreuve)}>
                    {v.valeur}
                  </span>
                </dd>
              </div>
            ))}
          </dl>
        </>
      )}
    </>
  )
}
