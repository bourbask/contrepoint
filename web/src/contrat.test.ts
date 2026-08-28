// Suite du prefixe EXP cote front, plan-de-tests.md §11 et §14bis.
// Hors ligne, sans reseau : les artefacts d'exemple de `fixtures/` sont lus
// depuis le disque et le rendu est produit par renderToStaticMarkup.
import { describe, expect, test } from 'vitest'
import { spawnSync } from 'node:child_process'
import { createHash } from 'node:crypto'
import { readFileSync, readdirSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { createElement } from 'react'
import { renderToStaticMarkup } from 'react-dom/server'

import { ContratRefuse, cheminEclat, verifierSchemas } from './contrat.ts'
import type { Instantane, Manifeste, Preuve } from './contrat.ts'
import { FORMES, direAbsence, disposer } from './graphe.ts'
import { Partition } from './Partition.tsx'
import { verifier } from '../scripts/verifier-artefacts.mjs'

const ici = (chemin: string) => new URL(chemin, import.meta.url)
const lire = (chemin: string) => readFileSync(ici(chemin), 'utf-8')
const lireJson = (chemin: string) => JSON.parse(lire(chemin)) as unknown

const manifeste = lireJson('./fixtures/index.json') as Manifeste
const instantane = lireJson('./fixtures/instantanes/an17-2026-07-21.json') as Instantane
const disposition = disposer(manifeste, instantane, 640)
const rendu = renderToStaticMarkup(
  createElement(Partition, { manifeste, instantane, disposition }),
)

const eclats: Preuve[] = readdirSync(ici('./fixtures/preuves'))
  .sort()
  .flatMap((f) => lireJson(`./fixtures/preuves/${f}`) as Preuve[])

const marqueurs = instantane.bandes.flatMap((b) => b.marqueurs)
const voix = disposition.systemes.flatMap((s) => s.voix)

describe('EXP-01 trois_marqueurs_trois_echelles_nommees', () => {
  test('chaque marqueur porte sa famille, son echelle nommee et sa date', () => {
    expect(marqueurs.length).toBeGreaterThan(0)
    for (const m of marqueurs) {
      expect(m.famille).toMatch(/^[a-z_]+$/)
      expect(m.echelle.length).toBeGreaterThan(0)
    }
    // La date est celle de l'instantane, portee une fois et lue par chaque voix.
    for (const v of voix) expect(v.etiquette).toContain(instantane.date)
    // Une bande porte bien les trois familles cote a cote.
    const troisVoix = disposition.systemes.find((s) => s.voix.length === 3)
    expect(troisVoix?.voix.map((v) => v.famille)).toEqual(
      manifeste.familles.map((f) => f.id),
    )
  })

  test('aucune graduation commune : une echelle par famille, jamais partagee', () => {
    const parFamille = new Map<string, Set<string>>()
    for (const m of marqueurs) {
      const s = parFamille.get(m.famille) ?? new Set<string>()
      s.add(m.echelle)
      parFamille.set(m.famille, s)
    }
    const toutes = [...parFamille.values()].flatMap((s) => [...s])
    expect(new Set(toutes).size).toBe(toutes.length)
    // Deux echelles graduees, deux domaines distincts : aucun axe partage.
    const domaines = disposition.echelles.map((e) => `${e.min}:${e.max}`)
    expect(new Set(domaines).size).toBe(domaines.length)
    expect(disposition.echelles.length).toBeGreaterThan(1)
  })

  test('aucune plage de pixels commune : ni origine, ni pole haut, ni milieu', () => {
    // Des domaines distincts ne suffisent pas. Normalisees sur les MEMES pixels,
    // deux graduations restent superposees, leur milieu est visible, et
    // l'addition entre familles redevient dessinable.
    const plages = disposition.echelles.map((e) => e)
    expect(plages.length).toBeGreaterThan(1)
    for (const cle of ['debut', 'fin'] as const) {
      const vues = plages.map((e) => e[cle])
      expect(new Set(vues).size).toBe(vues.length)
    }
    const milieux = plages.map((e) => (e.debut + e.fin) / 2)
    expect(new Set(milieux).size).toBe(milieux.length)
    // Et les bornes ecrites suivent la plage de leur echelle, pas une autre.
    for (const e of plages) {
      expect(e.bornes.map((b) => b.x)).toEqual([e.debut, e.fin])
    }
  })

  test('les bornes affichees sont celles que le manifeste declare', () => {
    // Derivees des valeurs observees, elles enverraient la plus petite valeur
    // publiee au pole : LFI a 0,82 se retrouvait au pole haut d'une echelle
    // qui va de 0 a 10, ou 0,82 est a 8,2 % de la course.
    for (const e of disposition.echelles) {
      const f = manifeste.familles.find((f) => f.echelle === e.id)
      expect(f).toBeDefined()
      expect([e.min, e.max, e.decimales]).toEqual([f?.min, f?.max, f?.decimales])
    }
    const experts = disposition.echelles.find((e) => e.id === 'ches_lrgen_0_10')
    expect([experts?.min, experts?.max]).toEqual([0, 10])
    // Aucune tete ne touche un pole sans y etre par sa valeur declaree.
    const lfi = disposition.systemes.find((s) => s.id === 'parti.lfi')
    const lfiExperts = lfi?.voix.find((v) => v.famille === 'experts')
    expect(lfiExperts?.x).toBeGreaterThan(experts?.debut ?? 0)
  })

  test('deux voix d une meme entite ne coincident jamais par construction', () => {
    // LFI porte votes −1,0000 et experts 0,82. Sur une plage commune les deux
    // tetes tombaient au meme pixel, ce qui se lit « les deux methodes
    // s accordent exactement » — un enonce qu aucune donnee ne porte.
    const lfi = disposition.systemes.find((s) => s.id === 'parti.lfi')
    expect(lfi?.voix.length).toBe(3)
    const abscisses = (lfi?.voix ?? []).map((v) => v.x)
    expect(new Set(abscisses).size).toBe(abscisses.length)
  })

  test('une voix sans valeur est hors de toute portee graduee', () => {
    // Defaut mesure : les pauses de LIOT et de NI, et les codes de nuance,
    // tombaient a l abscisse de la graduation −1,0000. Un enonce de position sur
    // une entite nommee, precisement la ou la donnee refuse de le publier.
    const sansValeur = voix.filter((v) => v.etat !== 'mesuree')
    expect(sansValeur.length).toBeGreaterThan(0)
    expect(sansValeur.some((v) => v.etat === 'non_mesuree')).toBe(true)
    expect(sansValeur.some((v) => v.etat === 'sans_graduation')).toBe(true)
    for (const v of sansValeur) {
      expect(v.portee).toBeNull()
      expect(v.x).toBe(disposition.gouttiere)
      for (const e of disposition.echelles) {
        expect(v.x < e.debut || v.x > e.fin).toBe(true)
      }
      expect(v.x < disposition.portee.debut || v.x > disposition.portee.fin).toBe(true)
    }
    // Aucune portee graduee ne se dessine sous une tete sans valeur.
    for (const v of voix) expect(v.portee === null).toBe(v.etat !== 'mesuree')
  })

  test('aucun ecart chiffre entre familles nulle part dans le modele de dessin', () => {
    const texte = JSON.stringify(disposition)
    expect(texte).not.toMatch(/moyenne|score|synth[eè]se|global|consensus|indice/i)
    // Aucune voix ne connait la valeur d'une autre : rien a additionner.
    for (const v of voix) expect(Object.keys(v)).not.toContain('autres')
  })
})

describe('EXP-02 aucune_comparaison_inter_legislature', () => {
  test('aucun champ d ecart, de ratio ni de fleche entre deux dates', () => {
    const interdit = /ecart_|delta|difference|variation|evolution|derive|fleche|precedent|depuis_/i
    const cles = (o: unknown, acc: string[] = []): string[] => {
      if (Array.isArray(o)) o.forEach((x) => cles(x, acc))
      else if (o && typeof o === 'object')
        for (const [k, v] of Object.entries(o)) { acc.push(k); cles(v, acc) }
      return acc
    }
    for (const k of cles(disposition)) expect(k).not.toMatch(interdit)
    for (const k of cles(instantane)) expect(k).not.toMatch(interdit)
    for (const k of cles(manifeste)) expect(k).not.toMatch(interdit)
  })

  test('une seule legislature est rendue, et le manifeste n en compare aucune', () => {
    expect(new Set(manifeste.instantanes.map((i) => i.legislature)).size).toBe(1)
    expect(rendu).toContain(instantane.date)
  })
})

describe('EXP-03 absence_dite_jamais_comblee', () => {
  // Deux formes d'absence, et deux seulement (docs/ton.md T7, tel que la
  // direction artistique le reformule) : « Position non publiée » quand la
  // mesure existe et que ses chiffres sont publies avec le motif
  // (`sous_seuil_de_publication` — LIOT porte un IQR de 0,687, calcule et
  // consigne : ce n'est pas la mesure qui manque, c'est la position que le
  // projet refuse de publier), « Non mesuré » quand aucune valeur n'existe.
  // Les confondre ferait passer un refus de publier pour une donnee absente.
  test('une famille sans mesure affiche son etat exact et son motif', () => {
    const absentes = voix.filter((v) => v.etat === 'non_mesuree')
    expect(absentes.length).toBeGreaterThan(0)
    for (const v of absentes) {
      expect(v.marqueur.motif_code).not.toBeNull()
      expect(v.marqueur.motif).not.toBeNull()
      const dit = direAbsence(v.marqueur.motif_code)
      expect(['Position non publiée', 'Non mesuré']).toContain(dit)
      expect(v.etiquette.toLocaleLowerCase('fr')).toContain(dit.toLocaleLowerCase('fr'))
      expect(rendu).toContain(dit)
      expect(rendu).toContain(v.marqueur.motif)
    }
  })

  test('une entite sans aucune mesure est dite, pas dessinee', () => {
    expect(instantane.sans_mesure.length).toBeGreaterThan(0)
    const dessinees = new Set(disposition.systemes.map((s) => s.id))
    for (const e of instantane.sans_mesure) expect(dessinees.has(e.entite)).toBe(false)
  })

  test('jamais « neutre », « centre », « n.d. » ni case vide muette', () => {
    expect(rendu).not.toMatch(/\bneutre\b|\bcentre\b|\bn\.d\.\b|non renseigné/i)
    for (const v of voix) expect(v.valeur === null && v.etat !== 'non_mesuree').toBe(false)
  })
})

describe('EXP-04 limites_de_longueur', () => {
  test('etiquette ≤ 40 caracteres dans les artefacts', () => {
    for (const b of instantane.bandes) {
      expect(b.libelle.length).toBeLessThanOrEqual(40)
      for (const m of b.marqueurs) expect(m.libelle.length).toBeLessThanOrEqual(40)
    }
    for (const f of manifeste.familles) expect(f.libelle.length).toBeLessThanOrEqual(40)
    for (const e of instantane.sans_mesure) expect(e.libelle.length).toBeLessThanOrEqual(40)
  })

  test('legende ou note ≤ 140 caracteres, une phrase', () => {
    const unePhrase = (s: string) => {
      expect(s.length).toBeLessThanOrEqual(140)
      expect(s.split('.').filter((p) => p.trim().length > 0).length).toBeLessThanOrEqual(1)
    }
    unePhrase(instantane.ancrage.note)
    for (const e of instantane.sans_mesure) unePhrase(e.motif)
    for (const m of marqueurs) if (m.motif !== null) unePhrase(m.motif)
    for (const v of voix) unePhrase(v.etiquette)
  })
})

describe('EXP-05 lexique_interdit_absent_de_lexport', () => {
  // La liste canonique vit dans scripts/lexique.sh et nulle part ailleurs
  // (_socle.md regle 3) : elle est lue, jamais recopiee.
  const script = readFileSync(new URL('../../scripts/lexique.sh', import.meta.url), 'utf-8')
  const motifs = ['AXES', 'QUALIF', 'AGREG'].map((nom) => {
    const trouve = new RegExp(`^${nom}='([^']+)'`, 'm').exec(script)
    if (trouve?.[1] === undefined) throw new Error(`scripts/lexique.sh ne declare plus ${nom}`)
    return new RegExp(trouve[1], 'i')
  })

  test('aucune occurrence dans les artefacts, cles comprises', () => {
    const textes = [
      JSON.stringify(manifeste), JSON.stringify(instantane), JSON.stringify(eclats),
    ]
    for (const m of motifs) for (const t of textes) expect(t).not.toMatch(m)
  })

  test('aucune occurrence dans le rendu du graphe ni dans les etiquettes', () => {
    for (const m of motifs) {
      expect(rendu).not.toMatch(m)
      for (const v of voix) expect(v.etiquette).not.toMatch(m)
    }
  })
})

describe('EXP-06 schema_publie_et_verifie_a_la_construction', () => {
  const schemas = new URL('../../schemas/', import.meta.url)

  test('les artefacts d exemple passent les schemas publies', () => {
    expect(verifier(schemas, ici('./fixtures/'))).toEqual([])
  })

  test('un artefact hors schema fait echouer bruyamment', () => {
    const casse = { ...manifeste, familles: [{ id: 'votes', libelle: 'Votes nominatifs' }] }
    const fautes = verifier(schemas, ici('./fixtures/'), { 'index.json': casse })
    expect(fautes.length).toBeGreaterThan(0)
    expect(fautes.join(' ')).toContain('index.json')
  })

  test('un marqueur « non mesuré » sans motif est refuse par le schema de l instantane', () => {
    // L instantane est le SEUL fichier que le front lit. L invariant I6 vivait
    // dans preuve-1.schema.json seulement : un marqueur sans motif passait les
    // controles et s affichait « non mesuré » nu.
    const nu = structuredClone(instantane) as Instantane
    const m = nu.bandes[0]!.marqueurs[0]!
    m.valeur = null
    m.valeur_code = null
    m.motif_code = null
    m.motif = null
    m.dispersion = null
    const fautes = verifier(schemas, ici('./fixtures/'), {
      'instantanes/an17-2026-07-21.json': nu,
    })
    expect(fautes.length).toBeGreaterThan(0)
    expect(fautes.join(' ')).toContain('instantanes/an17-2026-07-21.json')

    // Et le symetrique : valeur et code jamais tous deux presents.
    const deux = structuredClone(instantane) as Instantane
    deux.bandes[0]!.marqueurs[0]!.valeur_code = 'FI'
    expect(
      verifier(schemas, ici('./fixtures/'), { 'instantanes/an17-2026-07-21.json': deux }).length,
    ).toBeGreaterThan(0)
  })

  test('la porte d artefacts sort en 1 quand la racine publiee est absente', () => {
    // Muette, la porte laissait passer exactement le cas ou la publication est
    // cassee. Le drapeau est explicite, il n est pas le defaut.
    const script = fileURLToPath(new URL('../scripts/verifier-artefacts.mjs', import.meta.url))
    const absent = fileURLToPath(new URL('./fixtures/aucune-racine-publiee', import.meta.url))
    expect(spawnSync(process.execPath, [script, absent]).status).toBe(1)
    expect(spawnSync(process.execPath, [script, absent, '--absence-toleree']).status).toBe(0)
  })

  test('un majeur de schema inconnu est refuse, jamais rendu en « non mesuré »', () => {
    expect(() => verifierSchemas(manifeste.schemas)).not.toThrow()
    expect(() => verifierSchemas([...manifeste.schemas, 'contrepoint/instantane/2']))
      .toThrow(ContratRefuse)
    try {
      verifierSchemas(['contrepoint/instantane/2'])
    } catch (e) {
      expect((e as Error).message).not.toContain('non mesuré')
      expect((e as Error).message).toContain('contrepoint/instantane/2')
    }
  })
})

describe('EXP-07 instantane_de_lexport_complet', () => {
  test('le rendu du graphe est identique au rendu fige', () => {
    if (process.env['FIGER'] === '1') {
      const { writeFileSync } = require('node:fs') as typeof import('node:fs')
      writeFileSync(ici('./fixtures/rendu-fige.html'), rendu + '\n')
    }
    expect(rendu + '\n').toBe(lire('./fixtures/rendu-fige.html'))
  })
})

describe('EXP-08 aucune_couleur_seule_porteuse_dinformation', () => {
  test('chaque famille porte une forme distincte, en plus de sa couleur', () => {
    const formes = disposition.echelles.map(() => 0) // longueur indicative
    expect(formes.length).toBeGreaterThan(0)
    const parFamille = new Map(voix.map((v) => [v.famille, v.forme]))
    expect(new Set(parFamille.values()).size).toBe(parFamille.size)
    expect(parFamille.size).toBe(manifeste.familles.length)
  })

  test('une famille hors manifeste est refusee, jamais repliee sur la forme d une autre', () => {
    // Le repli modulaire donnait a une famille inconnue la forme ET la couleur
    // de la premiere famille declaree : deux familles indistinguables.
    const intrus = structuredClone(instantane) as Instantane
    intrus.bandes[0]!.marqueurs[0]!.famille = 'famille_inconnue'
    expect(() => disposer(manifeste, intrus, 640)).toThrow(ContratRefuse)
    // Et la liste des formes couvre les familles declarees, sans modulo.
    expect(manifeste.familles.length).toBeLessThanOrEqual(FORMES.length)
  })

  test('chaque marqueur est atteignable au clavier et porte un libelle accessible', () => {
    expect(new Set(voix.map((v) => v.etiquette)).size).toBe(voix.length)
    const boutons = rendu.match(/tabindex="0"/g) ?? []
    expect(boutons.length).toBe(voix.length)
    expect((rendu.match(/aria-label="/g) ?? []).length).toBeGreaterThanOrEqual(voix.length)
  })

  test('aucune couleur de parti issue de la source n est reprise', () => {
    expect(JSON.stringify(disposition)).not.toMatch(/couleurAssociee/i)
    // La couleur est indexee sur le rang de la famille, jamais sur l entite.
    for (const s of disposition.systemes)
      for (const v of s.voix)
        expect(v.rang).toBe(manifeste.familles.findIndex((f) => f.id === v.famille))
  })
})

describe('artefacts d exemple : provenance verifiable', () => {
  // Les fixtures ne valent que si leurs identifiants sont ceux que produit la
  // regle de deduplication de contrats.md §3. Recalcul complet, ligne par ligne.
  const canonique = (o: unknown): string =>
    o === null || typeof o !== 'object'
      ? JSON.stringify(o)
      : Array.isArray(o)
        ? `[${o.map(canonique).join(',')}]`
        : `{${Object.keys(o as object).sort().map((k) =>
            `${JSON.stringify(k)}:${canonique((o as Record<string, unknown>)[k])}`).join(',')}}`

  test('chaque ligne de preuve porte l identifiant que sa cle produit', () => {
    expect(eclats.length).toBeGreaterThan(0)
    for (const l of eclats) {
      const cle = [
        l.famille, l.entite, l.observation.debut, l.observation.fin,
        l.methode.id, l.methode.version, canonique(l.methode.parametres),
        l.entrees.map((e) => e.empreinte_contenu_sha256).sort().join(','),
      ].join('\x1f')
      expect(createHash('sha256').update(cle, 'utf-8').digest('hex')).toBe(l.id)
    }
  })

  test('tout marqueur pointe une ligne presente dans l eclat que son id designe', () => {
    const parId = new Map(eclats.map((l) => [l.id, l]))
    for (const m of marqueurs) {
      expect(parId.has(m.preuve)).toBe(true)
      expect(cheminEclat(manifeste.preuves.racine, m.preuve))
        .toBe(`preuves/${m.preuve.slice(0, 2)}.json`)
    }
  })

  test('le manifeste decrit l instantane qu il reference', () => {
    const entree = manifeste.instantanes[0]!
    const brut = lire('./fixtures/instantanes/an17-2026-07-21.json')
    expect(entree.octets).toBe(Buffer.byteLength(brut, 'utf-8'))
    expect(entree.empreinte_sha256).toBe(createHash('sha256').update(brut).digest('hex'))
    expect(entree.bandes).toBe(instantane.bandes.length)
  })
})

describe('EXP-09 fond declare des systemes', () => {
  test('tout rect de fond porte un remplissage declare, hors selecteur alterne', () => {
    // Un `<rect>` SVG sans `fill` est peint en NOIR par le navigateur. La seule
    // regle qui existait visait `:nth-child(odd)` : un systeme sur deux sortait
    // en pave noir, texte illisible. Invisible en theme sombre — le papier y est
    // deja presque noir — et flagrant en theme clair. Constate en ligne le
    // 2026-08-28, sur la page publiee.
    //
    // Le test lit la feuille de style plutot que le rendu : le rendu fige
    // n'embarque aucun style, et une porte raster couterait un navigateur.
    // Les commentaires sont retires d'abord : sans cela ils sont happes dans le
    // selecteur capture, et le test echoue en accusant une regle qui existe.
    const styles = readFileSync(new URL('./styles.css', import.meta.url), 'utf-8').replace(
      /\/\*[\s\S]*?\*\//g,
      ' ',
    )
    const regles = [...styles.matchAll(/(^|\})\s*([^{}@]*?\.fond)\s*\{([^}]*)\}/g)].map((m) => ({
      selecteur: m[2]!.trim(),
      corps: m[3]!,
    }))
    expect(regles.length).toBeGreaterThan(0)

    const base = regles.filter((r) => !/:nth-child|:nth-of-type/.test(r.selecteur))
    expect(
      base.some((r) => /\bfill\s*:/.test(r.corps)),
      `aucune regle de base ne donne un \`fill\` a .fond — les systemes non vises ` +
        `par le selecteur alterne seront peints en noir. Selecteurs vus : ` +
        regles.map((r) => r.selecteur).join(' | '),
    ).toBe(true)
  })
})

describe('EXP-10 ordre des lignes dans un panneau gradue', () => {
  test('les lignes d un panneau sont croissantes en abscisse', () => {
    // Support de l'amendement GV-07 du 2026-08-28. Le panneau des experts
    // rendait dans l'ordre de `bandes[]`, qui est un ordre de VOTES : le
    // pipeline y range par mediane ancree et ajoute a la fin les entites qui
    // n'en portent pas. Le trace sortait 0,82 · 3,45 · 6,60 · 7,73 · 8,82 ·
    // 2,30 · 1,73 · 6,27 — un zigzag qui detruisait la lecture, et qui ne
    // disait rien d'autre que « ces trois-la sont absentes d'une autre
    // famille ».
    //
    // Ce que ce test NE valide PAS, et que GV-07 continue d'interdire : un tri
    // par effectif, par dispersion ou par etat de mesure. Trier par l'abscisse
    // deja dessinee ne publie rien de neuf — l'oeil lit cet ordre sur l'axe
    // avant de le lire dans la colonne des noms.
    // L'entree est DESORDONNEE a dessein. Sur les fixtures livrees, les bandes
    // sont deja croissantes : le test y passait meme apres avoir retire le tri
    // — mutant verifie, il survivait. Un test qui ne distingue pas les deux
    // implementations ne garde rien.
    const melange = {
      ...instantane,
      bandes: [...instantane.bandes].reverse(),
    }
    const disposition = disposer(manifeste, melange, 760)
    expect(disposition.panneaux.length).toBeGreaterThan(0)

    for (const panneau of disposition.panneaux) {
      const abscisses = panneau.lignes.map((l) => l.x)
      const triees = [...abscisses].sort((a, b) => a - b)
      expect(
        abscisses,
        `panneau « ${panneau.libelleFamille} » : les lignes doivent etre croissantes ` +
          `en abscisse, obtenu ${JSON.stringify(abscisses)}`,
      ).toEqual(triees)
    }

    // Le cas de test doit mordre : au moins un panneau porte assez de lignes
    // pour qu'un ordre soit observable.
    expect(Math.max(...disposition.panneaux.map((p) => p.lignes.length))).toBeGreaterThan(2)
  })
})
