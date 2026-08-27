# Registre des sources

Ce document est le **registre des sources** au sens de la définition de fini,
point 22, et de RG-104 : toute source ajoutée y porte son producteur, son URL
exacte, son format, sa licence avec sa version, sa fréquence de mise à jour, sa
date de vérification et son statut dans la v0.

Vérification réseau du **2026-08-27** : chaque URL de ce document a été
interrogée ce jour-là, et le code de réponse obtenu figure dans la colonne
« Vérifié ». Une URL non interrogée ne figure pas ici.

Périmètre de la v0 : [adr/0000-perimetre-brique0.md](adr/0000-perimetre-brique0.md)
§1. Conditions de réutilisation par source : [../LICENSE-DONNEES](../LICENSE-DONNEES).

---

## 1. Le registre

`Code` est la valeur de `entrees[].source` d'une ligne de preuve
([brique0/contrats.md](brique0/contrats.md) §2.1). Un tiret signifie que la
source n'alimente aucune entrée : elle est lue, pas citée.

### 1.1 Dans la v0

| Code | Producteur | URL | Format | Licence | Fréquence | Vérifié 2026-08-27 | Statut |
|---|---|---|---|---|---|---|---|
| `an_scrutins_17` | Assemblée nationale | `https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip` | ZIP de 8 434 fichiers JSON, un par scrutin ; 26 317 479 o → 181 Mio | Licence Ouverte / Open Licence **v1.0** | quotidienne, reconstruction nocturne | **200**, `last-modified` 2026-08-27 10:25:39 GMT | v0 (v0.1) |
| `an_organe` | Assemblée nationale | `https://data.assemblee-nationale.fr/static/openData/repository/17/amo/tous_acteurs_mandats_organes_xi_legislature/AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip` | ZIP de 13 991 fichiers JSON ; 3 119 acteurs, 10 813 organes, 59 déports ; 13 600 736 o | Licence Ouverte / Open Licence **v1.0** | quotidienne, reconstruction nocturne | **200**, `last-modified` 2026-08-27 00:34:47 GMT | v0 (v0.1, v0.4) |
| `ches_2024` | Chapel Hill Expert Survey | `https://github.com/chesdata/chesdata.github.io/releases/download/ches-europe/CHES_2024_final_v2.csv` | CSV, 279 lignes de données, 99 668 o | **Aucune licence publiée.** Réutilisation soumise à citation, condition obtenue par échange écrit le 2026-08-27 | par vague : 1999, 2002, 2006, 2007, 2010, 2014, 2019, 2024 | **200**, `last-modified` 2026-08-04 17:28:50 GMT, SHA-256 `1c1ec053…` conforme | v0 (v0.3) |
| `ches_trend` | Chapel Hill Expert Survey | `https://github.com/chesdata/chesdata.github.io/releases/download/ches-trend/1999-2024_CHES_dataset_meansV2.csv` | CSV, 1 441 lignes de données, 634 466 o | idem `ches_2024` | une révision par vague | **200**, `last-modified` 2026-08-04 17:23:11 GMT, SHA-256 `254384ab…` conforme | v0 (v0.3) |
| `nuance_leg2024` | Ministère de l'intérieur | `https://static.data.gouv.fr/resources/elections-legislatives-des-30-juin-et-7-juillet-2024-resultats-definitifs-du-1er-tour/20240710-171318/resultats-definitifs-par-regions.csv` | CSV, 18 lignes, 9 206 o ; 22 codes de nuance distincts | **Licence Ouverte v2.0** (`license = lov2` sur data.gouv.fr) | figée : un dépôt par scrutin | **200**, `last-modified` 2024-07-10 17:13:18 GMT, SHA-256 `b0c25687…` conforme | v0 (v0.3) |
| `registre_partis` | Contrepoint | `data/registre/partis.json` (fichier du dépôt) | JSON, 42 970 o, SHA-256 `186fc819…` | Licence Ouverte v2.0 ([../LICENSE-DONNEES](../LICENSE-DONNEES)) | à chaque correction du registre | non applicable — fichier local | v0 (v0.4) |

### 1.2 Lues, jamais citées — documentaire

