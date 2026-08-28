# Glossaire normatif

**Article normatif unique.** Ce document est le **seul** endroit du dépôt où un
terme technique est défini (ISO/IEC Directives Part 2, article « Termes et
définitions »). Un terme défini ici et redéfini ailleurs est un défaut, et
`scripts/glossaire.sh` le refuse.

**Un terme, un concept** (ISO 704). Quand un mot recouvre deux concepts, le
concept **le plus employé garde le mot** et l'autre reçoit un nom distinct. Le
décompte d'occurrences qui fonde chaque arbitrage est donné dans l'entrée : il
est reproductible par la commande citée en fin de document.

**Ce glossaire dit ce qui devrait être, pas ce qui est.** Deux grades :

| Grade | Sens | Contrôlé par `scripts/glossaire.sh` |
|---|---|---|
| **Interdit** | forme absente du dépôt aujourd'hui, qui ne doit pas y entrer | oui — un emploi rend la porte rouge |
| **À migrer** | forme encore employée, à remplacer par un ticket séparé | non — signalée, jamais bloquante |

La distinction n'est pas un adoucissement : une porte rouge le jour de sa
naissance est une porte qu'on désarme à la première PR pressée. Les termes du
grade « à migrer » passent au grade « interdit » quand leur ticket est fermé.

Ce document ne recopie pas la liste de `scripts/lexique.sh`, qui reste la source
unique des termes interdits par `docs/juridique.md`. Les deux contrôles sont
disjoints : le lexique porte sur ce que le projet **refuse de dire**, le
glossaire sur ce que le projet **appelle comment**.

---

## 1. Population et matrice

### cellule

**Définition.** Une cellule est un couple `(acteur, scrutin)` pour lequel une
**position exprimée** — `pour`, `contre` ou `abstention` — est écrite dans la
matrice.

**Emplois attestés.** 30 emplois qualifiés « cellule observée / absente /
exprimée » : `docs/methode.md` §1-2, `docs/architecture.md` (l. 26, 97, 103,
111), `docs/adr/0001-stack.md` §1.2, `docs/brique0/positionnement.md` §0 et §4,
`docs/brique0/ingestion-votes.md` §7 et §8, `docs/regles-de-gestion.md`
RG-11 et RG-25, `docs/ton.md` §3, `docs/brique0/plan-de-tests.md`,
`docs/brique0/verification-2026-08-27.md`, `ROADMAP.md`.

**Concurrence.** Trois populations portent aujourd'hui le même mot.

| Concept | Population attestée | Emplois | Nom retenu |
|---|---|---|---|
| position exprimée écrite | 1 247 093 (corpus brut) ; 1 188 035 (corpus retenu) | **30** | **cellule** — garde le mot |
| couple documenté par la source, position exprimée **ou** non-votant | 1 270 476 ; dérivés 17 716 ambiguës, 23 383 `nonVotants`, 2 255 désaccords, 1 895 | **13** | **ligne nominative** |
| case du produit acteurs × scrutins, écrite ou non | 5 414 628 (642 × 8 434) ; 4 144 152 absences | **2** | **case de matrice** |

L'arbitrage est celui du nombre : 30 contre 13 contre 2.

**Ce que ce n'est pas.** Une cellule n'est **jamais** une absence ni un
non-votant : `RG-11` pose qu'aucune cellule n'est produite pour eux, et
`docs/methode.md` §1 que l'absence est une donnée manquante. C'est cet
invariant qu'une addition d'absences à des positions inverse.

**Interdit :** `cellule de vote`, `cellule votée`, `cellule nominative`, `cellule pleine`

**À migrer :** les treize emplois du sens « ligne nominative » — `1 270 476
cellules` (`docs/brique0/ingestion-votes.md` l. 116, 485, 517,
`docs/adr/0003-arbitrages-de-coherence.md` l. 61, 101,
`docs/brique0/plan-de-tests.md` l. 547) et
``cellules `nonVotants` `` (`ingestion-votes.md` l. 445).

---

### densité

**Définition.** La densité est le quotient **cellules observées ÷ (acteurs
retenus × scrutins retenus)**. Elle n'est jamais écrite sans le corpus sur
lequel elle porte.

**Emplois attestés.** 8 emplois, sur deux corpus.

