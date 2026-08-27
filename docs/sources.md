# Sources de données

État vérifié le **2026-08-27**. Toutes gratuites.

---

## Brique 0 — acteurs

| Source | Contenu | Format | État au 2026-08-27 |
|---|---|---|---|
| [AN open data — votes](https://data.assemblee-nationale.fr/travaux-parlementaires/votes) | Scrutins nominatifs, position de chaque député | JSON, XML — licence Etalab | ✅ répond 200. Producteur : **Assemblée nationale**. Archive `Scrutins.json.zip` le 2026-08-27 : 26 317 479 octets, `last-modified` 2026-08-27, empreinte d'archive `aa767a2a05f25e38badca738af3535cb9ab89b5fa95d0810a60af05eab1e4721`, empreinte de contenu `c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c` sur 8 434 fichiers (méthode : contrats.md §2.8). La valeur `c5e405f1…` consignée jusqu'ici est celle de l'autre construction, et elle est exacte pour elle. **La source sert deux constructions du même contenu** : `aa767a2a…`/`910e6022…` et `c5e405f1…`/`1f951dea…`, taille identique, `diff -rq` vide, même empreinte de contenu. L'empreinte d'archive et le MD5 du producteur sont documentaires ; la porte est la taille puis l'empreinte de contenu |
| [AN open data — acteurs, mandats, organes (AMO30)](https://data.assemblee-nationale.fr/travaux-parlementaires/votes) | Référentiel des acteurs, de leurs mandats et des organes, avec périodes de validité. Sans lui, aucun groupe n'est résoluble. **AMO50 ne convient pas** : figé au `last-modified` 2024-07-11, il ne contient pas le groupe RN référencé par les scrutins de 2025 | ZIP JSON — Licence Ouverte v1.0 | ✅ `last-modified` 2026-08-27 00:34:47 GMT, mise à jour quotidienne. Producteur : **Assemblée nationale**. 13 600 736 octets, empreinte d'archive `bbecd012…`, empreinte de contenu `0f49c00a8227d6cb8e658d374bacfec35238fe4e2dd6305f7df6ac4f515c5de6` sur 13 991 fichiers. Aucun MD5 publié sur la fiche |
| [AN open data — archives 15e législature](https://data.assemblee-nationale.fr/archives-anterieures/archives-15e/scrutins) | Scrutins des législatures antérieures | JSON, XML | ✅ à confirmer par téléchargement |
| [CHES — Europe](https://www.chesdata.eu/ches-europe/) | 279 partis, 31 pays, 609 politologues. Axes gauche-droite, économique, sociétal, UE. **Vagues 1999 → 2024** | CSV servi par GitHub Releases + codebook PDF | ✅ 200 le 2026-08-27. L'ancienne URL `…/2024-chapel-hill-expert-survey-ches` renvoie **404**. **Aucune licence publiée** ; **réutilisation soumise à citation, condition obtenue par échange écrit le 2026-08-27** — ni autorisation de republication, ni cession de droits. Publication des valeurs dérivées non suspendue ; la citation est portée par `entrees[].citation` de chaque ligne de preuve. Voir LICENSE-DONNEES |
| Manifesto Project (MARPOR) | Score RILE codé sur les programmes électoraux | inscription gratuite requise | à vérifier |
| ParlGov | Partis, élections, gouvernements. Référencé par le codebook CHES | à vérifier | à vérifier |
| Nuancier ministère de l'Intérieur | Classification administrative des candidats | circulaires + résultats sur data.gouv.fr | à vérifier |
| [Données parlementaires françaises — CIVIX](https://www.data.gouv.fr/datasets/donnees-parlementaires-francaises-votes-deputes-scrutins-civix) | Votes, députés, scrutins retraités | data.gouv.fr | alternative à évaluer |
| Wikidata | Propriété capitalistique des médias, appartenances partisanes | SPARQL | à vérifier |

### Sources exigeant une citation

Liste opposable : l'invariant I23 de [brique0/contrats.md](brique0/contrats.md)
§6 s'y adosse. Une entrée dont la `source` y figure porte `citation` non nulle et
identique caractère pour caractère ; toute autre entrée porte `null`.

| `source` | Citation exigée, mot pour mot | Vérifié le | Où |
|---|---|---|---|
| `ches_2024` | `Rovny, Jan, Jonathan Polk, Ryan Bakker, Liesbet Hooghe, Seth Jolly, Gary Marks, Marco Steenbergen, and Milada Anna Vachudova. 2025. "The 2024 Chapel Hill Expert Survey on political party positioning in Europe: Twenty-five years of party positional data." Electoral Studies 97 (October). doi:10.1016/j.electstud.2025.102981` | 2026-08-27 | `chesdata.eu/ches-europe/`, « When using the 2024 survey, please cite » |
| `ches_trend` | la même chaîne, mot pour mot | 2026-08-27 | `chesdata.eu/ches-europe/`, « When using the 1999–2024 trend file, please cite » |

Les deux jeux portent la même citation aujourd'hui, et ce n'est pas une raison
d'en faire une mention globale : chaque jeu CHES publie sa propre ligne — 2019,
2014, 2010, 2007, 2006, 2002 et 1999 en ont chacune une, différente — et un jeu
futur portera la sienne. La citation est une propriété de l'entrée.

`A VERIFIER` : les vagues CHES antérieures à 2024 ne sont pas dans le périmètre
de la brique 0 ; leurs citations sont sur la même page et se relèvent le jour où
une vague y entre. Aucune autre source du tableau ci-dessus n'énonce d'exigence
de citation à la date du 2026-08-27, ce qui reste à revérifier à chaque ajout de
source (RG-104).

### Producteurs, tels que les sources les publient

`entrees[].producteur` porte ces noms et jamais le code de `source` (RG-76).

| `source` | `producteur` | `derniere_mise_a_jour` relevée le 2026-08-27 | Comment |
|---|---|---|---|
| `an_scrutins_17`, `an_organe` | `Assemblée nationale` | `2026-08-27` | `last-modified` de l'archive |
| `ches_2024` | `Chapel Hill Expert Survey` | `2026-08-04` | date de dépôt de la ressource dans la version `ches-europe` (`updated_at` de l'API GitHub Releases) |
| `nuance_leg2024` | `Ministère de l'intérieur` | `2024-07-10` | `last-modified` de la ressource, et organisation déclarée sur data.gouv.fr |
| `registre_partis` | `Contrepoint` | `A VERIFIER` — `2026-08-27` est la date de `partis.exemple.json` ; celle du registre réel viendra avec lui | `git log -1 --format=%cs -- data/registre/partis.json` |

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
