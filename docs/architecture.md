# Architecture

Trois exécutables, un fichier de vérité, quatre artefacts publiés. Rien d'autre.
Le détail des données est dans [brique0/contrats.md](brique0/contrats.md), le
détail des choix techniques dans [adr/0001-stack.md](adr/0001-stack.md) ; ce
document dit qui fait quoi et surtout qui ne fait pas quoi.

---

## 1. Vue d'ensemble

```
  sources publiques            pipeline (binaire Rust)              artefacts
  ────────────────             ───────────────────────              ─────────

  AN /repository/17/  ──┐
  CHES (chesdata.eu)  ──┼─► [0] récupération ──► archives locales
  nuancier (JO, DGCL) ──┘        + SHA-256          (non versionnées)
                                                        │
                     data/registre/*.json ──────────┐   │
                     (édité à la main, relu)        │   ▼
                                                    └─► [1] ingestion
                                                            │ triplets
                                                            ▼
                                                        [2] matrice
                                                            │ cellules observées
                                                            ▼
                                                        [3] estimateur   (ALS rang 1, ancrage)
                                                            │ positions par député
                                                            ▼
                                                        [4] agrégation   (médiane, dispersion, seuils)
                                                            │
                                                            ▼
                                                        [5] preuves ──► data/preuves/positions.jsonl
                                                            │              (ajout seul, contrat)
                                                            ▼
                                                        [6] export ──► public/api/index.json
                                                                       public/api/instantanes/<id>.json
                                                                       public/api/preuves/<xx>.json
                                                                                │
                                                                                ▼
                                                                       front (React/TS, statique)
```

Un seul sens de circulation. Aucun composant n'écrit en amont de lui, aucun ne
lit un artefact produit par un autre autrement que par le fichier déclaré.

## 2. Les composants

### [0] Récupération

`scripts/recuperer-sources.sh`, script d'intégration continue, pas du code du
binaire. Le script est coupé en deux, et la coupure est la décision de
conception : un script qui appelle le réseau n'est pas testable hors ligne
(RG-119).

| Moitié | Contenu | Test |
|---|---|---|
| Logique | complétude d'un téléchargement contre la taille annoncée, empreinte de contenu (CON §2.8, `LC_ALL=C`), contrôle de stabilité, écriture du descripteur, choix de la date de source | `scripts/test-recuperer-sources.sh`, hors ligne, sourçant le script |
| Enveloppe réseau | `curl -sSIL` pour les en-têtes, `curl -C -` en boucle jusqu'à la taille annoncée, `unzip` | aucun test hors ligne, par construction ; échoue bruyamment (ING §9d) |

La porte est la taille annoncée, jamais le MD5 du producteur (RG-114). Deux
empreintes sont consignées : celle de l'archive atteste du téléchargement, celle
du contenu décide d'une ré-émission et déclenche seule l'échec bruyant quand
elle change à date de source constante (RG-20, RG-115).

Sorties, toutes sous `data/cache/`, non versionnées :

```
data/cache/<empreinte_archive>/<nom>.zip        l'archive, immuable (RG-116)
data/cache/<empreinte_archive>/extrait/         l'archive décompressée
data/cache/<empreinte_archive>/descripteur.txt  url, tailles, last-modified,
                                                etag, md5 documentaire,
                                                les deux empreintes, nb fichiers
data/cache/index.txt                            (nom, date_source, contenu, archive)
data/cache/derniere-source.txt                  horodatage ISO, source de
                                                CONTREPOINT_DATE_CALCUL (RG-117)
```

`scripts/archives.sh` dépose chaque entrée de cache comme asset de release
GitHub nommé par son SHA-256, et sait la retrouver : le cache d'Actions évince
à sept jours et ne peut pas porter la rejouabilité (ING §9e). Ne sont déposées
que les sources dont la licence autorise la redistribution (RG-118, ADR 0000
§8). `index.txt` y est déposé aussi : c'est le seul état qui doit survivre d'une
exécution à l'autre, sans quoi l'échec bruyant de RG-20 n'est pas détectable.

**Ne fait pas :** aucun parsing, aucune transformation, aucune écriture dans
`data/` hors du cache. Les archives récupérées ne sont pas versionnées.

### [1] Ingestion

Lit les scrutins et le référentiel des mandats, produit des triplets
`(acteur, scrutin, valeur)`. Porte les trois adaptateurs d'irrégularité de la
source et la jointure député → groupe sur période de validité.

**Ne fait pas :** aucun calcul de position, aucun filtre de participation, aucune
écriture de cellule pour une absence — une cellule absente n'est pas écrite du
tout (RG-11).

### [2] Matrice

Applique le seul filtre retenu — un scrutin sans minorité enregistrée n'entre pas
— et expose les cellules observées, avec le décompte des scrutins écartés et leur
motif.

**Ne fait pas :** aucune imputation, aucun masque de valeurs manquantes, aucune
densification. La matrice n'est jamais matérialisée en tableau plein.

### [3] Estimateur

Moindres carrés alternés de rang 1 avec constante par scrutin, sur cellules
observées, puis normalisation affine ancrée sur deux médianes de groupe lues dans
le registre d'entités. Calcule aussi le second axe, pour les contrôles.

**Ne fait pas :** ne publie rien, ne choisit jamais une ancre de remplacement, ne
somme rien en parallèle, ne tire aucune graine aléatoire.

### [4] Agrégation

Médiane par groupe, dispersion (**écart interquartile et écart-type de
rééchantillonnage — jamais l'étendue**, dont les deux bornes sont les coordonnées
de deux membres identifiables du groupe : RG-42, ADR 0003 §3), application des
trois conditions de publication. Produit soit une valeur, soit un motif de
non-mesure.