| Corpus | Valeur écrite | Emplois |
|---|---|---|
| corpus retenu (641 × 7 979) | 23,2 % ; 0,2323 | **5** — `docs/brique0/ingestion-votes.md` §5 et §7, `docs/adr/0003-arbitrages-de-coherence.md` §4, `docs/brique0/verification-2026-08-27.md`, `docs/brique0/plan-de-tests.md` §1 |
| corpus brut (642 × 8 434) | 0,2303 ; 23,03 % ; 23,0 % | **4** — `docs/brique0/positionnement.md` §0 et §1 mesure 3, `docs/adr/0001-stack.md` §1.1 et §1.2 |

Aucun des deux ne garde le mot nu : le qualificatif de corpus est obligatoire
dans les deux cas. C'est la seule entrée où l'arbitrage n'est pas un
renommage — les deux valeurs sont justes, et c'est le mot nu qui est faux.

**Ce que ce n'est pas.** La densité n'est pas le taux de participation : son
dénominateur est un produit d'effectifs retenus, pas un nombre de sièges.

**Interdit :** `densité brute`, `densité globale`, `densité observée`

**À migrer :** le mot nu de `docs/adr/0001-stack.md` §1.2 (« La densité est le
problème ») et de `docs/brique0/ingestion-votes.md` l. 334.

---

### corpus

**Définition.** Un corpus est un ensemble de scrutins délimité par un critère
écrit. Trois corpus existent, et ils portent trois noms distincts.

| Nom | Délimitation | Effectif |
|---|---|---|
| **corpus brut** | tous les scrutins de l'archive | 8 434 scrutins, 642 acteurs |
| **corpus retenu** | après le filtre de minorité non vide `min(pour, contre) ≥ 1` | 7 979 scrutins, 641 acteurs |
| **corpus d'essai** | une des sept variantes de sensibilité de l'ajustement | sept, `docs/brique0/ingestion-votes.md` §4 |

**Emplois attestés.** 40 emplois du mot. « corpus complet » 6 fois
(`ROADMAP.md` l. 158, `docs/brique0/ingestion-votes.md` l. 442,
`docs/adr/0003-arbitrages-de-coherence.md` l. 154,
`docs/brique0/verification-2026-08-27.md` l. 124) ; « corpus filtré » 1 fois
(`ROADMAP.md` l. 158) ; « sept corpus » et « corpus synthétiques » 4 fois
(`docs/brique0/ingestion-votes.md` l. 386, `docs/brique0/plan-de-tests.md`
l. 392, 396) ; « part du corpus » 3 fois. Le reste est le mot nu ou son emploi
ordinaire (`README.md` l. 80, `docs/adr/0000-perimetre-brique0.md` l. 241).

**Ce que ce n'est pas.** « Part du corpus » n'est pas un pourcentage
transposable : un pourcentage calculé sur le corpus brut ne se lit pas à côté
d'un effectif du corpus retenu. C'est le défaut A7 de l'audit du 2026-08-28.

**Interdit :** `corpus de travail`

**À migrer :** « corpus complet » → `corpus brut` ; « corpus filtré » →
`corpus retenu`.

---

### acteur

**Définition.** Un acteur est une **ligne de la matrice** : une personne
identifiée par un `acteurRef` pour laquelle au moins une cellule est écrite.

**Emplois attestés.** 91 emplois. Effectifs : 642 acteurs sur le corpus brut,
641 sur le corpus retenu (`docs/brique0/ingestion-votes.md` l. 442,
`docs/brique0/positionnement.md` §1 mesure 3,
`docs/adr/0001-stack.md` §1.1) ; 335 acteurs rattachés à un identifiant `PO0`
(`docs/brique0/positionnement.md` l. 68).

**Ce que ce n'est pas.** Un acteur n'est pas un député : le nombre d'acteurs
(642) et le nombre de députés porteurs d'un mandat de groupe (648) diffèrent, et
`RG-110` interdit d'exposer un identifiant d'acteur dans un artefact publié.

---

### député

**Définition.** Un député est une personne titulaire d'un mandat parlementaire
au sens du référentiel `AMO30`, avec sa période de validité.

**Emplois attestés.** 110 emplois. 648 députés portant un mandat de groupe,
18 chevauchements de périodes sur ces 648
(`docs/brique0/plan-de-tests.md` l. 153, `docs/adr/0003-arbitrages-de-coherence.md`
§1) ; `RG-46` (seuil de 200 votes pour l'inclusion d'un député dans la
dispersion).

**Ce que ce n'est pas.** Le député est l'unité **documentaire** ; l'acteur est
l'unité **de calcul**. Aucune coordonnée individuelle de député n'est publiée
(règle non négociable n° 5).

