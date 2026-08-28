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
import { disposer, formaterValeur } from './graphe.ts'
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

/** Marque : les glyphes de famille relies par une accolade. La marque EST la
 *  legende — les memes trois formes se retrouvent sur les panneaux. Les `fill`
 *  restent en attribut de presentation : aucune regle CSS ne porte sur
 *  `rect`, `circle` ou `path` nus, sinon elle gagnerait contre eux et
 *  effacerait la cle. */
function Marque(): JSX.Element {
  return (
    <header className="marque">
      <svg className="marque__cle" width="30" height="42" viewBox="0 0 30 42" aria-hidden="true" focusable="false">
        <path d="M8 3 C3 3 7 19 1 21 C7 23 3 39 8 39" fill="none" stroke="var(--regle)" strokeWidth="1.2" />
        <rect x="18.6" y="7.6" width="6.8" height="6.8" fill="var(--f-0)" />
        <path d="M22 17.6 L26.4 22 L22 26.4 L17.6 22 Z" fill="var(--f-1)" />
        <rect x="18.6" y="28.6" width="6.8" height="6.8" fill="none" stroke="var(--f-2)" strokeWidth="1.4" />
      </svg>
      <h1>Contrepoint</h1>
    </header>
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
        Trois familles de mesure par entité, affichées côte à côte, jamais moyennées.
      </p>

      <div className="bandeau">
        <span>Données arrêtées le {arret}</span>
        <span>
          {instantane.chambre} · XVII<sup>e</sup> législature · au {instantane.date}
        </span>
        <span>
          <a href="https://github.com/bourbask/contrepoint/issues">Signaler une erreur</a>
        </span>
      </div>

      <section>
        <h2>Familles de mesure et échelles</h2>
        <dl className="familles">
          {manifeste.familles.map((f, rang) => (
            <div className={`famille marqueur--${rang}`} key={f.id}>
              <dt>{f.libelle}</dt>
              <dd>{f.echelle}</dd>
              <dd className="graduation">
                {f.min === null || f.max === null || f.decimales === null
                  ? 'aucune borne publiée'
                  : `graduée de ${formaterValeur(f.min, f.decimales, f.min < 0)} à ${formaterValeur(f.max, f.decimales, f.min < 0)}`}
              </dd>
            </div>
          ))}
        </dl>
        <p className="note">
          Autant de graduations que de familles : aucune valeur n'est convertie d'une échelle vers
          une autre.
        </p>
      </section>

      <section>
        <h2>Positions publiées, par famille</h2>
        <p className="bloc">{instantane.ancrage.note}</p>
        <div ref={cadre}>
          <Partition
            manifeste={manifeste}
            instantane={instantane}
            disposition={disposition}
            onPreuve={ouvrirPreuve}
          />
        </div>

        {instantane.sans_mesure.length > 0 && (
          <>
            <h3>Entités sans mesure — {instantane.sans_mesure.length}</h3>
            {instantane.sans_mesure.map((e) => (
              <div className="retenue" key={e.entite}>
                <div className="retenue__tete">
                  <p className="retenue__nom">{e.libelle}</p>
                  <span className="retenue__etat">Aucune famille ne porte de valeur</span>
                </div>
                <p className="retenue__motif">{e.motif}</p>
              </div>
            ))}
          </>
        )}
      </section>

      <footer className="pied">
        <p>{manifeste.mention_paternite}</p>
        <p className="licences">Code AGPL-3.0-only · Données {manifeste.licence}</p>
        <ul>
          <li>
            <a href="https://github.com/bourbask/contrepoint/blob/main/docs/methode.md">Méthode</a>
          </li>
          <li>
            <a href="https://github.com/bourbask/contrepoint/blob/main/docs/utilisation.md">
              Lecture du graphe
            </a>
          </li>
          <li>
            <a href="https://github.com/bourbask/contrepoint/issues">Signaler une erreur</a>
          </li>
        </ul>
        <p className="pied__date">
          Données arrêtées le {arret} · instantané {instantane.id} · contrat {instantane.contrat}
        </p>
      </footer>

      <Preuve etat={preuve} onFermer={() => setPreuve(null)} />
    </main>
  )
}
