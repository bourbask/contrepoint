# 0001 — Pile technique

Statut : **accepté**. Date : 2026-08-27. Portée : brique 0, avec l'obligation
d'accueillir les briques 1 à 3 sans changer de langage.

Décide : langages, dépendances, forme des artefacts, chaîne de publication.
Ne décide pas : le modèle de données du registre d'entités, ni le seuil de
participation. Ces deux points sont méthodologiques, pas techniques.

---

## 1. Ce qui a été mesuré avant de choisir

Aucune ligne de ce document ne repose sur une estimation. Les chiffres viennent
de téléchargements et d'exécutions effectués le 2026-08-27 sur les sources
réelles. Chaque mesure est reproductible par la commande citée.

| # | Mesure | Valeur constatée |
|---|---|---|
| 1 | `Scrutins.json.zip`, législature 17 | 26 317 479 octets, 181 Mio décompressés, **8 434 fichiers** |
| 2 | Acteurs distincts apparaissant dans les scrutins | **642** |
| 3 | Matrice `acteur × scrutin` | 642 × 8 434 = 5,41 M cellules |
| 4 | Cellules observées | **1 247 093, soit 23,0 %** |
| 5 | `nombreVotants` par scrutin | p25 = 89, **médiane = 133**, p75 = 192, max = 574 |
| 6 | Parsing intégral des 8 434 fichiers (Python, `json` stdlib) | **1,2 s** |
| 7 | ALS rang 1 sur les cellules observées, 300 itérations (Rust sans dépendance) | **5,2 s** |
| 8 | Reproductibilité de cette sortie sur 3 exécutions | md5 identique, empreinte FNV `0x6e58dc07d24905de` |
| 9 | `votant` sérialisé comme objet nu au lieu d'un tableau | **35 671 blocs sur 130 851, soit 27 %** |
| 10 | `acteur.uid` dans AMO30 | **objet** `{"@xsi:type":…,"#text":"PA304016"}` — alors que `organe.uid` est une chaîne |
| 11 | Groupes politiques de la législature 17 résolus depuis AMO30 | **14**, avec `viMoDe.dateDebut` / `dateFin` |
| 12 | Députés à plus d'un groupe distinct sur la législature 17 | **633 sur 648** |
| 13 | Somme flottante parallèle `rayon` sur entrée fixe, 40 exécutions | **3 représentations binaires distinctes**, aucune égale au séquentiel |

Commandes de vérification :

```sh
# 1, 2, 3, 4, 5, 9
curl -O https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip
unzip -l Scrutins.json.zip | tail -3

# 10, 11, 12 — le référentiel frais est AMO30, pas AMO50
curl -C - -O https://data.assemblee-nationale.fr/static/openData/repository/17/amo/\
tous_acteurs_mandats_organes_xi_legislature/\
AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip
```

### 1.1 Le volume n'est pas le problème

181 Mio et 8 434 fichiers se parsent en 1,2 s avec la bibliothèque standard de
Python — le langage le plus lent des candidats. La performance de parsing ne
départage aucune pile. Toute exigence formulée en ces termes est un faux
critère, et le choix doit se faire ailleurs.

Corollaire : `polars` (0.55.2) et `arrow` (59.2.0) sont hors sujet pour ce
projet. Ils traitent des volumes deux à trois ordres de grandeur au-dessus.

### 1.2 La densité est le problème

23,0 % des cellules sont observées. Les 77 % restants ne sont **pas des zéros** :
ce sont des absences, et `methode.md` interdit de les traiter comme des
abstentions — les assimiler fabrique un centrisme artificiel.

Cela disqualifie toute ACP ou SVD classique, qui exige une matrice complète.
Le filtre de participation ne sauve rien : la médiane est à 133 votants sur
577 sièges, et le corpus s'effondre dès que le seuil est relevé.

| Seuil `nombreVotants` | Scrutins retenus | Part du corpus |
|---|---|---|
| ≥ 100 | 5 789 | 68,6 % |
| ≥ 150 | 3 581 | 42,5 % |
| ≥ 200 | 1 912 | 22,7 % |
| ≥ 300 | 359 | 4,3 % |
| ≥ 400 | 98 | 1,2 % |