---

### votant

**Définition.** Un votant est un élément de `decompteNominatif.*.votant[]` dans
un fichier de scrutin de la source.

**Emplois attestés.** 130 emplois. La structure : `pours.votant[]`,
`contres.votant[]`, `abstentions.votant[]`, `nonVotants.votant[]`
(`docs/brique0/ingestion-votes.md` §2) ; le piège de sérialisation « `votant`
objet nu au lieu d'un tableau », 35 671 blocs sur 130 851, 27,3 %
(`docs/adr/0001-stack.md` §1.6, `docs/brique0/ingestion-votes.md` §6).

**Ce que ce n'est pas.** `nombreVotants` (23 emplois) n'est pas un décompte de
votants au sens ci-dessus : c'est le champ de participation publié par la
source, médiane 133 sur 577 (`docs/adr/0001-stack.md` l. 64), **jamais employé
comme porte** (`docs/methode.md` §1, `docs/adr/0003-arbitrages-de-coherence.md`
§2). Un **non-votant** est un votant de `nonVotants` : une donnée manquante,
jamais une position (`docs/ton.md` §3).

---

## 2. Objets de la source

### bloc

**Définition.** Le mot `bloc` ne s'emploie jamais nu. Trois concepts, trois
désignations.

| Désignation | Concept | Population attestée | Emplois |
|---|---|---|---|
| **bloc de groupe** | objet `groupes.groupe[]` d'un scrutin, dit aussi bloc de ventilation | 101 208 ; 146 blocs `PO0` | **26** |
| **bloc de position** | objet `pours` / `contres` / `abstentions` / `nonVotants` | 130 851 sérialisations, dont 35 671 objets nus | **10** |
| **bloc politique** | ensemble de groupes que l'axe sépare | — | **12** |

Le bloc de groupe garde la racine par le nombre (26 contre 12 contre 10) ;
aucun des trois ne garde le mot **nu**.

**Emplois attestés.** Bloc de groupe : `docs/methode.md` §1, `ROADMAP.md`
l. 33, `docs/adr/0003-arbitrages-de-coherence.md` §1,
`docs/brique0/ingestion-votes.md` §2 et §8,
`docs/brique0/plan-de-tests.md`. Bloc de position :
`docs/brique0/ingestion-votes.md` §2 et §6, `docs/adr/0001-stack.md` §1.6,
`docs/brique0/plan-de-tests.md` l. 183-184. Bloc politique :
`docs/methode.md` §1 et §2 (« la séparation des blocs », « l'axe sépare
nettement les blocs »), `ROADMAP.md` l. 30, 46, 158,
`docs/adr/0000-perimetre-brique0.md` l. 66, `docs/adr/0001-stack.md` l. 120,
142, `docs/brique0/positionnement.md` l. 564 (« le bloc central »),
`docs/sources.md` l. 218.

**Ce que ce n'est pas.** Un **bloc explicatif** (`docs/ton.md` §2,
`RG-148`) est un objet d'interface, sans rapport : trois phrases au plus par
écran. Le mot est conservé, le contexte le sépare.

**Interdit :** `bloc de vote`

**À migrer :** les douze emplois du sens politique, qui sont tous nus — à
qualifier en `bloc politique`. Ils vivent dans `docs/methode.md` et `ROADMAP.md`,
que `docs/ton.md` §3 réserve à la PR de méthode : le ticket est donc bloqué par
cette contrainte, pas par le glossaire.

---

### empreinte

**Définition.** Le mot `empreinte` ne s'emploie jamais nu. Deux concepts.

| Désignation | Concept | Emplois |
|---|---|---|
| **empreinte de contenu** | SHA-256 de la concaténation des fichiers de l'archive, dans l'ordre `LC_ALL=C` de leurs chemins ; c'est elle qui décide de la déduplication du registre de preuves (`RG-78`, `RG-115`, `docs/brique0/contrats.md` §2.8) | **33** |
| **empreinte d'archive** | SHA-256 du fichier `.zip` livré ; **hors** de la clé de déduplication, car le répartiteur de la source la fait varier (`RG-77`, `RG-115`, `docs/brique0/verification-2026-08-27.md` §0) | **28** |

Aucun des deux ne garde le mot nu : 33 contre 28 ne fonde pas un arbitrage, et
la confusion des deux est précisément le défaut A4 de l'audit.

