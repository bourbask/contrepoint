# Brique 0 — de la matrice de votes à une position défendable

Statut : **proposition de méthode, mesurée**. Date des mesures : 2026-08-27.
Remplace la formulation « ACP / analyse des correspondances, premier axe » de
[docs/methode.md](../methode.md) §2 et de ROADMAP.md v0.2, qui n'est pas
applicable en l'état. Toutes les valeurs citées viennent d'exécutions sur les
fichiers réels de l'Assemblée nationale, XVIIe législature, et sont
reproductibles par les commandes du §1.

---

## 0. Décision en dix lignes

1. **L'ACP est écartée**, pas parce qu'elle serait moins savante qu'un modèle IRT, mais parce qu'elle exige une matrice complète que le corpus ne fournit pas : 23,03 % des cellules sont observées et les 77 % manquants ne sont pas répartis au hasard entre les groupes.
2. **L'IRT bayésien et W-NOMINATE sont écartés aussi**, sur une mesure et non sur une intuition : connaître le seul groupe d'un député classe correctement 94,28 % de ses votes ; le premier axe estimé en classe 95,41 %. Le gain de tout estimateur individuel plus fin porte sur 1,13 point de désaccord, dont une partie est procédurale et non idéologique.
3. **L'estimateur retenu** est un rang 1 sur cellules observées avec une constante par scrutin, déjà écrit et mesuré dans [docs/adr/0001-stack.md](../adr/0001-stack.md) §1.3, ici spécifié complètement et corrigé sur deux points (initialisation, normalisation).
4. **Le signe et l'échelle sont fixés par une transformation affine ancrée sur deux médianes de groupe nommées.** Après ancrage, deux initialisations qui produisent des axes exactement opposés donnent les mêmes positions à 1,6·10⁻¹⁵, et une permutation des lignes de la matrice ne les déplace pas.
5. **La position publiée est celle d'un groupe, jamais d'un député** — et cette règle est maintenant chiffrée : la magnitude de la position individuelle croît avec la seule assiduité (|x| médian 0,0276 dans le quartile le moins présent, 0,0484 dans le plus présent).
6. **La dispersion publiée est l'écart interquartile du groupe et son écart-type de rééchantillonnage**, pas une variance : l'axe n'a pas d'unité naturelle, une variance en unités arbitraires n'est pas lisible. Ni étendue, ni minimum, ni maximum : une borne d'étendue est la coordonnée d'un membre identifiable du groupe.
7. **Un deuxième axe existe et il est fort** (norme relative 0,652, il absorbe la moitié du résidu du premier). Il n'oppose pas la gauche à la droite mais la majorité relative à ses oppositions. Ne pas le mesurer, c'est laisser croire que la première dimension est tout le comportement de vote.
8. **La comparaison entre législatures n'est pas résoluble proprement** en v0 : les deux axes ne vivent pas dans le même espace, et l'ancrage sur groupes disparus est impossible. Ce qui s'affiche à la place est énoncé au §7.
9. **Aucune projection sur l'échelle CHES ni sur RILE.** Les trois familles ne sont pas moyennées, elles ne sont pas non plus recalibrées l'une sur l'autre : recalibrer, c'est moyenner avec une étape de plus.
10. **Le gain du rang 1 vaut 60,8 % du résidu** après constante par scrutin, soit 51,5 % de la variance totale — recompté le 2026-08-27 sur le corpus retenu de 7 979 scrutins ([verification-2026-08-27.md](verification-2026-08-27.md) §3) et acté par [../adr/0003-arbitrages-de-coherence.md](../adr/0003-arbitrages-de-coherence.md) §3. Les 2,1 % de l'ADR 0001 étaient faux d'un facteur trente ; les 59,1 % que ce document portait sont une mesure antérieure sur une autre matrice d'entrée, écartée au §11.

---

## 1. Ce qui a été mesuré, et comment le refaire

```sh
curl -C - -O https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip
sha256sum Scrutins.json.zip
# empreinte d'ARCHIVE, variable selon la construction servie : aa767a2a… ou c5e405f1…
# l'empreinte de CONTENU, seule stable, s'obtient par la méthode de contrats.md §2.8 :
#   c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c
# (2026-08-27, 26 317 479 octets)
curl -C - -O https://data.assemblee-nationale.fr/static/openData/repository/16/loi/scrutins/Scrutins.json.zip  # 10 107 912 octets
```

| # | Mesure | Valeur constatée |
|---|---|---|
| 1 | Scrutins du jeu 17 | **8 434**, du **2024-10-08** au **2026-07-21** |
| 2 | Types | 8 339 scrutins publics ordinaires, 72 solennels, 23 motions de censure |
| 3 | Acteurs distincts, cellules observées | 642 acteurs, **1 247 093** cellules, densité **0,2303** |
| 4 | Votes exprimés par délégation (`parDelegation`) | **191 628, soit 15,37 %** |
| 5 | Conformité à la position majoritaire du groupe | **93,00 %** (94,28 % hors abstentions) |
| 6 | Classification correcte, règle « position majoritaire du groupe » | **94,28 %** |
| 7 | Classification correcte, axe rang 1 + coupure optimale par scrutin | **95,41 %** |
| 8 | Part de la somme des carrés prise par une constante par scrutin | **15,2 %**, corpus retenu (7 979 scrutins), recomptage du 2026-08-27. La mesure d'exploration de ce tableau donnait 19,8 % : deux matrices d'entrée, pas deux convergences — §11 |
| 9 | Gain du rang 1 sur le résidu après cette constante | **60,8 %** pour la constante réajustée à chaque itération conjointement au rang 1 — c'est ce modèle qui est implémenté — soit 51,5 % de la variance totale, corpus retenu (7 979 scrutins). Une constante calculée **une fois** donne **59,2 %** (60,9 % hors abstentions ; identique en codage 0/1) : l'écart de 1,6 point est de modèle et non de convergence, mesuré le 2026-08-28 en ajustant les deux côte à côte. Le **59,1 %** que ce tableau portait jusqu'au 2026-08-28 venait de l'exploration Python hors dépôt ; la mesure côte à côte de l'implémentation Rust donne 59,2 % (verification-2026-08-27.md §3). Les 0,1 point d'écart entre les deux implémentations ne sont pas expliqués — `A VERIFIER` — §11 |
| 10 | Gain du rang 2 sur le résidu après le rang 1 | **50,1 %**, norme relative du second axe **0,652** |
| 11 | Corrélation entre les deux axes | 0,081 |
| 12 | Indétermination du signe, 4 initialisations | **2 sur 4 inversées**, \|corrélation\| = 1,000000 |
| 13 | Écart des positions ancrées entre deux initialisations opposées | **1,6·10⁻¹⁵** |
| 14 | Écart des positions ancrées après permutation des lignes | **1,1·10⁻¹⁵** |
| 15 | Recouvrement des acteurs XVI ∩ XVII | **422**, soit 65,7 % des acteurs de la XVIIe |
| 16 | Scrutins du jeu 17 antérieurs au 2024-10-08 | **aucun** |