Il n'existe pas de seuil qui produise à la fois une matrice dense et un corpus
utilisable. **L'estimateur doit donc travailler sur les cellules observées
uniquement.** C'est une contrainte d'architecture, pas un raffinement.

La conséquence est structurante : l'algèbre linéaire dont le projet a besoin
n'est pas « SVD d'une matrice creuse », c'est « moindres carrés alternés rang 1
sur une liste de triplets ». Ce n'est pas la même bibliothèque, et en pratique
ce n'est aucune bibliothèque.

### 1.3 L'estimateur fonctionne, et son signe est bien aléatoire

Modèle ajusté sur les cellules observées, avec une constante par scrutin :
`valeur ≈ b[scrutin] + x[député] × y[scrutin]`. Le premier axe `x`, agrégé par
groupe au 2026-07-21 :

> **Chiffres d'exploration.** Ce tableau a servi à départager les piles. Il est
> remplacé par `../brique0/positionnement.md` §6, qui est la spécification
> retenue : les effectifs y diffèrent, le rattachement n'étant pas fait de la
> même façon. Le recomptage du 2026-08-27 confirme les effectifs de
> `positionnement.md`, pas ceux-ci.

| Groupe | n | Moyenne | Écart-type intra |
|---|---|---|---|
| LFI-NFP | 71 | −1,439 | 0,046 |
| ECOS | 38 | −1,427 | 0,047 |
| GDR | 17 | −1,290 | 0,105 |
| SOC | 68 | −1,223 | 0,095 |
| LIOT | 23 | −0,043 | 0,422 |
| DEM | 37 | 0,105 | 0,246 |
| EPR | 91 | 0,178 | 0,254 |
| HOR | 35 | 0,369 | 0,203 |
| NI | 10 | 0,414 | 0,649 |
| DR | 48 | 0,791 | 0,172 |
| RN | 122 | 1,179 | 0,065 |
| UDDPLR | 17 | 1,189 | 0,062 |

Trois constats, tous vérifiés, tous à afficher sur le site :

**L'ordre est le bon.** Il n'a été soufflé nulle part : aucune étiquette n'entre
dans le calcul, l'axe émerge des votes seuls. Le résultat est un argument de
validité de la méthode, pas un habillage.

**L'écart-type intra-groupe est dérisoire** — 0,046 à 0,25 pour une amplitude
inter-groupes de 2,6 — sauf pour NI (0,649) et LIOT (0,422), qui ne sont pas des
groupes idéologiques. C'est la confirmation empirique de la limite annoncée dans
`methode.md` : l'axe sépare les blocs, pas les individus. Le chiffre existe
maintenant pour l'écrire sur le site plutôt que de l'affirmer.

**Le signe est bien indéterminé.** Sur 6 graines d'initialisation, la
corrélation avec la solution de référence vaut exactement ±1,000000 : l'axe est
unique et parfaitement reproductible, mais **3 graines sur 6 le renvoient
inversé**. Le point d'ancrage exigé par la roadmap n'est pas une précaution
théorique, c'est une nécessité constatée. Deux implémentations indépendantes
(NumPy et Rust sans dépendance) donnent une corrélation de −1,000000 : même
axe, signe opposé.

**Ce que l'axe explique.** Le gain du terme rang 1 au-delà d'une simple constante
par scrutin est de **60,8 % du résidu**, soit 51,5 % de la variance totale — la
constante par scrutin n'en expliquant que 15,2 %. L'axe n'est donc pas
marginal : il porte l'essentiel de ce qui reste une fois le sens du vote retiré.

> Une version antérieure de ce document annonçait **2,1 %**, chiffre faux d'un
> facteur trente et propagé dans deux autres documents. Recompté le 2026-08-27
> sur l'archive complète, protocole et commandes dans
> [../brique0/verification-2026-08-27.md](../brique0/verification-2026-08-27.md).

Cela ne lève pas la limite de séparation : l'axe explique bien le comportement de
vote, mais il le fait en séparant les blocs, pas les individus — la dispersion
intra-groupe reste dérisoire devant l'amplitude.

### 1.4 Deux pièges de parsing, mesurés

Le JSON de l'Assemblée est une transcription automatique de XML. Elle perd de l'information, et de deux façons qui cassent tout désérialiseur typé naïf :