**Emplois attestés.** 205 emplois du mot au total, dont environ 70 nus dans le
seul `docs/brique0/contrats.md`. `docs/ton.md` §3 ne connaît que « empreinte
SHA-256 » (4 emplois), qui ne distingue pas les deux populations. `RG-15` :
« chaque archive récupérée porte deux empreintes ».

**Ce que ce n'est pas.** Le **MD5 du producteur** n'est pas une empreinte au
sens de ce glossaire : il est consigné à titre documentaire et ne fait jamais
échouer la récupération (`RG-114`, `docs/brique0/ingestion-votes.md` §1,
`scripts/recuperer-sources.sh`).

**Interdit :** `empreinte du fichier`, `empreinte MD5`

**À migrer :** le mot nu de `docs/brique0/contrats.md`, et
« empreinte SHA-256 » de `docs/ton.md` §3, à scinder en deux formes canoniques.

---

## 3. Mesure et échelle

### amplitude

**Définition.** L'amplitude est celle de l'**échelle ancrée** et vaut **2,0 par
construction** : ancre gauche −1, ancre droite +1 (`docs/ton.md` §3).

**Emplois attestés.** 12 emplois.

| Concept | Valeur | Emplois |
|---|---|---|
| amplitude en unités ancrées | 2,0 | **4** — `docs/brique0/positionnement.md` l. 397, 481, 563, `docs/brique0/plan-de-tests.md` l. 89 |
| **amplitude brute** — amplitude de l'axe avant transformation affine | 2,6 | **3** — `docs/adr/0001-stack.md` l. 117, 143, `docs/brique0/ingestion-votes.md` l. 234 |

L'amplitude ancrée garde le mot : 4 contre 3, et surtout c'est la seule échelle
publiée.

**Ce que ce n'est pas.** L'amplitude n'est pas l'**étendue**, qui n'est pas
publiable : ses deux bornes sont les coordonnées de deux membres identifiables
du groupe (`docs/methode.md` §2, `docs/adr/0003-arbitrages-de-coherence.md` §3).

**Interdit :** `amplitude de l'axe`, `échelle brute`

**À migrer :** les trois emplois de 2,6 à qualifier `amplitude brute`, et le
« 28 % de l'axe » de `docs/brique0/ingestion-votes.md` l. 234, qui vaut 36 %
dans l'échelle publiée (défaut A8 de l'audit).

---

### unités brutes

**Définition.** Les unités brutes sont les unités de l'axe **avant** la
transformation affine d'ancrage. Elles ne sont ni publiables, ni comparables
d'une exécution à l'autre.

**Emploi attesté.** Un seul : `docs/brique0/positionnement.md` l. 386 (« moins
de 0,004 en unités brutes »). L'entrée existe malgré cet emploi unique parce
qu'elle est la contrepartie d'`amplitude brute` : sans elle, « brute » n'a pas
de référent.

**Ce que ce n'est pas.** À opposer aux **unités ancrées**, 7 emplois, seule
échelle publiée (`docs/ton.md` §3, `RG-44`).

---

### gain du rang 1

**Définition.** Le gain du rang 1 est la part de la somme des carrés du
**résidu après constante par scrutin** que le terme de rang 1 absorbe. Le
dénominateur est inscrit dans le nom : **gain du rang 1 sur le résidu après
constante par scrutin**.

**Emplois attestés.** 10 emplois du syntagme.

| Quantité | Valeur | Emplois |
|---|---|---|
| gain sur le résidu après constante par scrutin | 60,8 %, acté par `docs/adr/0003-arbitrages-de-coherence.md` §3 | **3** — `docs/brique0/verification-2026-08-27.md` §3, `docs/adr/0001-stack.md` l. 132, `docs/brique0/ingestion-votes.md` l. 452 |
| même quantité, mesure antérieure | 59,1 % | **7** — `docs/brique0/positionnement.md` §1 mesure 9, §5, §11, `docs/brique0/plan-de-tests.md` §1 et §16 |
| gain sur la **variance totale** | 51,5 % | **2** — `docs/brique0/verification-2026-08-27.md`, `docs/adr/0001-stack.md` l. 132 |

