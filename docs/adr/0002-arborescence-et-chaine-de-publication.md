# ADR 0002 — Arborescence et chaîne de publication

Statut : **accepté**. Date : 2026-08-27.

Décide : où vit le code, où vont les artefacts, comment une version est produite.
Ne décide pas : la pile technique (voir [0001](0001-stack.md)), ni les schémas
d'artefacts (voir [../brique0/contrats.md](../brique0/contrats.md)).

## Contexte

L'ADR 0001 retient un binaire Rust et un front React/TypeScript, et
`docs/architecture.md` fixe la sortie du pipeline en `public/api/`. Restait à
placer les deux bases de code et à décider comment le front consomme les
artefacts sans les dupliquer.

## Décision

```
pipeline/           binaire Rust — Cargo.toml, Cargo.lock versionné
web/                front Vite + React/TS — package.json, package-lock.json versionné
public/api/         artefacts produits par le pipeline, versionnés dans le dépôt
data/registre/      registre d'entités, corrigé à la main
data/preuves/       registre de preuves, JSONL en ajout seul
schemas/            JSON Schema des artefacts
docs/               documentation
```

`web/` déclare `publicDir: '../public'`. La construction du front recopie donc
`public/api/` dans `web/dist/` sans que les artefacts soient dupliqués dans le
dépôt, et `web/dist/` est ce que GitHub Pages publie.

**Les artefacts sont versionnés.** C'est inhabituel et c'est délibéré : ils sont
la sortie d'un calcul déterministe sur des sources horodatées, donc leur
historique git *est* l'historique des mesures. C'est aussi ce qui rend le jeu de
données utilisable seul, site éteint — exigence nommée dans la ROADMAP au titre
du risque d'abandon.

**Chaîne de version**, conforme aux règles du dépôt :

1. `release.yml`, déclenché à la main avec `patch` / `minor` / `major` : crée
   `chore/version-X.Y.Z` depuis `develop`, y écrit le numéro dans
   `pipeline/Cargo.toml` et `web/package.json`, met à jour `CHANGELOG.md` depuis
   les commits conventionnels, ouvre une PR vers `develop`.
2. Après fusion, une PR `develop` → `main` porte la montée de version.
3. `tag.yml`, sur poussée de `main` : lit le numéro, crée le tag `vX.Y.Z` et la
   publication GitHub si le tag n'existe pas. Idempotent.

Le numéro n'est jamais écrit à la main, et un tag n'est jamais créé sans passer
par `main`.

## Conséquences

- Deux chaînes d'outils, deux caches de CI. Coût accepté en 0001.
- `web/` échoue à la construction si un artefact ne respecte pas son schéma :
  c'est la vérification du contrat que 0001 déplace de la compilation vers la CI.
- Une seule vérification de statut est exigée par les protections de branches,
  `ci-ok`, qui agrège les autres. Ajouter un travail de CI ne demande donc pas de
  modifier la protection.