**Un tableau à un élément devient un objet nu.** `decompteNominatif.*.votant`
est un tableau dans 95 180 blocs et un objet nu dans **35 671** — 27 % des cas.
Un `Vec<Votant>` échoue sur plus d'un quart du corpus, et il échoue justement
sur les scrutins les plus serrés, ceux où un groupe n'a qu'un votant dans une
position. Il faut un adaptateur « un-ou-plusieurs » sur **chaque** champ
répétable, appliqué systématiquement et non au cas par cas.

**Le même nom de champ porte deux formes.** `organe.uid` est une chaîne.
`acteur.uid` est un objet `{"@xmlns:xsi":…, "@xsi:type":"IdActeur_type",
"#text":"PA304016"}`. Il faut un adaptateur « chaîne ou objet enveloppé xsi ».

Les nombres sont tous sérialisés en chaînes (`"nombreMembresGroupe": "123"`).

Ces trois adaptateurs sont la seule vraie complexité de l'ingestion. Ils sont
identiques dans les trois piles envisagées et ne les départagent pas — mais ils
plaident pour un langage à typage statique, où l'adaptateur est écrit une fois
et le compilateur signale le champ oublié.

> À VÉRIFIER : la variante XML (`Scrutins.xml.zip`) n'a pas ces
> ambiguïtés par construction, l'arité étant portée par le schéma. Elle est
> peut-être moins coûteuse à parser correctement, malgré l'a priori inverse.
> Vérification : parser les deux et comparer le nombre de cas particuliers.

### 1.5 Le téléchargement de l'Assemblée tronque

`AMO50…json.zip` annonce `content-length: 14342787` et a livré 3 050 645 puis
6 215 677 octets sur deux tentatives consécutives : le serveur ferme la
connexion en cours de transfert, sans erreur. Une archive tronquée est un
`unzip` en échec — bruyant, donc bénin. Mais un pipeline qui la traite comme
une absence de données publierait un paysage politique amputé.

Le serveur envoie `accept-ranges: bytes`. La reprise (`curl -C -`) a complété le
fichier du premier coup.

**Obligation retenue.** Tout téléchargement vérifie la taille reçue contre
`content-length`, reprend tant qu'elle diffère, puis enregistre le SHA-256 de
l'archive dans le registre de preuves. Un jeu de données dont l'empreinte est
inconnue ne rentre pas dans le pipeline. C'est aussi ce qui rend une position
rejouable à l'identique des mois plus tard.

**Piège de fraîcheur, découvert au passage.** `AMO50_acteurs_mandats_organes_divises`
porte `last-modified: 2024-07-11` : c'est un instantané pris à l'ouverture de la
législature. Il ne contient **pas** `PO845401` (groupe RN), pourtant référencé
par les scrutins de 2025. Le référentiel à utiliser est
`AMO30_tous_acteurs_tous_mandats_tous_organes_historique`
(`last-modified: 2026-08-27`, 13 600 736 octets), qui résout les 14 groupes de
la législature 17 avec leurs périodes de validité.

Les périodes ne sont pas décoratives : AD est dissous le 2024-09-11, UDR est actif du
2024-09-12 au 2025-09-04, UDDPLR est créé le 2025-09-05. Et 633 députés sur 648
relèvent de plus d'un groupe — pour l'essentiel le passage de « non inscrit » au
groupe constitué le 2024-07-18. Une jointure « dernier groupe connu »
attribuerait tout scrutin du 8 au 18 juillet 2024 à des non-inscrits.

### 1.6 Le parallélisme casse le déterminisme bit-à-bit

L'addition flottante n'est pas associative. `rayon` découpe le travail selon le
nombre de cœurs disponibles et le vol de tâches, donc l'ordre de réduction varie
d'une exécution à l'autre.

Constaté : `par_iter().sum()` sur un vecteur fixe de 200 000 termes produit
**3 représentations binaires distinctes sur 40 exécutions**, dont **aucune**
n'égale la somme séquentielle.

C'est incompatible avec la promesse du README (« même entrée, même sortie, bit
pour bit »). **Règle retenue : aucune réduction flottante parallèle sur le
chemin déterministe.** `rayon` reste admissible pour ce qui n'agrège pas de
flottants — décompression, parsing de fichiers indépendants — jamais pour une
somme, une moyenne ou un produit scalaire. La mesure 7 montre que la question ne
se pose pas : 5,2 s en mono-fil.