Les scripts d'exploration sont en Python hors dépôt, conformément au rôle que
l'ADR 0001 §2 lui assigne. Les valeurs des mesures 5 à 14 sont celles que la
suite de tests devra retrouver dans l'implémentation Rust, aux tolérances du §9.

### Deux pièges de données non encore consignés

**`organeRef: "PO0"`.** Sur **14 scrutins** — **12** le 2024-12-02, un le
2025-04-07, un le 2026-04-16, recompté le 2026-08-27 (voir
[verification-2026-08-27.md](verification-2026-08-27.md)) — la ventilation ne
porte pas de groupe résoluble : 1 916
cellules et 336 acteurs sont rattachés à un identifiant `PO0` absent du
référentiel AMO30. Le rattachement **n'est pas lisible dans ces 14 fichiers** :
il y est résolu par les mandats, avec période de validité. Partout ailleurs, le
groupe est lu dans le bloc de ventilation du scrutin, où il est daté par
construction — c'est la source actée par
[../adr/0003-arbitrages-de-coherence.md](../adr/0003-arbitrages-de-coherence.md)
§1, et AMO30 n'intervient qu'en recours. Un pipeline qui fait confiance au
scrutin *sans traiter ce cas* perd le groupe de 335 députés sur ces dates, ou
fabrique un treizième groupe.

**Deux identifiants pour UDR** (`PO847173` et `PO872880`), cohérent avec les
périodes de validité relevées par l'ADR 0001 §1.5. L'agrégation par
`organeRef` produit donc deux lignes pour un même parti : la réconciliation
appartient au registre d'entités, pas à l'estimateur.

---

## 2. Les données manquantes ne sont pas réparties au hasard

### La mesure

L'absence n'est pas un accident individuel : elle est structurée par groupe.
Part des sièges-scrutins où le député du groupe a exprimé un vote :

| Groupe | Taux de participation | Votes par délégation |
|---|---|---|
| RN | **0,339** | 15,1 % |
| LFI-NFP | 0,313 | 14,9 % |
| ECOS | 0,277 | 16,9 % |
| SOC | 0,269 | 16,2 % |
| UDR | 0,234–0,240 | 5,2–18,5 % |
| EPR | 0,228 | 18,1 % |
| HOR | 0,227 | 11,6 % |
| DEM | 0,221 | 14,3 % |
| DR | **0,176** | 18,1 % |
| GDR | 0,130 | 8,6 % |
| NI | 0,121 | 10,1 % |
| LIOT | **0,119** | 8,0 % |

Un facteur **2,8** entre le groupe le plus présent et le moins présent. Au
niveau individuel : participation médiane 0,233, premier décile 0,070 — un
dixième des députés a exprimé un vote sur moins de 7 % des scrutins de sa
propre fenêtre de mandat.

Cette structure n'est pas un hasard de calendrier. Elle a une cause
institutionnelle mesurable : **le scrutin public est demandé**, il n'est pas la
procédure normale. Le vote à main levée n'est ni enregistré ni publié
(Assemblée nationale, fiche de synthèse n° 56). Répartition des demandeurs des
8 434 scrutins :

| Demandeur | Scrutins |
|---|---|
| Présidence du groupe RN | 1 991 |
| Présidence du groupe LFI-NFP | 1 304 |
| Présidence du groupe EPR | 807 |
| Présidence de séance | 979 |
| Présidence du groupe DR | 538 |
| Présidence du groupe SOC | 463 |
| Présidence du groupe ECOS | 363 |
| Présidence du groupe HOR | 221 |
| Présidence du groupe UDR | 124 |

Deux groupes d'opposition ont provoqué 39 % du corpus. La matrice observée
n'est donc pas un échantillon des positions de l'Assemblée : c'est
l'enregistrement des moments où un groupe a demandé un scrutin public —
l'effet de sélection décrit par Hug (2010) sur les scrutins
enregistrés, ici avec le demandeur nominatif dans la donnée.

S'y ajoute que **15,37 % des positions enregistrées sont exprimées par
délégation** : la ligne du fichier porte le nom d'un député dont le vote a été
émis par un collègue. La position individuelle enregistrée n'est pas, dans un
cas sur sept, un acte personnel constaté.

### Ce que fait une ACP naïve là-dessus, et pourquoi c'est refusé

Une ACP ou une analyse des correspondances exige une matrice complète. Les
trois façons usuelles de la compléter échouent toutes ici :

| Traitement de l'absence | Effet mesurable | Décision |
|---|---|---|
| Absent = abstention (0) | Le vecteur d'un député présent à 7 % est composé à 93 % de zéros : il est projeté au centre. Le centre se remplit des députés les moins présents, et LIOT, GDR, NI y arrivent par leur assiduité | Interdit par methode.md, et faux |
| Absent = imputation par la moyenne du scrutin | Identique en pire : l'absent reçoit la position majoritaire du moment, donc l'axe mesure la composition des présents | Refusé |
| Suppression des lignes ou colonnes incomplètes | Il n'existe pas de seuil de participation donnant à la fois densité et corpus : à `nombreVotants ≥ 300` il reste 4,3 % des scrutins (ADR 0001 §1.2) | Impossible |

L'absence n'est pas ignorable au sens statistique : elle dépend du groupe, et
le groupe est l'objet à situer. C'est exactement le cas traité par
Rosas, Shomer & Haptonstahl (2015), et la conséquence connue est un déplacement
des positions estimées, non une simple perte de précision.

**Ce que l'estimateur retenu fait à la place** : il n'impute rien. Le résidu
n'est sommé que sur les cellules observées, l'absence n'entre dans aucune somme.
Cela ne corrige pas le mécanisme d'absence — aucun estimateur ne le corrige sans
un modèle explicite de la décision de voter — mais cela évite de fabriquer une
position là où il n'y en a pas.

