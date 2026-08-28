// Le graphe. SVG ecrit a la main, ADR 0001 §3 : aucune bibliotheque de
// graphiques ne sait afficher trois marqueurs que le projet refuse de moyenner,
// et l'ordre du DOM doit rester sous controle pour la lecture au clavier.
//
// La forme vient du contrepoint : un SYSTEME par entite, une VOIX par famille,
// chaque voix sur sa portee et sa propre graduation. Une voix sans mesure porte
// une PAUSE — le signe qui, en musique, note un silence voulu plutot qu'une
// mesure laissee vide. Cette pause se pose dans la GOUTTIERE, a gauche de toute
// graduation : posee sur l'axe, elle enoncerait une position que la donnee
// refuse de publier, et la mention en italique ne la rattraperait pas — l'oeil
// lit la position avant la legende.

import type { JSX } from 'react'
import type { Instantane, Manifeste } from './contrat.ts'
import type { Disposition, Forme, Voix } from './graphe.ts'
import { FORMES } from './graphe.ts'

type Props = {
  manifeste: Manifeste
  instantane: Instantane
  disposition: Disposition
  onPreuve?: (id: string) => void
}

/** Accolade de systeme : deux courbes qui se rejoignent en pointe a mi-hauteur. */
function accolade(x: number, y0: number, y1: number, l = 7): string {
  const ym = (y0 + y1) / 2
  return (
    `M${x + l},${y0}C${x + l * 0.1},${y0 + 2} ${x + l},${ym - l} ${x},${ym}` +
    `C${x + l},${ym + l} ${x + l * 0.1},${y1 - 2} ${x + l},${y1}`
  )
}

/** Tetes de note. Chaque famille en a une distincte : la couleur ne porte rien seule. */
function TeteDeNote({ forme }: { forme: Forme }): JSX.Element {
  switch (forme) {
    case 'losange':
      return <path className="tete tete--pleine" d="M0,-5.4L4.2,0L0,5.4L-4.2,0Z" />
    case 'cercle':
      return <circle className="tete tete--creuse" r="4.4" />
    case 'carre':
      return <rect className="tete tete--pleine" x="-4" y="-4" width="8" height="8" />
    case 'triangle':
      return <path className="tete tete--pleine" d="M0,-5.4L4.9,4L-4.9,4Z" />
    default:
      return <path className="tete tete--creuse" d="M-4.2,-4.2L4.2,4.2M-4.2,4.2L4.2,-4.2" />
  }
}

/** Pause : le silence est note, il n'est pas une mesure laissee vide. */
function Pause(): JSX.Element {
  return <rect className="pause" x="-6" y="-4.5" width="12" height="4.5" />
}

function Marqueur({
  voix,
  idMotif,
  onPreuve,
}: {
  voix: Voix
  idMotif: string | undefined
  onPreuve: ((id: string) => void) | undefined
}): JSX.Element {
  const yLigne = voix.y + 30
  // Le cote d'ecriture se decide sur la plage de CETTE voix, jamais sur une
  // plage commune : il n'y en a pas.
  const versGauche =
    voix.portee !== null && voix.x > (voix.portee.debut + voix.portee.fin) / 2
  const activer = (): void => onPreuve?.(voix.marqueur.preuve)
  return (
    <g
      className={`marqueur marqueur--${voix.rang} marqueur--${voix.etat}`}
      tabIndex={0}
      role="button"
      aria-label={voix.etiquette}
      {...(idMotif === undefined ? {} : { 'aria-describedby': idMotif })}
      onClick={activer}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault()
          activer()
        }
      }}
    >
      <rect className="cible" x={voix.x - 17} y={yLigne - 13} width={34} height={26} rx={3} />
      <g transform={`translate(${voix.x},${yLigne})`}>
        {voix.etat === 'non_mesuree' ? <Pause /> : <TeteDeNote forme={voix.forme} />}
      </g>
      {voix.valeur !== null && (
        <text
          className="valeur"
          x={versGauche ? voix.x - 11 : voix.x + 11}
          y={yLigne + 4}
          textAnchor={versGauche ? 'end' : 'start'}
        >
          {voix.valeur}
        </text>
      )}
      {voix.etat === 'non_mesuree' && (
        <text className="valeur valeur--absente" x={voix.x + 13} y={yLigne + 4}>
          non mesuré
        </text>
      )}
    </g>
  )
}