Ici le nombre ne décide pas : c'est l'ADR 0003 §3 qui acte 60,8 %, et le
décompte 7 contre 3 mesure exactement l'ampleur de la propagation restant à
faire (défaut A1 de l'audit, ticket ouvert).

**Ce que ce n'est pas.** Ce n'est pas une « part de variance expliquée » :
`docs/ton.md` §3 écarte cette forme, et les deux dénominateurs — résidu et
variance totale — ne donnent pas le même nombre.

**Interdit :** `score de position`, `note de position`

---

### dispersion

**Définition.** La dispersion d'un groupe est le couple {**écart interquartile
(IQR)**, **écart-type de rééchantillonnage**}, et rien d'autre
(`docs/ton.md` §3, `docs/methode.md` §2).

**Emplois attestés.** 47 emplois, dont 8 sous la forme canonique « dispersion
intra-groupe » (`docs/ton.md` §3, `docs/methode.md` §2,
`docs/adr/0003-arbitrages-de-coherence.md` §3, `RG-44`).

**Ce que ce n'est pas.** Ni la **variance**, illisible sur un axe sans unité ;
ni l'**étendue**, dont les bornes identifient deux personnes ; ni l'**écart-type
intra-groupe**, écarté par `RG-42`, `docs/ton.md` §2 et
`docs/adr/0003-arbitrages-de-coherence.md` §3 point 4.

**À migrer :** 7 emplois de « écart-type intra-groupe » ou « variance
intra-groupe » subsistent — `docs/adr/0001-stack.md` l. 117, 142-143,
`docs/brique0/registre-entites.md` §2.1 et §4.2, `ROADMAP.md`,
`docs/methode.md`. Celui de `registre-entites.md` fonde l'exception V13 sur un
chiffre que le dépôt déclare périmé (défaut A9 de l'audit).

---

### position

**Définition.** Le mot `position` ne s'emploie jamais nu. Cinq concepts.

| Désignation | Concept | Emplois |
|---|---|---|
| **position exprimée** | valeur d'une cellule : `pour` +1, `contre` −1, `abstention` 0 | **8** |
| **position estimée** | coordonnée d'un acteur sur l'axe, non publiable individuellement | **13** |
| **position publiée** | valeur d'un groupe dans un artefact, avec sa date et sa famille de mesure ; forme canonique **positionnement daté** | **7** |
| **position majoritaire** | sens majoritaire d'un groupe sur un scrutin | **5** |
| **position non publiée** | mesure calculée dont la position n'est pas publiée, la dispersion ou l'effectif étant hors des conditions de publication ; s'énonce avec le chiffre mesuré et le seuil en regard | **2** (LIOT, NI, instantané `an17-2026-07-21`) |

Le total de 293 occurrences du mot inclut ses emplois ordinaires et le nom du
document `positionnement.md` ; seuls les 35 emplois qualifiés ci-dessus sont
techniques.

**Ce que ce n'est pas.** Une position n'est ni une note, ni un rang, ni un
classement. `docs/ton.md` §3 canonise « position » et « positionnement daté »
pour le seul résultat publié.

Et une **position non publiée** n'est pas une position **manquante** : la ligne
de preuve existe, elle porte sa méthode, ses entrées et ses empreintes, et elle
se vérifie comme n'importe quelle autre. Seule la valeur est retenue, avec les
chiffres qui justifient qu'elle le soit. Les confondre revient à présenter un
refus motivé comme un trou dans les données.

---

## 4. Contrôle et artefacts

### porte

**Définition.** Une porte est un **contrôle dont l'échec arrête l'exécution ou
la publication**. Un contrôle qui signale sans arrêter n'est pas une porte.

**Emplois attestés.** Environ 35 emplois nominaux, dans quatre compositions,
et le terme n'était défini nulle part.

| Composition | Ce qu'elle arrête | Fondement |
|---|---|---|
| **porte de complétude** | la récupération d'une archive : la taille annoncée, jamais le MD5 du producteur | `RG-114`, `docs/architecture.md` l. 62, `docs/sources.md` §3, `docs/brique0/plan-de-tests.md` l. 119 |
| **porte d'entrée dans la matrice** | l'inclusion d'un scrutin — `nombreVotants` n'en est **jamais** une | `RG-15`, `docs/methode.md` §1, `ROADMAP.md` l. 32, `docs/adr/0003-arbitrages-de-coherence.md` §2 |
| **porte bloquante d'intégration continue** | la fusion d'une PR ; elle exige un travail en face dans `.github/workflows/` | `RG-108`, `docs/regles-de-gestion.md` l. 187, `scripts/portes-de-ci.sh` |
| **porte de couverture** | la publication au titre d'un taux de couverture mesuré | `docs/brique0/plan-de-tests.md` §15, `README.md` l. 56 |

**Ce que ce n'est pas.** Une porte n'est pas un avertissement. Une porte
déclarée sans travail en face est **réputée inexistante**
(`docs/regles-de-gestion.md` l. 187) — et une porte dont l'échec est avalé par
un `|| true` est pire qu'absente, puisqu'elle affirme le vert.

**Interdit :** `porte souple`, `porte molle`, `porte indicative`

---

### instantané

**Définition.** Un instantané est un artefact publié
`public/api/instantanes/<id>.json` (`docs/ton.md` §3).

**Emplois attestés.** 48 emplois, dans trois sens.

| Concept | Emplois | Nom retenu |
|---|---|---|
| artefact publié | **plus de 20** — `docs/brique0/contrats.md` §2 à §10, `docs/architecture.md` l. 146, `docs/ton.md` §3, `README.md` l. 22, `ROADMAP.md` l. 81 | **instantané** — garde le mot |
| référence figée d'un test `insta` | **au moins 8** — `docs/tdd.md` l. 56-59, `docs/brique0/plan-de-tests.md` l. 44, 74, 89, 321, 566-568, `docs/brique0/positionnement.md` §9, `docs/adr/0001-stack.md` l. 330, 516 | **instantané de test** |
| état daté d'une source | **1** — `docs/adr/0001-stack.md` l. 190 (« un instantané pris à l'ouverture de la législature ») | **état figé** |