**Le résidu de ce choix, chiffré.** Ne pas imputer ne suffit pas : la magnitude
de la position individuelle reste liée à l'assiduité.

| Quartile de participation | n | \|position\| médiane | Votes exprimés, médiane |
|---|---|---|---|
| Q1 (< 0,14) | 161 | 0,0276 | 633 |
| Q2 | 160 | 0,0375 | 1 579 |
| Q3 | 160 | 0,0462 | 2 256 |
| Q4 (≥ 0,32) | 161 | 0,0484 | 3 369 |

Corrélation de Spearman entre \|position\| et participation : **+0,336**. Entre
position signée et participation : **−0,013**. Lecture : l'absence ne tire pas
vers la gauche ou vers la droite, elle tire **vers zéro**. La position estimée d'un député peu
présent est plus proche de zéro que son comportement de vote observé, ce qui est
le centrisme artificiel que methode.md interdit de fabriquer — et il subsiste
malgré l'interdiction d'imputer.

**Conséquence retenue.** Ce retrait vers zéro est une raison supplémentaire de
ne publier aucune position individuelle (ADR 0000 §2), et il impose de vérifier
que l'agrégation de groupe ne s'y expose pas. Elle ne s'y expose pas pour les
groupes constitués — relever le seuil de participation à 0,20 déplace la médiane
de LFI-NFP de 0,0001, celle de RN de 0,0001, celle de ECOS de 0,0002 — mais elle
s'y expose pour LIOT (−0,0010 → +0,0100) et NI (+0,0139 → −0,0015). D'où la
règle de non-publication du §6.

---

## 3. ACP / AFC contre IRT et point idéal bayésien : la comparaison

### Le chiffre qui tranche

La question n'est pas « quel estimateur est le plus correct en théorie », c'est
« que reste-t-il à expliquer ». Sur les 1 175 920 votes exprimés hors
abstention :

| Règle de prédiction | Classification correcte |
|---|---|
| « Le député vote la position majoritaire de son groupe » | **94,28 %** |
| Axe rang 1 estimé + coupure optimale par scrutin | **95,41 %** |
| Gain de l'axe sur la connaissance du seul groupe | **+1,13 point** |

Un estimateur plus sophistiqué se dispute donc, au mieux, une fraction de 5,72 %
de votes non expliqués par l'appartenance. Et ces 5,72 % ne sont pas de la
nuance idéologique pure : ils contiennent les votes par délégation, les erreurs
de manipulation d'un boîtier, et le vote tactique.

C'est le résultat que la littérature sur les parlements à discipline élevée
prédit, et qu'elle a déjà documenté sur le cas français. Rosenthal & Voeten
(2004), sur les scrutins de la IVe République, soutiennent que les hypothèses
d'erreur des méthodes paramétriques — indépendance et identité de distribution
des écarts au modèle — sont sévèrement violées quand la discipline est forte et
que les institutions récompensent le comportement tactique ; ils recommandent la
méthode non paramétrique de Poole (2000) plutôt qu'un modèle paramétrique.
Spirling & McLean (2007) vont plus loin sur les Communes : le vote y est classé
correctement à **99,1 %** en une dimension, contre 91,9 % pour le 107ᵉ Sénat
américain, et l'ordre obtenu **n'est pas interprétable** — les députés
travaillistes réputés les plus à gauche (Corbyn, Benn, Skinner, Abbott) sont
placés à droite de l'ensemble du cabinet, parce que ce que la dimension capte
est la structure gouvernement contre opposition, pas une position sur un
continuum. Bräuninger, Müller & Stecker montrent sur les Länder allemands et les
Communes que le poids des considérations tactiques dépasse souvent celui des
préférences de contenu, et que les motions déposées par un parti hors du système
sont rejetées quasi universellement — donc peu informatives.

### Le deuxième axe confirme le diagnostic sur la XVIIe législature

Le second axe estimé, dont la norme vaut 0,652 de celle du premier et qui
absorbe **50,1 %** du résidu restant, ordonne les groupes ainsi :

| Pôle négatif | | Pôle positif | |
|---|---|---|---|
| RN | −0,034 | EPR | +0,061 |
| LFI-NFP | −0,025 | DEM | +0,056 |
| UDR | −0,024 | HOR | +0,052 |
| GDR | −0,020 | LIOT | +0,029 |
| ECOS | −0,018 | DR | +0,024 |
| SOC | −0,004 | NI | +0,017 |

Ce n'est pas une seconde dimension idéologique : c'est le socle de la majorité
relative d'un côté, ses oppositions de l'autre, RN et LFI-NFP au même pôle. Sa
corrélation avec le premier axe est de 0,081 : les deux structures coexistent
dans la même matrice. Le premier axe range correctement les groupes de gauche à
droite — il faut le dire, c'est un argument de validité — mais il ne le fait
qu'avec 60,8 % du résidu, et une structure procédurale de poids comparable
se tient juste derrière.

### Coût, dépendances, déterminisme

| Estimateur | Implémentation disponible | Déterminisme | Coût pour ce projet |
|---|---|---|---|
| ACP / AFC | Toute bibliothèque | Signe et ordre des axes non garantis | **Inapplicable** : exige une matrice complète |
| Rang 1 sur cellules observées (retenu) | ≈ 60 lignes, aucune dépendance (ADR 0001 §1.8) | Reproductible à l'exécution ; signe indéterminé, corrigé par ancrage | 5,2 s en Rust |
| Optimal Classification (Poole 2000) | R + Fortran | Déterministe, mais ne rend qu'un ordre, pas une position | Dépendance système, interdite par l'ADR 0001 |
| W-NOMINATE | R + Fortran | Idem | Idem |
| IRT bayésien MCMC (Clinton, Jackman & Rivers 2004 ; IDEAL) | R, Stan, JAGS | **Non déterministe** sans graine épinglée, et alors dépendant de la version du générateur | Contredit la promesse de reproductibilité, pour +1,13 point au maximum |
| IRT variationnel / EM (Imai, Lo & Olmsted) | R (`emIRT`) | Déterministe à initialisation fixée | Voie d'évolution acceptable, sans gain aujourd'hui |

