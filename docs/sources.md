# Sources de données

État vérifié le **2026-08-27**. Toutes gratuites.

---

## Brique 0 — acteurs

| Source | Contenu | Format | État au 2026-08-27 |
|---|---|---|---|
| [AN open data — votes](https://data.assemblee-nationale.fr/travaux-parlementaires/votes) | Scrutins nominatifs, position de chaque député | JSON, XML — licence Etalab | ✅ répond 200 |
| [AN open data — archives 15e législature](https://data.assemblee-nationale.fr/archives-anterieures/archives-15e/scrutins) | Scrutins des législatures antérieures | JSON, XML | ✅ à confirmer par téléchargement |
| [CHES 2024](https://www.chesdata.eu/2024-chapel-hill-expert-survey-ches) | 279 partis, 31 pays, 609 politologues. Axes gauche-droite, économique, sociétal, UE. **Vagues 1999 → 2024** | téléchargement libre + codebook | ✅ documenté |
| Manifesto Project (MARPOR) | Score RILE codé sur les programmes électoraux | inscription gratuite requise | à vérifier |
| ParlGov | Partis, élections, gouvernements. Référencé par le codebook CHES | à vérifier | à vérifier |
| Nuancier ministère de l'Intérieur | Classification administrative des candidats | circulaires + résultats sur data.gouv.fr | à vérifier |
| [Données parlementaires françaises — CIVIX](https://www.data.gouv.fr/datasets/donnees-parlementaires-francaises-votes-deputes-scrutins-civix) | Votes, députés, scrutins retraités | data.gouv.fr | alternative à évaluer |
| Wikidata | Propriété capitalistique des médias, appartenances partisanes | SPARQL | à vérifier |

### Écartées

| Source | Raison |
|---|---|
| **NosDéputés.fr / Regards Citoyens** | API en erreur 500 le 2026-08-27 (`/api`, `/synthese/data/json`, `/deputes/enmandat/json`). Service dégradé — ne pas construire dessus. L'AN est interrogée en direct. À reconsidérer si le service revient, la donnée y est plus commode. |

---

## Cadre du nuancier — à consigner avec la donnée

Le nuançage n'est pas une mesure scientifique et n'est pas stocké comme telle.
Éléments à conserver avec chaque valeur :

- Décret **2014-1479 du 9 décembre 2014** : distingue l'**étiquette** (choisie par le candidat) de la **nuance** (attribuée par l'administration).
- Circulaire du **2 février 2026** (municipales) : LFI classée **extrême gauche** — changement par rapport à 2020 où elle relevait du bloc « gauche » — et l'UDR **extrême droite**.
- Recours de LFI et de l'UDR devant le **Conseil d'État**.
- Au **3 mars 2026** : le Conseil d'État n'a censuré ni le classement du RN, ni celui de LFI, ni celui de l'UDR dans les extrêmes.
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
