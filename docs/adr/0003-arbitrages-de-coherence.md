# ADR 0003 — Arbitrages de cohérence entre documents de conception

Statut : **acté**. Date : 2026-08-27.

Décide : la source du rattachement d'un député à son groupe, et l'absence de
seuil de participation. Acte formellement les cinq autres points que
[../brique0/plan-de-tests.md](../brique0/plan-de-tests.md) §16 relevait comme
contradictions de spécification.

Ne décide pas : l'estimateur (voir
[../brique0/positionnement.md](../brique0/positionnement.md) §4), le périmètre de
la v0 (voir [0000-perimetre-brique0.md](0000-perimetre-brique0.md)).

---

## Contexte

La conception de la brique 0 a été écrite en parallèle dans plusieurs documents.
Sept points s'y contredisaient d'un document à l'autre. Le plan de tests les a
relevés à son §16 parce qu'un test écrit avant l'arbitrage épingle la mauvaise
version — mais **un plan de tests ne tranche pas une spécification** : il en
constate le conflit et refuse d'avancer. C'est le rôle de cet ADR.

Cinq des sept points ont été tranchés par une mesure ou une relecture, et la
correction est déjà appliquée dans les documents concernés ; ils sont récapitulés
au §3 pour que ce document se lise seul. Les deux autres — la source du
rattachement au groupe et le filtre de participation — n'avaient pas d'acte : ce
sont les §1 et §2.

Aucun des deux n'est un arbitrage de goût. Les deux actent une mesure déjà faite,
consignée dans
[../brique0/ingestion-votes.md](../brique0/ingestion-votes.md) et
[verification-2026-08-27.md](../brique0/verification-2026-08-27.md).

---

## 1. Le rattachement d'un député à son groupe vient du bloc de ventilation du scrutin

### Contexte

`ingestion-votes.md` §8 retient le groupe porté par le bloc `ventilationVotes`
du fichier de scrutin, AMO30 ne servant que de recours. `positionnement.md` §1
**écrivait** l'inverse : « le rattachement d'un député à son groupe ne peut donc
**pas** être lu dans le fichier de scrutin ; il doit venir des mandats, avec
période de validité » — phrase corrigée depuis, elle n'est plus dans le
document. Les deux ne pouvaient pas être vrais, et le cycle 4 des tests
(`ING-16` à `ING-19`) était bloqué tant que le point n'était pas acté.

`positionnement.md` §1 tire sa conclusion d'un seul cas — les blocs
`organeRef: "PO0"`, référence pendante qui ne se résout pas dans AMO30 : 146
blocs, sur 14 scrutins, 1 895 cellules et 335 acteurs. Le cas est réel ; il ne
suffit pas à disqualifier la source. `ingestion-votes.md` §8 a mesuré le
remplacement complet.

### Décision

**Le groupe retenu est celui du bloc `ventilationVotes` du scrutin.** Il est daté
par construction — le bloc vit dans le fichier dont `dateScrutin` est la date —
il ne demande aucune jointure, donc aucune fenêtre de validité à interpréter, et
il est ce que l'Assemblée a publié ce jour-là.

Ce que la jointure par mandat aurait coûté, mesuré sur les **1 270 476 cellules**
du corpus complet, contre un index des mandats AMO30 `typeOrgane = "GP"`,
`legislature = "17"` :

| Comparaison | Cellules |
|---|---|
| Groupe identique | 1 250 505 (98,4 %) |
| Plusieurs mandats GP valides à la date du scrutin | 17 716 (1,4 %) |
| Groupe différent | 2 255 (0,2 %) |

Les **2 255 désaccords vont tous dans le même sens** : AMO30 dit encore `NI`
(`PO840056`) là où la ventilation dit un groupe constitué, parce que le mandat de
non-inscrit porte une `dateFin` en retard sur la constitution du groupe. Une
jointure par mandat classerait ces votes chez les non-inscrits — dont la
dispersion interne est la plus grande du jeu, et qui ne sont pas un parti. Le cas
réel est documenté dans `ingestion-votes.md` §8.

