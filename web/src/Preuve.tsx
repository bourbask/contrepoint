// La preuve d'un marqueur, a un clic. La tracabilite est l'actif du projet :
// elle n'est pas dans une page annexe.
//
// `<dialog>` natif : la capture du focus, la fermeture par Echap et l'arriere-
// plan inerte sont fournis par la plateforme, sans code a ecrire ni a tester.

import { useEffect, useRef, type JSX } from 'react'
import type { Preuve } from './contrat.ts'
import { direAbsence } from './graphe.ts'

export type EtatPreuve =
  | { etat: 'chargement'; id: string }
  | { etat: 'lue'; id: string; ligne: Preuve; brut: string }
  | { etat: 'refus'; id: string; message: string }

function Champ({ nom, children }: { nom: string; children: React.ReactNode }): JSX.Element {
  return (
    <>
      <dt>{nom}</dt>
      <dd>{children}</dd>
    </>
  )
}

function Corps({ ligne, brut }: { ligne: Preuve; brut: string }): JSX.Element {
  const valeur =
    ligne.valeur !== null
      ? String(ligne.valeur)
      : ligne.valeur_code !== null
        ? ligne.valeur_code
        : direAbsence(ligne.motif_code)
  const graduee =
    ligne.echelle.min !== null && ligne.echelle.max !== null && ligne.echelle.decimales !== null
  return (
    <>
      <p className="cadran">
        <span className={`cadran__valeur${ligne.valeur === null ? ' cadran__valeur--texte' : ''}`}>
          {valeur}
        </span>
        <span className="cadran__bornes">
          {graduee
            ? `sur une échelle de ${ligne.echelle.min} à ${ligne.echelle.max}, ${String(ligne.echelle.decimales)} décimales`
            : 'sans échelle graduée'}
        </span>
      </p>
      <dl className="champs">
        <Champ nom="Entité mesurée">
          <span className="chiffre">{ligne.entite}</span>
        </Champ>
        <Champ nom="Famille de mesure">{ligne.famille}</Champ>
        <Champ nom="Valeur">
          <span className="chiffre">{valeur}</span>
        </Champ>
        <Champ nom="Échelle">
          {ligne.echelle.libelle} <span className="chiffre">({ligne.echelle.id})</span>
        </Champ>
        {ligne.motif !== null && <Champ nom="Motif">{ligne.motif}</Champ>}
        <Champ nom="Période observée">
          <span className="chiffre">
            du {ligne.observation.debut} au {ligne.observation.fin}
          </span>
        </Champ>
        <Champ nom="Date de la source">
          <span className="chiffre">{ligne.date_source}</span>
        </Champ>
        <Champ nom="Date du calcul">
          <span className="chiffre">{ligne.date_calcul}</span>
        </Champ>
        <Champ nom="Méthode">
          <span className="chiffre">
            {ligne.methode.id} {ligne.methode.version}
          </span>
        </Champ>
        <Champ nom="Paramètres">
          <span className="chiffre">
            {Object.entries(ligne.methode.parametres)
              .map(([c, v]) => `${c} = ${String(v)}`)
              .join(' · ')}
          </span>
        </Champ>
        {ligne.dispersion !== null && (
          <Champ nom="Dispersion interne">
            <span className="chiffre">
              effectif {ligne.dispersion.effectif} · IQR {ligne.dispersion.iqr} · écart-type de
              rééchantillonnage {ligne.dispersion.ecart_type_reechantillonnage}
            </span>
          </Champ>
        )}
        <Champ nom="Logiciel">
          <span className="chiffre">
            {ligne.logiciel.version}
            {ligne.logiciel.commit === null ? '' : ` · ${ligne.logiciel.commit}`}
          </span>
        </Champ>
      </dl>

      {ligne.entrees.map((e) => (
        <div className="preuve__entree" key={e.empreinte_contenu_sha256}>
          <dl className="champs">
            <Champ nom="Fichier d'entrée">
              <a href={e.url} rel="noreferrer noopener">
                {e.url}
              </a>
            </Champ>
            <Champ nom="Producteur">{e.producteur}</Champ>
            <Champ nom="Mise à jour de la source">
              <span className="chiffre">{e.derniere_mise_a_jour}</span>
            </Champ>
            <Champ nom="Récupéré le">
              <span className="chiffre">{e.recupere_le}</span>
            </Champ>
            <Champ nom="Empreinte de l'archive">
              <span className="chiffre">{e.empreinte_sha256}</span>
            </Champ>
            <Champ nom="Empreinte du contenu">
              <span className="chiffre">{e.empreinte_contenu_sha256}</span>
            </Champ>
            {e.citation !== null && <Champ nom="Citation exigée">{e.citation}</Champ>}
          </dl>
        </div>
      ))}

      <pre className="preuve__brut">{brut}</pre>
    </>
  )
}

export function Preuve({
  etat,
  onFermer,
}: {
  etat: EtatPreuve | null
  onFermer: () => void
}): JSX.Element {
  const boite = useRef<HTMLDialogElement>(null)
  useEffect(() => {
    const d = boite.current
    if (d === null) return
    if (etat !== null && !d.open) d.showModal()
    if (etat === null && d.open) d.close()
  }, [etat])

  return (
    <dialog className="preuve" ref={boite} onClose={onFermer} aria-label="Preuve de la position">
      <div className="preuve__tete">
        <h2>Ligne de preuve</h2>
        <button className="bouton" type="button" onClick={onFermer}>
          Fermer
        </button>
      </div>
      <div className="preuve__corps">
        {etat === null || etat.etat === 'chargement' ? (
          <p>Lecture de la ligne de preuve.</p>
        ) : etat.etat === 'refus' ? (
          <p className="refus">{etat.message}</p>
        ) : (
          <Corps ligne={etat.ligne} brut={etat.brut} />
        )}
      </div>
    </dialog>
  )
}