| Rôle | URL | Format | Vérifié 2026-08-27 | Ce qu'on y lit |
|---|---|---|---|---|
| Fiche source des votes | `https://data.assemblee-nationale.fr/travaux-parlementaires/votes` | page HTML | **200** | le MD5 par ressource et le champ `date` — **ni l'un ni l'autre n'est une porte**, §3 |
| Texte de la licence amont | `https://data.assemblee-nationale.fr/content/download/28755/file/Licence_Ouverte.pdf` | PDF | **200** | « Cette licence est une version 1.0 de la Licence Ouverte ». La page HTML n'affiche pas le numéro ; le PDF, si |
| Codebook CHES 2024 | `https://github.com/chesdata/chesdata.github.io/releases/download/ches-europe/CHES.2024.Codebook.pdf` | PDF, 356 182 o | **200** | définition des variables, `country = 6` pour la France |
| Index des jeux CHES | `https://www.chesdata.eu/ches-europe/` | page HTML | **200** | les URL de dépôt et la ligne de citation exigée par jeu |
| Variante XML des scrutins | `https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.xml.zip` | ZIP XML, 30 795 168 o | **200** | même contenu que le JSON, arité portée par le schéma. Non récupérée : le pipeline lit le JSON |

### 1.3 Hors v0 — ça attend, ce n'est pas écarté

| Source | Producteur | URL | Format | Licence | Fréquence | Vérifié 2026-08-27 | Pourquoi dehors |
|---|---|---|---|---|---|---|---|
| ParlGov | ParlGov | `https://parlgov.org/data/parlgov-development_csv-utf-8/party.csv` | CSV, 1 770 lignes, 362 485 o ; `country_id = 43`, 80 partis français | `A VERIFIER` | `A VERIFIER` | **200**, `last-modified` 2025-09-20 20:06:22 GMT, SHA-256 `b8c6006c…` conforme | ADR 0000 §1 : apport marginal, aucun écran ne le consomme. Retenu comme **table de correspondance** `cmp` ↔ `chess` ([brique0/registre-entites.md](brique0/registre-entites.md) §2.7) |
| Manifesto Project (MARPOR) | Wissenschaftszentrum Berlin für Sozialforschung | `https://manifesto-project.wzb.eu/` | API JSON, clé requise | `A VERIFIER` | version annuelle `MPDS20XXa` | **200** sur la page ; l'API exige un compte et une clé | ADR 0000 §1 et §8 : inscription **manuelle**, donc contraire à la règle « rien qui exige une action manuelle récurrente » |
| Wikidata | Wikimedia | `https://query.wikidata.org/sparql` | SPARQL, JSON | CC0 1.0 — `A VERIFIER` sur le point d'accès lui-même | continue | **200** | ADR 0000 §1 : sert les briques 1 à 3, aucun usage en brique 0. Utilisé hors pipeline comme contrôle croisé d'identité, **jamais comme source de date** (§3) |
| Scrutins XVIe législature | Assemblée nationale | `https://data.assemblee-nationale.fr/static/openData/repository/16/loi/scrutins/Scrutins.json.zip` | ZIP JSON | Licence Ouverte v1.0 | quotidienne | **200** | ADR 0000 §1 et §3 : la v0 couvre la XVIIe seule |
| Archives XVe législature | Assemblée nationale | `https://data.assemblee-nationale.fr/archives-anterieures/archives-15e/scrutins` | page HTML d'index vers ZIP JSON / XML | Licence Ouverte v1.0 | figée | **200** sur l'index ; les archives elles-mêmes n'ont pas été téléchargées | idem |
| Données parlementaires CIVIX | CIVIX | `https://www.data.gouv.fr/datasets/donnees-parlementaires-francaises-votes-deputes-scrutins-civix` | jeu data.gouv.fr, votes et scrutins retraités | `A VERIFIER` | `A VERIFIER` | **200** sur la page du jeu | Retraitement tiers d'une source que le pipeline interroge en direct. Aucune évaluation n'a été conduite : `A VERIFIER` |

### 1.4 Écartées, avec le motif

| Source | URL interrogée | Mesure du 2026-08-27 | Motif |
|---|---|---|---|
| Grille de nuances, annexes Légifrance | `https://www.legifrance.gouv.fr/download/pdf/circ?id=45565` | **403** à toute requête non navigateur | Non récupérable par script. ADR 0000 §8 refuse une source exigeant une étape manuelle récurrente. Contournement retenu : les **codes attribués** dans les résultats définitifs, `nuance_leg2024` ci-dessus |
| NosDéputés.fr / Regards Citoyens | `https://www.nosdeputes.fr/deputes/json`, `https://www.nosdeputes.fr/17/scrutins/json` | `deputes/json` : **200**, 827 546 o de JSON valide. `17/scrutins/json` : **200**, 2 413 o de **HTML**, pas de JSON — corps identique à celui de la racine du site | L'API répond ; les scrutins de la XVIIe n'y sont pas exposés. Écartée pour absence de la donnée voulue, pas pour panne. L'Assemblée est interrogée en direct |