### 1.7 « Bit pour bit » est relatif à une implémentation épinglée

Même SVD, même matrice 4 × 3, trois bibliothèques :

| Implémentation | Valeurs singulières |
|---|---|
| `nalgebra` 0.35.0 | 1.8477590650225735, 1.4142135623730951, 0.7653668647301795 |
| `gonum` v0.17.0 | 1.8477590650225735, 1.414213562373095, 0.7653668647301798 |
| `ml-matrix` 6.15.0 | 1.8477590650225737, 1.4142135623730951, 0.7653668647301796 |

Les trois diffèrent sur les derniers bits. La reproductibilité bit-à-bit est
donc une propriété du couple (implémentation, version), jamais une propriété du
résultat mathématique.

Deux conséquences, non négociables :

- le registre de preuves consigne la version du langage, de la bibliothèque de
  calcul et la cible de compilation, au même titre que la date de la source ;
- la suite de tests compare des sorties à une **tolérance déclarée** pour ce qui
  relève de la validité méthodologique (ordre des groupes, signe, corrélation),
  et à l'**octet** seulement pour la non-régression d'une version donnée.

Promettre l'égalité binaire entre deux implémentations serait une promesse
fausse. Promettre qu'une version donnée redonne exactement le même fichier est
tenable, et c'est ce qui est promis.

### 1.8 La contrainte d'algèbre linéaire, tranchée

L'exigence était : aucune dépendance de calcul ne doit réclamer une
bibliothèque système. Vérifié par compilation et par `ldd` :

| Dépendance | Version | Maintenance | Décision |
|---|---|---|---|
| `nalgebra` | 0.35.0 | 2026-05-24 | retenue — compile et s'exécute ; `ldd` ne montre que `libc`, `libm`, `libgcc` |
| `sprs` | 0.11.5 | 2026-07-26 | retenue — pur Rust — tire une seconde copie de `nalgebra` (0.34.2) |
| `linfa-linalg` | 0.2.1 | 2025-05-15 | retenue — pur Rust |
| `linfa` + `linfa-reduction` | 0.8.1 | 2025-12-23 | retenue — compile ; l'ACP passe par `linfa-linalg`, **pas** par `ndarray-linalg` |
| `ndarray` | 0.17.2 | 2026-01-10 | retenue — pur Rust |
| `gonum.org/v1/gonum` | v0.17.0 | — | retenue — compile avec `CGO_ENABLED=0`, binaire statique |
| `ml-matrix` | 6.15.0 | 2026-08-05 | retenue — JS pur, 2 dépendances transitives |
| `ndarray-linalg` | 0.18.1 | 2026-01-12 | **disqualifiée** |

**Pourquoi `ndarray-linalg` est disqualifié, précisément.** Le piège est plus
vicieux qu'un échec de compilation. Sans fonctionnalité d'implémentation sous-jacente, la crate
compile sans erreur — mais la méthode `.svd()` **n'existe pas** : l'échec est un
`E0599: no method named 'svd'`, pas un défaut d'édition de liens. Un
développeur peut ajouter la dépendance, voir la compilation réussir, et ne
découvrir le problème qu'en écrivant l'appel. Et les 19 fonctionnalités d'implémentation
sous-jacente disponibles se réduisent à trois familles — `openblas`, `netlib`,
`intel-mkl` — toutes en `-system` (bibliothèque installée) ou `-static`
(compilation depuis les sources, exigeant `cmake` et un compilateur Fortran ;
`gfortran` est absent de la machine de développement). Aucune n'est acceptable
pour une CI à construction reproductible.

`ml-pca` (4.1.1, publié en 2022-11-11) est écarté sur la maintenance, et de
toute façon inutilisable ici : comme toute ACP sur étagère, il exige une matrice
complète.

**Le vrai résultat de cette section.** L'estimateur nécessaire — moindres carrés
alternés rang 1 sur cellules observées — s'écrit en **environ 60 lignes sans
aucune dépendance**, tourne en 5,2 s sur la matrice réelle, et rend une
empreinte identique sur 3 exécutions avec une initialisation déterministe (pas
de générateur aléatoire, donc pas de problème de graine). Il n'accepte pas de
matrice dense, ce qui est exactement ce qu'il faut.

