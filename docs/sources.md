# Sources de données

État vérifié le **2026-08-27**. Toutes gratuites.

---

## Brique 0 — acteurs

| Source | Contenu | Format | État au 2026-08-27 |
|---|---|---|---|
| [AN open data — votes](https://data.assemblee-nationale.fr/travaux-parlementaires/votes) | Scrutins nominatifs, position de chaque député | JSON, XML — licence Etalab | ✅ répond 200 |
| [AN open data — acteurs, mandats, organes (AMO30)](https://data.assemblee-nationale.fr/travaux-parlementaires/votes) | Référentiel des acteurs, de leurs mandats et des organes, avec périodes de validité. Sans lui, aucun groupe n'est résoluble. **AMO50 ne convient pas** : figé au `last-modified` 2024-07-11, il ne contient pas le groupe RN référencé par les scrutins de 2025 | ZIP JSON — Licence Ouverte v1.0 | ✅ `last-modified` 2026-08-27, mise à jour quotidienne |
| [AN open data — archives 15e législature](https://data.assemblee-nationale.fr/archives-anterieures/archives-15e/scrutins) | Scrutins des législatures antérieures | JSON, XML | ✅ à confirmer par téléchargement |
| [CHES — Europe](https://www.chesdata.eu/ches-europe/) | 279 partis, 31 pays, 609 politologues. Axes gauche-droite, économique, sociétal, UE. **Vagues 1999 → 2024** | CSV servi par GitHub Releases + codebook PDF | ✅ 200 le 2026-08-27. L'ancienne URL `…/2024-chapel-hill-expert-survey-ches` renvoie **404**. **Aucune licence publiée** ; seule condition : citation Rovny et al. 2025, *Electoral Studies* 97. Publication des valeurs dérivées suspendue, voir LICENSE-DONNEES |
| Manifesto Project (MARPOR) | Score RILE codé sur les programmes électoraux | inscription gratuite requise | à vérifier |
| ParlGov | Partis, élections, gouvernements. Référencé par le codebook CHES | à vérifier | à vérifier |
| Nuancier ministère de l'Intérieur | Classification administrative des candidats | circulaires + résultats sur data.gouv.fr | à vérifier |
| [Données parlementaires françaises — CIVIX](https://www.data.gouv.fr/datasets/donnees-parlementaires-francaises-votes-deputes-scrutins-civix) | Votes, députés, scrutins retraités | data.gouv.fr | alternative à évaluer |
| Wikidata | Propriété capitalistique des médias, appartenances partisanes | SPARQL | à vérifier |

### Écartées

| Source | Raison |
|---|---|
| **NosDéputés.fr / Regards Citoyens** | L'API fonctionne — `/deputes/json` répond 200 avec du JSON valide ; seule la page d'accueil renvoie 500. La raison de l'écart est autre : `/17/scrutins/json` répond 200 avec un corps vide, les scrutins de la XVIIe législature n'y sont pas exposés. Écartée pour cette raison, pas pour une panne. L'AN est interrogée en direct. |

---

## Cadre du nuancier — à consigner avec la donnée

Le nuançage n'est pas une mesure scientifique et n'est pas stocké comme telle.
Éléments à conserver avec chaque valeur :

- Décret **2014-1479 du 9 décembre 2014** : distingue l'**étiquette** (choisie par le candidat) de la **nuance** (attribuée par l'administration).
- Circulaire du **2 février 2026** (municipales) : LFI classée **extrême gauche** — changement par rapport à 2020 où elle relevait du bloc « gauche » — et l'UDR **extrême droite**.
- Recours de LFI, de l'UDR et d'Éric Ciotti devant le **Conseil d'État**.
- Décisions du **2026-02-27**, n° 512694, 512695, 512981 et 512983 : recours **rejetés**. Le classement de LFI en extrême gauche et de l'UDR en extrême droite n'est pas entaché d'erreur manifeste d'appréciation ; le seuil de nuançage à 3 500 habitants est validé.
- Séquence identique en 2023 concernant le RN.

C'est précisément le genre de reclassement sans changement de comportement de
vote que l'outil doit rendre visible.

---

## Brique 1 — presse écrite

Corpus à constituer : toute rédaction française disposant d'un flux RSS
exploitable, sur l'ensemble du spectre. Un seul type de parser, aucun cas
particulier.

Contraintes à mesurer avant de coder :
- quels flux servent un chapô utile et lesquels ne servent que le titre ;
- quels éditeurs déclarent un opt-out de fouille de textes.

## Brique 2 — YouTube

Flux RSS de chaîne (gratuits, stables, sans clé). Sous-titres automatiques pour
le contenu : disponibilité et qualité à mesurer avant de construire dessus.

## Brique 3 — TV / radio

Pages d'actualité écrites des rédactions audiovisuelles. Parsers HTML dédiés,
donc coût de maintenance récurrent — raison pour laquelle cette brique vient en
dernier.

---

## Pistes non retenues pour l'instant

| Source | Note |
|---|---|
| GDELT 2.0 | Gratuit, couvre les médias français, fournit déjà tonalité et thèmes. Raccourci réel, mais méthode opaque : incompatible avec l'exigence de reproductibilité de bout en bout. À reconsidérer comme source de recoupement, jamais comme source primaire. |
| Media Cloud | API gratuite, collections France. À évaluer pour élargir le corpus au-delà du RSS. |
| Europresse et équivalents | Payant. Hors budget. |
| Réseaux sociaux | Pas d'accès gratuit stable. |
