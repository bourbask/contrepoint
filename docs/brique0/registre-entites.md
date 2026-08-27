# Registre d'entités — partis

Roadmap v0.4. Contrat des briques 1 à 3. Une erreur d'appariement ici se
propage dans toutes les mesures : c'est le seul fichier du projet où la
relecture humaine ligne par ligne est obligatoire (docs/definition-of-done.md,
point 17).

Sources vérifiées le **2026-08-27**. Schéma formel :
[`schemas/registre-partis-1.schema.json`](../../schemas/registre-partis-1.schema.json)
— transcription de la liste blanche du §3, c'est-à-dire de la règle V1. Fichier
réel lu par le pipeline :
[`data/registre/partis.json`](../../data/registre/partis.json). Extrait de
référence, fixture du test REG-01 :
[`data/registre/partis.json`](../../data/registre/partis.json). La fixture de test qui lui correspond vit avec les autres, dans [`echantillons/registre-l17.json`](echantillons/registre-l17.json).

**Les deux fichiers sont identiques octet pour octet au 2026-08-27**, et
`186fc8195d357d08c65ba0bceb45da90a0132df3bd859e039e7303eee96a9680` est leur
empreinte commune, pour 42 970 octets. L'extrait couvre la XVIIe législature en
entier — 14 groupes, 12 entités, 9 sources — il n'y avait donc rien à ajouter
Le fichier réel est `data/registre/partis.json`. Il n'existe qu'un seul registre sous `data/` : la fixture de test est un fichier distinct, sous `echantillons/`, et rien ne se recopie à la main.

---

## 1. Ce que le registre est, et ce qu'il n'est pas

Le registre est une **table d'identifiants**. Il ne porte aucune valeur mesurée,
aucun axe, aucun score, aucune date de calcul. Les positions vivent dans le
registre de preuves (v0.5), qui référence les entités du registre par leur `id`.

Il ne contient **aucune personne**. Le rattachement d'un député à un groupe est
relu à chaque exécution depuis AMO30 : 633 députés sur 648 relèvent de plus d'un
groupe sur la XVIIe législature (ADR 0001, mesure 12), une table tenue à la main
serait périmée en permanence et fausse en silence.

Trois natures d'objets, jamais confondues :

| Objet | Ce que c'est | Exemple XVIIe |
|---|---|---|
| **Parti** | Personne morale durable, existant hors du Parlement | Les Républicains |
| **Coalition** | Alliance électorale datée, sans existence parlementaire | Nouveau Front populaire |
| **Groupe parlementaire** | Organe d'une chambre pour une législature, avec un `uid` de la source | `PO845425` Droite Républicaine |

---

## 2. Les sources, et l'identifiant réel de chacune

Toutes les valeurs de cette section ont été lues dans la donnée ou dans sa
documentation le 2026-08-27, pas déduites.

### 2.1 Assemblée nationale — groupes parlementaires

Référentiel : `AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip`
(13 600 736 octets, SHA-256 `bbecd012…663061`). Les objets `organe` de
`codeType = "GP"` et `legislature = "17"` donnent **14 groupes**, bornés par
`viMoDe.dateDebut` / `viMoDe.dateFin` :

| `uid` | `libelleAbrev` | `libelle` | début | fin |
|---|---|---|---|---|
| `PO840056` | NI | Non inscrit | 2024-07-01 | — |
| `PO845401` | RN | Rassemblement National | 2024-07-18 | — |
| `PO845407` | EPR | Ensemble pour la République | 2024-07-18 | — |
| `PO845413` | LFI-NFP | La France insoumise - Nouveau Front Populaire | 2024-07-18 | — |
| `PO845419` | SOC | Socialistes et apparentés | 2024-07-18 | — |
| `PO845425` | DR | Droite Républicaine | 2024-07-18 | — |
| `PO845439` | ECOS | Écologiste et Social | 2024-07-18 | — |
| `PO845454` | DEM | Les Démocrates | 2024-07-18 | — |
| `PO845470` | HOR | Horizons & Indépendants | 2024-07-18 | — |
| `PO845485` | LIOT | Libertés, Indépendants, Outre-mer et Territoires | 2024-07-18 | — |
| `PO845514` | GDR | Gauche Démocrate et Républicaine | 2024-07-18 | — |
| `PO845520` | AD | À Droite | 2024-07-18 | 2024-09-11 |
| `PO847173` | UDR | UDR | 2024-09-12 | 2025-09-04 |
| `PO872880` | UDDPLR | Union des droites pour la République | 2025-09-05 | — |

Pièges relevés dans la source, tous vérifiés :

- **`organePrecedentRef` est nul sur les 63 GP et les 58 PARPOL.** La source ne
  déclare aucune succession. La chaîne AD → UDR → UDDPLR n'existe nulle part
  dans la donnée : elle est déclarée à la main dans le registre, ou elle
  n'existe pas.
- **`libelleAbrev` ≠ `libelleAbrege` sur `PO872880`** : `UDDPLR` d'un côté,
  `UDR` de l'autre. Aucune clé ne se fait sur une abréviation.
- **`NI` n'est pas un groupe** : c'est l'agrégat des députés sans groupe. Il a
  un `uid` et des dates comme les autres, et son écart-type intra est de 0,649
  contre 0,065 pour le RN (ADR 0001, §1.3). Ne jamais l'agréger comme un parti.
- **AMO50 est inutilisable** : figé au 2024-07-11, il ne contient pas
  `PO845401` (groupe RN) pourtant référencé par les scrutins de 2025.

### 2.2 Assemblée nationale — partis

`codeType = "PARPOL"` : **58 organes**, dont `PO744856` La France Insoumise,
`PO684932` Parti socialiste, `PO710396` Les Républicains, `PO761239`
Rassemblement national, `PO852530` Les Écologistes - EELV, `PO852528` Horizons,
`PO852532` UDR - Union des Droites pour la République.

