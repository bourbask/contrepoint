# Plan de tests — brique 0

Statut : **spécification de tests, à écrire avant le code** (docs/tdd.md).
Périmètre : v0.1 → v0.7 de la brique 0, XVIIe législature seule.
Sources normatives : [ADR 0000](../adr/0000-perimetre-brique0.md),
[ADR 0001](../adr/0001-stack.md), [ingestion-votes.md](ingestion-votes.md),
[positionnement.md](positionnement.md), [registre-entites.md](registre-entites.md),
[definition-of-done.md](../definition-of-done.md).

Ce document est la liste des tests à écrire, dans l'ordre où ils doivent être
écrits. Il ne contient aucun code d'implémentation. Chaque test porte ce qui
casse s'il disparaît — sans quoi un test se supprime en revue sous prétexte
qu'« il ne sert plus ».

---

## 1. Ce que la suite prouve, et ce qu'elle ne peut pas prouver

**Prouvé par la suite hors ligne :** le codage des positions, la structure de la
matrice, les invariances de l'estimateur, l'ancrage, les règles de validation du
registre d'entités, la forme du registre de preuves, la forme du contrat
d'export, et l'absence des objets interdits (coordonnée individuelle, moyenne
inter-familles, terme de lexique, axe à pôle dépréciatif).

**Non prouvable hors ligne, et il faut le dire :**

| Affirmation | Pourquoi la suite ne la prouve pas | Où elle est vérifiée |
|---|---|---|
| Les chiffres de corpus (8 434 scrutins, 7 979 retenus, densité 0,232, gain 0,608, `s2/s1` 0,652, IQR par groupe) | Ils portent sur les 26,3 Mo d'archive, non redistribuable comme fixture et hors de portée d'une suite rapide | Niveau 2 (§2), sur archive en cache, hors PR |
| L'intégrité du téléchargement (reprise `Range`, taille annoncée, empreinte de contenu) | Exige le réseau, seule étape qui n'a pas de test hors ligne (ingestion-votes.md §9d) | Niveau 3, échec bruyant |
| L'égalité bit-à-bit entre deux implémentations | Fausse par construction (ADR 0001 §1.7) | Jamais. Aucun test ne l'affirme |
| La justesse des trois libellés `PAN` / `PSE` / `MG` | Inférés des mandats, aucune énumération officielle lue | Rien n'en dépend : le codage ne lit que le bloc, pas la cause |

Un test qui prétendrait établir une ligne de la colonne de droite avec les
fixtures serait faux. La règle est de ne pas l'écrire.

---

## 2. Trois niveaux d'exécution, et ce que chacun bloque

| Niveau | Contenu | Entrée | Durée cible | Bloque |
|---|---|---|---|---|
| **1 — unitaire** | Tout ce qui est listé aux §5 à §11 sauf mention contraire | `docs/brique0/echantillons/` et matrices synthétiques construites dans le test | < 10 s, `cargo test` | Toute PR |
| **2 — corpus** | Les seuils de sanité de positionnement.md §9 (contrôles 8, 9, 10, 11) et l'instantané de sortie complète | Archives réelles, prises dans le cache par SHA-256, jamais téléchargées par le test | minutes | La publication, pas la PR |
| **3 — récupération** | Reprise sur troncature, taille annoncée puis empreinte de contenu, `last-modified` d'une source censée être vivante | Réseau | — | La publication |

Le niveau 1 comprend, hors `cargo test`, la suite shell du §4bis :
`./scripts/test-recuperer-sources.sh`, exécutée par le travail `garde-fous` de
`ci.yml`.

Le niveau 1 passe **machine débranchée**, sans clé, sans jeton
(definition-of-done.md §7). Il ne lit aucun fichier hors du dépôt. Un test de
niveau 2 ou 3 présent dans le binaire de niveau 1 est un défaut : la séparation
se fait par cible de test distincte, pas par un drapeau d'environnement lu au
milieu d'une fonction.