Le motif d'écart a changé trois fois dans la journée, la description aussi.
Première version : « API en erreur 500 ». Deuxième : « l'API répond, mais
`17/scrutins/json` rend un corps vide ». Mesure retenue, la seule vérifiée en
distinguant les deux points d'accès : `deputes/json` sert **827 546 octets de
JSON valide** — l'API fonctionne —, et `17/scrutins/json` sert **2 413 octets de
HTML**, pas du JSON. Les scrutins de la XVIIe n'y sont pas exposés, et c'est la
seule raison de l'écarter.

Vérification, les deux ensemble, sans quoi la conclusion se trompe de point
d'accès :

```sh
for u in deputes/json 17/scrutins/json; do
  curl -sS -o /dev/null -w "$u : %{http_code} %{size_download}o %{content_type}\n" \
    "https://www.nosdeputes.fr/$u"
done
```

---

## 2. Ce que le pipeline récupère réellement

`scripts/recuperer-sources.sh` porte deux entrées, et deux seulement :
`an_scrutins_17` et `an_organe`. Toute autre source de ce registre est lue hors
de ce script. Une URL de ce tableau n'est **jamais construite par
concaténation**.

Mesures de la première exécution réelle du script
([brique0/verification-2026-08-27.md](brique0/verification-2026-08-27.md) §0) :

| Source | Octets annoncés et reçus | Empreinte d'archive | Empreinte de contenu | Fichiers | MD5 relevé |
|---|---|---|---|---|---|
| Scrutins | 26 317 479 | `aa767a2a…` | `c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c` | 8 434 | `910e6022c9eba71f932df42267778c46` |
| AMO30 | 13 600 736 | `bbecd012…` | `0f49c00a8227d6cb8e658d374bacfec35238fe4e2dd6305f7df6ac4f515c5de6` | 13 991 | `4d74d2b6179eb4879d0aa31afd1b2f97`, calculé localement — AMO30 ne publie aucun MD5 |

---

## 3. Les cinq pièges du registre

Ce sont des faits mesurés le 2026-08-27, pas des précautions. Ils sont ici parce
que c'est ici qu'on les cherche.

**1. La source sert plusieurs constructions du même contenu.** Trois
récupérations de `Scrutins.json.zip` le même jour ont rendu **deux archives
d'octets différents** — `aa767a2a…`/MD5 `910e6022…` et `c5e405f1…`/MD5
`1f951dea…` — pour une taille identique à l'octet, `diff -rq` vide sur les
8 434 fichiers, et la **même** empreinte de contenu `c8457f34…`. Le
`last-modified` ne suit ni l'une ni l'autre : l'exécution du script a reçu
`aa767a2a…` sous `last-modified` 10:25:39 GMT, là où le protocole de mesure
l'avait vue sous 04:25:40 GMT. Signature d'un répartiteur devant plusieurs
serveurs.

> **Conséquence.** L'empreinte d'archive et le MD5 du producteur sont
> **documentaires**. La porte est la **taille annoncée**, puis l'**empreinte de
> contenu** ([brique0/contrats.md](brique0/contrats.md) §2.8, RG-115). Un
> pipeline qui déciderait d'une ré-émission sur l'empreinte d'archive
> réécrirait le registre de preuves entier à chaque exécution, au hasard du
> serveur qui répond.
>
> L'empreinte de contenu se calcule sous `LC_ALL=C` : sans lui, la même archive
> rend `503255ac…` au lieu de `c8457f34…`, parce que `VTANR5L17V1.json`,
> `VTANR5L17V10.json` et `VTANR5L17V100.json` ne s'ordonnent pas de la même
> façon sous `fr_FR.UTF-8`.