**Ce n'est pas un référentiel d'adhésion.** Les mandats `typeOrgane = "PARPOL"`
portent des bornes du type 2012-12-01, 2022-12-02, 2025-12-03 : c'est une
déclaration annuelle de rattachement au titre du financement de la vie
politique. Constat mesuré sur les membres en cours au 2026-08-27 :

| Groupe | Partis déclarés par ses membres |
|---|---|
| LFI-NFP | FI 67, REZRE 2, PEYIA 1, aucune déclaration 1 |
| SOC | PS 62, PPROG 2, RPS 2, AAREU 1, aucune déclaration 1 |
| ECOS | ECOLO 33, **PCF 5** |
| GDR | **PCF 9**, PLR 2, MDES 2, P974 1, NR 1, THN 1 |
| DR | REP 40, NR 3, aucune déclaration 5 |
| RN | RN 117, aucune déclaration 5 |
| HOR | HOR 30, UDIE 1, aucune déclaration 5 |
| UDDPLR | UDR 16, aucune déclaration 1 |
| **EPR** | **ESBMP 77**, TPH 1, REP 1, ALCEN 1, GENNC 1, aucune déclaration 9 |
| **DEM** | **ESBMP 34**, aucune déclaration 3 |
| LIOT | RPS 12, UDIE 6, AHIP 1, ARCDE 1, NR 1, aucune déclaration 1 |

Deux conséquences dures. **`ESBMP`** (`PO833003`, « Ensemble ! (majorité
présidentielle) ») est un véhicule de financement : 111 députés EPR et DEM s'y
rattachent, **aucun ne déclare Renaissance**, et la source ne distingue pas EPR
de DEM. La composition de ces deux groupes est laissée vide dans le registre,
avec son motif. **Le PCF apparaît dans deux groupes** — ECOS et GDR : un parti
n'appartient pas à un groupe, il y est représenté.

Autre piège : une vingtaine de PARPOL portent la même `dateFin` 2025-12-03
(`PO684938` MODEM, `PO684936` EELV, `PO761294` LAREM…). C'est une maintenance du
référentiel, pas une dissolution — le MoDem existe en 2026. Une `dateFin` de
PARPOL n'éteint pas un parti.

### 2.3 CHES

Le chemin de `docs/sources.md` ne répond plus : `chesdata.eu/2024-chapel-hill-expert-survey-ches`
renvoie 404 au 2026-08-27. Les fichiers sont désormais servis par GitHub
Releases, listés depuis `https://www.chesdata.eu/ches-europe/` :

- vague 2024 : `.../releases/download/ches-europe/CHES_2024_final_v2.csv`
  (279 lignes, SHA-256 `1c1ec053…c6a8`) ;
- trend 1999-2024 : `.../releases/download/ches-trend/1999-2024_CHES_dataset_meansV2.csv`
  (SHA-256 `254384ab…4033`) ;
- codebook : `.../releases/download/ches-europe/CHES.2024.Codebook.pdf`.

France = `country` **6**. Les 10 partis de la vague 2024, `party_id` réel :

| `party_id` | `party` | `family` | `lrgen` | `seat` |
|---|---|---|---|---|
| 627 | FI | 6 | 0,82 | 79 |
| 601 | PCF | 6 | 1,73 | 9 |
| 605 | LE/EELV | 7 | 2,30 | 34 |
| 602 | PS | 5 | 3,45 | 70 |
| 613 | MoDem | 3 | 5,36 | 33 |
| 626 | RE | 3 | 6,27 | 98 |
| 631 | Horizons | 3 | 6,60 | 26 |
| 609 | LR | 2 | 7,73 | 66 |
| 610 | RN | 1 | 8,82 | 142 |
| 630 | REC | 1 | 9,73 | 0 |

**Le `party_id` est conservé à travers les changements de nom**, ce qui en fait la clé
externe du registre :

| `party_id` | Libellés successifs dans le fichier de tendance |
|---|---|
| 605 | VERTS (1999→2010), EELV (2014, 2019), LE/EELV (2024) |
| 609 | RPR (1999, 2002), UMP (2006→2014), LR (2019, 2024) |
| 610 | FN (1999→2014), RN (2019, 2024) |
| 613 | UDF (1999→2006), MODEM (2010, 2014), MoDem (2019, 2024) |
| 626 | LREM (2019), RE (2024) |

Absents de CHES 2024 : UDR, LIOT, Place publique, Génération.s, LO, NPA.
Corollaire opérationnel : `libelle_source` est stocké à titre de trace, jamais
comme clé de jointure.

### 2.4 Manifesto Project

L'accès reste **hors v0** (ADR 0000 : inscription manuelle). Vérifié : la page
API exige « *Manifesto Project Database Account* » puis « *API Key* » générée
depuis le profil, avec un quota journalier ; seules `list_core_versions`,
`list_metadata_versions`, `get_core_codebook` et `get_core_citation` sont
accessibles sans clé — et `get_core_codebook` renvoie le schéma de codage des
quasi-phrases, pas la liste des partis.

Les codes CMP inscrits dans l'extrait viennent donc de **deux sources tierces
concordantes** : la colonne `cmp_id` du fichier de tendance CHES et la colonne `cmp` de
`party.csv` de ParlGov.

| Parti | CMP | Concordance CHES / ParlGov |
|---|---|---|
| Les Écologistes – EELV | 31110 | oui |
| PCF | 31220 | oui |
| PS | 31320 | oui |
| MoDem | 31624 | oui |
| LR | 31626 | oui |
| RN | 31720 | oui |
| LFI, Renaissance, Horizons, Reconquête | — | `cmp_id` et `cmp` vides des deux côtés |

*A VERIFIER* : ces six codes n'ont pas été confrontés à la source Manifesto.
Vérification : créer un compte, générer une clé, appeler
`api_get_core.json?key=MPDS20XXa` et comparer la colonne `party` sur
`countryname = "France"`.

### 2.5 Nuancier du ministère de l'Intérieur

Deux textes lus sur Légifrance :