**Conséquence assumée** : les valeurs de référence les plus intéressantes
(0,608, 0,652, l'ordre des douze groupes) ne gardent pas les PR. Ce qui garde
les PR, c'est la classe d'erreur qui les produit — codage, imputation, ancrage,
ordre — et elle est testable sur cinq fichiers.

---

## 3. Outillage, et ce qui n'est pas ajouté

| Besoin | Retenu | Écarté, et pourquoi |
|---|---|---|
| Tests par instantané | `insta` 1.48.0, déjà à l'ADR 0001 §3 | — |
| Tests de propriété | **Tables de permutations fixes écrites dans le test** | `proptest` / `quickcheck` : un générateur aléatoire dans une suite qui doit être déterministe est une contradiction, et le nombre de permutations utiles ici est de l'ordre de la dizaine. Réévaluable si une propriété résiste à l'énumération |
| Couverture | `cargo llvm-cov`, outil de CI | Aucune : ce n'est pas une dépendance du binaire, elle n'entre pas dans `Cargo.lock` du produit |
| Comparaison flottante | Trois tolérances déclarées au §4, dans un seul module | Une macro d'égalité approchée par test : la tolérance se met à dériver test par test |

Justification de dépendance exigée par definition-of-done.md §21 : la
bibliothèque standard ne fournit ni instantané ni couverture ; elle fournit tout
le reste, y compris les permutations et les comparaisons.

---

## 4. Épinglage numérique : trois tolérances, et où elles vivent

Le piège du test flottant est de devenir faux au premier changement de
plateforme, puis d'être relâché jusqu'à ne plus rien détecter. Trois régimes
distincts, jamais mélangés :

| Régime | Objet | Tolérance | Portabilité |
|---|---|---|---|
| **T1 — octet** | Artefact publié, après arrondi à 4 décimales | égalité stricte de fichier | **Uniquement** sur la cible épinglée (`rustc` et cible de compilation consignés dans la ligne de preuve, ADR 0001 §1.7). Le test ne s'exécute pas ailleurs, et le dit |
| **T2 — invariance** | Permutations, initialisations, idempotence de l'ancrage | `1e-12` sur la position ancrée | Toutes plateformes. Mesuré à 1,6·10⁻¹⁵ (positionnement.md §5) : marge de trois ordres de grandeur, ce qui laisse la place au flottant sans laisser passer une erreur d'algorithme |
| **T3 — non-régression de valeur** | Médiane ancrée d'un groupe contre l'instantané figé | `0,02` en unités ancrées, amplitude 2,0 | Toutes plateformes. Seuil de positionnement.md §9 contrôle 15. 1 % de l'amplitude : au-dessus, plus aucune erreur de méthode ne passe ; en dessous, le bruit d'implémentation ferait échouer la suite |

Les trois valeurs vivent dans **un seul** module `tolerances`, sans variante
locale. Une tolérance modifiée est un diff d'une ligne, visible en revue, et
elle est refusée si le motif n'est pas écrit dans la PR (docs/tdd.md §4).

Règle dure : **T1 n'est jamais appliqué à un flottant intermédiaire.** La
promesse « bit pour bit » du README porte sur le fichier arrondi, pas sur le
calcul. Un test qui compare deux `f64` non arrondis à l'égalité est un test qui
sera désactivé dans six mois.

---

## 4bis. Script `recuperer-sources` — récupération des archives

Seul module du plan écrit en shell, parce qu'il n'appartient pas au binaire
(architecture.md §2, composant [0]). Suite :
`scripts/test-recuperer-sources.sh`, sans cadre de test, sans dépendance —
des assertions dans un script exécutable, à l'image du reste de l'outillage.

Elle **source** le script testé et n'exerce que ses fonctions pures. L'enveloppe
réseau (`telecharger`, `principal`) n'a pas de test hors ligne, par construction
(ingestion-votes.md §9d, RG-119) : elle est exercée à la main et relève du
niveau 3.

Fixtures : construites dans le test, dans un répertoire temporaire. Aucun octet
du corpus réel n'est nécessaire, aucune horloge n'est lue.

| ID | Test | Entrée | Sortie attendue | Ce qui casse si le test disparaît |
|---|---|---|---|---|
| REC-01 | `telechargement_complet` | fichier de 100 octets, annonces de 100 et 101 octets, et fichier absent | Complet à 100/100 ; incomplet à 100/101 ; incomplet si le fichier n'existe pas | La porte de complétude est la taille annoncée (RG-114). Sans elle, une archive tronquée par la fermeture de connexion du serveur passe pour une donnée : le paysage publié est amputé sans erreur (ADR 0001 §1.5) |
| REC-02 [C] | `empreinte_contenu_en_ordre_d_octets` | trois fichiers `V1`, `V10`, `V100` dont l'ordre de collation `fr_FR.UTF-8` est l'inverse de l'ordre d'octets ; l'attendu est écrit à la main, jamais relevé d'une exécution | L'empreinte vaut le SHA-256 de la concaténation en ordre d'octets ; elle est inchangée sous `LC_ALL=fr_FR.UTF-8` et indépendante du répertoire d'extraction | Sans `LC_ALL=C`, la même archive rend `503255ac…` au lieu de `c8457f34…` selon la locale de la machine (verification-2026-08-27.md §0). L'empreinte de contenu entre dans la clé de déduplication : dépendante de la locale, elle ré-émet le registre de preuves au changement de machine |
| REC-03 | `contenu_modifie_sans_annonce_refuse` | index portant `(scrutins, date, aaa)` | Même date et même contenu : accepté. Même date et contenu différent : **refusé**, avec le message qui nomme les deux empreintes. Date nouvelle : accepté. Index absent : accepté | C'est l'échec bruyant de RG-20. Sans lui, une donnée modifiée en silence par le producteur entre dans le pipeline sous une date inchangée, et le registre de preuves affirme un calcul sur des octets qui n'existent plus |
| REC-04 | `descripteur_deterministe` | les mêmes champs passés dans deux ordres | Fichier trié, identique à l'octet dans les deux cas | Le descripteur est la seule trace de ce qui a été téléchargé. Un descripteur dont l'ordre suit l'ordre d'appel n'est pas comparable d'une exécution à l'autre |
| REC-05 | `date_de_source_iso_complete` | trois horodatages ISO, et un `last-modified` HTTP | Le plus récent l'emporte ; le résultat vérifie `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$` et est relu **à l'identique** par le `date -u -d` de pipeline.yml ; un `last-modified` HTTP devient un horodatage ISO complet | `CONTREPOINT_DATE_CALCUL` en dérive (contrats.md §8.1). Une date réduite au jour produit une ligne de preuve invalide contre le schéma `preuve-1`, et `date -u -d` d'une chaîne vide rend l'heure courante — c'est-à-dire l'horloge, précisément ce que le pipeline ne lit jamais |
| REC-06 | `cache_immuable` | entrée de cache déjà présente, et empreinte inconnue | L'entrée présente n'est pas réécrite ; l'empreinte inconnue crée la sienne | Le cache indexé par empreinte est ce qui rend un calcul de 2026 rejouable en 2027 (RG-116, ingestion-votes.md §9e). Un cache réécrit n'est plus une archive, c'est une copie du jour |

| REC-07 | `paternite` | producteur, licence et date de source de deux sources différentes | La mention est dérivée des trois arguments ; celle de CHES ne nomme ni l'Assemblée nationale ni une Licence Ouverte | Deux des quatre sources ne sont pas de l'Assemblée. La mention était écrite en dur : elle attribuait à l'Assemblée un fichier du Chapel Hill Expert Survey et un fichier du ministère de l'intérieur, sous une licence qu'aucun des deux ne porte. C'est une fausse attribution, pas une approximation (RG-76) |
| REC-08 | `archives.sh deposer` sur une source hors liste blanche | descripteurs portant l'URL de CHES, celle du nuancier, et une URL qui imite celle de l'Assemblée sans en être une | Refus dans les trois cas, **avant tout appel réseau** à `gh` | RG-118 et ADR 0000 §8 : CHES ne publie aucune licence, et la condition obtenue le 2026-08-27 est une exigence de citation, pas une cession de droits. Un dépôt de release est public et immuable ; l'asset mal déposé ne se reprend pas |

### §4ter — les portes de contrôle elles-mêmes

`scripts/test-portes.sh`. Un contrôle qui ne peut plus rien affirmer ne protège
plus rien, et ce mode de panne ne se voit pas : la sortie reste plausible.

| Id | Suite | Entrée | Attendu | Ce qui casse sans lui |
|---|---|---|---|---|
| POR-01 | `par_lots` | 23 000 chemins longs, ~2,6 Mio d'`argv` | Avec découpage, « rien trouvé » reste distinct de « rien lu ». Que `grep` échoue **sans** découpage est un constat affiché, jamais une assertion : `ARG_MAX` dépend du noyau et de la limite de pile, donc de la machine — la première version l'affirmait et échouait en CI, où la limite est plus haute | Le 2026-08-28, une copie de cache égarée a mis 22 490 fichiers dans l'index : `securite.sh` et `lexique.sh` ont rendu 126 sur **chaque** motif. Le même défaut avait frappé une porte de CI sur 22 426 fichiers, et n'avait été corrigé que là |
| POR-02 | `par_lots` | une occurrence placée dans le **dernier** lot | Elle est trouvée, et une seule ligne est rapportée | Une boucle qui s'arrête un lot trop tôt, ou qui recopie le même lot, ne planterait pas : elle rendrait vert |
| POR-03 | `par_lots` | un fichier illisible au milieu de la liste | « rien lu », jamais « rien trouvé » | Le découpage ne doit pas avaler l'erreur d'un lot. C'est la distinction que tout ce contrôle existe pour tenir |

Le découpage est écrit à la main et non délégué à `xargs` : `xargs` rend 123 dès
qu'une invocation rend entre 1 et 125, sans dire laquelle ni pourquoi — « rien
trouvé » et « grep en erreur » y deviendraient le même code.


Les contrôles de niveau 3 restants — reprise réelle sur troncature, empreinte de contenu,
fraîcheur d'une source censée être vivante — sont exercés par une exécution
manuelle de `scripts/recuperer-sources.sh` et consignés dans
[verification-2026-08-27.md](verification-2026-08-27.md) §0.

---

## 5. Module `ingestion` — lecture des scrutins et du référentiel

Fixtures : les cinq scrutins verbatim et les deux index de
`docs/brique0/echantillons/`.

| ID | Test | Entrée | Sortie attendue | Ce qui casse si le test disparaît |
|---|---|---|---|---|
| ADA-01 | `votant_tableau_ou_objet_nu` | `VTANR5L17V5268.json` | Les blocs où `votant` est un objet nu rendent une liste d'un élément ; aucun bloc n'est perdu | 27,3 % du corpus cesse d'être lu, préférentiellement sur les scrutins serrés — et l'échec est silencieux si le désérialiseur avale l'erreur (ingestion-votes.md §4a) |
| ADA-02 | `uid_chaine_ou_objet_xsi` | `organes-groupes-l17.json` (`organe.uid` chaîne) et la forme enveloppée relevée dans AMO30, citée verbatim par ADR 0001 §1 mesure 10 — `mandats-gp-l17.json` est dérivé et n'en porte aucune | Les deux formes rendent la même chaîne `PA…` / `PO…` | La jointure au référentiel tombe entièrement : `acteur.uid` est un objet dans AMO30 (§4c) |
| ADA-03 | `mise_au_point_tableau_de_nuls` | `VTANR5L17V2767.json` | `pours`/`contres` en objet nu ou `null`, `abstentions`/`nonVotants` en tableau dont les éléments sont `null` : rendu = liste vide, jamais un votant inexistant | Un `[null, null]` compté comme deux entrées invente 2 mises au point sur 8 270 scrutins (§4b) |
| ADA-04 | `valeurs_numeriques_serialisees_en_chaine` | `VTANR5L17V156.json` | `"163"` lu comme 163, et un non-numérique est une erreur, pas un 0 | Tout le jeu sérialise les nombres en chaîne ; un `unwrap_or(0)` transforme un champ illisible en scrutin à zéro votant |
| ING-05 | `legislature_lue_dans_la_donnee` | les cinq scrutins | `scrutin.legislature == "17"` lu du champ ; aucune dérivation d'un libellé ni du chemin | Le libellé de la fiche source dit « XV législature » et il est faux (ADR 0000 §3). Un pipeline qui fait confiance au libellé étiquette tout le corpus sur la mauvaise législature |
| ING-06 | `coherences_de_synthese` | `VTANR5L17V156.json` | `nombreVotants == pour + contre + abstentions` et `suffragesExprimes == pour + contre` ; violation = erreur bloquante | La seule vérification bon marché que le fichier a été lu correctement de bout en bout ; sans elle, un décalage d'un bloc passe |
| ING-07 | `longueur_nominative_egale_decompte` | les cinq scrutins | Pour les 4 positions, `len(votant[]) == decompteVoix.<position>` | Attrape la perte de votants due à ADA-01 même si l'adaptateur ne lève pas d'erreur. C'est le garde-fou de l'adaptateur, pas de l'adaptateur seul |
| ING-08 | `un_acteur_au_plus_une_fois_par_scrutin` | les cinq scrutins | Aucun `acteurRef` en double, toutes positions confondues | Un doublon crée deux cellules contradictoires pour la même case et l'estimateur les moyenne en silence |
| ING-09 [C] | `non_votants_volontaires_jamais_lu_comme_categorie` | `VTANR5L17V156.json` | Le pipeline n'expose aucun accès à `decompteVoix.nonVotantsVolontaires` comme catégorie de position ; le champ est ignoré ou consigné comme doublon | Le champ vaut exactement `abstentions` dans 101 208 blocs sur 101 208. Le lire double-compte les abstentions et fabrique une cinquième catégorie absente de la source |
| ING-10 | `trois_causes_de_non_votant_reconnues_sans_etre_interpretees` | `VTANR5L17V5268.json` | `MG`, `PAN`, `PSE` acceptées ; une quatrième valeur inconnue est une erreur bloquante ; **aucune** des trois ne change le codage | Une nouvelle cause apparue dans la source passerait pour une position ; et un codage qui dépend du libellé s'effondre le jour où le libellé change |
| ING-11 | `par_delegation_est_une_position` | `VTANR5L17V5268.json` (les deux valeurs présentes) | `parDelegation = "true"` code comme `"false"` | 15,4 % des cellules exprimées. Les traiter en empêchement viderait un septième de la matrice (§3) |
| ING-12 | `mise_au_point_ingeree_jamais_appliquee` | `VTANR5L17V2767.json` | La cellule vaut le vote de la machine ; la mise au point est un attribut du scrutin, comptée et exposée | 3 043 entrées, dont 746 combleraient une absence par une intention déclarée — interdit par methode.md. Et une matrice qui applique les mises au point est incohérente avec `sort.code` de la source |
| ING-13 | `ordre_des_fichiers_sans_effet` [P] | les cinq scrutins, présentés dans 5 ordres fixes | Sortie identique à l'octet | La machine de mesure a livré `VTANR5L17V5646` avant `VTANR5L17V2136` : l'ordre du système de fichiers entre dans la sortie si rien ne l'interdit (definition-of-done.md §6) |
| ING-14 | `dedoublonnage_des_mandats_gp` | `mandats-gp-l17.json`, cas mandat dupliqué | Regroupement par `(organeRef, dateDebut)`, `dateFin` maximale retenue, `null` = en cours ; aucun chevauchement résiduel entre deux `organeRef` | 18 chevauchements sur 648 députés, **0 cellule ambiguë** après dédoublonnage. Sans la règle, la jointure choisit au hasard |
| ING-15 | `mandat_ref_ne_donne_pas_le_groupe` [C] | `VTANR5L17V156.json` + `mandats-gp-l17.json` | Le rattachement ne consulte jamais `votant.mandatRef` pour le groupe | `mandatRef` pointe toujours un mandat `ASSEMBLEE`, 1 270 476 / 1 270 476. Le piège est de croire tenir la jointure ; le test empêche de le recroire |

### Cas particulier `PO0` — conflit de spécification, acté par l'ADR 0003 §1

ingestion-votes.md §8 retient **le groupe du bloc de ventilation**, AMO30 ne
servant que de recours pour les 146 blocs `PO0`. positionnement.md §1 **écrivait**
l'inverse : « le rattachement ne peut donc pas être lu dans le fichier de
scrutin ; il doit venir des mandats » — phrase corrigée depuis. Les deux ne
pouvaient pas être vrais.

Ce plan retient **ingestion-votes.md** : c'est la version mesurée sur les
1 270 476 cellules, elle explique les 280 désaccords par un retard de `dateFin`
du mandat de non-inscrit, et elle évite une jointure là où la source publie déjà
la réponse datée. C'est ce que
[../adr/0003-arbitrages-de-coherence.md](../adr/0003-arbitrages-de-coherence.md)
§1 acte : le blocage est levé, les tests ci-dessous peuvent être écrits.

| ID | Test | Entrée | Sortie attendue | Ce qui casse si le test disparaît |
|---|---|---|---|---|
| ING-16 | `po0_resolu_par_les_mandats` | `VTANR5L17V6256.json` + `mandats-gp-l17.json` | Chaque bloc `PO0` non vide est résolu par le mandat GP de ses votants à `dateScrutin`, unanimement ; un bloc vide est sans objet | 335 acteurs sur 14 scrutins perdent leur groupe, ou un treizième groupe apparaît dans l'agrégation |
| ING-17 [C] | `po0_non_resolu_est_bloquant` | bloc `PO0` construit dans le test, votants sans mandat GP à la date | Erreur bloquante. **Pas** de groupe « inconnu », pas de bloc ignoré | Un groupe « inconnu » remonte dans une agrégation publiée et devient une bande du graphe |
| ING-18 | `desaccord_ventilation_amo30_tranche_pour_la_ventilation` | `mandats-gp-l17.json`, cas de désaccord | Le groupe retenu est celui de la ventilation ; le désaccord est compté et exposé, pas tu | Les 280 cellules concernées basculeraient chez les non-inscrits, dont la dispersion interne est la plus grande du jeu |
| ING-19 | `periode_de_validite_respectee` | `mandats-gp-l17.json`, cas des trois groupes successifs | Un vote du 2025-05-01 est attribué au groupe valide **ce jour-là**, jamais au dernier groupe connu | Invariant nommément exigé par definition-of-done.md §9. Nul sur les données de la v0 (aucun scrutin avant le 2024-10-08) et réel dès la XVIe législature : le test protège l'extension, pas la v0 |

---

## 6. Module `matrice` — du scrutin aux triplets

| ID | Test | Entrée | Sortie attendue | Ce qui casse si le test disparaît |
|---|---|---|---|---|
| MAT-01 [C] | `absence_nest_pas_une_position` | `VTANR5L17V5268.json` (35 votants sur 577 sièges) | Le nombre de triplets émis égale le nombre de votants des blocs `pours`/`contres`/`abstentions`. Aucun triplet pour un acteur absent, aucun pour un non-votant | **Le test central du projet.** Le mauvais codage déplace la ligne la plus touchée de 0,72 sur une amplitude de 2,6, et il est invisible sur la sortie publiée : corrélation 0,9969, moyennes de groupe à moins de 0,04. Le seul chiffre corrompu de façon visible est l'écart-type intra — c'est-à-dire l'indicateur de dispersion que la roadmap exige de publier |
| MAT-02 | `codage_des_trois_positions` | les trois blocs de `VTANR5L17V156.json` | `pours` → +1, `contres` → −1, `abstentions` → 0 | Une inversion de signe retourne l'axe entier, et l'ancrage la masque : après ancrage LFI-NFP → −1 et RN → +1 quand même. L'erreur ne se voit qu'ici |
| MAT-03 [C] | `abstention_et_absence_ne_sont_pas_le_meme_objet` | matrice construite dans le test, une abstention et une absence sur le même scrutin | La cellule d'abstention existe avec la valeur 0 ; la cellule d'absence **n'existe pas**. Aucune structure de masque n'est produite | Un masque se perd à la sérialisation suivante ; une ligne non écrite ne se perd pas (ingestion-votes.md §5) |
| MAT-04 | `filtre_minorite_vide` | `VTANR5L17V1.json` (motion de censure, `contre = 0`) et `VTANR5L17V2767.json` | Les deux écartés, motif `minorite_vide` ; `VTANR5L17V156.json` retenu | Un scrutin à variance nulle n'apporte rien à l'axe mais entre dans les décomptes publiés et dans la constante par scrutin |
| MAT-05 [C] | `aucun_seuil_de_participation` | `VTANR5L17V5268.json`, 35 votants | Retenu. `nombreVotants` est **publié**, jamais utilisé comme porte | La mesure dit que le seuil justifiable est l'absence de seuil : à ≥ 300, RN passe devant DR et LIOT traverse le zéro. Un seuil réintroduit « par prudence » dégrade l'axe et personne ne le remarque |
| MAT-06 | `decompte_retenus_ecartes_expose_avec_motif` | les cinq scrutins | Un décompte par motif, sortie visible de la v0.1 | La case de roadmap v0.1 promet cette sortie. Sans test, elle est cochée sans exister (definition-of-done.md, section finale) |
| MAT-07 | `tri_canonique_des_triplets` | les cinq scrutins, ordres d'entrée permutés | Triplets triés par `(uid_scrutin, acteurRef)`, ordre lexicographique | Sans tri explicite, l'itération sur une table de hachage rend la sortie non reproductible d'un processus à l'autre — `HashMap` de Rust est semé par processus, l'erreur se manifeste une fois sur deux |
| MAT-08 | `entete_porte_les_empreintes_dentree` | matrice produite à partir de deux empreintes de contenu factices | L'en-tête porte les deux **empreintes de contenu** et la version du code d'ingestion, et rien d'autre de variable. **Pas les empreintes d'archive** : le répartiteur de la source les fait varier à contenu identique, et le cache s'invaliderait sans cause (ingestion-votes.md §9b, contrats.md §2.8) | C'est ce qui rend le cache invalidable **sans horloge**. Sans en-tête, la seule façon de savoir si la matrice est à jour est une date, donc une horloge dans le pipeline |
| MAT-09 [C] | `aucune_imputation` | matrice à 77 % de cellules absentes | Le nombre de valeurs entrant dans les sommes de l'estimateur égale exactement le nombre de cellules observées | Les trois façons usuelles de compléter la matrice sont mesurées comme fausses ici (positionnement.md §2). Ce test est celui qui rend l'interdiction structurelle plutôt que déclarative |

---

## 7. Module `estimateur` — rang 1 sur cellules observées, ancrage

Les fixtures de l'Assemblée ne conviennent pas : trois scrutins retenus sur cinq
ne définissent pas un axe, et l'ancrage exige les médianes de LFI-NFP et de RN.
Les tests de ce module prennent donc une **matrice synthétique construite dans
le test** — `v[i,j] = b[j] + x[i]·y[j]` exactement, sur un sous-ensemble observé
déclaré — dont la solution est connue analytiquement. Les valeurs du corpus réel
sont au niveau 2.

| ID | Test | Entrée | Sortie attendue | Ce qui casse si le test disparaît |
|---|---|---|---|---|
| EST-01 | `constante_par_scrutin_est_la_moyenne_observee` | matrice synthétique | `b[j]` = moyenne des valeurs **observées** de la colonne, pas de la colonne complétée | La constante absorbe le fait qu'un scrutin est majoritairement pour ou contre. Calculée sur des zéros de remplissage, elle absorbe la composition des présents à la place |
| EST-02 | `initialisation_ne_depend_pas_de_lindice_de_ligne` [C] | matrice synthétique, deux ordres de lignes | `x[i]` initial = moyenne des résidus observés de la ligne | Deux initialisations mathématiquement neutres — indice de ligne contre moyenne des résidus — donnent des axes **exactement opposés** (corrélation −1,000). L'initialisation par indice est la faute naturelle, et elle rend la permutation des lignes signifiante |
| EST-03 | `aucun_generateur_pseudo_aleatoire` [C] | revue outillée : absence de `rand`, de `SystemTime`, de graine dans le module | Aucun | Une graine non fixée est un non-déterminisme (definition-of-done.md §6). Le test le refuse à l'endroit où la tentation existe |
| EST-04 | `solution_exacte_recuperee_sur_matrice_de_rang_1` | matrice synthétique sans bruit | Positions ancrées égales aux `x` de construction à T2 | Sans ce test, tous les tests d'invariance passeraient sur un estimateur qui rend une constante : invariant et faux |
| EST-05 [P] | `invariance_par_permutation_des_lignes` | 6 permutations fixes des lignes | Écart des positions ancrées ≤ T2 | Mesuré 1,1·10⁻¹⁵. Une itération sur ensemble non ordonné, une initialisation par indice, un tri instable : trois fautes ordinaires que seule cette propriété détecte |
| EST-06 [P] | `invariance_par_permutation_des_colonnes` | 6 permutations fixes des scrutins | Idem | Même classe de faute côté scrutins. Séparé de EST-05 parce qu'un code peut être invariant d'un côté et pas de l'autre |
| EST-07 [P] | `invariance_a_linitialisation_signe_compris` | 4 initialisations, dont deux produisant des axes opposés | Écart ≤ T2 après ancrage | 2 initialisations sur 4 renvoient l'axe inversé avec \|corrélation\| = 1,000000. Sans ancrage testé, « gauche » change de côté d'une exécution à l'autre — et selon ADR 0000 §6 c'est une **majeure** |
| EST-08 | `ancrage_exact` | matrice synthétique avec deux groupes d'ancrage nommés | `m(LFI-NFP) == −1` et `m(RN) == +1` à T2 | L'échelle n'est pas identifiée : selon la normalisation, les positions sortent autour de 0,05 ou autour de 1,3. Une valeur publiée sans convention d'échelle n'est comparable à rien |
| EST-09 [P] | `ancrage_idempotent` | positions déjà ancrées | Réapplication sans effet, écart ≤ T2 | Un double ancrage accidentel dans un enchaînement de fonctions ne se voit sur aucune sortie : l'ordre des groupes est conservé |
| EST-10 | `mediane_deterministe_sur_effectif_pair` | groupe de 4 membres | La moitié **inférieure** des deux valeurs centrales | Une médiane par moyenne des deux centrales est un troisième estimateur qui s'invite dans la chaîne, et elle rend l'ancrage dépendant de l'arrondi |
| EST-11 [C] | `ancre_absente_arrete_le_pipeline` | registre d'entités où `RN` n'a pas d'ancre valide à la date de calcul | Erreur bloquante. **Aucune** ancre de remplacement choisie automatiquement | Un choix automatique d'ancre inverse ou réétalonne l'axe en silence, ce qui invalide toutes les preuves déjà publiées |
| EST-12 | `ancre_lue_dans_le_registre_pas_dans_le_code` [C] | registre où l'ancre est déplacée sur un autre groupe | La sortie change ; aucune constante `"PO845413"` n'existe dans le module | definition-of-done.md §16 interdit la constante en dur, et positionnement.md blocage 3 exige l'attribut d'ancre au registre. Un identifiant en dur rend l'arbitrage invisible |
| EST-13 | `second_axe_calcule_jamais_publie_comme_position` | matrice synthétique de rang 2 | Le second axe existe, sert aux alarmes, et n'apparaît dans aucune sortie publiée | Sa norme relative est 0,652 : ne pas le calculer laisse croire que le premier axe est tout le comportement de vote ; le publier créerait une seconde dimension étiquetée, interdite |
| EST-14 [C] | `aucune_reduction_flottante_parallele` | revue outillée : absence de `rayon` dans `Cargo.toml`, absence de `par_iter` sur le chemin de calcul | Aucun | `par_iter().sum()` a produit 3 représentations binaires distinctes sur 40 exécutions, aucune égale au séquentiel. La faute s'ajoute en une ligne de `Cargo.toml` et casse T1 sans casser T2 : aucun test de valeur ne la détecte |
| EST-15 | `alarme_separation_des_axes` | matrice synthétique où `s2/s1` sort de [0,10 ; 0,90] | La famille « votes » passe en « non mesuré » ; aucune valeur publiée | Un tirage sur 25 a produit un ajustement dégénéré (`s2/s1 = 0,002`). Sans alarme, ce tirage-là se publie |
| EST-16 | `alarme_pouvoir_explicatif` | matrice synthétique de gain < 0,40 | Idem | Transforme le risque accepté « le premier axe est peu informatif » (ADR 0000 §8) en résultat affiché plutôt qu'en publication silencieuse |
| EST-17 | `ancrage_sur_les_medianes_des_deux_groupes_ancres` | positions brutes de deux groupes ancres, dont deux membres sous les 200 votes exprimés | Les deux médianes d'ancrage valent exactement ∓1 ; les membres sous le seuil ne déplacent ni la médiane d'ancrage ni l'échelle ; un groupe d'ancrage sans membre retenu **bloque** | L'ancrage vivait dans `examples/verification-corpus.rs`, hors de portée de la CI : un appelant pouvait passer à `agreger` des positions non ancrées sans qu'aucun test ne le voie. Et la coïncidence entre l'ensemble de la médiane d'ancrage et celui de la médiane publiée n'était retenue par rien — rompue, l'ancre droite est mesurée à +1,0002 |

---

## 8. Module `agregation` — du député au groupe

| ID | Test | Entrée | Sortie attendue | Ce qui casse si le test disparaît |
|---|---|---|---|---|
| AGR-01 [C] | `aucune_coordonnee_individuelle_en_sortie` | sortie complète du pipeline sur fixtures | Aucun `PA…` accompagné d'une valeur de position, dans aucun fichier produit ni aucune réponse d'interface | Invariant exigé par definition-of-done.md §9 et ADR 0000 §2. C'est le principal rempart contre l'accusation de ciblage de personnes, et il se casse en ajoutant un champ « pour déboguer » |
| AGR-02 | `mediane_de_groupe_et_pas_moyenne` | groupe hétérogène construit dans le test | La médiane. Une moyenne donnerait une valeur différente sur ce cas | Sur NI et LIOT, l'écart moyenne / médiane atteint le tiers de la valeur. Et la médiane est ce qui définit l'ancrage : deux estimateurs concurrents dans la même chaîne rendent l'ancrage incohérent avec la valeur publiée |
| AGR-03 | `dispersion_publiee_est_iqr_et_reechantillonnage` | groupe construit dans le test | IQR et écart-type de rééchantillonnage. **Aucune variance, aucune étendue** dans la sortie | Une variance sur un axe sans unité n'est pas lisible et invite à comparer deux exécutions ; une borne d'étendue **est** la coordonnée d'un membre identifiable, écartée par l'ADR 0003 §3. ROADMAP.md v0.2 et methode.md disent « variance intra-groupe » ; positionnement.md §6 les remplace — voir §16 |
| AGR-04 | `regle_de_non_publication` | groupes construits : IQR > 0,25 ; écart-type de rééchantillonnage > 0,05 ; effectif retenu < 10 | Case « non mesuré » **avec sa raison**, jamais une médiane accompagnée d'un avertissement | Aujourd'hui NI (IQR 0,623) et LIOT (0,687) échouent au critère. Une valeur publiée avec avertissement est citée sans l'avertissement |
| AGR-05 | `groupe_eteint_a_la_date_de_reference_absent` | `organes-groupes-l17.json` ; date de référence après le 2025-09-04 | `UDR` (`PO847173`) absent, pas présenté avec un effectif résiduel ; `AD` absent en toute date de scrutin | Une agrégation « dernier groupe observé » laisse UDR avec 1 membre et fabrique sur le graphe un groupe sans effectif réel |
| AGR-06 | `date_de_reference_est_une_entree` [C] | même matrice, deux dates de référence | Sorties différentes et cohérentes ; aucune lecture d'horloge | Si la date de référence vient de l'horloge, deux exécutions du même jeu à deux dates donnent deux sorties : le déterminisme tombe, et la composition des groupes affichés dépend du moment du calcul |
| AGR-07 | `effectif_retenu_publie_avec_la_valeur` | groupe construit dans le test | Chaque valeur porte son effectif et sa date | ADR 0000 §5 : « Chiffres : jamais seuls ». Une médiane sans effectif est incitation à comparer deux groupes de 9 et de 129 membres |
| AGR-08 | `seuil_de_200_votes_ne_sapplique_quau_calcul_de_la_mediane` | groupe avec un membre à 12 votes exprimés | Le membre entre dans la matrice et dans l'estimation, il n'entre pas dans la médiane du groupe | L'ordre de ces deux opérations est inversable sans que rien ne casse visiblement, et l'inverser appauvrit la matrice sans corriger le mécanisme d'absence |
| AGR-09 | `deux_uid_pour_une_meme_entite_ne_sont_pas_fusionnes_par_lestimateur` | `PO847173` et `PO872880` | Deux lignes de groupe distinctes ; la réconciliation appartient au registre d'entités | Un `libelleAbrev` égal fusionnerait deux périodes distinctes ; ici les deux abrégés diffèrent (`UDR` / `UDDPLR`) et évitent la fusion par accident. Le test empêche de compter sur cet accident |
| AGR-10 | `ecart_type_de_reechantillonnage_est_celui_du_jackknife` | quatre plis de médianes écrites à la main, −0,02 / −0,01 / +0,01 / +0,02 | `√((K−1)/K · Σ(θₖ − θ̄)²)` = **0,0273861278752583**, en dur | Le facteur du jackknife ne vit que dans une ligne de `agregation.rs`, et la construction des plis vit dans `examples/verification-corpus.rs`, que la CI n'exécute pas. Sans ce test, le passage au bootstrap — `√(Σ/(K−1))` = 0,0182574185835055, un facteur 1,5 — survit à toute la suite, et avec lui un générateur aléatoire dans le pipeline |
| AGR-11 [C] | `debug_de_membre_nexpose_pas_la_position` | un `Membre` construit dans le test, formaté par `{:?}` | L'acteur et le groupe restent lisibles ; **ni le mot `position`, ni sa valeur** | AGR-01 ne couvre que la chaîne rendue. Un `#[derive(Debug)]` sur `Membre` apparie l'acteur et sa coordonnée : un `dbg!`, un `{:?}` dans un message d'erreur ou une trace sur la sortie d'erreur publient une coordonnée individuelle sans passer par `rendre` |
| AGR-12 | `le_seuil_de_liqr_est_le_premier_effectif_sans_extreme` | effectifs 2 à 32, positions distinctes croissantes | Le seuil de calcul de l'IQR égale le premier effectif où ni Q1 ni Q3 n'est un extrême, et rien n'est calculé en deçà | Le seuil valait **4**, et à 4 comme à 5 membres la médiane basse de la moitié inférieure **est** le minimum du groupe : l'IQR y portait une borne d'étendue (I19). Le test recompte la propriété par balayage plutôt que de comparer la constante à elle-même — les mutants 4, 5 et 7 sont tués |

---

## 9. Module `registre` — registre d'entités

Fixtures : `docs/brique0/echantillons/registre-l17.json`, plus des variantes **fautives**
minimales construites dans le test. Une variante fautive par règle, pas un
fichier fautif universel : un test qui échoue pour deux raisons ne dit laquelle.

| ID | Test | Entrée | Sortie attendue | Ce qui casse si le test disparaît |
|---|---|---|---|---|
| REG-01 | `extrait_de_reference_valide` | `echantillons/registre-l17.json`, et le registre réel `data/registre/partis.json` | Valide selon les 25 règles, et valide contre `schemas/registre-partis-1.schema.json` | Sans un cas vert, la suite de refus passerait aussi sur un validateur qui refuse tout |
| REG-02 [C] | `cle_inconnue_refusee` (V1) | ligne portant `score: 0.42` | Refus | C'est ce qui empêche l'apparition silencieuse d'un champ de valorisation dans le fichier que trois briques consomment |
| REG-03 | `schema_litteral` (V2) | `schema` modifié | Refus | Un registre d'un autre schéma lu par le validateur courant produit des vérifications vides |
| REG-04 | `date_reelle` (V3) | `2026-02-30` | Refus | Une date impossible passe tous les contrôles de motif et casse toute comparaison de période en aval |
| REG-05 | `id_unique_et_prefixe_coherent` (V4) | `coalition.x` avec `nature: parti` | Refus | Le préfixe est le seul contrôle bon marché de la distinction parti / coalition, qui est la distinction que le registre existe pour porter |
| REG-06 | `reference_pendante_refusee` (V5, V6) | `composition` citant une entité inexistante ; source déclarée non citée | Refus dans les deux sens | Une référence pendante fait disparaître une composition en silence ; une source déclarée jamais citée est une source oubliée en cours de retrait |
| REG-07 [C] | `injectivite_par_source_et_par_date` (V7) | deux entités appariées au même `party_id` CHES sur des périodes qui se chevauchent | Refus | **L'erreur fatale du projet.** Deux partis sur un même identifiant externe propagent une position mesurée sur le mauvais parti dans toutes les briques, et rien en aval ne peut la détecter |
| REG-08 | `cardinalite_un_par_date` (V8) | deux `an_organe` actifs à la même date sur `parti.rn` | Refus ; les deux mêmes lignes sur périodes disjointes passent | Sans le test, la règle se relâche pour faire passer un cas réel et l'injectivité perd son sens |
| REG-09 [C] | `valeur_nulle_exige_un_motif` (V9) | `valeur: null` sans motif ; `valeur` non nulle avec motif | Refus dans les deux sens | « Absence de donnée dite, jamais comblée » (ADR 0000 §5) au niveau du registre. Le motif est ce qui distingue une absence constatée d'un oubli |
| REG-10 | `etabli_le_present_et_pas_dans_le_futur` (V10) | `etabli_le` > `date_registre` | Refus | Un appariement est une déclaration humaine datée ; sans date d'établissement, la relecture ligne par ligne exigée n'a pas de point de reprise |
| REG-11 | `bornes_ordonnees_et_incluses` (V11, V12, V13) | `debut > fin` ; identifiant hors période du porteur ; groupe hors période de législature | Refus | Une période d'appariement plus large que celle de l'entité fait dater une mesure d'avant l'existence de son objet |
| REG-11b | `exception_v13_nommee_sur_uid` (V13) | `PO840056`, `debut` 2024-07-01, législature ouverte le 2024-07-18 ; puis le **même** écart sur `PO845401` | Le premier passe, le second est refusé | V13 et V16 sont insatisfiables ensemble sur l'organe des non-inscrits (registre-entites.md §4.2). Sans le second cas, l'exception se généralise et V13 ne dit plus rien ; sans le premier, aucun registre conforme n'est constructible |
| REG-12 [C] | `aucun_chevauchement` (V14) | deux périodes du même couple (porteur, source, valeur) qui se chevauchent | Refus | Deux périodes qui se chevauchent rendent la jointure par date non déterministe : deux exécutions peuvent choisir deux lignes |
| REG-13 [C] | `groupe_egal_a_la_source` (V15, V16) | `nom`, `sigle`, `debut`, `fin` divergents de `libelle`, `libelleAbrev`, `viMoDe` de `organes-groupes-l17.json` | Refus | **C'est ce qui rend le registre falsifiable contre sa source.** Sans lui, le registre devient une opinion sur les groupes ; avec lui, une divergence est soit une source qui a bougé, soit une main qui a édité — les deux exigent un humain |
| REG-14 | `sigle_nest_pas_une_cle` | `PO872880` : `libelleAbrev` `UDDPLR` ≠ `libelleAbrege` `UDR` | Aucune jointure ne se fait sur une abréviation | Deux entités distinctes portent la même abréviation dans la source ; une clé sur l'abréviation les fusionne |
| REG-15 | `succession_relie_deux_periodes_contigues` (V18) | relation dont `date` ≠ `fin du prédécesseur + 1 jour` | Refus ; le cas AD → UDR (2024-09-11 / 2024-09-12) passe | `organePrecedentRef` est nul sur les 63 GP : la chaîne de succession est **déclarée à la main**, donc elle est fausse si rien ne la contrôle |
| REG-16 [C] | `lexique_interdit_absent_cles_comprises` (V20) | valeur puis clé portant un terme interdit | Refus | definition-of-done.md §11 vérifie le diff ; ce test vérifie le fichier entier, y compris ce qui est entré avant la mise en place du grep |
| REG-17 | `longueurs_maximales` (V21) | `sigle` de 41 caractères ; `remarque` de 141 | Refus | Les limites de l'ADR 0000 §5 sont des contraintes techniques, pas des préférences : le graphe est dimensionné dessus |
| REG-18 [C] | `aucun_champ_de_valorisation_ni_nom_de_personne` (V22) | ligne portant une coordonnée ; ligne portant un nom de député | Refus | Découle de V1, testé séparément parce que c'est l'invariant qu'un contributeur pressé cassera en premier — et parce que le registre ne doit contenir aucune personne |
| REG-19 [P] | `forme_canonique_idempotente` (V23) | fichier réordonné, indenté à 4 espaces, avec CRLF, avec BOM | Refus ; et `format(format(x)) == format(x)` sur l'extrait | Une main qui édite sans passer par le formateur rend le diff illisible, ce qui tue la relecture ligne par ligne — le seul contrôle obligatoire du projet |
| REG-20 [C] | `famille_absente_jamais_remplie_par_une_autre` | entité sans identifiant CHES et sans code de nuance | Deux lignes `null` avec motifs distincts ; aucune valeur reprise de l'autre famille | C'est la règle des trois familles appliquée un étage plus bas. La violer ici est indétectable en aval : la valeur arrive dans le graphe comme une mesure |
| REG-21 [C] | `au_plus_une_ancre_par_pole_et_par_date` (V24) | deux groupes portant `ancre_axe.pole = "gauche"` sur des périodes qui se chevauchent ; les deux mêmes sur périodes disjointes | Refus, puis acceptation | Deux ancres du même pôle à la même date rendent la transformation d'ancrage non définie : le pipeline choisirait selon l'ordre de lecture du fichier, et deux exécutions donneraient deux axes. Test **inécrivable avant** le contrat `0.3.0`, le registre n'ayant pas le champ |
| REG-22 [C] | `ancre_manquante_arrete_le_pipeline` (V25) | registre sans ancre `droite` valide à la date d'agrégation | Échec bruyant ; aucune ligne `votes` émise, aucune ancre de remplacement choisie | RG-31. Une substitution silencieuse d'ancre change l'échelle de toutes les positions publiées sans changer leur identifiant d'échelle — le défaut exact que le découplage de contrats.md §2.3 rend visible. Test **inécrivable avant** le contrat `0.3.0` |

| REG-23 [C] | `echantillon_et_registre_declarent_les_memes_sources` | `docs/brique0/echantillons/registre-l17.json` et `data/registre/partis.json` | Les deux déclarent les mêmes `url`, `empreinte_sha256`, `remarque` et `licence` par source | La bascule du nuancier vers le fichier régional du 2nd tour (RG-111, qui écarte un fichier portant des noms de candidats) a été appliquée au registre et pas à l'échantillon. Celui-ci a gardé l'empreinte de la source nominative et « 22 codes » au lieu de 17. Aucun test ne comparait les deux : le défaut était muet, et l'échantillon renvoyait un reproducteur vers le fichier écarté |

REG-21 et REG-22 sont les deux tests que le blocage 3 de positionnement.md §11
laissait inécrivables : sans le champ `ancre_axe`, il n'existait rien à déclarer,
donc rien à contredire. REG-11b est le test de l'exception nommée à V13, qui n'a
pu s'écrire qu'une fois la date d'ouverture de la législature sourcée
(registre-entites.md §4.1).

V17 (existence de l'identifiant dans le fichier CHES / ParlGov téléchargé) est un
test de **niveau 2** : aucune fixture CHES n'entre dans le dépôt (ADR 0000 §4).
Hors ligne, il est marqué sauté avec son motif et sa date de reprise, comme
l'exige definition-of-done.md.

---

## 10. Module `preuves` — registre de preuves

| ID | Test | Entrée | Sortie attendue | Ce qui casse si le test disparaît |
|---|---|---|---|---|
| PRE-01 [C] | `ligne_sans_date_refusee` | ligne sans `date_source` ; ligne sans `date_calcul` | Refus dans les deux cas | « Une position sans date ne veut rien dire » (README). Une ligne sans date est indatable après coup : l'information est perdue à l'écriture, pas à la lecture |
| PRE-02 [C] | `ligne_sans_preuve_de_source_refusee` | ligne sans empreinte d'archive | Refus | Les archives de l'Assemblée sont reconstruites chaque nuit et **rétroactivement modifiables**. Sans SHA-256, « rejouable » veut dire « recalculable sur des données qui ont changé » |
| PRE-03 | `ligne_porte_la_version_qui_la_produite` | ligne sans version de contrat | Refus | Règle dure de l'ADR 0000 §6 : sans ce champ, la politique de version est décorative et une preuve publiée devient non interprétable |
| PRE-04 | `fonction_fixe_epinglee` | ligne sans version de langage ni cible de compilation | Refus | ADR 0001 §1.7 : la reproductibilité bit-à-bit est une propriété du couple (implémentation, version). Sans épinglage consigné, elle n'est vérifiable par personne |
| PRE-05 [P] | `ajout_idempotent` | même calcul appliqué deux fois | Fichier identique à l'octet, aucune ligne dupliquée | Le registre est en ajout seul : sans idempotence, chaque exécution du cron hebdomadaire double le fichier et le graphe affiche N marqueurs empilés |
| PRE-06 [P] | `ajout_seul_jamais_de_reecriture` | registre existant, nouveau calcul de valeur différente | Les lignes antérieures sont **inchangées octet pour octet** ; une nouvelle ligne est ajoutée | « Aucun recalcul silencieux » (ADR 0000 §6). Une réécriture en place détruit l'historique, qui est le seul mécanisme d'où les dérives sortent gratuitement |
| PRE-07 [P] | `rejouabilite_octet_pour_octet` [T1] | reconstruction complète depuis les fixtures | Fichier identique à l'octet à la référence figée | Exigé par definition-of-done.md §5. C'est la promesse principale du projet ; sans test, elle n'est vérifiée que le jour où quelqu'un la met en doute |
| PRE-08 [C] | `horloge_absente_des_valeurs_calculees` | pipeline exécuté deux fois avec deux dates de calcul injectées | Toutes les valeurs identiques ; seul le champ `date_calcul` diffère | definition-of-done.md §6. L'horloge injectée comme paramètre est ce qui rend ce test possible : une horloge lue au fond d'une fonction ne se teste pas |
| PRE-09 [C] | `trois_familles_jamais_moyennees` | lignes de preuve des trois familles pour une même entité | Trois lignes distinctes, chacune avec sa méthode et son échelle nommée. **Aucune** ligne dont la valeur dérive de deux familles | La règle non négociable n° 6. La violation naturelle n'est pas une moyenne assumée : c'est un champ « position consolidée » ajouté pour simplifier le front |
| PRE-10 [C] | `aucun_nom_de_champ_de_consolidation` | schéma des lignes | Refus de tout nom de champ appartenant à une liste noire déclarée (`moyenne`, `consolide`, `synthese`, `score_global`, `indice`, `note`) | Le lexique interdit se contourne en renommant. La liste noire porte sur la **forme** que prendrait la violation, pas sur son vocabulaire d'affichage |
| PRE-11 | `valeur_publiee_trace_vers_une_ligne` | export du front | Chaque valeur affichable pointe une ligne existante du registre | definition-of-done.md §15. Une valeur sans preuve est exactement l'estimation affichée à la place d'une mesure, interdite par la règle permanente de ROADMAP.md |
| PRE-12 | `date_darret_derivee_du_registre` | registre dont la dernière date de calcul est connue | Le bandeau « données arrêtées le … » vaut cette date ; aucune saisie manuelle, aucune horloge | Mitigation du risque d'abandon (ADR 0000 §8) : le bandeau ne vaut que s'il est dérivé. Saisi à la main, il reste juste tant que quelqu'un y pense |
| PRE-13 | `codage_consigne_dans_la_ligne` | ligne de la famille votes | Le codage `+1/0/−1` et le choix `abstention = 0` sont consignés | Le choix `abstention = 0` est une décision, pas une évidence (positionnement.md §4). Non consignée, elle devient indiscernable d'un défaut |

---

## 10bis. Module `familles` — les familles `experts` et `administratif`

Les deux familles que le pipeline n'a pas produites jusqu'ici. Elles ne
partagent avec `votes` **ni source, ni échelle, ni méthode, ni objet mesuré**
(contrats.md §2.2 et §2.3), et l'écart entre les trois est le produit : aucun
test de ce module ne compare deux familles autrement que pour vérifier qu'elles
ne se rencontrent pas.

Fixtures : **aucun fichier**, et c'est délibéré. L'ADR 0000 §4 interdit de
commiter un extrait de CHES — aucune licence n'y est publiée. Le fichier de
résultats du ministère, lui, porte `Nom candidat n`, `Prénom candidat n` et
`Elu n`, que RG-111 interdit d'ingérer « y compris dans un fichier intermédiaire
ou de cache ». Les deux dialectes de CSV sont donc **fabriqués dans la suite**,
avec les colonnes réelles et des valeurs inventées : le test exerce la forme des
sources sans en republier la donnée. Le corpus réel est exercé au niveau 3, par
une exécution du binaire.

| ID | Test | Entrée | Sortie attendue | Ce qui casse si le test disparaît |
|---|---|---|---|---|
| FAM-01 | `lecture_csv_deux_dialectes` | CSV à virgule non guillemeté ; CSV à point-virgule guillemeté, avec un séparateur et un guillemet doublé **dans** un champ, et des fins de ligne CRLF | Les champs sont rendus tels quels, guillemets retirés, `"DUPOND;LE JEUNE"` en un seul champ | Un lecteur qui coupe sur le séparateur sans connaître les guillemets décale toutes les colonnes suivantes : la nuance lue devient celle du candidat d'à côté, sans erreur et sans trace |
| FAM-02 | `lrgen_filtre_le_pays_et_ne_comble_pas` | vague portant deux pays dont un `party_id` commun, et une ligne sans valeur mesurée ; en-tête amputé | Seul `country = 6` entre ; la ligne sans valeur est absente, jamais à zéro ; l'en-tête amputé est un refus | `party_id` n'est unique que dans un pays. Sans le filtre, une entité française reçoit la position d'un parti étranger, et la ligne publiée ne porte rien qui le dise |
| FAM-03 [C] | `aucune_colonne_nominative_ingeree` | fichier de nuances dont les colonnes `Nom`, `Prénom` et `Elu` portent des valeurs qui **sont** des codes valides | Seule la colonne `Nuance candidat n` est lue ; aucune valeur nominative n'entre | RG-111. Un lecteur qui balaye toutes les colonnes puis filtre après coup a déjà constitué, en mémoire et dans sa sortie, l'appariement d'une nuance à une personne |
| FAM-04 [C] | `appariement_par_le_registre_seul` | registre où une coalition ne déclare aucun appariement CHES | Aucune ligne `experts` pour elle ; une ligne par appariement déclaré, ni plus ni moins | L'appariement parti → source est une **déclaration du registre**, pas une lecture de la source (registre-entites.md §2.5). Apparier par le libellé attribue à une coalition une position que l'enquête n'a pas mesurée |
| FAM-05 [C] | `absence_dite_avec_son_code` | entité absente de CHES ; entité que le nuancier ne tranche pas | `hors_source` dans le premier cas, `source_indeterminee` dans le second, chacun avec son motif ; jamais une case vide | contrats.md §2.4. Une absence sans code oblige le front à lire de la prose pour savoir s'il dessine une bande, et une case vide se lit comme un centre |
| FAM-06 [C] | `citation_ches_mot_pour_mot` | lignes `experts` ; citation abrégée injectée dans une entrée | Chaque entrée CHES porte la citation de sources.md §4, caractère pour caractère ; l'abrégée est refusée par I23 ; le nuancier n'en porte aucune | CHES n'accorde **aucun droit de republication** : la citation est la seule condition écrite obtenue, et elle est portée par la donnée. Une citation reformulée ou remontée en mention globale ne satisfait pas la condition et attribue à un jeu la citation d'un autre |
| FAM-07 [C] | `nuance_publiee_en_code_jamais_en_nombre` | lignes `administratif` | `valeur` nulle, `valeur_code` renseigné, `echelle.min`, `max` et `decimales` nuls | Le nuançage n'est pas une mesure scientifique : c'est une classification administrative révisée par circulaire et contestée au Conseil d'État. Publiée comme un nombre, elle entre sur un axe et devient moyennable |
| FAM-08 [C] | `echelles_propres_jamais_partagees` | lignes des deux familles | Une échelle par famille, distincte, et jamais celle des votes | Règle non négociable n° 6. La forme mécanique de l'interdiction de moyenner est l'échelle : deux familles qui partagent une graduation sont additionnables sans qu'aucune revue le voie |
| FAM-09 [C] | `referentiel_et_observation_ne_se_traitent_pas_pareil` | `party_id` apparié et absent de la vague ; code apparié et non constaté au tour observé | Refus dans le premier cas ; ligne d'absence motivée dans le second, le code non constaté nommé dans le motif | CHES énumère des partis : un `party_id` disparu est une divergence registre/source, comme V15 et V16. Le nuancier énumère ce que des candidatures ont porté : `COM` est constaté au 1er tour de 2024 et pas au 2nd, et le confondre avec une divergence arrêterait le pipeline sur un fait normal — ou, dans l'autre sens, publierait une nuance que le scrutin n'a jamais portée |
| FAM-10 | `lignes_valides_et_identifiant_stable` | lignes des deux familles | Les invariants du §6 et la confrontation au registre passent ; deux constructions rendent le même octet | Filet de non-régression : une ligne qui ne passe pas `verifier` n'est pas publiée, et ce test le dit avant l'exécution réelle plutôt qu'après |

---

## 11. Module `export` — contrat de sortie consommé par le front

Le contrat d'export n'est spécifié nulle part en détail : l'ADR 0001 §6 dit
seulement que le pipeline publie un schéma JSON et que le front échoue
bruyamment à la construction si le schéma ne correspond pas. Les tests ci-dessous
portent sur ce qui est déjà arbitré ; le reste est listé au §17.

| ID | Test | Entrée | Sortie attendue | Ce qui casse si le test disparaît |
|---|---|---|---|---|
| EXP-01 [C] | `trois_marqueurs_trois_echelles_nommees` | export sur fixtures | Chaque marqueur porte sa famille, son échelle nommée et sa date. Aucune graduation commune, aucun écart chiffré entre familles | La bibliothèque de graphiques n'existe pas qui refuse de moyenner trois séries sur un axe commun (ADR 0001 §3) : c'est le code du projet qui doit refuser, donc c'est le test du projet qui doit l'exiger |
| EXP-02 [C] | `aucune_comparaison_inter_legislature` | export portant deux numéros de législature | Aucun champ d'écart, de ratio ni de flèche entre deux législatures | Interdiction dérivée de l'ADR 0000 §3, et positionnement.md §7 montre qu'elle tient même avec deux législatures ingérées : le bloc central se déplace de 0,15 à 0,25 en un an sans cause interprétable |
| EXP-03 [C] | `absence_dite_jamais_comblee` | groupe en « non mesuré » | Le mot « non mesuré » et la raison ; jamais « neutre », « centre », ni une case vide muette | Une case vide se lit comme un centre. C'est la façon la plus directe de publier une position que personne n'a mesurée |
| EXP-04 | `limites_de_longueur` | étiquettes et légendes de l'export | Étiquette ≤ 40 caractères, légende ≤ 140, une phrase | ADR 0000 §5. Le graphe est dimensionné sur ces limites : les dépasser casse le rendu avant de casser le ton |
| EXP-05 [C] | `lexique_interdit_absent_de_lexport` | export complet, clés comprises | Aucune occurrence | Le grep de definition-of-done.md §11 porte sur le diff. Un terme introduit par une donnée plutôt que par du code y échappe |
| EXP-06 | `schema_publie_et_verifie_a_la_construction` | export ne correspondant pas au schéma publié | Échec bruyant de la construction du front | C'est la couverture explicite de la concession de l'ADR 0001 §6 : les types du registre sont définis deux fois, sans compilateur entre les deux. Sans ce contrôle, la divergence se découvre en production |
| EXP-07 [T1] | `instantane_de_lexport_complet` | fixtures | Fichier identique à l'octet à l'instantané figé | Filet de non-régression global : attrape ce que les tests nominatifs ne prévoient pas. Il ne remplace aucun d'entre eux — un instantané dit qu'une sortie a changé, jamais qu'elle est fausse |
| EXP-08 | `aucune_couleur_seule_porteuse_dinformation` | rendu du graphe | Chaque marqueur porte une forme et un `aria-label` distincts | Exigence d'accessibilité de l'ADR 0001 §7 étape 6. `couleurAssociee` de l'AN est disponible et tentant : c'est une convention éditoriale de l'Assemblée, pas une donnée du projet |

---

## 12. Récapitulatif des tests de propriété

Une propriété se justifie quand un exemple ne peut pas couvrir la classe
d'erreur. Ici, six seulement, et chacune correspond à une faute déjà mesurée.

| Propriété | Tests | Pourquoi un exemple ne suffit pas |
|---|---|---|
| Invariance par permutation (lignes, colonnes, ordre des fichiers) | ING-13, EST-05, EST-06, MAT-07 | L'erreur ne se manifeste que pour certains ordres. Un exemple unique a de fortes chances de tomber sur un ordre où le bug est muet |
| Invariance à l'initialisation, signe compris | EST-07 | Deux initialisations sur quatre inversent l'axe. Tester une seule initialisation, c'est tester la moitié du problème |
| Idempotence | EST-09, PRE-05, REG-19 | Un double passage accidentel ne change aucune sortie observable dans deux cas sur trois. Seule la propriété le voit |
| Ajout seul | PRE-06 | Ce qui doit être vérifié est la **non**-modification de lignes arbitraires déjà présentes, donc sur un registre quelconque |
| Forme canonique | REG-19 | Le nombre de façons de dénormaliser un JSON est ouvert ; la propriété `format ∘ format = format` les couvre toutes |
| Rejouabilité | PRE-07 | La propriété porte sur la composition de toute la chaîne, pas sur un maillon |

Les permutations sont des **tables fixes écrites dans le test**, pas des tirages.
Une suite déterministe ne tire pas au sort ce qu'elle vérifie ; et une propriété
qui ne tient que sur certaines permutations aléatoires serait un test qui échoue
un jour sur dix, c'est-à-dire un test qui finit désactivé.

---

## 13. Les tests qui refusent une erreur de conception

Marqués [C] ci-dessus. Ils ne cherchent pas un défaut de code : ils échouent si
une décision d'architecture est renversée, y compris par un code parfaitement
correct. C'est la seule catégorie qu'un développeur pressé n'écrit jamais de
lui-même, parce qu'elle ne correspond à aucun bug observé.

| Décision protégée | Tests | Forme que prendrait la violation |
|---|---|---|
| Les trois familles ne sont jamais moyennées | PRE-09, PRE-10, EXP-01, REG-20 | Un champ « position consolidée » ajouté pour simplifier le front, ou un identifiant repris d'une famille voisine faute de mieux |
| Absent ≠ abstention | MAT-01, MAT-03, MAT-09, ING-09, ING-11 | Un `unwrap_or(0.0)`, un masque de valeurs manquantes, ou la lecture de `nonVotantsVolontaires` |
| Aucune position sans date | PRE-01, PRE-02, PRE-03, AGR-06, AGR-07 | Un champ de date rendu optionnel pour faire passer un cas limite |
| Aucune coordonnée individuelle publiée | AGR-01, AGR-11, REG-18 | Un champ de débogage laissé dans l'export |
| Aucun axe ni champ à pôle dépréciatif | REG-02, REG-16, REG-18, EXP-05, PRE-10 | Un nom technique anodin portant une valorisation |
| Aucun identifiant technique ne nomme une entité mesurée | REG-21, EXP-05 | Un `echelle.id` ou un `methode.id` construit sur le sigle des deux groupes ancres, gravé dans chaque ligne par la règle d'ajout seul (RG-112) |
| L'ancrage de l'axe est de la donnée, jamais du code | REG-21, REG-22 | Deux ancres en dur dans le pipeline, ou une ancre de remplacement choisie en silence quand la déclarée manque |
| Aucune horloge dans une valeur calculée | PRE-08, AGR-06, EST-03, MAT-08 | Une date par défaut prise sur l'horloge quand l'entrée n'en fournit pas |
| Déterminisme du signe et de l'échelle | EST-02, EST-07, EST-11, EST-12 | Une ancre de remplacement choisie automatiquement quand l'ancre déclarée manque |
| Aucune réduction flottante parallèle | EST-14 | Une ligne de `Cargo.toml` |
| Aucune comparaison inter-législature | EXP-02 | Une flèche « ce parti a bougé de » ajoutée quand la XVIe sera ingérée |
| Le registre est falsifiable contre sa source | REG-13, REG-07 | Une valeur corrigée à la main dans le registre au lieu d'être corrigée dans la lecture de la source |
| Une référence pendante est bloquante | ING-17, REG-06 | Un groupe « inconnu » ou une valeur par défaut |
| L'absence est dite, jamais comblée | REG-09, AGR-04, EXP-03 | Une case vide, une médiane accompagnée d'un avertissement |

---

## 14. Ordre d'écriture — cycles TDD

Chaque cycle produit quelque chose qui se montre, conformément à ROADMAP.md.
Dans chaque cycle, tous les tests listés sont écrits et **rouges** avant la
première ligne d'implémentation.

| # | Cycle | Tests écrits d'abord | Ce qui se montre à la fin |
|---|---|---|---|
| **−1** | La récupération des archives | REC-01 → REC-06 | Six tests rouges puis verts hors ligne, puis une exécution réelle : deux archives, leurs tailles, leurs deux empreintes et la date de source. Le cron hebdomadaire peut s'exécuter |
| **0** | Les trois adaptateurs de sérialisation | ADA-01, ADA-02, ADA-03, ADA-04 | Quatre tests rouges, puis verts, sur des fixtures réelles. Aucune logique métier encore écrite. Les trois pièges de l'ADR 0001 §1.4 sont neutralisés avant qu'ils ne coûtent |
| **1** | Lecture d'un scrutin et ses cohérences | ING-05, ING-06, ING-07, ING-08 | Le contenu d'un scrutin, affiché : date, votants, décomptes, avec les invariants de la source vérifiés |
| **2** | Le codage | MAT-01, MAT-02, MAT-03, ING-09, ING-10, ING-11, ING-12 | Les triplets d'un scrutin. **Le cycle le plus important du projet** : MAT-01 est écrit avant tout ce qui pourrait le contourner |
| **3** | Le filtre et le décompte | MAT-04, MAT-05, MAT-06 | La sortie visible de la roadmap v0.1 : retenus, écartés, motifs — sur cinq scrutins d'abord, sur 8 434 au niveau 2 |
| **4** | Le rattachement au groupe | ING-14, ING-15, ING-16, ING-17, ING-18, ING-19 | Les triplets portent un groupe daté, y compris sur les blocs `PO0`. Le tableau des désaccords ventilation / AMO30 est affiché |
| **5** | La matrice canonique | MAT-07, MAT-08, MAT-09, ING-13 | Un artefact de matrice trié, à en-tête portant les empreintes, identique d'une exécution à l'autre |
| **6** | L'estimateur | EST-01, EST-02, EST-03, EST-04 | Un axe sur matrice synthétique, exact. Rien de publiable encore, et c'est voulu : les invariances viennent avant les vraies données |
| **7** | Les invariances | EST-05, EST-06, EST-07, EST-13, EST-14 | Six permutations et quatre initialisations qui donnent le même axe. Le déterminisme cesse d'être une intention |
| **8** | L'ancrage | EST-08, EST-09, EST-10, EST-11, EST-12, EST-17 | Un axe où LFI-NFP vaut −1 et RN +1, ancres lues dans le registre, pipeline qui s'arrête si l'ancre manque |
| **9** | L'agrégation et la règle de non-publication | AGR-01 → AGR-12 | Les bandes de partis avec médiane, IQR, écart-type de rééchantillonnage, effectif, date — et NI et LIOT en « non mesuré » avec leur raison. La première sortie qui ressemble au produit |
| **10** | Les alarmes | EST-15, EST-16 | Deux corpus synthétiques dégradés qui font passer la famille « votes » en « non mesuré ». Le risque accepté de l'ADR 0000 §8 devient un comportement observable |
| **11** | Le registre d'entités | REG-01 → REG-23 | Un validateur qui refuse vingt-deux fichiers fautifs et accepte l'extrait. Une PR de correction du registre devient possible |
| **12** | Le registre de preuves | PRE-01 → PRE-13 | Le JSONL en ajout seul, rejouable, idempotent. Le contrat des briques 1-3 existe |
| **13** | L'export et le front | EXP-01 → EXP-08 | Le graphe de la roadmap v0.6, sur fixtures |
| **14** | Le niveau 2 | Contrôles 8, 9, 10, 11 de positionnement.md §9, sur archive en cache | Les chiffres du corpus réel retrouvés par l'implémentation Rust, aux tolérances du §4. C'est ici, et seulement ici, que 0,608 et 0,652 sont vérifiés. **0,608, pas 0,591** : la valeur actée est celle du recomptage sur le corpus retenu (ADR 0003 §3, positionnement.md §11), et un test écrit sur 0,591 épinglerait la mesure écartée |

Deux points d'ordre non négociables : **MAT-01 avant tout code de matrice**
(cycle 2), parce que l'erreur qu'il attrape est invisible sur la sortie publiée ;
**EST-07 avant l'ancrage** (cycle 7 avant 8), parce qu'un ancrage écrit d'abord
masque l'indétermination du signe au lieu de la corriger, et le test d'invariance
passerait alors pour la mauvaise raison.

---

## 14bis. Où vit chaque suite

La porte 1 désigne chaque suite **par son fichier**, jamais par le répertoire qui
la contiendra. Ce tableau est donc à la fois la convention d'emplacement et la
condition d'activation de la porte : une suite mord le jour où son fichier naît,
et pas avant.

| Préfixe | Fichier de la suite |
|---|---|
| `REC` | `scripts/test-recuperer-sources.sh` |
| `ADA` | `pipeline/tests/adaptateurs.rs` |
| `ING` | `pipeline/tests/ingestion.rs` |
| `MAT` | `pipeline/tests/matrice.rs` |
| `EST` | `pipeline/tests/estimateur.rs` |
| `AGR` | `pipeline/tests/agregation.rs` |
| `REG` | `pipeline/tests/registre.rs` |
| `PRE` | `pipeline/tests/preuves.rs` |
| `FAM` | `pipeline/tests/familles.rs` |
| `EXP` | `pipeline/tests/export.rs`, `web/src/contrat.test.ts` |

Pointer le répertoire `pipeline/` aurait fait basculer sept préfixes — 96
identifiants — au premier fichier Rust créé. Un contrôle qui refuse quatre-vingt-
seize fois d'un coup se fait contourner, pas satisfaire.

La même falaise se rejouait à l'échelle d'un préfixe. Les quatre adaptateurs de
sérialisation du cycle 0 (`ADA-01` à `ADA-04`) sont écrits quinze cycles avant le
reste de l'ingestion : les loger dans `pipeline/tests/ingestion.rs` aurait fait
refuser la porte dix-neuf fois pour quatre tests écrits. Ils portent donc leur
propre préfixe et leur propre fichier, et `ING` reste « en attente » jusqu'au
cycle 1. Le découpage suit la frontière réelle : `ADA` ne connaît que la forme
sérialisée de la source, `ING` connaît le scrutin.

## 15. Couverture minimale exigée pour merger

Trois portes, dans l'ordre de force. La première seule est un critère de
qualité ; les deux autres sont des détecteurs d'oubli.

### 15.0 Où chaque porte est implémentée, et laquelle ne l'est pas encore

Une porte déclarée ici et absente de `ci.yml` est contournée dès la première PR
pressée : c'est arrivé deux fois le 2026-08-27. Le tableau dit donc, pour chaque
porte, le travail qui l'exécute — ou pourquoi elle ne peut pas encore l'être.

| Porte | Travail de `ci.yml` | État |
|---|---|---|
| Porte 1 — identifiants de test déclarés | `portes` → `./scripts/portes-de-ci.sh identifiants` | **Active.** S'applique par préfixe, dès que la suite de ce préfixe existe. Au 2026-08-27 : `REC` et `ADA`, soit dix identifiants écrits ; les 92 autres sont annoncés « en attente », pas passés sous silence |
| Porte 2 — couverture de **lignes** sur `matrice` et `estimateur` | `couverture-et-determinisme` | **Active.** La couverture de **branches** exigerait `-Z coverage-options=branch`, donc le compilateur *nightly* : l'épingler contredirait la reproductibilité que l'ADR 0001 impose, *nightly* étant une cible mouvante. Le seuil de branches reste **déclaré et non tenu**, et c'est dit ici plutôt que masqué par un drapeau qui échoue |
| Porte 3 — lignes ≥ 70 % sur le binaire | `couverture-et-determinisme` | **Squelette**, même garde, même commande |
| Lexique sur le diff | `garde-fous` → `./scripts/lexique.sh code` et `docs` | **Active** depuis l'origine |
| Deux exécutions consécutives, artefacts identiques | `couverture-et-determinisme` | **Squelette.** C'est le contrôle 1 de contrats.md §8.2 ; il exige le binaire `contrepoint` |
| Isolement de l'horloge, et ajout seul | `couverture-et-determinisme` | **Squelettes.** Contrôles 2 et 3 de contrats.md §8.2 |
| Aucun test sauté sans date de reprise | `portes` → `./scripts/portes-de-ci.sh tests-ignores` | **Active.** Sans source Rust ni TypeScript elle le dit et ne prétend rien |
| I12 et I13 de contrats.md §6 | `portes` → `./scripts/portes-de-ci.sh invariants` | **Active** sur les artefacts qui existent, `data/registre/partis.json` compris |
| Artefacts contre `schemas/` | `artefacts` | **Active** depuis l'origine |
| Suite hors ligne de récupération (§4bis) | `garde-fous` → `./scripts/test-recuperer-sources.sh` | **Active** |
| Tolérance modifiée sans motif dans la description de la PR | aucun | **Non automatisable.** Le motif est dans un champ de PR, pas dans l'arbre ; c'est un point de revue, et il reste écrit ci-dessous comme tel |
| Contrôles 8 à 11 et 15 de positionnement.md §9 | aucun | **Niveau 2**, hors PR par construction (§2) |

Chaque porte active a été prouvée capable d'échouer : violation fabriquée,
rouge constaté, violation retirée, vert constaté. Une porte qui n'a jamais
montré son rouge est une liste qu'on construit et qu'on n'utilise pas.

`ci-ok` reste la seule vérification exigée par les protections de branches. Les
travaux `portes` et `couverture-et-determinisme` y figurent en `needs` ; ils ne
sont jamais ajoutés à `CHECKS` de `scripts/setup-github.sh`, un travail gardé
par `hashFiles` ne publiant aucun contexte quand il saute
([../mise-en-route.md](../mise-en-route.md) §1).

### Porte 1 — la liste nominative des invariants, bloquante

Tout identifiant de test de ce document (`ADA-…`, `ING-…`, `MAT-…`, `EST-…`, `AGR-…`,
`REG-…`, `PRE-…`, `FAM-…`, `EXP-…`) doit exister comme fonction de test dans l'arbre, dès
que la PR touche le module concerné. La vérification est une boucle de grep en
CI : les identifiants sont dans le document, les noms de fonctions sont dans le
code, l'un cite l'autre.

La boucle va **dans les deux sens** : un identifiant déclaré ici et absent de la
suite est une porte fantôme ; un identifiant cité dans une suite et absent d'ici
est un test que le plan ne connaît pas, donc une couverture qu'aucun document
n'oppose. Les deux sont bloquants. La correspondance préfixe → suite est dans
`scripts/portes-de-ci.sh` et nulle part ailleurs : `REC` →
`scripts/test-recuperer-sources.sh`, `ADA` → `pipeline/tests/adaptateurs.rs`, `EXP` →
`pipeline` et `web`, tout le reste → `pipeline`. Un préfixe dont aucune suite n'existe encore est annoncé « en
attente » avec son décompte, et n'échoue pas — la porte ne prétend pas mesurer
ce qui n'est pas écrit.

**Pourquoi c'est la porte principale.** Un pourcentage de couverture mesure
l'exécution, pas l'assertion : un test qui appelle tout le pipeline et n'affirme
rien couvre 100 % des lignes. Ce que le projet doit garantir n'est pas un taux,
c'est une liste — celle de definition-of-done.md §9, de positionnement.md §9 et
des 25 règles du registre. Une liste se vérifie exactement ; un taux s'approche.

### Porte 2 — couverture sur deux modules (lignes ; branches hors de portée sur stable)

Sur `matrice` (codage, filtre) et sur `estimateur` (ancrage, alarmes)
uniquement.

**Pourquoi ces deux-là, et pourquoi 90 %.** Ce sont les deux modules où une
erreur est **invisible sur la sortie publiée** : le mauvais codage donne une
corrélation de 0,9969 avec le bon et des moyennes de groupe à moins de 0,04, et
un ancrage fautif produit toujours −1 et +1 aux extrémités. Partout ailleurs, une
erreur se voit. 90 % de branches, pas 100 % : le dernier dixième est fait de
branches d'erreur d'entrées-sorties dont le test coûte une simulation de système
de fichiers, ce qui achète peu et casse souvent.

### Porte 3 — plancher global 70 % de lignes sur le binaire du pipeline

**Pourquoi un plancher si bas.** Il ne mesure pas la qualité, il détecte le
module entier arrivé sans test — le seul mode de défaillance qu'un taux global
attrape mieux qu'une revue. Le fixer plus haut le transformerait en objectif, et
un objectif de couverture se satisfait par des tests sans assertion, qui coûtent
du temps de CI et donnent une fausse assurance.

### Ce qui n'a pas de porte de couverture

Le front. Sa couverture utile est le contrat : EXP-06 échoue à la construction si
le schéma diverge, EXP-07 échoue si le rendu change, EXP-01 à EXP-05 échouent si
une règle éditoriale est violée. Un taux de couverture sur du code de rendu
mesure surtout la quantité de tests de rendu écrits.

### Portes non chiffrées, également bloquantes

- Le grep de lexique de definition-of-done.md §11 sur le diff. Travail `garde-fous`.
- Deux exécutions consécutives donnant des artefacts identiques, empreinte dans la description de la PR (§4). Squelette, travail `couverture-et-determinisme`.
- Aucun test sauté — `#[ignore]`, `.skip(`, `.todo(`, `xit(` — sans **date de reprise sur la ligne**. Travail `portes`. Le motif seul ne suffit pas : c'est la date qui empêche « à réactiver plus tard » de durer trois ans.
- Aucune tolérance modifiée sans motif dans la description de la PR. Point de revue : le motif vit dans un champ de PR, hors de l'arbre, et aucun travail ne peut le lire.
- Les invariants mécaniques I12 et I13 de contrats.md §6, sur tout artefact de `public/api/` et de `data/`. Travail `portes`.

---

## 16. Contradictions de spécification — toutes actées

Ces points ne sont pas des choix de tests : ce sont des spécifications qui se
contredisent. Un test écrit avant l'arbitrage épinglerait la mauvaise version.
**Un plan de tests ne tranche pas une spécification** : il constate le conflit.
L'arbitrage est [../adr/0003-arbitrages-de-coherence.md](../adr/0003-arbitrages-de-coherence.md).

Le tableau n'est pas supprimé : il documente que les contradictions ont existé,
et laquelle chaque cycle aurait figée sans arbitrage.

| # | Contradiction | Cycle bloqué | Version actée |
|---|---|---|---|
| 1 | **Source du rattachement au groupe.** ingestion-votes.md §8 : le bloc de ventilation, AMO30 en recours. positionnement.md §1 : « le rattachement ne peut pas être lu dans le fichier de scrutin ; il doit venir des mandats » | 4 | **Acté** — ADR 0003 §1 : le bloc de ventilation du scrutin, mesuré sur 1 270 476 lignes nominatives (280 désaccords sur cellules exprimées, tous par retard de `dateFin` du mandat de non-inscrit). AMO30 : recours pour les blocs `PO0`, périodes de validité, contrôle croisé. `ING-16` à `ING-19` peuvent être écrits |
| 2 | **Nombre de scrutins à `PO0`.** ingestion-votes.md §8 : 14 scrutins, dont 13 le 2024-12-02 et un le 2026-04-16. positionnement.md §1 : 14 scrutins, dont 13 le 2024-12-02, un le 2025-04-07 **et** un le 2026-04-16 — soit 15 énumérés pour 14 annoncés | 4 | **Acté** — ADR 0003 §3 : recompté sur l'archive complète, **14** au total, **12** le 2024-12-02, 1 le 2025-04-07, 1 le 2026-04-16. Juste sur le total, faux sur la ventilation dans les deux documents |
| 3 | **Gain du rang 1.** ADR 0001 §1.3 : 2,1 %. positionnement.md §1 et blocage 1 **portaient** 59,1 %, invariant au codage | 14 | **Acté** — ADR 0003 §3 : **60,8 % du résidu** après constante par scrutin, recompté ; 51,5 % de la variance totale, corpus retenu de 7 979 scrutins. L'ADR 0001 se trompait d'un facteur trente. Les 59,1 % venaient d'une autre matrice d'entrée, pas d'une convergence moindre (positionnement.md §11) ; les six occurrences et les trois `0,591` de ce plan sont corrigés depuis le 2026-08-28 |
| 4 | **Dispersion publiée.** ROADMAP.md v0.2 et methode.md §2 : « variance intra-groupe publiée ». positionnement.md §6 : IQR et étendue, « aucune variance » | 9 | **Acté** — ADR 0003 §3, tranché par la relecture juridique : **IQR et écart-type de rééchantillonnage, jamais la variance, jamais l'étendue** — une borne d'étendue est la coordonnée d'un membre identifiable. `AGR-03` l'épingle ; ROADMAP.md v0.2 et methode.md §2 mis à jour dans la PR de l'ADR |
| 5 | **Fixture du cas `votant` objet nu.** ADR 0001 §7 étape 0 cite `VTANR5L17V5646`. `echantillons/README.md` livre `VTANR5L17V5268` | 0 | **Acté** — ADR 0003 §3 : `VTANR5L17V5268`, la fixture qui existe |
| 6 | **Filtre de participation.** ROADMAP.md v0.1 et methode.md exigent « un seuil documenté ». ingestion-votes.md §6 : aucun seuil, la mesure dit que le seuil justifiable est l'absence de seuil | 3 | **Acté** — ADR 0003 §2 : **aucun seuil**. Filtre unique `min(pour, contre) ≥ 1` — 455 écartés sur 8 434, dont les 23 motions de censure, 7 979 retenus. `nombreVotants` publié, jamais une porte. `MAT-05` l'épingle ; ROADMAP.md v0.1 et methode.md §1 mis à jour dans la PR de l'ADR |
| 7 | **Version de la Licence Ouverte.** ADR 0000 §4 : « lire 2.0 dans le PDF », marqué `A VERIFIER`. ingestion-votes.md §1 : v1.0, lu dans le PDF, citation verbatim | 1 | **Acté** — ADR 0003 §3 : **v1.0** pour l'Assemblée, **`lov2`** pour le nuancier sur data.gouv.fr. Deux sources, deux versions, pas une contradiction |

Le §5 de ce plan porte le même arbitrage pour le cas `PO0` : il est désormais
acté par l'ADR 0003 §1, et `ING-16` n'attend plus rien.

---

## 17. `A VERIFIER`

| Point | Comment le vérifier |
|---|---|
| Le contrat d'export front n'est spécifié par aucun document : ni noms de champs, ni forme du schéma publié, ni emplacement | Écrire la spec d'export avant le cycle 13. Les tests EXP-01 à EXP-08 portent sur des règles, pas sur des champs, précisément pour ne pas inventer un schéma |
| Total réel des scrutins à `organeRef: "PO0"` et leurs dates | Recompter sur `Scrutins.json.zip`, identifiée par son empreinte de **contenu** `c8457f34…` — l'empreinte d'archive varie selon la construction servie (contrats.md §2.8) —, au cycle 4 |
| Valeurs de référence à l'échelle des fixtures (nombre de triplets par scrutin, décomptes par motif, positions de la matrice synthétique) | Aucune n'est écrite ici : elles sont **produites par la première implémentation verte**, lues, et alors seulement figées comme instantané. Les inventer maintenant serait fabriquer une référence |
| Faisabilité de la porte 2 à 90 % de branches | Mesurer avec `cargo llvm-cov` au cycle 5, quand `matrice` existe. Si le chiffre est inatteignable pour une raison structurelle, c'est le chiffre qui change, avec son motif écrit — pas la porte qui disparaît |
| Fonctionnement de `insta` sur une cible unique pour les tests T1 | Vérifier au cycle 5 que l'instantané peut être conditionné à la cible de compilation sans drapeau d'environnement lu à l'exécution |