Les **17 716 cellules ambiguës** proviennent de **18 chevauchements de périodes**
sur 648 députés : mandats dupliqués à périodes identiques, ou `dateFin`
concurrentes. La jointure devrait donc trancher, sans que la source dise comment.

**Rôle d'AMO30**, non supprimé mais borné à trois emplois :

1. **recours pour les blocs `organeRef: "PO0"`** — le groupe est résolu par le
   mandat GP des votants du bloc à la date du scrutin. Vérifié sur les 146
   blocs : la résolution est unanime dans chaque bloc, 10 blocs sont vides donc
   sans objet, aucun bloc n'est ambigu. Un bloc `PO0` non résolu est une erreur
   bloquante du pipeline, jamais un groupe « inconnu » qui remonterait dans une
   agrégation ;
2. **source du libellé et de la période de validité des groupes** — l'agrégation
   se fait à une date de référence explicite portée par la ligne de preuve, et
   n'inclut que les groupes dont la validité couvre cette date ;
3. **contrôle croisé** — le désaccord est compté et exposé, jamais tu.

Quand AMO30 est consulté, la règle de dédoublonnage s'applique : regrouper les
mandats par `(organeRef, dateDebut)`, retenir la `dateFin` maximale, `null`
valant « en cours ». Après cela, aucun chevauchement ne subsiste entre deux
`organeRef` différents.

`votant.mandatRef` **n'est pas** une porte d'entrée vers le groupe : vérifié sur
les 1 270 476 cellules, il pointe toujours un mandat `typeOrgane = "ASSEMBLEE"`,
sans une exception. Il identifie le siège, pas l'appartenance.

### Conséquences

- ~~`positionnement.md` §1 énonce encore la version écartée~~ **Corrigé.** Le §1
  acte le bloc de ventilation, daté par construction, AMO30 en recours.
- Les périodes de validité **restent dans le modèle**. Elles sont sans effet sur
  la v0 — le premier scrutin de la XVIIe législature est daté du 2024-10-08 et
  aucun scrutin n'a eu lieu pendant la fenêtre de non-inscription de juillet
  2024 — et redeviennent nécessaires dès la XVIe, premier lot d'extension.
- Tests épinglés : `ING-16` (résolution `PO0`), `ING-17` (bloc `PO0` non résolu
  bloquant), `ING-18` (le désaccord tranche pour la ventilation), `ING-19`
  (période de validité respectée).

---

## 2. Aucun seuil de participation

### Contexte

`ROADMAP.md` v0.1 et `docs/methode.md` §1 exigeaient « un seuil de participation
documenté », au titre du risque nommé dans la roadmap : « sans seuil, l'axe
mesure surtout qui était présent ». L'objection est vérifiable, et
`ingestion-votes.md` §6 l'a vérifiée. Elle ne survit pas à la mesure.

La médiane de `syntheseVote.nombreVotants` est de **133 votants pour 577
sièges** — 23 %. Tout seuil coupe le corpus. **Sur les 8 434 scrutins bruts**
(ADR 0001 §1.2) : ≥ 100 en conserve 5 789, soit 68,6 % ; ≥ 150, 3 581, soit
42,5 % ; ≥ 200, 1 912, soit 22,7 % ; ≥ 300, 359, soit 4,3 %. Le tableau de
sensibilité ci-dessous compte **sur les 7 979 retenus** et donne donc d'autres
nombres pour les mêmes seuils — 5 535 et non 5 789 à ≥ 100, soit 69,4 % de sa
propre base. Une part ne se lit jamais sans sa base : les deux séries sont
justes, elles ne portent pas sur le même corpus.