| Scrutin | Nature | NOR | Signature |
|---|---|---|---|
| Législatives des 30 juin et 7 juillet 2024 | Instruction | `IOMA2415630C` | 2024-06-11 |
| Municipales des 15 et 22 mars 2026 | Circulaire | `INTP2602966C` | 2026-02-02 |

La grille est en **annexe PDF** (2 annexes pour 2024, 4 pour 2026). Les deux
téléchargements Légifrance répondent **403 à toute requête non navigateur** :
la grille officielle n'est pas récupérable par script. C'est exactement le type
de source que l'ADR 0000 refuse comme source primaire.

Contournement retenu, entièrement scriptable et sous Licence Ouverte v2.0
(champ `license = lov2` sur data.gouv.fr) : les **codes attribués**
figurent dans les fichiers de résultats définitifs. `resultats-definitifs-par-regions.csv`
du 1er tour 2024 (SHA-256 `b0c25687…6939`) donne **22 codes distincts** :

```
COM DIV DSV DVC DVD DVG ECO ENS EXD EXG FI
HOR LR RDG REC REG RN SOC UDI UG UXD VEC
```

Le fichier `resultats-definitifs-par-circonscription.csv` du 2nd tour porte en
outre, par candidat, `Nuance candidat n`, `Nom candidat n` et `Elu n`. **Ces
trois colonnes ne sont pas ingérées** : l'appariement d'une nuance administrative
à une personne physique est hors périmètre (RG-111). Seul l'ensemble des codes
distincts constatés est utilisé — la famille `administratif` mesure des partis et
des coalitions, jamais des personnes.

Trois faits à conserver avec la donnée :

- **La nuance est attribuée à un candidat, pas à un parti.** Un appariement
  parti → code est une déclaration du registre, pas une lecture de la source.
- **`UG` et `ENS` sont des codes de coalition.** `UG` couvre indistinctement
  LFI, PS, écologistes et PCF ; `ENS` couvre RE, MoDem et Horizons. Ils sont
  portés par les entités `coalition.nfp` et `coalition.ensemble`, jamais par un
  parti.