**Ne fait pas :** ni minimum, ni maximum, ni rang, ni quantile d'ordre autre que
Q1, Q2, Q3 sur une distribution de positions individuelles. Ces valeurs ne sont
pas seulement non publiées, elles ne sont pas calculées — il n'existe donc aucun
emplacement d'où elles pourraient fuir.

**Ne fait pas :** ne conserve aucune coordonnée individuelle dans sa sortie, ne
moyenne jamais deux familles, ne comble jamais une absence.

### [5] Preuves

Écrit une ligne JSONL par valeur ou par non-mesure, en ajout seul, avec entité,
famille, échelle, période observée, dates, méthode, entrées empreintées, version
du contrat et du logiciel. Valide chaque ligne contre `schemas/preuve-2.schema.json`
et refuse toute clé hors schéma.

**Ne fait pas :** ne modifie ni ne supprime jamais une ligne existante, n'écrit
aucune valeur qui ne porte pas son échelle et sa source.

### [6] Export

Projette le registre en trois fichiers statiques : le manifeste, un instantané
par date, les éclats de preuves indexés par les deux premiers caractères de
l'identifiant. Construit les bandes selon la règle déterministe RG-54.

**Ne fait pas :** ne calcule aucune valeur, ne reformate aucune ligne de preuve,
ne crée aucun marqueur qui n'existe pas déjà dans le registre.

### Le front

`web/`. React + TypeScript, compilé en statique par Vite, publié sur GitHub
Pages. `publicDir` pointe `../public` : les artefacts sont recopiés dans
`web/dist/` à la construction, jamais dupliqués dans le dépôt (ADR 0002). SVG
écrit à la main, `d3-scale` pour les seules échelles. Une bande par entité, un
marqueur par famille, une graduation par échelle, clic vers l'éclat de preuves.

| Fichier | Rôle |
|---|---|
| `src/contrat.ts` | types du contrat de sortie, refus d'un majeur de schéma inconnu (contrats.md §5.2), extraction du texte exact d'une ligne de preuve dans un éclat (I16) |
| `src/graphe.ts` | disposition : une fonction pure du contrat vers des coordonnées, sans DOM |
| `src/Partition.tsx` | le SVG : un système par entité, une portée par famille, une accolade qui les relie |
| `src/Preuve.tsx` | la ligne de preuve, dans un `<dialog>` natif |
| `src/fixtures/` | artefacts d'exemple et leur script de construction, pour le développement et les tests |
| `scripts/verifier-artefacts.mjs` | contrôle des artefacts contre `schemas/`, exécuté par `npm run prebuild` |

Les graduations affichées sont déduites des valeurs que l'instantané porte sur
chaque échelle : le contrat ne publie `min`, `max` et `decimales` que dans la
ligne de preuve, qui n'est chargée qu'au clic. Les bornes affichées sont donc
des valeurs observées, jamais des bornes supposées.

**Ne fait pas :** aucun appel réseau hors des fichiers de `public/api/`, aucune
liste de familles, d'échelles ou de motifs codée en dur — donc aucune moyenne
possible, puisque le front ne sait pas ce qu'il additionnerait. Aucun calcul,
aucun filtre de données, aucun stockage.

### Le registre d'entités

`data/registre/*.json`, édité à la main, relu ligne par ligne, corrigé par pull
request. C'est le seul fichier du dépôt qui ne soit pas produit par un programme,
et le seul contrat que les briques 1 à 3 consommeront.

**Ne fait pas :** ne porte aucune valeur de position — il porte des identités, des
identifiants externes, des périodes de validité et les deux ancres de l'axe.

## 3. Les frontières qui comptent

| Frontière | Ce qu'elle garantit |
|---|---|
| [4] → [5] | Une position individuelle ne peut pas fuir : elle n'existe plus après l'agrégation |
| [5] → [6] | Le registre est la source unique ; l'export ne peut inventer aucun nombre |
| [6] → front | Le front ne reçoit que des valeurs déjà accompagnées de leur échelle et de leur preuve |
| registre d'entités → [3] | Les ancres de l'axe sont de la donnée versionnée, pas du code |
| [0] → [1] | Une donnée entrant dans le pipeline a une empreinte, donc un calcul est rattachable au fichier exact qui l'a produit |

## 4. Ce qui n'existe pas, et n'est pas prévu

Aucune base de données, aucun serveur d'application, aucune API dynamique :
l'état est constitué de fichiers versionnés et de fichiers statiques servis. Pas
de compte, pas de session, pas de mesure d'usage. Aucune clé d'API, donc aucun
secret à faire tourner. Aucune file de traitement : l'exécution est un binaire
lancé par un cron hebdomadaire, avec déclenchement manuel possible.

## 5. Le point faible connu

Les types du registre d'entités sont définis deux fois — en Rust dans le
pipeline, en TypeScript dans le front — sans garantie de compilateur entre les
deux. La couverture retenue : le pipeline publie un schéma JSON, et la
construction du front échoue si le schéma ne correspond pas. Le contrat est donc
vérifié à l'exécution de l'intégration continue, pas à la compilation. C'est
assumé et réévaluable (ADR 0001 §6).

Cette vérification est `web/scripts/verifier-artefacts.mjs`, lancé par
`npm run prebuild` avant chaque construction et par le test EXP-06. Elle valide
`public/api/` contre les fichiers de `schemas/` et rien d'autre : réécrire ces
schémas en TypeScript en ferait une troisième définition des mêmes types, donc
une aggravation du point faible plutôt qu'une couverture.