**Aucune des bibliothèques d'algèbre linéaire validées ci-dessus n'est requise
pour la brique 0.** La contrainte « pas de LAPACK système » est satisfaite de la
manière la plus solide possible : pas de bibliothèque du tout. Elles restent
utiles à connaître pour les briques 1 et 2, où l'ACP de l'espace de citation
porte sur une matrice `rédaction × acteur` petite et dense — là, une SVD sur
étagère est légitime.

---

## 2. Ce que les mesures imposent

Avant de comparer les piles, les critères réels, dans l'ordre :

1. **Typage statique.** Les trois adaptateurs de la section 1.4 sont la seule
   complexité de l'ingestion. Un langage qui signale le champ manquant à la
   compilation les rend sûrs ; un langage dynamique les transforme en dette.
2. **Un seul langage du pipeline aux briques 1-3.** TF-IDF, cosinus, clustering
   agglomératif et log-odds ratio doivent arriver sans changer de pile.
3. **Contrôle de l'ordre des opérations flottantes.** Exigé par 1.6 et 1.7.
4. **Dépendances minimales et à construction reproductible.** Satisfait de fait, la
   section 1.8 ayant vidé le besoin.
5. **Sortie en fichiers statiques versionnés.** GitHub Pages, zéro serveur.
6. **Valeur sur le marché de l'emploi.** Contrainte d'entrée du propriétaire.
7. **Performance.** Dernier critère. 1,2 s de parsing et 5,2 s de calcul :
   aucune pile ne peut échouer ici.

Python est écarté sans débat : le propriétaire ne l'aime pas, et les critères 1,
3 et 6 lui sont défavorables. Il reste l'outil de vérification exploratoire hors
dépôt — c'est son rôle dans ce document, pas dans le produit.

---

## 3. Proposition A — Rust pour le pipeline, React + TypeScript pour le front

Un binaire Rust unique, invoqué par GitHub Actions, qui écrit du JSON versionné.
Un front React/TS compilé en statique par Vite, qui lit ces fichiers.

**Dépendances du pipeline**, toutes vérifiées le 2026-08-27 :

| Crate | Version | Dernière publication | Rôle |
|---|---|---|---|
| `serde` | 1.0.229 | 2026-07-18 | dérivation des structures |
| `serde_json` | 1.0.151 | 2026-07-20 | lecture des scrutins et du référentiel |
| `sha2` | 0.11.0 | 2026-03-25 | empreinte des archives (1.5) |
| `csv` | 1.4.0 | 2025-10-17 | CHES, Manifesto |
| `insta` | 1.48.0 | 2026-06-11 | tests par instantané, hors ligne |

Calcul : **aucune dépendance**, l'ALS de la section 1.8.

Téléchargement et décompression délégués à `curl -C -` et `unzip`, présents sur
les exécuteurs GitHub Actions. `ureq` (3.4.0) et `zip` (8.6.0 stable — la
version la plus récente, 9.0.0-pre3, est une préversion) sont ainsi évités : la
reprise sur troncature exigée par 1.5 est un drapeau de `curl`, et serait du
code à écrire et à tester dans le binaire.

`quick-xml` (0.42.0, 2026-08-22) est la porte de sortie si la vérification 1.4
conclut en faveur du XML, et sera nécessaire pour les flux RSS de la brique 1.

**Front** : `react` 19.2.8, `react-dom` 19.2.8, `typescript` 7.0.2, `vite`
8.2.2. Graphe en SVG écrit à la main, échelles par `d3-scale` (4.0.2).

Le choix du SVG manuel n'est pas de l'ascétisme. Le graphe demandé — une bande
par parti, **trois marqueurs jamais moyennés**, curseur temporel, clic vers les
preuves — n'est aucun type de graphique standard. `echarts` (6.1.0) et
`@observablehq/plot` (0.6.17) fournissent des familles de graphiques ; ici il
faut refuser la moyenne que toute bibliothèque de graphiques propose par défaut,
et garder la main sur l'ordre du DOM pour l'accessibilité. `d3-scale` seul
apporte les échelles sans imposer de rendu. `d3` complet (7.9.0, dernière
publication 2024-03-12) n'est pas nécessaire.