- Le nuançage distingue l'**étiquette** (choisie par le candidat) de la
  **nuance** (attribuée par l'administration) — décret 2014-1479 du
  2014-12-09 — et il est révisé par circulaire, contesté au Conseil d'État
  (docs/sources.md).

*A VERIFIER* : les libellés officiels des 22 codes. Vérification : ouvrir
`https://www.legifrance.gouv.fr/download/pdf/circ?id=45565` dans un navigateur
et transcrire l'annexe. Conséquence en attendant : `ECO` et `VEC` ne sont pas
appariés — l'identifiant de nuance des Écologistes est `null` avec son motif.

### 2.6 Wikidata

Pivot d'identité utile, requêtable sans clé. Contrainte
`?p wdt:P31/wdt:P279* wd:Q7278 ; wdt:P17 wd:Q142` :

| Entité | QID |
|---|---|
| La France insoumise | `Q27978402` |
| Parti socialiste | `Q170972` |
| Parti communiste français | `Q192821` |
| Les Écologistes – Europe Écologie Les Verts | `Q613786` |
| Les Républicains | `Q20012759` |
| Rassemblement national | `Q205150` |
| Renaissance | `Q23731823` |
| Mouvement démocrate | `Q587370` |
| Horizons | `Q108846587` |
| Union des droites pour la République | `Q127038760` |
| Reconquête | `Q109932430` |
| Place publique | `Q58366009` |
| Nouveau Front populaire (coalition) | `Q126487286` |

Wikidata distingue explicitement le parti du groupe parlementaire — `Q3550126`
« Groupe Union des démocrates et indépendants », `Q30503094` « groupe La France
insoumise », tous deux `P31 = groupe parlementaire à l'Assemblée nationale ».
C'est la même distinction que le registre porte, et elle sert de contrôle
croisé.

**Wikidata n'est jamais source de date.** `P571` est multivalué : `Q127038760`
(UDR) en porte deux, 2012-07-17 et 2024-08-31 ; `Q109932430` (Reconquête)
également, 2021-04-30 et 2021-12-05. Une date reprise sans arbitrage serait un
tirage au sort. Là où `P571` est univoque, elle est reprise avec sa source.

Et le pivot échoue parfois : « Ensemble » ne renvoie qu'une page d'homonymie
(`Q112895317`). L'entité `coalition.ensemble` a donc un identifiant Wikidata
`null` avec son motif.

### 2.7 ParlGov

Accessible sans compte : `https://parlgov.org/data/parlgov-development_csv-utf-8/party.csv`
(SHA-256 `b8c6006c…ad8ba`), `country_id = 43` pour la France, **80 partis**.
Hors v0 (ADR 0000) mais retenu comme **table de correspondance**, parce qu'il
porte à la fois `cmp` (Manifesto) et `chess` (`party_id` CHES) et confirme les
codes de §2.4.

| `id` | `name_short` | `cmp` | `chess` |
|---|---|---|---|
| 2644 | FI | — | — |
| 1539 | PS | 31320 | 602 |
| 686 | PCF | 31220 | 601 |
| 873 | V | 31110 | 605 |
| 658 | UMP\|LR | 31626 | 609 |
| 270 | FN | 31720 | 610 |
| 2643 | REM\|R | — | — |
| 2857 | H | — | — |
| 2860 | R! | — | — |

Piège : `name_short` agrège les dénominations successives avec une barre
verticale (`UMP|LR`, `REM|R`, `UDF|MD`), et pour l'entrée 270 il vaut encore
`FN` alors que `name` vaut déjà « Rassemblement National ». Absents : UDR,
Place publique.

---

## 3. Modèle de données

**Un seul fichier**, `data/registre/partis.json`, JSON UTF-8, indentation de 2
espaces, fins de ligne LF. JSON parce que la pile le lit déjà des deux côtés —
`serde_json` côté pipeline, natif côté front (ADR 0001) — et qu'aucune
dépendance n'est à ajouter. Un fichier plutôt que N : le contrôle d'injectivité
des identifiants (§6, règle V7) est global, un registre éclaté le rendrait
faux par construction.

### 3.1 Enveloppe

| Champ | Type | Contrainte |
|---|---|---|
| `schema` | chaîne | `contrepoint/registre-partis/1`, littéral |
| `version` | chaîne | version du contrat de sortie (ADR 0000 §6) |
| `date_registre` | date | date de la dernière modification du fichier |
| `licence` | chaîne | Licence Ouverte / Open Licence (Etalab) |
| `legislatures` | tableau | voir 3.1.1 |
| `sources` | tableau | voir 3.2 |
| `entites` | tableau | partis et coalitions, voir 3.3 |
| `groupes` | tableau | groupes parlementaires, voir 3.4 |
| `relations` | tableau | événements, voir 3.5 |

Aucune autre clé racine. En particulier, **rien ne distingue l'extrait du fichier
réel dans le fichier** : une clé `statut` a existé dans l'extrait et a été
retirée, parce que V1 la refuse et qu'une fixture qui ne valide pas contre le
schéma qu'elle est censée démontrer ne démontre rien. Ce qu'un fichier est se lit
à son nom, pas à un champ.

### 3.1.1 `legislatures[]`

| Champ | Contrainte |
|---|---|
| `numero` | `^[0-9]{1,2}$` |
| `chambre` | `AN` |
| `debut` | date d'**ouverture** de la législature, jamais dérivée d'un autre fait (§4) |
| `fin` | date ou `null` |
| `source` | `id` déclaré dans `sources` |
| `url` | URL de la source qui atteste `debut` ; obligatoire pour `declaration_publique` |
| `etabli_le` | date d'établissement humain |
| `remarque` | ≤ 140 caractères |

`source`, `url` et `etabli_le` sont là parce qu'ils manquaient : `debut` a circulé
sans source déclarée, et deux dates candidates ont été confondues. Le §4 dit
laquelle est laquelle.

### 3.2 `sources[]`

| Champ | Contrainte |
|---|---|
| `id` | `^[a-z0-9_]+$`, unique |
| `libelle` | non vide |
| `url` | absolue, ou `null` pour `declaration_publique` |
| `recupere_le` | date de consultation |
| `empreinte_sha256` | 64 hexadécimaux, ou `null` si la source est une requête |
| `cardinalite` | `un_par_date` \| `plusieurs` |
| `licence` | énoncée, y compris « conditions non vérifiées » |
| `remarque` | ≤ 140 caractères, une phrase (ADR 0000 §5) |

`cardinalite = un_par_date` signifie : à une date donnée, une entité a **au
plus un** identifiant de cette source. `plusieurs` est réservé aux sources dont
la clé est intrinsèquement multiple — codes de nuance par scrutin,
rattachements financiers annuels.

### 3.3 `entites[]`

| Champ | Contrainte |
|---|---|
| `id` | `^(parti\|coalition)\.[a-z0-9-]+$`, unique, **immuable** |
| `nature` | `parti` \| `coalition`, cohérent avec le préfixe de `id` |
| `nom` | non vide |
| `sigle` | ≤ 40 caractères (ADR 0000 §5) |
| `debut`, `fin` | dates ou `null` (§4) |
| `identifiants` | tableau, voir 3.6 |
| `composition` | tableau, non vide seulement si `nature = coalition` |
| `remarque` | ≤ 140 caractères |

### 3.4 `groupes[]`

| Champ | Contrainte |
|---|---|
| `id` | `^groupe\.an[0-9]{2}\.[a-z0-9-]+$`, unique, immuable |
| `chambre` | `AN` |
| `legislature` | présent dans `legislatures` |
| `uid_an` | `^PO[0-9]+$`, unique dans le fichier |
| `nom`, `sigle` | recopiés de `libelle` / `libelleAbrev` de la source |
| `debut`, `fin` | **égaux** à `viMoDe.dateDebut` / `viMoDe.dateFin` |
| `ancre_axe` | objet ou `null`, voir 3.4.1 |
| `composition` | tableau, voir 3.7 |
| `remarque` | ≤ 140 caractères |

Un groupe n'a pas de bloc `identifiants` : son identifiant externe est
`uid_an`, et aucune autre source ne référence les groupes parlementaires.

`remarque` figure ici parce que le §3.7 l'exige déjà : « une composition vide
n'est pas un oubli, c'est une absence documentée par `remarque` sur le porteur ».
Elle était employée par onze groupes sur quatorze et absente du tableau, donc
refusée par V1.

### 3.4.1 `ancre_axe`

| Champ | Contrainte |
|---|---|
| `pole` | `gauche` \| `droite` |
| `debut` | date, incluse dans la période du groupe |
| `fin` | date ou `null` |
| `etabli_le` | date d'établissement humain |
| `remarque` | ≤ 140 caractères |

`null` pour un groupe qui n'ancre rien, c'est-à-dire pour douze des quatorze.

C'est le champ que positionnement.md §5 exigeait et que son blocage 3 signalait
sans qu'il existe. Ce qu'il porte : **quels groupes fixent les deux pôles de
l'axe des votes, et sur quelle période**. La médiane de l'ancre `gauche` vaut
−1,0000, celle de l'ancre `droite` vaut +1,0000 ; la transformation affine qui le
réalise est la convention, et elle est nommée par l'identifiant d'échelle
`votes_an17_ancre_v1` (contrats.md §2.3). Les deux ne se confondent pas :
**l'identifiant nomme la convention, ce champ nomme les ancres.**

Renseigné au 2026-08-27 pour `groupe.an17.lfi-nfp` (pôle `gauche`, IQR 0,047) et
`groupe.an17.rn` (pôle `droite`, IQR 0,052) — les deux plus gros groupes des
extrémités et les plus homogènes (positionnement.md §5). Les deux périodes
débutent au 2024-07-18, `debut` de leur groupe, et ne sont pas closes.

Ce champ n'a **pas** de `source` : le choix d'une ancre n'est pas la lecture d'une
source externe, c'est une décision de méthode datée, dont le fondement est
positionnement.md §5 et dont la trace est `etabli_le` et l'historique git. Lui
coller un `id` de `sources` aurait déclaré une provenance qui n'existe pas.

Les deux pôles sont **descriptifs** : aucune extrémité d'un axe gauche-droite
n'est une insulte, et le champ ne porte aucune valeur mesurée
(docs/juridique.md règle 2).

### 3.5 `relations[]`

`type` ∈ `renommage`, `succession_groupe`, `fusion`, `scission`, `dissolution`.
Champs : `de`, `vers` (`id` existants), `date`, `source`, `url`, `remarque`.
Une relation est un **événement ponctuel** : elle ne crée ni ne clôt une
période, elle explique la contiguïté de deux périodes déjà écrites.

### 3.6 `identifiants[]`

| Champ | Contrainte |
|---|---|
| `source` | `id` déclaré dans `sources` |
| `valeur` | chaîne non vide, ou `null` |
| `libelle_source` | libellé lu dans la source, trace uniquement, jamais clé |
| `debut`, `fin` | période de validité de **l'appariement**, pas de l'entité |
| `etabli_le` | date à laquelle un humain a établi la ligne |
| `motif` | **obligatoire si `valeur` est `null`**, interdit sinon |
| `remarque` | ≤ 140 caractères |

`valeur: null` avec motif est la forme normale de l'absence. Elle est
**dite, jamais comblée** (ADR 0000 §5) : une entité absente d'une source porte
la ligne et son motif, elle ne porte pas un identifiant approchant.

### 3.7 `composition[]`

| Champ | Contrainte |
|---|---|
| `entite` | `id` d'une entité de `nature = parti` |
| `debut`, `fin` | inclus dans la période du porteur |
| `source` | `id` déclaré dans `sources` |
| `url` | URL de la déclaration citée, ou `null` ; **obligatoire** si `source = declaration_publique` |
| `etabli_le` | date d'établissement humain |
| `remarque` | ≤ 140 caractères |

Une composition vide n'est pas un oubli : c'est une absence documentée par
`remarque` sur le porteur. Voir EPR et DEM dans l'extrait.

---

## 4. Périodes de validité

Convention unique, appliquée partout :

- **Bornes incluses**, granularité au jour, format `YYYY-MM-DD` strict.
- `debut: null` = « antérieur à la couverture du registre », jamais « inconnu
  comblé ». `fin: null` = « en cours au `date_registre` ».
- Deux périodes du même couple (entité, source) ne se **chevauchent** jamais.
- Une succession se traduit par des bornes **contiguës** : `fin` du prédécesseur
  = `debut` du successeur − 1 jour. Vérifié dans la source : AD `fin`
  2024-09-11, UDR `debut` 2024-09-12 ; UDR `fin` 2025-09-04, UDDPLR `debut`
  2025-09-05.
- Une **interruption** est écrite comme telle, avec un trou entre deux périodes.
  Cas réel : Charlotte Parmentier-Lecocq est membre de HOR du 2024-09-10 au
  2024-10-27, puis non inscrite le 2026-03-27, puis HOR à partir du 2026-03-28.
  Ce trou existe dans AMO30 ; aucune interpolation ne le comble.
- Une période n'est jamais dérivée d'un libellé. La fiche du jeu 17 est
  intitulée « XV législature » sur data.assemblee-nationale.fr : le libellé est
  faux, le chemin `/repository/17/` est autoritatif (ADR 0000 §3).

### 4.1 La date d'ouverture d'une législature, et les deux dates qu'on lui prête

`legislatures[0].debut` a circulé à **2024-07-08** sans source déclarée. C'est
faux au sens où c'était écrit : trois faits distincts portent trois dates, et
`debut` désigne le troisième.

| Fait | Date | Où elle se lit |
|---|---|---|
| Début du mandat des députés élus | **2024-07-07** | AMO30, `mandatsAssemblee[].dateDebut` des acteurs de la XVIIe — 2024-07-07 sur les six mandats de la fixture `mandats-gp-l17.json`, le jour du second tour |
| Proclamation des résultats définitifs du second tour par les commissions de recensement | **2024-07-08** | communiqués préfectoraux du 2024-07-08 ; les résultats définitifs de data.gouv.fr sont déposés le 2024-07-10 |
| **Ouverture de la XVIIe législature — première séance** | **2024-07-18** | compte rendu de la séance du jeudi 18 juillet 2024 de l'Assemblée nationale : « *Je déclare ouverte la XVIIe législature de l'Assemblée nationale et la session de droit prévue par l'article 12 de la Constitution.* », séance ouverte à quinze heures — <https://www.assemblee-nationale.fr/dyn/17/comptes-rendus/seance/session-de-droit-de-2024/seance-du-jeudi-18-juillet-2024> (consulté le 2026-08-27) |

**`debut` vaut donc 2024-07-18**, et le registre le déclare avec cette source et
son URL (§3.1.1). C'est la seule des trois dates qui soit une propriété de la
**législature** : les deux autres sont des propriétés des mandats et du scrutin.
Le choix découle de la source, pas d'une préférence — c'est aussi la date que
treize des quatorze organes `GP` portent en `viMoDe.dateDebut`, ce qui est
attendu : un groupe se constitue à l'ouverture.

`A VERIFIER` : la date de proclamation du 2024-07-08 est reprise de communiqués
préfectoraux et de la date de mise à jour de la page du ministère de l'intérieur,
tous deux lus le 2026-08-27 ; la page du ministère répond **403** à toute requête
non navigateur. Elle n'est employée nulle part dans le registre — elle figure ici
pour dire ce que `debut` n'est pas. Vérification : ouvrir
`https://www.interieur.gouv.fr/actualites/actualites-du-ministere/elections-legislatives-2024-resultats-definitifs`
dans un navigateur, ou retrouver l'arrêté de proclamation au Journal officiel.

### 4.2 L'organe des non-inscrits ouvre avant la législature — l'exception à V13

Avec `debut = 2024-07-18`, treize groupes sur quatorze sont inclus dans la période
de leur législature. Le quatorzième ne l'est pas et ne le sera jamais : `PO840056`
« Non inscrit » porte `viMoDe.dateDebut = 2024-07-01`, dix-sept jours avant
l'ouverture, et sept jours avant la date qui circulait. Aucun choix de `debut`
parmi les trois dates du §4.1 ne rend ce groupe inclus.

Deux règles s'affrontaient donc, et aucun registre ne pouvait les satisfaire
toutes deux : **V13** exige l'inclusion, **V16** exige l'égalité stricte avec
`viMoDe`. Retenir V13 obligeait à écrire une `dateDebut` fausse ; retenir V16
obligeait à violer V13.

**V13 cède, nommément, pour ce seul organe.** La raison est celle du §2.1 :
`PO840056` **n'est pas un groupe**. C'est l'agrégat administratif des députés sans
groupe, ouvert avec la mandature et non constitué au sein de la législature — son
écart-type intra est de 0,649 contre 0,065 pour le RN, et le registre ne l'agrège
jamais comme un parti. Une exception nommée à une règle de période est
vérifiable ; une date recopiée de travers ne l'est pas. V16 ne cède pas : c'est
elle qui rend le registre falsifiable contre sa source, et une exception y
ouvrirait la porte à la correction à la main que le contrôle existe pour attraper.

L'exception est portée par V13 au §6, sur l'`uid_an` et non sur un libellé.

Un identifiant a **sa propre** période, distincte de celle de l'entité. Le RN
existe depuis 1972-10-27 ; son identifiant `an_organe` vaut `PO684946` jusqu'au
2018-11-30 puis `PO761239` — bornes reprises telles quelles de `viMoDe`. Son
`party_id` CHES vaut 610 sans coupure sur toute la période.

---

## 5. Les cas durs, avec leur exemple réel

### 5.1 Un groupe agrège plusieurs partis

`ECOS` (`PO845439`) : 33 membres déclarent ECOLO, 5 déclarent PCF. Le registre
porte deux lignes de composition. Toute agrégation qui traite le groupe comme
un parti attribue à ECOLO les votes de 5 députés communistes.

### 5.2 Un parti sans groupe, et un groupe sans parti

**Place publique** : présent dans Wikidata (`Q58366009`), absent des 58 PARPOL
de l'AN, absent des 80 partis ParlGov, absent des 10 partis CHES 2024, absent
de la grille de nuances. Un identifiant sur sept sources. Il est conservé
parce qu'il existe dans la coalition NFP, et six lignes portent leur motif.

**LIOT** (`PO845485`) : aucun parti dominant — 12 membres déclarent RPS, 6
UDIE, 3 autres partis. Sa composition reste vide. Son écart-type intra est de
0,422 contre 0,046 pour LFI-NFP (ADR 0001 §1.3) : ce n'est pas un bloc
idéologique et le registre ne le présente pas comme tel.

### 5.3 Changement de nom

Trois occurrences réelles, trois traitements différents parce que les sources
ne se comportent pas pareil :

| Cas | CHES | AN | Registre |
|---|---|---|---|
| FN → RN (2018) | `party_id` 610 inchangé | deux organes, `PO684946` clos au 2018-11-30 puis `PO761239` | une entité, deux lignes `an_organe` bornées |
| UMP → LR (2015) | `party_id` 609 inchangé | `PO684942` clos au 2015-11-30 puis `PO710396` | idem |
| Groupe LR → DR (2024) | sans objet | nouvel `uid` `PO845425` pour la XVIIe | deux groupes distincts, un par législature |

Un changement de nom **ne change jamais l'`id`** de l'entité : `id` est
immuable, et le renommer serait une majeure (ADR 0000 §6).

### 5.4 Scission

`À Droite` (`PO845520`) est créé le 2024-07-18 et dissous le 2024-09-11 ; `UDR`
(`PO847173`) est actif du 2024-09-12 au 2025-09-04 ; `UDDPLR` (`PO872880`) depuis le
2025-09-05. Vérifié sur des trajectoires individuelles : Gérault Verny, Marc
Chavent, Sophie Ricourt Vaginay, Vincent Trébuchet et Éric Michoux suivent
exactement AD → UDR → UDDPLR aux mêmes dates. Le registre écrit deux relations
(`succession_groupe` puis `renommage`) et une relation `scission` de
`parti.lr` vers `parti.udr` — **aucune n'est lue dans une source**, les trois
sont des déclarations humaines datées et sourcées.

### 5.5 Coalition électorale

`coalition.nfp` porte le code de nuance `UG` et cinq lignes de composition. Les
partis membres gardent leurs propres identifiants et leur propre code (`FI`,
`SOC`, `COM`). Une valeur CHES n'est jamais reportée d'une coalition vers ses
membres, ni d'un membre vers la coalition.

`coalition.ensemble` existe uniquement pour porter `ENS`, sans QID exploitable.
Une coalition n'apparaît jamais dans la `composition` d'un groupe : les groupes
sont composés de partis.

### 5.6 Un député change de groupe en cours de mandature

**Jamais dans ce fichier.** Le pipeline lit les mandats de AMO30 et fait la
jointure sur période. Cas réels :

| Député | Trajectoire XVIIe |
|---|---|
| Philippe Fait | NI 2024-07-08→18, EPR 2024-07-19→2025-06-13, HOR depuis 2025-06-14 |
| Christine Le Nabour | EPR 2024-07-19→2026-07-22, HOR depuis 2026-07-23 |
| Gérault Verny | NI, AD, UDR, UDDPLR aux dates de §5.4 |

Une jointure « dernier groupe connu » attribuerait à des non-inscrits tous les
scrutins du 8 au 18 juillet 2024. Un scrutin du 2025-05-01 auquel Philippe Fait
a participé compte pour EPR, pas pour HOR. Le test correspondant est exigé par
docs/definition-of-done.md, point 9.

Aucune coordonnée individuelle n'est attachée à ces noms, ici ou ailleurs (ADR
0000 §2).

