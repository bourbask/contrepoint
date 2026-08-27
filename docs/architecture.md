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

Script d'intégration continue, pas du code du binaire. `curl -C -` en boucle
jusqu'à la taille annoncée, `unzip`, SHA-256 consigné, échec bruyant si une
empreinte change sans que la date de source change (RG-20).

**Ne fait pas :** aucun parsing, aucune transformation, aucune écriture dans
`data/`. Les archives récupérées ne sont pas versionnées.

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

Médiane par groupe, dispersion (IQR, étendue, écart-type de rééchantillonnage),
application des trois conditions de publication. Produit soit une valeur, soit un
motif de non-mesure.

**Ne fait pas :** ne conserve aucune coordonnée individuelle dans sa sortie, ne
moyenne jamais deux familles, ne comble jamais une absence.

### [5] Preuves

Écrit une ligne JSONL par valeur ou par non-mesure, en ajout seul, avec entité,
famille, échelle, période observée, dates, méthode, entrées empreintées, version
du contrat et du logiciel. Valide chaque ligne contre `schemas/preuve-1.schema.json`
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

React + TypeScript, compilé en statique par Vite, publié sur GitHub Pages. SVG
écrit à la main, `d3-scale` pour les seules échelles. Une bande par entité, un
marqueur par famille, une graduation par échelle, clic vers l'éclat de preuves.

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
