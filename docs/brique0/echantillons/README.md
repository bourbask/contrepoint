# Échantillons de scrutins — fixtures de test

Extraits de sources publiques en **Licence Ouverte / Open Licence version 1.0**.
Producteur : Assemblée nationale. Récupérés le **2026-08-27**.

Aucun fichier de ce répertoire n'est écrit à la main. Tout est reconstruit par
`extraire-echantillons.py` depuis les deux archives ci-dessous.

## Provenance

| Archive | URL | Taille | SHA-256 | MD5 publié par l'AN | `last-modified` |
|---|---|---|---|---|---|
| `Scrutins.json.zip` | `https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip` | 26 317 479 o | empreinte de **contenu** `c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c` (méthode : [contrats.md](../contrats.md) §2.8) — l'empreinte d'archive varie selon la construction servie, `aa767a2a…` ou `c5e405f1…`, pour un contenu identique | MD5 documentaire, `910e6022…` ou `1f951dea…` selon la construction | 2026-08-27 |
| `AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip` | `https://data.assemblee-nationale.fr/static/openData/repository/17/amo/tous_acteurs_mandats_organes_xi_legislature/AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip` | 13 600 736 o | `bbecd01274d2bc9f46fcaa276b06868862ae7680131da3162e35b5cbef663061` | non publié sur la fiche | 2026-08-27 00:34:47 GMT |

Reconstruction :

```sh
unzip -q Scrutins.json.zip -d s17
unzip -q AMO30_....json.zip -d amo30
python3 docs/brique0/echantillons/extraire-echantillons.py s17 amo30 docs/brique0/echantillons
```

## Contenu

Les cinq scrutins sont **verbatim** — copie octet pour octet du fichier de
l'archive, sans troncature. Les deux index de référentiel sont dérivés et
portent un champ `_provenance` qui le dit.

| Fichier | Date | Votants | Ce qu'il couvre |
|---|---|---|---|
| `VTANR5L17V1.json` | 2024-10-08 | 197 | Premier scrutin de la législature. `typeVote.codeTypeVote = "MOC"` : une motion de censure n'enregistre que les `pour`, donc `contre = 0` et minorité vide par construction |
| `VTANR5L17V156.json` | 2024-10-26 | 163 | Cas nominal. `nombreVotants = pour + contre + abstentions`, `suffragesExprimes = pour + contre` |
| `VTANR5L17V5268.json` | 2026-01-29 | 35 | `votant` sérialisé en **objet nu** ; les **trois** valeurs de `causePositionVote` (`MG`, `PAN`, `PSE`) ; `parDelegation` aux deux valeurs |
| `VTANR5L17V2767.json` | 2025-06-27 | 23 | **Minorité vide** (`contre = 0`) → scrutin écarté ; et **mise au point** nominative |
| `VTANR5L17V6256.json` | 2026-04-16 | 110 | `organeRef = "PO0"` sur **tous** les blocs de groupe : référence pendante, aucun `organe/PO0.json` dans AMO30 |
| `organes-groupes-l17.json` | — | — | Les 14 groupes politiques de la XVIIe, avec `viMoDe.dateDebut` / `dateFin`. Dérivé |
| `mandats-gp-l17.json` | — | — | Cinq députés, un cas de rattachement chacun : mandat dupliqué, trois groupes successifs, retour dans le même groupe, désaccord ventilation / AMO30, présidence de l'Assemblée. Dérivé |

Les identifiants `acteurRef` sont conservés : ils sont publics, indispensables
pour tester la jointure, et ne portent aucune valeur de position. Aucune
coordonnée individuelle n'existe dans ce répertoire.

## Données à caractère personnel

Les identifiants d'acteurs et les mandats conservés ici sont indispensables pour
tester la jointure et ne portent aucune valeur de position. Ce sont des **données
à caractère personnel** au sens du RGPD et de la Licence Ouverte 2.0 ; leur
présence est signalée dans [LICENSE-DONNEES](../../../LICENSE-DONNEES) et dans
[docs/juridique.md](../../juridique.md). Aucune coordonnée individuelle n'existe
dans ce répertoire.

## Les deux familles qui n'ont pas d'échantillon ici, et pourquoi

`experts` et `administratif` n'ont **aucun fichier** dans ce répertoire. Ce
n'est pas un oubli, c'est le seul état conforme :

- **CHES** ne publie aucune licence. La condition obtenue par échange écrit le
  2026-08-27 est une **exigence de citation**, pas une cession de droits, et
  l'ADR 0000 §4 en tire la conséquence : aucune fixture CHES commitée. Le dépôt
  distribue le script, l'URL, la date et les empreintes — jamais la copie, même
  réduite.
- Le fichier de **résultats définitifs par circonscription** est en Licence
  Ouverte v2.0 et serait redistribuable, mais il porte `Nuance candidat n`,
  `Nom candidat n`, `Prénom candidat n`, `Sexe candidat n` et `Elu n`. RG-111
  interdit d'ingérer une colonne nominative « y compris dans un fichier
  intermédiaire ou de cache » : un extrait commité de ce fichier serait
  exactement l'appariement d'une nuance administrative à des personnes nommées
  que le projet refuse de constituer.

Les deux dialectes de CSV sont donc fabriqués dans `pipeline/tests/familles.rs`,
avec les colonnes réelles et des valeurs inventées. Le corpus réel est exercé au
niveau 3, par une exécution du binaire sur le cache.