### 5.7 Présent chez CHES, absent du nuancier — et l'inverse

| Cas | CHES 2024 | Nuance 2024 | Traitement |
|---|---|---|---|
| Renaissance | `party_id` 626 | aucun code propre, candidats nuancés `ENS` | identifiant de nuance `null` + motif ; `ENS` porté par la coalition |
| Reconquête | `party_id` 630, 0 siège | code `REC` | les deux lignes remplies, aucun siège : le registre l'accepte |
| UDR | absent, créé après le terrain | aucun code, créé après le scrutin | six lignes `null` sur sept, un seul QID |
| `DSV`, `REG`, `RDG` | aucun parti CHES | codes attribués | aucune entité créée : un code sans parti n'est pas une entité |
| Les Écologistes | `party_id` 605 | `ECO` **ou** `VEC`, indéterminé | `null` + motif jusqu'à lecture de l'annexe |

La règle qui découle : **une famille de mesure absente ne se remplit pas avec
une autre.** C'est la même règle qu'au niveau du graphe (les trois familles ne
sont jamais moyennées), appliquée un étage plus bas.

---

## 6. Règles de validation automatique

Le validateur tourne en CI et **bloque**. Toute règle violée refuse le
registre : un registre incohérent n'est pas corrigé automatiquement, il est
rejeté.