**Goulots anticipés.** Aucun sur les données. Le seul coût réel est le temps de
compilation en CI, à couvrir par le cache de `~/.cargo` et `target/`.
Le second est humain : Rust est le langage le plus lent à écrire des trois, et
l'ingestion est du remodelage de JSON irrégulier — le domaine où le typage
statique coûte le plus cher avant de rapporter.

**Coût de mise en place.** Moyen. `cargo init`, quatre dépendances, les trois
adaptateurs. La partie déjà démontrée dans ce document — parsing, ALS,
reproductibilité — a été écrite et exécutée : ce n'est pas une hypothèse.

**Valeur sur le marché.** Le propriétaire a déjà plusieurs projets Rust : la
pile ne démontre rien de neuf sur ce plan, elle consolide. React + TypeScript
est la combinaison front la plus demandée, et c'est la moitié de la pile où il a
le moins à montrer. Le vrai actif employable de ce projet n'est d'ailleurs pas
le langage : c'est un pipeline déterministe, testé hors ligne, avec un registre
de preuves et une méthode écrite. Cela se raconte en entretien quel que soit le
langage.

> À VÉRIFIER : toute affirmation quantitative sur les rémunérations. Aucune
> n'est avancée ici faute de source vérifiée. À étayer par les enquêtes
> salariales publiées si le sujet doit être tranché sur des chiffres.

**Ce qui est perdu.** Deux langages, donc deux chaînes d'outils, deux caches de CI,
deux ensembles de conventions. Et l'itération la plus lente des trois sur le
code d'ingestion.

---

## 4. Proposition B — Go pour le pipeline, React + TypeScript pour le front

Identique à A, `gonum` remplaçant Rust.

