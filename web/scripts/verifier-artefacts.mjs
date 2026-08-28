// EXP-06 — le contrat pipeline vers front, verifie a la construction du front.
//
// Un seul validateur, sur les schemas publies de `schemas/`. Reecrire ces
// schemas en TypeScript en ferait une troisieme definition des memes types :
// c'est exactement le point faible que l'ADR 0001 §6 concede, et l'aggraver
// pour le couvrir n'aurait aucun sens. La construction echoue bruyamment,
// avant publication, plutot que le navigateur d'un lecteur.
import { existsSync, readdirSync, readFileSync } from 'node:fs'
import { fileURLToPath, pathToFileURL } from 'node:url'
import Ajv2020 from 'ajv/dist/2020.js'

/** @param {URL} racine @param {string} nom */
const schema = (racine, nom) => JSON.parse(readFileSync(new URL(nom, racine), 'utf-8'))

/**
 * Le schema qui gouverne un artefact, par son chemin relatif a `public/api/`.
 * @param {string} relatif
 * @returns {string | null}
 */
export function schemaDe(relatif) {
  if (relatif === 'index.json') return 'manifeste-1.schema.json'
  if (/^instantanes\/[^/]+\.json$/.test(relatif)) return 'instantane-1.schema.json'
  if (/^preuves\/[0-9a-f]{2}\.json$/.test(relatif)) return 'eclat-preuves-1.schema.json'
  return null
}

/** @param {URL} racine @returns {string[]} */
function artefacts(racine) {
  const dossier = fileURLToPath(racine)
  if (!existsSync(dossier)) return []
  return readdirSync(dossier, { recursive: true, withFileTypes: true })
    .filter((e) => e.isFile() && e.name.endsWith('.json'))
    .map((e) => `${e.parentPath ?? e.path}/${e.name}`.slice(dossier.length).replace(/^\/+/, ''))
    .sort()
}

/**
 * Valide les artefacts contre les schemas publies.
 * @param {URL} racineSchemas
 * @param {URL} racineArtefacts
 * @param {Record<string, unknown>} [remplacements] contenu substitue, pour les tests
 * @returns {string[]} une faute par ligne ; vide si tout passe
 */
export function verifier(racineSchemas, racineArtefacts, remplacements = {}) {
  const ajv = new Ajv2020({ strict: true, allErrors: true })
  ajv.addSchema(schema(racineSchemas, 'preuve-1.schema.json'))
  /** @type {Map<string, import('ajv').ValidateFunction>} */
  const compiles = new Map()
  /** @type {string[]} */
  const fautes = []
  const chemins = [...new Set([...artefacts(racineArtefacts), ...Object.keys(remplacements)])].sort()
  for (const relatif of chemins) {
    const nom = schemaDe(relatif)
    if (nom === null) {
      fautes.push(`${relatif} : aucun schéma associé — ajouter la correspondance dans schemaDe`)
      continue
    }
    const donnee = Object.hasOwn(remplacements, relatif)
      ? remplacements[relatif]
      : JSON.parse(readFileSync(new URL(relatif, racineArtefacts), 'utf-8'))
    let valider = compiles.get(nom)
    if (valider === undefined) {
      valider = ajv.compile(schema(racineSchemas, nom))
      compiles.set(nom, valider)
    }
    if (!valider(donnee)) {
      for (const e of valider.errors ?? []) {
        fautes.push(`${relatif} contre ${nom} : ${e.instancePath || '/'} ${e.message ?? ''}`)
      }
    }
  }
  return fautes
}

// Entree de `npm run prebuild`.
if (import.meta.url === `file://${process.argv[1]}`) {
  const schemas = new URL('../../schemas/', import.meta.url)
  // `--absence-toleree` : developpement local, ou la sortie du pipeline n'est
  // pas sur le disque. Sans ce drapeau l'absence est un echec, parce que c'est
  // exactement le cas ou la publication est cassee : une porte muette la laisse
  // passer et le lecteur recoit un site sans donnees.
  const arguments_ = process.argv.slice(2)
  const toleree = arguments_.includes('--absence-toleree')
  const racine = arguments_.find((a) => !a.startsWith('--'))
  const api =
    racine === undefined
      ? new URL('../../public/api/', import.meta.url)
      : pathToFileURL(`${racine.replace(/\/*$/, '')}/`)
  if (!existsSync(fileURLToPath(api))) {
    const chemin = fileURLToPath(api)
    if (toleree) {
      console.log(`${chemin} est absent, --absence-toleree : aucun artefact à vérifier.`)
      process.exit(0)
    }
    console.error(`${chemin} est absent : aucun artefact publiable, construction interrompue.`)
    console.error("  En développement local, sans sortie de pipeline : npm run build:local")
    process.exit(1)
  }
  const fautes = verifier(schemas, api)
  if (fautes.length > 0) {
    console.error('Artefacts hors contrat, construction interrompue :')
    for (const f of fautes) console.error(`  ${f}`)
    process.exit(1)
  }
  console.log('Artefacts conformes aux schémas publiés.')
}