### Structure

- **V1** Chaque clé rencontrée appartient à la liste blanche des §3.1 à 3.7.
  Une clé inconnue est un refus — c'est ce qui empêche l'apparition silencieuse
  d'un champ de valorisation.
- **V2** `schema` vaut littéralement `contrepoint/registre-partis/1`.
- **V3** Toute date respecte `^\d{4}-\d{2}-\d{2}$` et est une date réelle.
- **V4** Tout `id` respecte son motif, est unique toutes catégories confondues,
  et `nature` s'accorde avec le préfixe.
- **V5** Tout `id` cité dans `composition`, `relations` ou ailleurs existe.
- **V6** Tout `source` cité existe dans `sources`, et toute source déclarée est
  citée au moins une fois.

### Identifiants — le cœur du contrôle

- **V7** *Injectivité par source et par date.* Pour une source donnée, un même
  `valeur` ne peut pas désigner deux entités sur des périodes qui se
  chevauchent. C'est la règle qui attrape l'erreur fatale : deux partis
  appariés au même `party_id` CHES.
- **V8** *Cardinalité.* Pour une source `un_par_date`, une entité a au plus un
  identifiant actif à toute date. Deux lignes `an_organe` sur `parti.rn` passent
  parce que leurs périodes sont disjointes.
- **V9** `valeur: null` ⟹ `motif` non vide ; `valeur` non nul ⟹ `motif` nul.
- **V10** `etabli_le` présent sur toute ligne, ≤ `date_registre`.