**2. Le serveur tronque sans erreur.** Il ferme la connexion en cours de
transfert sans code d'erreur (ADR 0001 §1.5). La reprise `curl -C -` jusqu'à la
taille annoncée est **obligatoire** — huit tentatives par défaut dans
`scripts/recuperer-sources.sh` — même quand elle n'est pas exercée : le
2026-08-27, les deux archives sont arrivées complètes du premier coup.

**3. Le bon référentiel est celui qui est reconstruit quotidiennement.** AMO30
est reconstruit chaque nuit ; **AMO50 est figé** au `last-modified` 2024-07-11
et **ne contient pas** `PO845401`, le groupe RN, pourtant référencé par les
scrutins de 2025. Un instantané pris à l'ouverture de la législature ne résout
pas les groupes créés après elle.

**4. Une fiche de source peut être mal libellée.** La fiche du jeu 17 décrit son
contenu comme « les scrutins AN […] pour la XV législature ». Le libellé est
faux. Autoritatifs : le chemin `/repository/17/` et le champ
`scrutin.legislature = "17"` de chaque fichier. Ne jamais dériver un numéro de
législature d'un libellé humain. Le même piège vaut pour le chemin AMO30, qui
porte `..._xi_legislature/` et sert bien la XVIIe.

**5. Une source sans licence publiée ne voit pas ses valeurs dérivées
publiées.** CHES est dans ce cas : vérifié le 2026-08-27, aucune licence n'y est
publiée. La condition obtenue par échange écrit le 2026-08-27 est **étroite** —
réutilisation soumise à citation. Ce n'est **ni** une autorisation de
republication, **ni** une cession de droits. La distinction décide de deux
choses :

| Cas | Décision (ADR 0000 §8, RG-118) |
|---|---|
| Licence autorisant explicitement la redistribution — archives de l'Assemblée, Licence Ouverte v1.0 | La copie est conservée et déposée comme asset de release, indexée par son SHA-256, avec la mention `Assemblée nationale — Licence Ouverte v1.0 — données du <last-modified>` |
| Sans licence publiée, ou licence n'autorisant pas la redistribution — **CHES** | Le dépôt distribue le script, l'URL, la date et les empreintes. **Jamais la copie** |

Les valeurs dérivées de CHES sont publiées, parce que la condition écrite existe
et qu'elle est portée par la donnée : `entrees[].citation`, invariant I23. Une
source qui n'aurait ni licence ni condition écrite ne verrait rien publié.

Le nuancier échappe à la difficulté par le contournement du §1.4 : les codes
attribués sont dans un fichier `lov2`, la grille en annexe PDF ne l'est pas.

---

## 4. Citations exigées

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

Aucune autre source de ce registre n'énonce d'exigence de citation à la date du
2026-08-27. Ce constat se revérifie à chaque ajout de source (RG-104).

`A VERIFIER` : les citations des vagues CHES antérieures à 2024, qui sont hors
périmètre de la brique 0. Vérification : relever la ligne « please cite » de la
vague concernée sur `https://www.chesdata.eu/ches-europe/` le jour où elle entre
dans le périmètre.

---

## 5. Producteurs et dates de dernière mise à jour

`entrees[].producteur` porte le nom que la source publie, jamais le code de
`source` (RG-76). `derniere_mise_a_jour` est la date de la **source**, pas celle
de la récupération.

