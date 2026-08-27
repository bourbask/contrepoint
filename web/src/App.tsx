// Enveloppe du graphe : lecture des artefacts, bandeau de date, entites sans
// mesure, mention de paternite, ouverture de la preuve.
//
// Aucun appel reseau hors de `public/api/`. Aucun calcul sur les valeurs.

import { useCallback, useEffect, useRef, useState, type JSX } from 'react'
import {
  ContratRefuse,
  cheminEclat,
  extraireLigne,
  verifierSchemas,
  type Instantane,
  type Manifeste,
  type Preuve as LignePreuve,
} from './contrat.ts'
import { disposer } from './graphe.ts'
import { Partition } from './Partition.tsx'
import { Preuve, type EtatPreuve } from './Preuve.tsx'

const API = `${import.meta.env.BASE_URL}api/`
const LARGEUR_PAR_DEFAUT = 640

async function lire(chemin: string): Promise<unknown> {
  const r = await fetch(`${API}${chemin}`)
  if (!r.ok) throw new ContratRefuse(`${API}${chemin} : réponse ${r.status}.`)
  return r.json()
}

type Chargement =
  | { etat: 'lecture' }
  | { etat: 'refus'; message: string }
  | { etat: 'pret'; manifeste: Manifeste; instantane: Instantane }

/** Marque : trois voix reliees par une accolade, en petit. */
function Marque(): JSX.Element {
  return (
    <div className="marque">
      <svg className="marque__cle" width="34" height="40" aria-hidden="true">
        <path className="accolade" d="M13,6C6.7,8 13,16 6,20C13,24 6.7,32 13,34" fill="none" />
        <g className="v0">
          <path className="tete tete--pleine" d="M22,3.6L26.2,9L22,14.4L17.8,9Z" fill="currentColor" />
        </g>
        <g className="v1">
          <circle className="tete tete--creuse" cx="22" cy="20" r="4.4" />
        </g>
        <g className="v2">
          <rect className="tete tete--pleine" x="18" y="27" width="8" height="8" fill="currentColor" />
        </g>
      </svg>
      <h1>Contrepoint</h1>
    </div>
  )
}

export function App(): JSX.Element {
  const [chargement, setChargement] = useState<Chargement>({ etat: 'lecture' })
  const [preuve, setPreuve] = useState<EtatPreuve | null>(null)
  const [largeur, setLargeur] = useState(LARGEUR_PAR_DEFAUT)
  const cadre = useRef<HTMLDivElement>(null)

  useEffect(() => {
    let vivant = true
    void (async () => {
      try {
        const manifeste = (await lire('index.json')) as Manifeste
        verifierSchemas(manifeste.schemas)
        const entree = manifeste.instantanes.at(-1)
        if (entree === undefined) throw new ContratRefuse('Le manifeste ne référence aucun instantané.')
        const instantane = (await lire(entree.url)) as Instantane
        if (vivant) setChargement({ etat: 'pret', manifeste, instantane })
      } catch (e) {
        if (vivant) setChargement({ etat: 'refus', message: (e as Error).message })
      }
    })()
    return () => {
      vivant = false
    }
  }, [])

  useEffect(() => {
    const cible = cadre.current
    if (cible === null) return
    const observateur = new ResizeObserver(([e]) => {
      if (e !== undefined) setLargeur(Math.round(e.contentRect.width))
    })
    observateur.observe(cible)
    return () => observateur.disconnect()
  }, [chargement.etat])

  const ouvrirPreuve = useCallback(
    (id: string) => {
      if (chargement.etat !== 'pret') return
      setPreuve({ etat: 'chargement', id })
      void (async () => {
        try {
          const chemin = cheminEclat(chargement.manifeste.preuves.racine, id)
          const r = await fetch(`${API}${chemin}`)
          if (!r.ok) throw new ContratRefuse(`${API}${chemin} : réponse ${r.status}.`)
          const texte = await r.text()
          const brut = extraireLigne(texte, id)
          if (brut === null) throw new ContratRefuse(`Aucune ligne ${id} dans ${chemin}.`)
          setPreuve({ etat: 'lue', id, ligne: JSON.parse(brut) as LignePreuve, brut })
        } catch (e) {
          setPreuve({ etat: 'refus', id, message: (e as Error).message })
        }
      })()
    },
    [chargement],
  )

  if (chargement.etat !== 'pret') {
    return (
      <main className="page">
        <Marque />
        {chargement.etat === 'lecture' ? (
          <p className="these">Lecture des artefacts publiés.</p>
        ) : (
          <div className="refus">
            <p>Les données publiées ne sont pas lisibles par cette version de l'affichage.</p>
            <p>{chargement.message}</p>
          </div>
        )}
      </main>
    )
  }

  const { manifeste, instantane } = chargement
  const disposition = disposer(manifeste, instantane, largeur)
  const arret = instantane.date_arretee.slice(0, 10)

  return (
    <main className="page">
      <Marque />
      <p className="these">
        Trois mesures de position par entité — votes, experts, administration —, affichées côte à
        côte et jamais moyennées.
      </p>

      <p className="bandeau">
        <span>Données arrêtées le {arret}</span>
        <span>
          {instantane.chambre} · XVII<sup>e</sup> législature · au {instantane.date}
        </span>
        <span>{disposition.systemes.length} entités</span>
      </p>

      <div ref={cadre}>
        <Partition
          manifeste={manifeste}
          instantane={instantane}
          disposition={disposition}
          onPreuve={ouvrirPreuve}
        />
      </div>

      <p className="these">{instantane.ancrage.note}</p>

      {instantane.sans_mesure.length > 0 && (
        <section className="sans-mesure">
          <h2>Entités sans mesure</h2>
          <dl>
            {instantane.sans_mesure.map((e) => (
              <div key={e.entite}>
                <dt>{e.libelle}</dt>
                <dd>{e.motif}</dd>
              </div>
            ))}
          </dl>
        </section>
      )}

      <footer className="pied">
        <h2>Source et licence</h2>
        <p>{manifeste.mention_paternite}</p>
        <p>{manifeste.licence}</p>
        <ul>
          <li>
            <a href="https://github.com/bourbask/contrepoint/blob/main/docs/methode.md">Méthode</a>
          </li>
          <li>
            <a href="https://github.com/bourbask/contrepoint/blob/main/docs/utilisation.md">
              Comment se lit le graphe
            </a>
          </li>
          <li>
            <a href="https://github.com/bourbask/contrepoint/issues">Signaler une erreur</a>
          </li>
        </ul>
      </footer>

      <Preuve etat={preuve} onFermer={() => setPreuve(null)} />
    </main>
  )
}