**Aucune rupture ne désigne une valeur.** La distance de composition d'un scrutin
— la moitié de la somme des écarts absolus entre la part de chaque groupe parmi
les votants et sa part de l'effectif déclaré — décroît de 0,258 pour les scrutins
sous 50 votants à 0,091 au-delà de 300, **strictement, sans coude**, et ne
descend jamais sous 0,09 : même les scrutins les plus suivis ne sont pas des
échantillons proportionnels de l'Assemblée. Un seuil serait un arbitrage déguisé
en mesure.

**Le seuil n'améliore rien et, poussé, dégrade.** L'axe a été ajusté sur sept
corpus, corrélation de Pearson des coordonnées avec le corpus de référence
— minorité non vide, aucun seuil de participation :

Comptes **sur les 7 979 scrutins retenus**, filtres cumulés après la minorité
non vide.

| Corpus | Scrutins | Corrélation | Ordre des groupes |
|---|---|---|---|
| aucun filtre | 8 434 | 1,0000 | identique |
| minorité non vide | 7 979 | référence | référence |
| \+ `nombreVotants` ≥ 50 | 7 635 | 1,0000 | identique |
| \+ ≥ 100 | 5 535 | 0,9996 | DEM et NI permutent, à +0,10 tous les deux |
| \+ ≥ 150 | 3 451 | 0,9979 | idem |
| \+ ≥ 200 | 1 846 | 0,9923 | idem |
| \+ ≥ 300 | 339 | **0,9497** | **RN passe devant DR ; LIOT passe à droite du centre** |

À ≥ 300, DR (+0,82) et RN (+0,79) se confondent et LIOT traverse le zéro : sur
339 scrutins, l'axe cesse de séparer les blocs de droite. Le corpus complet
produit un ordre stable et un écart DR / RN de 0,41. C'est l'inverse de ce que le
risque annonçait.

### Décision

**Aucun seuil de participation.** Un seul filtre, qui n'est pas un seuil mais une
définition : **un scrutin sans minorité enregistrée n'entre pas dans la
matrice**, sa variance étant nulle et sa contribution à tout axe nulle.
Condition : `min(decompte.pour, decompte.contre) ≥ 1`.

| | Scrutins | Part |
|---|---|---|
| Total, XVIIe législature, du 2024-10-08 au 2026-07-21 | **8 434** | 100 % |
| Écartés — minorité vide, scrutin public | 432 | 5,1 % |
| Écartés — minorité vide, motion de censure | 23 | 0,3 % |
| **Retenus** | **7 979** | **94,6 %** |

Les **23 motions de censure sont écartées en totalité, et par construction** :
l'article 49 alinéa 2 de la Constitution ne fait voter que les députés favorables
à la censure, donc `contre = 0` dans les 23 fichiers. Les scrutins les plus
visibles de la législature ne portent aucune information de position
gauche-droite, parce que l'institution n'enregistre qu'un seul camp. C'est à
dire sur le site, pas à découvrir plus tard.

Matrice retenue : **7 979** scrutins, **641** députés ayant exprimé au moins une
position, **1 188 035** cellules observées, densité **23,2 %** — valeurs
recomptées sur l'archive complète le 2026-08-27.

**`nombreVotants` est publié par scrutin** dans le registre de preuves et affiché
avec le décompte. Il n'est **jamais** employé comme porte d'entrée dans la
matrice. La roadmap demandait un seuil documenté ; la mesure dit que le seuil
justifiable est l'absence de seuil, et c'est cela qui est documenté.

### Conséquences

- `ROADMAP.md` v0.1, son tableau des risques et `docs/methode.md` §1 sont mis à
  jour dans la même PR que cet ADR (`definition-of-done.md` §19).
- Test épinglé : `MAT-05`.
- Le seul seuil qui subsiste dans la chaîne porte ailleurs et n'est pas un filtre
  de scrutin : l'inclusion d'un député dans la médiane de son groupe exige 200
  votes exprimés (`positionnement.md` §6). Son effet est mesuré nul sur les
  groupes constitués.