| `source` | `producteur` | `derniere_mise_a_jour` relevée le 2026-08-27 | D'où elle vient |
|---|---|---|---|
| `an_scrutins_17`, `an_organe` | `Assemblée nationale` | `2026-08-27` | `last-modified` de l'archive |
| `ches_2024`, `ches_trend` | `Chapel Hill Expert Survey` | `2026-08-04` | date de dépôt de la ressource dans la version `ches-europe` / `ches-trend` (`updated_at` de l'API GitHub Releases) |
| `nuance_leg2024` | `Ministère de l'intérieur` | `2024-07-10` | `last-modified` de la ressource, et organisation déclarée sur data.gouv.fr |
| `registre_partis` | `Contrepoint` | `2026-08-27` | `git log -1 --format=%cs -- data/registre/partis.json` |

---

## 6. Cadre du nuançage — à conserver avec la donnée

Le nuançage n'est pas une mesure scientifique et n'est pas stocké comme telle.

- Décret **2014-1479 du 2014-12-09** : distingue l'**étiquette**, choisie par le
  candidat, de la **nuance**, attribuée par l'administration.
- Instruction **`IOMA2415630C`** du 2024-06-11, législatives des 30 juin et
  7 juillet 2024. Circulaire **`INTP2602966C`** du 2026-02-02, municipales des
  15 et 22 mars 2026. Grilles en annexe PDF, 2 annexes pour 2024, 4 pour 2026 —
  et les téléchargements Légifrance répondent 403 (§1.4).
- La circulaire de 2026 rattache LFI à la nuance **extrême gauche** — écart avec
  2020, où elle relevait du bloc « gauche » — et l'UDR à **extrême droite**.
- Recours de LFI, de l'UDR et d'Éric Ciotti devant le **Conseil d'État**.
  Décisions du **2026-02-27**, n° 512694, 512695, 512981 et 512983 : recours
  **rejetés**. Les deux rattachements ne sont pas entachés d'erreur manifeste
  d'appréciation ; le seuil de nuançage à 3 500 habitants est validé.
- Séquence identique en 2023 concernant le RN.

Trois faits qui commandent le modèle de données
([brique0/registre-entites.md](brique0/registre-entites.md) §2.5) :

- **La nuance est attribuée à un candidat, pas à un parti.** Un appariement
  parti → code est une déclaration du registre, pas une lecture de la source.
- **`UG` et `ENS` sont des codes de coalition** — `UG` couvre indistinctement
  LFI, PS, écologistes et PCF ; `ENS` couvre RE, MoDem et Horizons. Ils sont
  portés par `coalition.nfp` et `coalition.ensemble`, jamais par un parti.
- **Aucune colonne nominative n'est ingérée** (RG-111). `Nuance candidat n`,
  `Nom candidat n` et `Elu n` du fichier par circonscription restent hors
  périmètre : seul l'ensemble des 22 codes distincts est utilisé.

C'est précisément ce genre de rattachement révisé sans changement de
comportement de vote que l'outil rend visible.

`A VERIFIER` : les libellés officiels des 22 codes. Vérification : ouvrir
`https://www.legifrance.gouv.fr/download/pdf/circ?id=45565` dans un navigateur
et transcrire l'annexe. En attendant, `ECO` et `VEC` ne sont pas appariés.

---

## 7. Briques 1 à 3 — prospection, aucune source arrêtée

Aucune de ces sources n'est dans le registre : aucune URL n'a été arrêtée,
aucune licence n'a été lue. Une ligne y entrera par RG-104 le jour où une source
précise sera retenue.

| Brique | Piste | Ce qui reste à mesurer avant de coder |
|---|---|---|
| 1 — presse écrite | flux RSS des rédactions françaises à flux exploitable, un seul type de parser | quels flux servent un chapô utile et lesquels ne servent que le titre ; quels éditeurs déclarent un opt-out de fouille de textes |
| 2 — YouTube | flux RSS de chaîne, gratuits et sans clé ; sous-titres automatiques pour le contenu | disponibilité et qualité des sous-titres |
| 3 — TV / radio | pages d'actualité écrites des rédactions audiovisuelles | coût de maintenance de parsers HTML dédiés — raison pour laquelle cette brique vient en dernier |

### Pistes non retenues

| Source | Note |
|---|---|
| GDELT 2.0 | Gratuit, couvre les médias français, fournit déjà tonalité et thèmes. Raccourci réel, mais méthode opaque : incompatible avec l'exigence de reproductibilité de bout en bout. À reconsidérer comme source de recoupement, jamais comme source primaire |
| Media Cloud | API gratuite, collections France. À évaluer pour élargir le corpus au-delà du RSS |
| Europresse et équivalents | Payant. Hors budget |
| Réseaux sociaux | Pas d'accès gratuit stable |

---

## 8. Comment revérifier ce document

```sh
# codes de réponse de toutes les URL du registre
grep -ohE 'https?://[^ `)]+' docs/sources.md | sort -u |
  while read -r u; do printf '%s %s\n' "$(curl -sS -o /dev/null -w '%{http_code}' -IL "$u")" "$u"; done

# taille et date annoncées par l'Assemblée
curl -sI https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip |
  grep -iE 'content-length|last-modified'

# la version de la licence amont n'est que dans le PDF
curl -sL https://data.assemblee-nationale.fr/content/download/28755/file/Licence_Ouverte.pdf |
  pdftotext - - | grep -i 'version 1.0'

# empreintes de contenu, méthode de contrats.md §2.8
./scripts/recuperer-sources.sh
```