export function Partition({ manifeste, instantane, disposition, onPreuve }: Props): JSX.Element {
  const { marge, bord } = disposition
  const graduees = new Set(disposition.echelles.flatMap((e) => e.familles))

  // ---- Lecture : la graduation de chaque echelle, tracee une seule fois ----
  const lignes: JSX.Element[] = []
  let yl = 26
  for (const e of disposition.echelles) {
    const familles = manifeste.familles.filter((f) => e.familles.includes(f.id))
    lignes.push(
      <g className="lecture" key={e.id}>
        {familles.map((f) => {
          const rang = manifeste.familles.indexOf(f)
          return (
            <g className={`marqueur--${rang}`} key={f.id}>
              <g transform={`translate(${marge + 6},${yl - 4})`}>
                <TeteDeNote forme={FORMES[rang] as Forme} />
              </g>
              <text className="famille" x={marge + 20} y={yl}>
                {f.libelle}
              </text>
            </g>
          )
        })}
        <line className="portee" x1={e.debut} y1={yl + 18} x2={e.fin} y2={yl + 18} />
        {e.bornes.map((b) => (
          <g key={b.libelle}>
            <line className="barre" x1={b.x} y1={yl + 12} x2={b.x} y2={yl + 24} />
            <text
              className="graduation"
              x={b.x}
              y={yl + 38}
              textAnchor={b.x > (e.debut + e.fin) / 2 ? 'end' : 'start'}
            >
              {b.libelle}
            </text>
          </g>
        ))}
      </g>,
    )
    yl += 58
  }
  for (const f of manifeste.familles.filter((f) => !graduees.has(f.id))) {
    const rang = manifeste.familles.indexOf(f)
    lignes.push(
      <g className={`lecture marqueur--${rang}`} key={f.id}>
        <g transform={`translate(${marge + 6},${yl - 4})`}>
          <TeteDeNote forme={FORMES[rang] as Forme} />
        </g>
        <text className="famille" x={marge + 20} y={yl}>
          {f.libelle}
        </text>
        <text className="note" x={marge + 20} y={yl + 17}>
          Code de nuance, sans échelle graduée.
        </text>
      </g>,
    )
    yl += 44
  }

  return (
    <>
      <svg
        className="lecture-svg"
        width={disposition.largeur}
        height={yl}
        role="group"
        aria-label={`Lecture des ${manifeste.familles.length} familles de mesure, au ${instantane.date}.`}
      >
        <text className="entete" x={marge} y={14}>
          Lecture
        </text>
        {lignes}
      </svg>

      <svg
        className="partition"
        width={disposition.largeur}
        height={disposition.hauteur}
        role="group"
        aria-label={`Positions de ${disposition.systemes.length} entités, au ${instantane.date}.`}
      >
        {disposition.systemes.map((s) => (
          <g className="systeme" key={s.id}>
            <rect className="fond" x={0} y={s.y} width={disposition.largeur} height={s.hauteur} />
            <path
              className="accolade"
              d={accolade(8, s.y + 22, s.y + s.hauteur - 18)}
              fill="none"
            />
            <text className="titre" x={marge} y={s.y + 18}>
              {s.libelle}
            </text>
            {!disposition.etroit && (
              <text className="entite" x={bord} y={s.y + 18} textAnchor="end">
                {s.id}
              </text>
            )}
            {s.voix.map((v) => {
              const idMotif = v.marqueur.motif === null ? undefined : `motif-${s.id}-${v.famille}`
              return (
                <g className="voix" key={v.famille}>
                  <text className="famille" x={marge} y={v.y + 13}>
                    {v.marqueur.libelle}
                  </text>
                  {v.dispersion !== null && v.yDispersion !== null && (
                    <text
                      className="dispersion"
                      x={disposition.etroit ? marge : bord}
                      y={v.yDispersion}
                      textAnchor={disposition.etroit ? 'start' : 'end'}
                    >
                      {v.dispersion}
                    </text>
                  )}
                  {v.portee !== null && (
                    <line
                      className="portee"
                      x1={v.portee.debut}
                      y1={v.y + 30}
                      x2={v.portee.fin}
                      y2={v.y + 30}
                    />
                  )}
                  <Marqueur voix={v} idMotif={idMotif} onPreuve={onPreuve} />
                  {v.marqueur.motif !== null && (
                    <text
                      className="motif"
                      id={idMotif}
                      x={marge}
                      y={v.y + v.hauteur - 6}
                    >
                      {v.marqueur.motif}
                    </text>
                  )}
                  {v.etat === 'sans_graduation' && (
                    <text className="note" x={v.x + 13 + 26} y={v.y + 34}>
                      sans échelle graduée
                    </text>
                  )}
                </g>
              )
            })}
          </g>
        ))}
      </svg>
    </>
  )
}