---

## 3. Les cinq points déjà tranchés, et par quoi

Récapitulés ici pour que cet ADR se lise seul. La correction est appliquée dans
les documents cités.

| # | Point | Ce qui a tranché | Version actée |
|---|---|---|---|
| 2 | Nombre de scrutins à `organeRef: "PO0"` — 14 annoncés, 15 énumérés | Recomptage sur l'archive complète, [verification-2026-08-27.md](../brique0/verification-2026-08-27.md) §2 | **14 scrutins** : **12** le 2024-12-02, **1** le 2025-04-07, **1** le 2026-04-16. Les deux documents étaient justes sur le total, faux sur la ventilation |
| 3 | Gain du terme de rang 1 — 2,1 % contre 59,1 % | Recomptage, [verification-2026-08-27.md](../brique0/verification-2026-08-27.md) §3 | **60,8 % du résidu** après constante par scrutin (51,5 % de la variance totale), la constante seule en prenant 15,2 %. Corpus retenu, 7 979 scrutins. L'ADR 0001 se trompait d'un facteur trente, et une phrase destinée au site en avait été tirée. Les 59,1 % / 19,8 % de `positionnement.md` §1 sont une mesure antérieure sur une autre matrice d'entrée, non une convergence moindre du même ajustement : la part de la constante par scrutin est une forme close qu'aucune itération ne déplace (`positionnement.md` §11) |
| 4 | Dispersion publiée — « variance intra-groupe » contre « IQR et étendue » | Relecture juridique | **Écart interquartile et écart-type de rééchantillonnage. Jamais la variance** — illisible sur un axe sans unité — **et jamais l'étendue** : un minimum et un maximum **sont** les coordonnées de deux membres identifiables du groupe. Sur un groupe de neuf membres, avec un code et un appariement publics, elles sont réidentifiables en une exécution. L'IQR porte la même information sans exposer personne |
| 5 | Fixture du cas `votant` objet nu | Existence du fichier | **`VTANR5L17V5268`**, la fixture qui existe. `VTANR5L17V5646` n'a jamais été livrée |
| 7 | Version de la Licence Ouverte | Lecture du PDF de licence | **v1.0** pour l'Assemblée nationale, **`lov2`** pour le nuancier publié sur data.gouv.fr. Deux sources, deux versions : ce n'était pas une contradiction |

---

## Conséquences générales

- La propagation dans les documents amont est faite **dans la même PR** :
  `ROADMAP.md` v0.1 et v0.2, `docs/methode.md` §1 et §2,
  `plan-de-tests.md` §16, `docs/README.md`. C'est l'exigence n° 19 de
  `definition-of-done.md`, et la raison pour laquelle il n'y a pas de PR de
  rattrapage documentaire.
- Le §16 du plan de tests **n'est pas supprimé** : chaque ligne y renvoie
  désormais à cet ADR. Le tableau documente que les contradictions ont existé, ce
  qui est la seule trace de la manière dont elles ont failli être épinglées dans
  un test.
- ~~Deux occurrences résiduelles de versions écartées subsistent et sont à
  corriger dans le même lot~~ **Corrigées le 2026-08-28.** `positionnement.md`
  §1 acte le bloc de ventilation comme source du rattachement, AMO30 en recours ;
  `ingestion-votes.md` §8 porte la ventilation recomptée. Les deux passages que
  ce paragraphe désignait n'existent plus : ne pas les chercher.
- La propagation du point 3 du §3 — le gain du rang 1 — **n'était pas faite**
  dans `positionnement.md`, qui portait encore 59,1 % en six endroits, ni dans
  `plan-de-tests.md`, qui pinnait 0,591 comme valeur de référence de son niveau 2
  tout en actant 60,8 % à son §16. Corrigé le 2026-08-28. Un test écrit d'après
  le plan aurait épinglé la valeur écartée — exactement ce que le §16 dit vouloir
  empêcher.