Le seul apport propre d'un modèle IRT est un intervalle a posteriori **par
acteur**. Or la coordonnée par acteur n'est publiée nulle part (ADR 0000 §2). Le
gain porte exactement sur l'objet que le produit refuse d'afficher, tandis que
l'incertitude dont le produit a besoin — celle de la position **de groupe** —
s'obtient par rééchantillonnage des scrutins sans changer d'estimateur (§6).

**Décision.** Non à l'ACP, non à l'IRT, et le « ils deviennent justifiés si le
premier axe s'avère instable » de methode.md §2 est conservé mais reformulé : le
déclencheur n'est pas l'instabilité — elle est mesurée et faible — c'est
l'apparition d'un usage qui exigerait une position individuelle publiée. Cet
usage est exclu par l'ADR 0000. **L'IRT n'a donc pas de déclencheur dans le
périmètre actuel.**

---

## 4. Spécification de l'estimateur retenu

Sur les seules cellules observées, avec `pour = +1`, `contre = −1`,
`abstention = 0`, l'absence n'entrant dans aucune somme :

```
v[i,j] ≈ b[j] + x[i] · y[j]
```

1. **Constante par scrutin** `b[j]` = moyenne des valeurs observées du scrutin `j`. Elle absorbe le fait qu'un scrutin est majoritairement pour ou majoritairement contre, qui n'est pas une information de position.
2. **Résidu** `r[i,j] = v[i,j] − b[j]`, calculé une fois.
3. **Initialisation** `x[i]` = moyenne des résidus observés du député `i`. Elle ne dépend **pas** de l'indice de ligne, seulement de la donnée : condition nécessaire pour que la permutation des lignes ne change rien (mesure 14). Aucun générateur pseudo-aléatoire, donc aucune graine.
4. **Moindres carrés alternés**, 300 itérations, `y` puis `x`, normalisation de `x` à la norme 1 à chaque itération, l'échelle étant portée par `y`.
5. **Sommes séquentielles uniquement.** Aucune réduction flottante parallèle (ADR 0001 §1.6).
6. **Second axe** : mêmes étapes sur le résidu du premier. Il n'est pas publié comme position ; il sert au critère de séparation du §5 et à la note de méthode.
7. **Normalisation affine finale** du §5, appliquée avant tout arrondi ou écriture.