### Périodes

- **V11** `debut ≤ fin` quand les deux sont présents, partout.
- **V12** La période d'un identifiant ou d'une composition est incluse dans
  celle de son porteur.
- **V13** La période d'un groupe est incluse dans celle de sa législature.
  **Exception nommée, une seule** : `uid_an = "PO840056"`, l'organe des députés
  non inscrits, dont `viMoDe.dateDebut` (2024-07-01) précède l'ouverture de la
  XVIIe (2024-07-18). Ce n'est pas un groupe constitué mais l'agrégat des députés
  sans groupe (§2.1), et V16 interdit de corriger sa date. L'exception porte sur
  l'`uid_an`, jamais sur un libellé ni sur une abréviation (V14 du même
  raisonnement : `libelleAbrev` n'est pas une clé). Motif complet : §4.2.
- **V14** Aucun chevauchement entre deux périodes du même couple
  (porteur, source, valeur).

### Confrontation à la source — ce qui rend le registre falsifiable

- **V15** Tout `uid_an` existe dans les organes de AMO30, avec
  `codeType = "GP"` et la bonne `legislature`.
- **V16** `nom`, `sigle`, `debut`, `fin` d'un groupe sont **égaux** à
  `libelle`, `libelleAbrev`, `viMoDe.dateDebut`, `viMoDe.dateFin`. Une
  divergence est un refus : soit la source a bougé, soit le registre a été
  édité à la main. Les deux exigent un humain.
- **V17** Tout identifiant `ches_2024` existe dans le fichier téléchargé avec
  `country = 6`. Idem `parlgov` sur `country_id = 43`, `nuance_leg2024` sur les
  22 codes constatés.
- **V18** Toute relation `renommage` ou `succession_groupe` relie deux périodes
  contiguës : `date` = `debut` du successeur = `fin` du prédécesseur + 1 jour.
- **V19** Une composition non vide de source `an_parpol_mandats` correspond à au
  moins un mandat PARPOL actif dans AMO30. Sinon, `remarque` obligatoire.

### Ancres de l'axe

- **V24** *Unicité par pôle et par date.* À une date donnée, au plus **un** groupe
  porte `ancre_axe.pole = "gauche"` et au plus **un** porte `"droite"`. Deux
  ancres du même pôle sur des périodes qui se chevauchent sont un refus : la
  transformation du §5 de positionnement.md n'est plus définie, et le pipeline
  choisirait selon l'ordre de lecture. La période d'un `ancre_axe` est incluse
  dans celle de son groupe (V12, même raisonnement).
- **V25** *Les deux pôles existent à la date d'agrégation.* Si, à cette date, un
  des deux pôles n'est porté par aucun groupe, le pipeline **échoue** et ne publie
  pas la famille `votes`. Il ne choisit jamais une ancre de remplacement, ne
  reprend pas l'ancre de la période précédente et n'ancre pas sur un seul pôle
  (RG-31). Une ancre qui disparaît est un changement de convention à trancher par
  un humain, pas un défaut à combler.

V24 et V25 sont ce qui donne un contenu à l'invariant I5 du contrat de sortie :
avant elles, I5 vérifiait que le groupe nommé comme ancre existait à la date, pas
qu'il avait été **déclaré** comme ancre (contrats.md §6).