**Ce que ce n'est pas.** Un instantané de test n'est pas un artefact : il ne
sort jamais de `pipeline/tests/`. Trois emplois écrivent déjà « instantané
figé », qui est la forme la plus proche de la cible.

**Interdit :** `instantané de référence`

**À migrer :** les emplois du sens « test », par substitution
d'`instantané de test` ou `référence figée`.

---

### contrat

**Définition.** Un contrat est le **contrat de sortie** : la description
versionnée des artefacts publiés, en majeure / mineure / patch
(`docs/adr/0000-perimetre-brique0.md` §6, `docs/ton.md` §3).

**Emplois attestés.** 114 emplois du mot, dont **19** sous la forme canonique
« contrat de sortie » (`docs/regles-de-gestion.md`, `docs/ton.md` §3,
`docs/brique0/contrats.md`, `docs/brique0/registre-entites.md`,
`CONTRIBUTING.md`). Versions citées : `0.2.0`, `0.3.0`.

**Concurrence.** Deux autres emplois, chacun minoritaire :

- le **schéma de sérialisation** partagé entre Rust et TypeScript
  (`docs/adr/0001-stack.md` §6, `docs/brique0/plan-de-tests.md`) — 1 à 2
  emplois, nom proposé : **schéma de sérialisation** ;
- « le registre de preuves est le contrat » — tournure figurée, à reformuler.

19 contre 2 : le contrat de sortie garde le mot.

**Ce que ce n'est pas.** Le fichier `docs/brique0/contrats.md` porte les deux
premiers sens ; son nom au pluriel n'autorise pas le mot nu à l'intérieur.

**Interdit :** `contrat interne`

---

## 5. Termes écartés de ce glossaire

Un terme dont l'emploi réel n'est pas établi n'entre pas ici.

| Terme candidat | Pourquoi il n'a pas d'entrée |
|---|---|
| **niveau 1 / 2 / 3** | il est **déjà défini**, au §2 de `docs/brique0/plan-de-tests.md`, et sa définition n'est pas contestée. Le défaut est un renvoi manquant depuis `docs/definition-of-done.md`, `docs/tdd.md`, `docs/architecture.md` et `docs/regles-de-gestion.md` — un ticket de renvoi, pas une entrée de glossaire. Lui en donner une créerait la seconde définition que ce document interdit. |

---

## 6. Reproduire les décomptes

Les décomptes d'occurrences de ce document sont obtenus ainsi, depuis la racine
du dépôt :

```sh
LC_ALL=C git ls-files -z '*.md' | LC_ALL=C xargs -0 grep -ioE -- 'cellules? (observ|absent|exprim)[a-zé]*' | wc -l
```

en substituant le motif de la ligne à mesurer. Les motifs de classement par
sens sont ceux des tableaux ci-dessus, appliqués aux 34 fichiers Markdown
suivis. `A VERIFIER` : les décomptes sont exacts au 2026-08-28 et se
déplaceront avec les corrections en cours ; ce sont des ordres de grandeur
d'arbitrage, pas des valeurs publiées.

Le contrôle automatique est `scripts/glossaire.sh`.