**Vérifié** : `gonum.org/v1/gonum` v0.17.0 compile avec `CGO_ENABLED=0` et
produit un binaire statique (`ldd` : « n'est pas un exécutable dynamique »).
`mat.SVD` fonctionne. Aucune bibliothèque système. La contrainte est satisfaite
aussi proprement qu'en Rust.

`encoding/json` de la bibliothèque standard suffit à l'ingestion, avec des
`UnmarshalJSON` personnalisés pour les adaptateurs de 1.4.

**Ce qui plaide pour.** Compilation quasi instantanée, donc CI plus simple et
boucle d'itération plus courte que Rust sur du remodelage de JSON. Un seul
binaire statique. Bibliothèque standard large : HTTP avec reprise par en-tête
`Range` et `archive/zip` sont dedans, ce qui permet d'internaliser le
téléchargement sans dépendance, si la dépendance à `curl` est écartée.
`gonum` couvre aussi les besoins des briques 1-2 (`stat`, distances).

**Goulots anticipés.** Aucun sur les données. La gestion des données manquantes
en Go est plus verbeuse qu'en Rust — `Option<T>` contre pointeur nul — et
l'absence est ici le cœur du sujet méthodologique (mesure 4). C'est précisément
le point où le système de types de Rust paie sa lenteur d'écriture.

**Coût de mise en place.** Faible, le plus bas des trois côté pipeline.

**Valeur sur le marché.** Go est un langage que le propriétaire connaît sans
l'avoir mis en avant autant que Rust — la pile démontre donc quelque chose de
neuf. `A VERIFIER` : le volume de débouchés Go contre Rust en infrastructure et
en pipeline de données ; vérifiable par les enquêtes salariales et les offres
publiées, non tranché ici.

**Ce qui est perdu.** Le système de types qui rend l'absence explicite, là où
l'absence est le sujet. Et, à titre subjectif mais légitime, le langage que le
propriétaire préfère.

---

## 5. Proposition C — TypeScript de bout en bout

Un seul langage. Pipeline exécuté par Node en CI, front React/TS, types du
registre d'entités partagés entre les deux.

**Vérifié** : `ml-matrix` 6.15.0 (2026-08-05, deux dépendances transitives :
`is-any-array`, `ml-array-rescale`) fournit une SVD en JS pur qui fonctionne.
Node est mono-fil par défaut, donc l'ordre des réductions flottantes est stable
et le problème de 1.6 ne se pose pas. Aucune bibliothèque système, par
construction.

**Ce qui plaide pour.** Un langage, une chaîne d'outils, un cache de CI, un
`package.json`. Les types du registre d'entités — le contrat des briques 1 à 3,
et selon la roadmap le point où une erreur se paie partout — sont écrits une
fois et partagés littéralement entre calcul et affichage, sans génération de
code ni duplication. C'est le seul candidat qui offre cela, et l'argument n'est
pas mince.

**Goulots anticipés.** Le typage de TypeScript s'évanouit à l'exécution : les
trois adaptateurs de 1.4 exigent une validation à la frontière, donc du code de
vérification à écrire là où Rust et Go l'obtiennent du compilateur. Sur des
données réputées irrégulières, c'est le mauvais endroit pour économiser. Le
parsing de 181 Mio reste sans difficulté, et l'ALS en JS sera plus lent que 5,2 s
sans que cela importe.

**Coût de mise en place.** Le plus bas des trois au total, un seul écosystème.

**Valeur sur le marché.** TypeScript est la compétence la plus commune des
trois, donc la moins distinctive (`A VERIFIER` : même vérification que ci-dessus) : un
pipeline de données en TypeScript se remarque moins qu'un pipeline en Rust ou en
Go.

**Ce qui est perdu.** La garantie statique là où elle sert le plus. Et le projet
cesse de démontrer une compétence de traitement de données hors du navigateur,
ce qui est une part de son intérêt.

---

## 6. Recommandation

**Proposition A — Rust pour le pipeline, React + TypeScript pour le front.**

Les trois piles satisfont les contraintes dures : aucune ne réclame de
bibliothèque système, aucune ne butera sur les volumes, toutes publient en
statique. Le choix se fait donc sur les critères mous, et trois arguments
tranchent.

**L'absence est le sujet du projet, pas un cas limite.** 77 % de la matrice est
manquante, et `methode.md` interdit de combler. Un langage qui rend l'absence
impossible à ignorer accidentellement est aligné sur la contrainte
méthodologique centrale. C'est l'argument principal, et il est méthodologique
avant d'être technique.

**Le déterminisme se déclare mieux qu'il ne s'espère.** 1.6 et 1.7 montrent que
la promesse du README est fragile et dépend de choix explicites — pas de
réduction parallèle, versions épinglées, tolérances déclarées. Rust rend ces
choix visibles : `rayon` est une ligne de `Cargo.toml` qui est délibérément non
écrire, et `Cargo.lock` épingle l'arbre entier.

**Le propriétaire écrira effectivement ce code.** Contrainte « pas de temps
libre » : la pile qu'il aime est celle qui a une chance d'être finie. Go serait
défendable, et plus rentable sur la démonstration de compétence neuve — mais le
risque nommé en tête de la roadmap est l'abandon, pas le mauvais choix de
langage.

Et le socle est déjà validé : le parsing des 8 434 fichiers, l'ALS, la
reproductibilité bit-à-bit et l'ordre gauche-droite ont été exécutés en Rust
pour écrire ce document. La proposition A n'est pas un pari.

**Ce que cette recommandation concède.** Deux langages plutôt qu'un. Les types
du registre d'entités seront définis deux fois — une fois en Rust, une fois en
TypeScript — sans garantie de compilateur entre les deux. C'est la faiblesse
réelle de A face à C, sur le fichier que la roadmap désigne comme le plus
dangereux du projet. Elle se couvre ainsi : le registre est un fichier de
données versionné, pas du code ; le pipeline en publie un schéma JSON, et le
front échoue bruyamment à la construction si le schéma ne correspond pas. Le
contrat est vérifié à l'exécution de la CI plutôt qu'à la compilation. C'est
moins bon, c'est suffisant, et c'est écrit ici pour être réévalué si le registre
devient effectivement une source de bugs.

---

## 7. Plan de mise en place

Ordonné pour que chaque étape produise une sortie visible, conformément à la
roadmap. L'étape 0 existe parce que trois pièges de la section 1 sont plus
faciles à tester qu'à corriger après coup.

| # | Étape | Sortie visible | Couvre |
|---|---|---|---|
| 0 | `cargo init contrepoint-pipeline`, `Cargo.lock` versionné. Adaptateurs « un-ou-plusieurs » et « chaîne ou objet xsi », avec un test sur `VTANR5L17V5268` (cas `votant` objet nu) et sur un `acteur.uid` enveloppé | 2 tests qui échouent avant les adaptateurs et passent après | 1.4 |
| 1 | Script CI de récupération : `curl -C -` en boucle jusqu'à `content-length`, SHA-256 consigné, échec bruyant si l'empreinte change sans que la date de source change | tableau des archives, taille, empreinte, `last-modified` | 1.5 |
| 2 | Ingestion des scrutins → triplets `(acteur, scrutin, valeur)`, l'absence n'étant jamais écrite. Référentiel depuis AMO30, jointure sur période de validité | décompte des scrutins retenus et écartés, et pourquoi (roadmap v0.1) | 1.2, 1.5 |
| 3 | ALS rang 1, initialisation déterministe sans générateur aléatoire, signe fixé par deux points d'ancrage déclarés dans un fichier de données | positions par groupe, **dispersion intra publiée**, et le gain sur le résidu énoncé | 1.3, 1.6 |
| 4 | Registre de preuves JSONL en ajout seul : entité, valeur, méthode, source, date de source, date de calcul, empreinte d'archive, version de `rustc` et du pipeline | reconstruction complète depuis les archives redonnant le même fichier | 1.7 |
| 5 | Test de non-régression : instantané `insta` de la sortie complète, plus une assertion de validité méthodologique (ordre des groupes, `|corr|` ≈ 1 avec la référence) à tolérance déclarée | suite hors ligne, sans réseau, sans jeton | 1.7 |
| 6 | Front Vite + React/TS, SVG manuel, `d3-scale`. Une bande par parti, trois marqueurs distincts, curseur temporel, clic vers la preuve. Thèmes clair et sombre, navigation au clavier, `aria-label` par marqueur, jamais la couleur comme seul porteur d'information | le graphe (roadmap v0.6) | — |
| 7 | GitHub Actions : hebdomadaire plus déclenchement manuel, caches `~/.cargo` et `target/`, publication sur Pages. En cas d'échec, le site conserve la dernière sortie valide et affiche « données arrêtées le … » | pipeline qui tourne seul (roadmap v0.7) | risque d'abandon |

**Règles permanentes de la pile**, à faire respecter par la CI plutôt que par la
discipline :

- `Cargo.lock` et `package-lock.json` versionnés ; construction avec
  `--locked` / `npm ci`.
- Aucune réduction flottante parallèle sur le chemin déterministe. `rayon`
  n'entre pas dans `Cargo.toml` de la brique 0 ; l'y ajouter demande de modifier
  ce document.
- Aucune dépendance dont une fonctionnalité d'implémentation sous-jacente réclame une bibliothèque
  système. `ndarray-linalg` est nommément exclu, ainsi que toute crate qui en
  dépend.
- Les tests ne touchent pas le réseau. Les échantillons de test sont
  versionnés, avec leur empreinte.

---

## 8. Ce qui reste ouvert

| Point | Comment le trancher |
|---|---|
| JSON ou XML pour les scrutins (1.4) | Parser les deux, compter les cas particuliers de chaque côté. Décider avant l'étape 2, le changer après coûte l'ingestion entière. |
| Seuil de participation (1.2) | Méthodologique. Publier la sensibilité de l'axe au seuil plutôt que choisir un chiffre : c'est une sortie du projet, pas un paramètre à cacher. |
| Suffisance du rang 1 | Si l'axe s'avère instable dans le temps, les estimateurs de référence (IRT bayésien, W-NOMINATE) deviennent justifiés — comme prévu par `methode.md`, et pas avant. Ils changeraient le calcul, pas la pile. |
| Fraîcheur des archives AN | AMO50 est figé au 2024-07-11, AMO30 est quotidien. Vérifier `last-modified` de chaque archive à chaque exécution et échouer si une source censée être vivante ne bouge plus. |
| Points d'ancrage du signe | Deux entités et le sens attendu, dans un fichier de données versionné et non dans le code. Le choix est un arbitrage à documenter publiquement. |
| Contrat de schéma entre Rust et TypeScript | Réévaluer si le registre d'entités devient une source de bugs (concession de la section 6). |