### Conformité de projet

- **V20** Aucun terme du lexique interdit de docs/juridique.md dans le fichier,
  clés comprises.
- **V21** `sigle` ≤ 40 caractères, `remarque` et `motif` ≤ 140 caractères, une
  phrase.
- **V22** Aucun champ numérique de valorisation, aucune coordonnée, aucun nom
  de personne. Découle de V1, énoncé séparément parce que c'est l'invariant
  qu'un contributeur pressé cassera en premier.
- **V23** *Forme canonique.* Le fichier est **identique octet pour octet** à sa
  ré-sérialisation : tableaux triés (`sources`, `entites`, `groupes` par `id` —
  `sources` l'était par ordre d'apparition dans le §2 et non par `id`, ce qui
  était un refus V23 non détecté ;
  `relations` par (`date`, `de`)), clés dans l'ordre déclaré aux §3, 2 espaces,
  LF, une fin de ligne finale, UTF-8 sans BOM. Une main qui édite sans passer
  par le formateur est détectée.

Tests dédiés exigés par docs/definition-of-done.md : un cas qui échoue par
règle pour V7, V8, V9, V14, V16, V23, V24 et V25 au minimum, sur des fixtures
issues de sources en Licence Ouverte. Le plan de tests les porte sous
`REG-01` à `REG-22` (plan-de-tests.md §9).

Ce que le schéma formel couvre, et ce qu'il ne couvre pas.
`schemas/registre-partis-1.schema.json` transcrit V1 — la liste blanche, avec
`additionalProperties: false` partout —, V2, la forme des `id` (V4), les
longueurs (V21), V9 et l'obligation de `url` sur une composition de source
`declaration_publique`. Il ne peut pas exprimer l'unicité et l'existence des `id`
(V4, V5, V6), l'injectivité (V7), la cardinalité (V8), la réalité d'une date
(V3), l'inclusion et le non-chevauchement des périodes (V10 à V14), la
confrontation à AMO30 (V15 à V19), le lexique (V20), l'absence de valorisation
(V22), la forme canonique (V23) ni l'unicité des ancres (V24, V25) : le validateur
du registre les vérifie en plus, et c'est lui qui bloque.

---

## 7. Procédure de correction humaine

Le registre se corrige **par pull request**, jamais autrement. Il n'y a pas de
formulaire, pas de base à éditer en ligne : le fichier est le seul état.

1. **Ouvrir une PR d'un seul objet** : une entité, un appariement, une
   relation. Une PR qui touche cinq partis se scinde.
2. **Citer la source dans la ligne modifiée.** Toute valeur non nulle porte un
   `etabli_le` mis à jour, et la source doit être vérifiable par un tiers à
   l'URL déclarée dans `sources`. Aucune correction « de mémoire ».
3. **Passer par le formateur** avant de committer (V23). Un fichier de données
   modifié à la main sans script reproductible est refusé
   (docs/definition-of-done.md, point 18).
4. **Relecture ligne par ligne obligatoire**, y compris pour un caractère. Le
   diff est lisible par construction : le format canonique fait qu'une
   correction d'appariement tient en une ligne.
5. **`id` immuable.** Aucun renommage, aucune suppression. Une entité qui
   cesse d'exister reçoit une `fin`. Une entité créée par erreur est corrigée
   en `fin = debut` avec un `remarque` — jamais retirée du fichier, sans quoi
   les lignes de preuve déjà publiées pointeraient dans le vide.
6. **Conséquence sur le registre de preuves.** Corriger un appariement est un
   **patch** au sens de l'ADR 0000 §6, et ré-émet des lignes de preuve. Aucune
   ligne antérieure n'est modifiée : la valeur corrigée est une nouvelle ligne.
   Modifier un `id` serait une majeure, ce qui est la raison de la règle 5.
7. **Journal.** L'historique git est le journal. Aucun fichier de suivi
   parallèle : il divergerait.

Une réclamation externe suit la même voie que n'importe quelle correction
(docs/juridique.md, règle 4). Une réclamation fondée est un patch ; une
réclamation sur la méthode se répond par la source citée dans la ligne.

---

## 8. Ce qui reste ouvert

| Point | Comment le trancher |
|---|---|
| Libellés officiels des 22 codes de nuance | Ouvrir `legifrance.gouv.fr/download/pdf/circ?id=45565` dans un navigateur, transcrire l'annexe. Débloque l'appariement `ECO` / `VEC`. |
| Codes CMP des six partis | Compte Manifesto + clé d'API, comparer à `api_get_core.json`. Hors v0 ; les valeurs actuelles restent marquées comme reprises de tiers. |
| Composition de EPR et DEM | Aucune source de données ne la publie et `ESBMP` ne la donne pas. Trancher par déclaration publique sourcée, ou laisser vide indéfiniment. |
| Date de création de UDR | Deux valeurs `P571` sur Wikidata. Une source primaire (Journal officiel des associations) tranche. |
| Grille de nuances des municipales 2026 | `INTP2602966C`, quatre annexes, 26 nuances de candidat et 25 de liste selon la presse. Hors v0 : aucun écran ne consomme les municipales. |
| Vérification de `parlgov` et `ches` en CI | Les deux sources sont hors périmètre v0 mais servent V17. Décider si la CI télécharge CHES à chaque exécution ou seulement à la modification du registre. |
| Formateur canonique de V23 | La règle décrit la forme, aucun script ne la produit : les deux fichiers ont été écrits par un script jetable, non commité, ce que le point 18 de docs/definition-of-done.md refuse. Écrire `scripts/formater-registre.py`, ou ce que la pile choisira, et le faire tourner en CI sur le fichier commité. |
| Date de proclamation des résultats du 2024-07-08 | Reprise de communiqués préfectoraux et d'une page ministérielle qui répond 403 hors navigateur (§4.1). Elle n'entre dans aucun champ ; la retrouver au Journal officiel pour clore la note. |