Le choix `abstention = 0` est une décision, pas une évidence : elle place
l'abstention à mi-distance du pour et du contre, ce qui est un modèle. Il est
retenu parce qu'il est vérifié sans effet sur la structure — hors abstentions,
le gain du rang 1 passe de 59,1 % à 60,9 % (mesure d'exploration, une seule
matrice d'entrée pour les deux codages, §11) et l'ordre des groupes est
inchangé — et parce que l'écarter reviendrait à traiter l'abstention comme une absence,
c'est-à-dire à confondre les deux choses que methode.md sépare. Le codage est
consigné dans la ligne de preuve.

---

## 5. Déterminisme : signe, échelle, quasi-dégénérescence

### Le problème, mesuré

Le couple `(x, y)` et le couple `(−x, −y)` donnent le même produit : le signe
n'est pas identifié. Sur 4 initialisations, **2 renvoient l'axe inversé**, avec
une corrélation de exactement ±1,000000 à la solution de référence. Deux
initialisations mathématiquement neutres — indice de ligne contre moyenne des
résidus — donnent des axes **exactement opposés** (corrélation −1,00000000).
L'échelle n'est pas identifiée non plus : la normalisation par la norme donne
des positions autour de 0,05, celle de l'ADR 0001 §1.3 autour de 1,3. Aucune des
deux n'est plus vraie que l'autre, et une valeur publiée sans convention
d'échelle n'est comparable à rien, même d'une exécution à la suivante.

### La procédure exacte

Soit `m(g)` la médiane des positions des membres du groupe `g` (moitié inférieure
des deux valeurs centrales si l'effectif est pair, pour rester déterministe).
Points d'ancrage nommés : `A = LFI-NFP`, `B = RN`.

```
x_publié = ( 2·x − m(A) − m(B) ) / ( m(B) − m(A) )
```

Par construction `m(A) = −1` et `m(B) = +1` exactement. Cette unique
transformation fixe le signe **et** l'échelle. Résultat mesuré :

| Contrôle | Écart maximal sur les 642 positions |
|---|---|
| Deux initialisations produisant des axes opposés | **1,6·10⁻¹⁵** |
| Permutation aléatoire des lignes de la matrice | **1,1·10⁻¹⁵** |

L'égalité n'est pas binaire : elle est à 2·10⁻¹⁵, ce qui est le comportement
attendu de l'arithmétique flottante et non un défaut. **Conséquence pour la
promesse « bit pour bit » du README** : l'arrondi à 4 décimales est appliqué
avant écriture, et c'est le fichier arrondi qui est identique à l'octet. La
promesse porte sur l'artefact publié, pas sur les flottants intermédiaires — la
formulation de l'ADR 0001 §1.7 est déjà celle-là.

### Choix des ancres, et ce qui le casse

LFI-NFP (73 membres, écart interquartile 0,047 en unités ancrées) et RN (129
membres, IQR 0,052) sont les deux plus gros groupes des extrémités de l'axe et
les plus homogènes. Deux fragilités à nommer :

- **Une ancre qui disparaît casse la convention.** Un groupe dissous, renommé ou scindé rend l'ancrage inapplicable et, selon l'ADR 0000 §6, un changement de convention de signe est une **majeure**. Le nom du groupe ne suffit donc pas : l'ancre est stockée dans le registre d'entités comme un identifiant de groupe avec période de validité, et le pipeline échoue bruyamment si l'ancre est absente. Il ne choisit **jamais** une ancre de remplacement tout seul.
- **L'ancrage est party-relatif.** Les positions ne sont pas en « unités de gauche-droite », elles sont en unités « médiane LFI-NFP à médiane RN ». Toute lecture qui suppose une échelle absolue est fausse. C'est à écrire dans la légende, dans les 140 caractères autorisés.

### Quasi-dégénérescence et critère d'instabilité

L'ordre des axes n'est stable que si leurs poids sont séparés. Mesures :

| Indicateur | Valeur de référence | Bootstrap sur les scrutins, 25 tirages |
|---|---|---|
| Norme relative du second axe, `s2/s1` | **0,652** | médiane 0,652, max 0,665, **min 0,002** |
| Corrélation de rang de l'ordre des groupes contre la référence | 1 | médiane 0,993, **min 0,888** |

Deux enseignements. Les axes 1 et 2 sont **bien séparés** — 0,652 est loin de 1,
il n'y a pas de quasi-dégénérescence sur ce corpus, donc pas de risque
d'échange des deux axes. Mais **un tirage sur 25 a produit un ajustement
dégénéré** (`s2/s1 = 0,002`, le second axe s'effondre) : la procédure peut
échouer sur un corpus légèrement différent, et il faut donc un critère qui le
détecte au lieu de publier le résultat.

**Critère retenu, à faire échouer la CI :**

| Contrôle | Bande d'acceptation | Action si hors bande |
|---|---|---|
| `s2/s1` | **0,10 ≤ s2/s1 ≤ 0,90** | Publication de la famille « votes » suspendue, colonne affichée « non mesuré » |
| Gain du rang 1 sur le résidu après constante par scrutin | **≥ 0,40** (mesuré 0,608, corpus retenu, recomptage du 2026-08-27) | Idem |
| Corrélation de rang des médianes de groupe contre l'ordre de référence figé | **≥ 0,95** (mesuré 1) | Idem |
| Écart-type de rééchantillonnage de la médiane d'un groupe | **≤ 0,05** en unités ancrées | Ce groupe seul passe en « non mesuré » |

La borne haute 0,90 sur `s2/s1` est la détection de quasi-dégénérescence ; la
borne basse 0,10 est la détection d'effondrement. Aucune n'est franchie
aujourd'hui, et c'est ce qui les rend utilisables comme alarmes.

---

## 6. Agrégation au niveau du groupe

### Estimateur : la médiane

Sur les 12 groupes, moyenne, médiane et moyenne pondérée par le nombre de votes
exprimés diffèrent de moins de 0,004 en unités brutes — sauf pour NI et LIOT, où
l'écart entre moyenne et médiane atteint 0,004 pour 0,014 de position, c'est-à-dire
le tiers de la valeur. La médiane est donc retenue : identique aux autres
estimateurs là où le groupe est homogène, plus robuste là où il ne l'est pas, et
insensible au retrait vers zéro d'un membre peu présent. Elle est aussi ce qui
définit l'ancrage du §5, ce qui évite deux estimateurs concurrents dans la même
chaîne.

### Dispersion publiée : écart interquartile et rééchantillonnage, pas variance ni étendue

Une variance sur un axe sans unité n'est pas lisible et invite à la comparaison
entre exécutions, qui n'a pas de sens. En **unités ancrées**, où l'**amplitude
ancrée** est 2,0 par construction — l'amplitude **brute** d'un ajustement, elle,
n'est pas 2,0 et dépend de l'ajustement : l'ADR 0001 §1.3 en mesurait 2,6 sur
son tableau d'exploration. Corpus retenu, 7 979 scrutins, rattachement du §6 :

| Groupe | n | Médiane ancrée | IQR | Écart-type de rééchantillonnage |
|---|---|---|---|---|
| LFI-NFP | 73 | **−1,0000** | 0,047 | 0 (ancre) |
| ECOS | 38 | −0,9876 | 0,033 | 0,0061 |
| GDR | 18 | −0,8619 | 0,083 | 0,0109 |
| SOC | 70 | −0,8352 | 0,057 | 0,0098 |
| LIOT | 25 | +0,1435 | **0,687** | 0,0176 |
| DEM | 41 | +0,1814 | 0,205 | 0,0158 |
| EPR | 115 | +0,2408 | 0,226 | 0,0207 |
| NI | 9 | +0,2664 | **0,623** | 0,0438 |
| HOR | 43 | +0,3837 | 0,200 | 0,0151 |
| DR | 63 | +0,7013 | 0,113 | 0,0123 |
| UDR | 18 | +0,9900 | 0,042 | 0,0060 |
| RN | 129 | **+1,0000** | 0,052 | 0 (ancre) |

Trois lectures à porter sur le site, chacune vérifiable dans ce tableau :

- **L'ordre obtenu correspond à l'ordre gauche-droite décrit par la littérature, sans qu'aucune étiquette n'ait été fournie au calcul.** C'est un argument de validité de la méthode.
- **L'IQR d'un groupe constitué vaut 2 à 6 % de l'amplitude ancrée** : l'axe sépare les groupes, pas les députés — affirmation de methode.md, maintenant chiffrée.
- **L'étendue n'est pas publiable.** Elle rendrait visible une limite réelle — un membre peu présent est ramené vers zéro et s'écarte fortement de la médiane de son groupe (§2) — mais un minimum et un maximum **sont** les coordonnées de deux membres du groupe. Sur un groupe de neuf membres, avec un code et un appariement publics, ces deux coordonnées sont réidentifiables en une exécution. L'IQR porte la même information sans exposer personne : sur un groupe constitué il vaut 2 à 6 % de l'amplitude, ce qui suffit à établir que l'axe sépare les groupes et non les députés.
- **L'ancrage supprime l'essentiel du bruit.** Avant ancrage, l'écart-type de rééchantillonnage était proportionnel à la valeur du groupe — c'était du flottement d'échelle, pas de l'incertitude de position. Après ancrage il tombe **entre 0,0000 et 0,0255**, soit au plus 1,3 % de l'amplitude. La fourchette « 0,006 à 0,021 » que ce paragraphe portait jusqu'au 2026-08-28 était fausse dans les deux sens : elle contredisait déjà le tableau ci-dessus, où NI vaut 0,0438, et elle décrivait une exploration Python antérieure à l'implémentation Rust. `A VERIFIER` — la fourchette 0,0000 à 0,0255 est celle du cycle 2, sur 25 plis déterminés ; elle se recompte par `cargo run --release --example verification-corpus` après `scripts/recuperer-sources.sh`, et le tableau ci-dessus n'a pas encore été réécrit sur cette exécution.
- **Le 0,0000 des deux ancres n'est pas une précision, c'est une tautologie.** LFI-NFP et RN portent un écart-type de rééchantillonnage nul parce que chaque pli est réancré sur leurs médianes : elles valent −1 et +1 par construction, dans le corpus complet comme dans chacun des 25 plis. Lu seul, l'artefact leur attribue la précision parfaite qu'elles n'ont pas. Toute lecture de cette colonne doit écarter les deux lignes d'ancre, et la publication ne doit jamais présenter leur zéro comme une mesure d'incertitude.

### Règle de non-publication

Un groupe n'est publié que si les trois conditions tiennent :

1. **IQR ≤ 0,25** en unités ancrées ;
2. **écart-type de rééchantillonnage ≤ 0,05** ;
3. **effectif retenu ≥ 10** députés ayant exprimé au moins 200 votes.

Aujourd'hui, **NI** (IQR 0,623) et **LIOT** (IQR 0,687) échouent au critère 1.
Ce n'est pas un défaut de l'estimateur : les non-inscrits ne sont pas un parti,
et LIOT réunit des députés d'au moins quatre partis déclarés distincts
(registre-entites.md §2.2). Leur case « votes » s'affiche **non
mesuré**, avec la raison — dispersion interne au-delà du seuil publié — et
jamais une valeur médiane accompagnée d'un avertissement, qui serait citée sans
l'avertissement. C'est l'application directe du « absence de donnée dite,
jamais comblée » de l'ADR 0000 §5.

Aucun seuil de participation n'est appliqué **avant** l'estimation : il
appauvrirait la matrice sans corriger le mécanisme d'absence. Le seuil de 200
votes ne porte que sur l'inclusion d'un député dans le calcul de la médiane de
son groupe, où son effet est mesuré comme nul sur les groupes constitués (§2).

---

## 7. Comparer d'une législature à l'autre : non résoluble proprement

Le problème est réel et il n'est pas d'implémentation. Deux matrices de
législatures différentes produisent deux axes qui ne vivent pas dans le même
espace : les scrutins ne sont pas les mêmes, l'ordre du jour n'est pas le même,
et l'échelle est fixée par une convention interne à chaque estimation. Un écart
de 0,3 entre deux législatures peut être un déplacement de position, un
changement d'ordre du jour, ou un changement de la composition des ancres.

**Ce qui existe dans la littérature et pourquoi ça ne s'applique pas ici.** La
comparabilité dans le temps s'obtient par des observations pontantes : DW-NOMINATE
par les élus siégeant sur plusieurs mandats, Bailey (2007) par des votes
identiques soumis à plusieurs institutions. Techniquement, le matériau existe :
**422 acteurs siègent dans les deux jeux XVI et XVII**, soit 65,7 % des acteurs
de la XVIIe. Mais l'hypothèse de pontage — les préférences des députés communs
sont constantes entre les deux législatures, et seul l'ordre du jour change — est
**invérifiable avec ces seules données** et fausse dans le cas français : entre
2022 et 2024 les groupes se recomposent, un député commun change de groupe, et
attribuer le déplacement à la personne ou à l'agenda demande une information
extérieure à la matrice.

**Et l'instabilité existe déjà à l'intérieur d'une seule législature.** En
coupant la XVIIe en deux moitiés chronologiques et en réestimant :

| Groupe | 1ʳᵉ moitié | 2ᵉ moitié |
|---|---|---|
| SOC | −0,897 | −0,761 |
| LIOT | +0,083 | +0,233 |
| DEM | +0,080 | +0,308 |
| EPR | +0,181 | +0,338 |
| HOR | +0,275 | +0,526 |
| DR | +0,612 | +0,792 |

La corrélation des positions individuelles entre les deux moitiés est de 0,941 et
celle des médianes de groupe de 0,9913 : l'ordre est stable. **Les valeurs, non.**
Le bloc central se déplace de 0,15 à 0,25 en une année, sur une amplitude de 2,0,
sans qu'aucune interprétation en termes de déplacement politique soit soutenable
— l'ordre du jour, lui, a changé. Un curseur temporel afficherait ce mouvement
comme une dérive. Il n'y en a pas.

**Décision.** Aucune comparaison inter-législature, et aucune dérive
intra-législature, n'est calculée ni affichée. Cela confirme la mise hors v0 du
curseur temporel et de la vue des dérives (ADR 0000 §1) pour une raison plus
forte que « il n'y a qu'une législature » : **même avec deux, la comparaison
serait non défendable**.

**Ce qui s'affiche à la place**, quand la XVIe sera ingérée :

1. Chaque législature reçoit son propre axe, ancré sur ses propres groupes, avec sa date, et les deux graphes sont côte à côte, **jamais superposés sur un axe commun**.
2. Ce qui se compare entre les deux n'est pas la position mais **l'ordre des groupes** — une quantité invariante à l'échelle — accompagné de la corrélation de rang mesurée.
3. La légende énonce que les deux échelles sont ancrées séparément. Aucune flèche, aucun écart chiffré entre les deux dates.

---

## 8. Comparer à l'échelle CHES ou à RILE : non

**Ce n'est pas fait, et ce n'est pas un manque.** Trois raisons, dans l'ordre de
force :

1. **La règle des trois familles l'interdit dans son esprit.** Recalibrer l'axe des votes sur l'échelle CHES 0–10 par régression, c'est utiliser CHES comme référence pour corriger les votes. C'est un moyennage avec une étape supplémentaire, et il détruit l'écart entre familles, qui est la seule information que le graphe existe pour montrer.
2. **Les objets mesurés ne sont pas les mêmes.** L'axe des votes situe un **groupe parlementaire** sur son comportement dans une chambre ; CHES situe un **parti** par jugement d'experts ; le nuancier situe une **candidature** par décision administrative. Le registre d'entités relie les identifiants ; il ne rend pas les mesures interchangeables.
3. **Même la comparabilité interne de CHES a demandé un dispositif dédié.** Bakker, Jolly, Polk & Poole (2014) ont dû recourir à des vignettes d'ancrage traitées comme votes-ponts pour rendre les placements comparables entre pays. Aucun instrument équivalent n'existe entre l'axe issu des votes et l'échelle CHES. Fabriquer la correspondance par corrélation, sans ponts, produirait un nombre non reproductible depuis les sources.

**Ce qui s'affiche à la place.** Trois marqueurs sur la même bande de parti,
chacun sur **sa propre échelle nommée** : « votes, unités médiane LFI-NFP à
médiane RN », « CHES, échelle 0–10, vague 2024 », « nuance, code administratif et
date de circulaire ». Une graduation par famille, aucune graduation commune,
aucune moyenne, aucun écart chiffré entre familles. Le lecteur voit que les
marqueurs ne coïncident pas ; c'est le produit.

Une seule quantité inter-familles est calculable sans recalibrage et reste
admissible comme diagnostic **interne**, pas comme valeur affichée : la
corrélation de rang entre l'ordre des groupes selon les votes et leur ordre
selon CHES. Elle sert à détecter une erreur d'appariement dans le registre
d'entités — un ordre soudain discordant signale un identifiant mal relié — et non
à valider une famille par une autre. Elle ne sort pas des tests.

---

## 9. Tests de sanité en intégration continue

Ce qui doit **échouer** quand le calcul se dégrade. Les valeurs de référence
sont celles du §1, à figer sur une fixture commitée de scrutins en Licence
Ouverte au moment de l'implémentation.

| # | Contrôle | Échoue si |
|---|---|---|
| 1 | Codage : l'absence n'entre dans aucune somme | Une cellule non observée contribue au résidu, ou une abstention est comptée comme absence |
| 2 | Invariance par permutation des lignes de la matrice | Écart des positions ancrées > 10⁻¹² |
| 3 | Invariance par permutation des colonnes | Idem |
| 4 | Invariance à l'initialisation : au moins 4 initialisations, dont deux donnant des axes opposés | Écart des positions ancrées > 10⁻¹² |
| 5 | Ancrage exact | `m(LFI-NFP) ≠ −1` ou `m(RN) ≠ +1` à 10⁻¹² près |
| 6 | Idempotence de l'ancrage : réappliquer la transformation ne change rien | Écart > 10⁻¹² |
| 7 | Ancre absente du registre d'entités à la date de calcul | Le pipeline ne s'arrête pas en erreur |
| 8 | Ordre des groupes contre l'ordre de référence figé | Corrélation de rang < 0,95 |
| 9 | Séparation des axes | `s2/s1` hors [0,10 ; 0,90] |
| 10 | Pouvoir explicatif | Gain du rang 1 sur le résidu après constante par scrutin < 0,40 |
| 11 | Apport sur la règle de groupe | Classification de l'axe ≤ classification par la position majoritaire du groupe |
| 12 | Règle de non-publication | Un groupe d'IQR > 0,25 ou d'écart-type de rééchantillonnage > 0,05 apparaît avec une valeur dans une sortie publiée |
| 13 | Aucune coordonnée individuelle publiée | Un identifiant d'acteur apparaît avec une valeur de position dans un fichier de sortie ou une réponse d'interface |
| 14 | Déterminisme d'exécution | Deux exécutions consécutives ne donnent pas des artefacts arrondis identiques à l'octet |
| 15 | Non-régression | Une position de groupe s'écarte de plus de 0,02 de l'instantané figé, sans mise à jour explicite de la référence dans la même modification |
| 16 | Rattachement au groupe | Un scrutin à `organeRef: "PO0"` fait perdre le groupe d'un député, ou le rattachement ignore une période de validité |
| 17 | Aucune réduction flottante parallèle sur le chemin de calcul | Détecté par revue et par le test 14 |
| 18 | Absence de comparaison inter-législature | Une sortie contient un écart, une flèche ou un ratio entre deux législatures |

Les tests 8, 9, 10 et 12 sont ceux qui transforment le risque accepté « le
premier axe est peu informatif » (ADR 0000 §8) en résultat : quand ils échouent,
la colonne « votes » s'affiche non mesurée et la v0 sort avec deux familles.

---

## 10. Ce qui n'est pas défendable, et donc n'est pas livré

| Non livré | Pourquoi |
|---|---|
| Position d'un député | Magnitude confondue avec l'assiduité (+0,336), 15,37 % des positions exprimées par délégation, et 94,28 % du comportement expliqué par le seul groupe |
| Distance entre deux groupes lue comme un écart idéologique | L'échelle est ancrée sur deux groupes ; seuls l'ordre et les écarts relatifs à l'amplitude ancrée 2,0 ont un sens |
| Dérive, évolution, comparaison de dates | Le bloc central se déplace de 0,15 à 0,25 entre les deux moitiés de la même législature sans cause interprétable |
| Position sur l'échelle CHES ou RILE | Recalibrer, c'est moyenner |
| Position de NI et de LIOT | Dispersion interne au-delà du seuil publié |
| Un chiffre unique de « position du parti » toutes familles confondues | Interdit par methode.md §3 |
| Une seconde dimension étiquetée | Le second axe oppose la majorité relative à ses oppositions ; il est mesuré, servi aux alarmes, et jamais présenté comme une position |

---

## 11. Points de blocage et `A VERIFIER`

**Blocage 1 — gain du rang 1. ~~Bloquant~~ Acté le 2026-08-27** par
[../adr/0003-arbitrages-de-coherence.md](../adr/0003-arbitrages-de-coherence.md)
§3, sur le recomptage de
[verification-2026-08-27.md](verification-2026-08-27.md) §3.

**Valeur normative : 60,8 % du résidu** après constante par scrutin, soit 51,5 %
de la variance totale, la constante seule en prenant 15,2 %. Corpus retenu,
7 979 scrutins. Les trois valeurs se relisent sur les trois sommes des carrés du
recomptage — 1 108 825,4 autour de la moyenne globale, 939 865,3 avec constante
par scrutin, 368 791,6 avec constante et rang 1 — et rien n'est à croire sur
parole ici : (939 865,3 − 368 791,6) / 939 865,3 = 0,6076.

Trois valeurs ont circulé pour cette quantité. D'où vient chacune :

- **2,1 %**, ADR 0001 dans une version antérieure — faux d'un facteur trente,
  corrigé.
- **59,1 %**, mesures 8 et 9 du §1 de ce document — mesure antérieure, écartée.
  Son invariance au codage tient toujours et n'est pas en cause (`+1/0/−1` ou
  `1/0`, avec ou sans les abstentions : 0,5912 / 0,5912 / 0,6095 / 0,6095 ; un
  double centrage scrutin **et** député donne 0,6121 pour un axe corrélé à
  0,998 au précédent).
- **60,8 %**, recomptage du 2026-08-27 — la valeur retenue.

**L'écart 59,1 / 60,8 n'est pas une convergence d'estimateur.** Le dire serait
une erreur, et elle a été écrite une fois. La part prise par la constante par
scrutin passe elle aussi de 19,8 % à 15,2 % entre les deux mesures ; or cette
constante est la moyenne des valeurs observées de chaque scrutin (§4, point 1),
une forme close sans itération. Aucun nombre d'itérations des moindres carrés
alternés ne la déplace. Les deux jeux de chiffres viennent donc de **deux
matrices d'entrée différentes**, mesurées séparément, et non d'un même ajustement
mieux convergé. Ce qui suit de cette lecture : les deux valeurs ne convergeront
pas l'une vers l'autre, et remplacer l'une par l'autre sans dire laquelle est
mesurée sur quoi refabrique la contradiction.

La phrase « l'axe ne résume qu'une petite part du comportement de vote », tirée
des 2,1 %, n'atteint pas le site : elle est fausse. La formulation juste est
celle du §3 — l'axe explique bien le résidu, mais l'appartenance au groupe
expliquait déjà 94,28 % des votes.

**Blocage 2 — le second axe n'est prévu nulle part.** ROADMAP.md v0.2 et
methode.md ne parlent que du premier axe. Une structure gouvernement contre
opposition de norme relative 0,652 existe dans la même matrice. Elle n'est pas
publiée comme position, mais elle doit être mentionnée dans la page de méthode,
sans quoi le premier axe est présenté comme l'ensemble du comportement de vote.

**Blocage 3 — l'ancre est un objet du registre d'entités. ~~Bloquant~~ Résolu.**
Le §5 impose que les deux ancres soient stockées comme identifiants de groupe avec
période de validité, et que l'absence d'une ancre arrête le pipeline.

Le registre porte désormais un champ `ancre_axe` sur les groupes — pôle, période
de validité, date d'établissement —, avec les règles de validation qui
l'accompagnent : au plus une ancre par pôle à une date donnée, et arrêt du
pipeline si un pôle manque à la date d'agrégation. Voir
[registre-entites.md](registre-entites.md) et
`schemas/registre-partis-1.schema.json`.

Le champ ne porte pas de `source`, et c'est délibéré : **le choix d'une ancre
n'est pas la lecture d'une source, c'est une décision de méthode datée.** Les
deux tests qui étaient inécrivables — la déclaration de l'ancre, et la fixation
du signe qui en dépend — existent maintenant.

**Précision, pas blocage.** Le jeu 17 ne contient **aucun scrutin avant le
2024-10-08**. L'exemple de l'ADR 0001 §1.5 sur « tout scrutin du 8 au 18 juillet
2024 » attribué à des non-inscrits ne correspond à aucune donnée existante. La
règle de période de validité reste nécessaire — 14 scrutins à `PO0` et deux
identifiants UDR le prouvent autrement.

| `A VERIFIER` | Comment |
|---|---|
| Volume et pages de Spirling & McLean : le DOI `10.1093/pan/mpl009` et la date d'accès anticipé (2006-11-22) sont lus sur le tiré à part ; la pagination du volume ne l'est pas | Ouvrir la notice Political Analysis du volume 15 |
| Année, volume et pages exacts de Bräuninger, Müller & Stecker (relevé : *Political Analysis* 24(2), 189–210, mise en ligne 2017-01-04) et de Rosas & Shomer (2008), *Legislative Studies Quarterly* | Notices des éditeurs |
| Référence exacte du théorème de Davis–Kahan, si le critère de séparation des axes du §5 doit être justifié par autre chose que la mesure de rééchantillonnage | Notice de la publication d'origine |
| Volume et pages de Imai, Lo & Olmsted, *American Political Science Review*, estimation rapide de points idéaux | Notice APSR |
| Le second identifiant UDR (`PO847173` / `PO872880`) correspond-il à deux périodes du même parti ou à deux entités distinctes | Lire `viMoDe` des deux organes dans AMO30 et arbitrer dans le registre d'entités |
| Corpus exact de la mesure à 59,1 % / 19,8 % des mesures 8 et 9 du §1. Le recomptage du 2026-08-27 déclare le sien — 7 979 scrutins retenus ; le tableau du §1 ne déclare pas le sien, et sa mesure 3 porte sur le corpus complet | Rejouer l'ajustement sur les 8 434 scrutins puis sur les 7 979, et comparer les trois sommes des carrés à celles de `verification-2026-08-27.md` §3 |

---

## Références

- **Assemblée nationale**, *Les votes à l'Assemblée nationale*, fiche de synthèse n° 56 — modes de votation, demande de scrutin public, délégation de vote. <https://www.assemblee-nationale.fr/dyn/synthese/fonctionnement-assemblee-nationale/travail-legislatif/les-votes-a-l-assemblee-nationale> (consulté le 2026-08-27)
- **Bailey, M. A.** (2007), « Comparable Preference Estimates across Time and Institutions for the Court, Congress, and Presidency », *American Journal of Political Science* 51(3), 433–448.
- **Bakker, R., Jolly, S., Polk, J. & Poole, K.** (2014), « The European Common Space: Extending the Use of Anchoring Vignettes », *The Journal of Politics* 76(4).
- **Bräuninger, T., Müller, J. & Stecker, C.**, « Modeling Preferences Using Roll Call Votes in Parliamentary Systems », *Political Analysis* 24(2), 189–210.
- **Clinton, J., Jackman, S. & Rivers, D.** (2004), « The Statistical Analysis of Roll Call Data », *American Political Science Review* 98(2), 355–370.
- **Fazekas, Z. & Hansen, M. E.** (2022), « Incentives for non-participation: absence in the United Kingdom House of Commons, 1997–2015 », *Public Choice* 191(1), 51–73.
- **Hug, S.** (2010), « Selection Effects in Roll Call Votes », *British Journal of Political Science* 40(1), 225–235.
- **Imai, K., Lo, J. & Olmsted, J.**, « Fast Estimation of Ideal Points with Massive Data », *American Political Science Review*.
- **Poole, K. T.** (2000), « Nonparametric Unfolding of Binary Choice Data », *Political Analysis* 8(3), 211–237.
- **Rosas, G. & Shomer, Y.** (2008), « Models of Nonresponse in Legislative Politics », *Legislative Studies Quarterly*.
- **Rosas, G., Shomer, Y. & Haptonstahl, S. R.** (2015), « No News Is News: Nonignorable Nonresponse in Roll-Call Data Analysis », *American Journal of Political Science* 59(2), 511–528.
- **Rosenthal, H. & Voeten, E.** (2004), « Analyzing Roll Calls with Perfect Spatial Voting: France 1946–1958 », *American Journal of Political Science* 48(3), 620–632.
- **Spirling, A. & McLean, I.** (2007), « UK OC OK? Interpreting Optimal Classification Scores for the U.K. House of Commons », *Political Analysis*, DOI 10.1093/pan/mpl009.
