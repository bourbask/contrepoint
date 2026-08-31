# Garde-fous de conception

Contrepoint mesure des positions politiques. Le calcul est reproductible : quiconque
récupère les sources et relance le pipeline obtient les mêmes nombres, à l'octet.

**Ce n'est pas le calcul qui peut être biaisé. Ce sont les décisions qui l'entourent.**
Quelles sources on retient, quelles entités entrent dans le corpus, ce qu'on publie et
ce qu'on retient, quels mots on emploie, ce que le dessin met en avant. Aucune de ces
décisions n'est vérifiable en relançant un programme.

Ce document liste ces endroits et, pour chacun, la règle qui **retire la discrétion**.
Une règle qui laisse un choix au moment de l'appliquer n'en est pas une.

## La règle qui gouverne les autres

> **Une règle qui n'a de sens que comme correction du biais de son auteur est une
> mauvaise règle.**

Chaque règle ci-dessous doit tenir sans qu'on sache qui tient le projet. Si l'une
d'elles ne se justifie qu'en invoquant les opinions de quelqu'un, elle est à réécrire.

C'est aussi pourquoi ce document ne contient **aucune déclaration d'orientation
politique**. En recherche, la déclaration d'intérêts remplace une reproductibilité
impossible ; ici la reproductibilité existe, donc la déclaration n'ajouterait rien —
et elle inviterait à relire chaque choix à travers un prisme au lieu de le vérifier.

## 1. Le corpus

> **Le corpus est celui de la source, jamais celui qu'on choisit.**

Si une source couvre dix entités, on en publie dix. On n'en retire pas une qu'on
juge marginale, on n'en ajoute pas une qu'on estime manquante. Écarter une entité
exige un motif **de la source** — absente, non appariable, hors périmètre déclaré —
consigné dans le registre, jamais un jugement sur son importance.

*Défaut constaté le 2026-08-31 :* la vague CHES 2024 porte dix partis français, les
artefacts n'en publiaient que huit. MoDem et Reconquête étaient mesurés et non
publiés. La couverture s'arrêtait aux entités à présence parlementaire, ce qu'aucun
document n'annonçait.

**Conséquence de ce défaut, et raison d'être de cette règle :** l'axe se trouvait
tronqué **à gauche mais pas à droite**. Les codes de nuance `EXD` et `UXD` sont
présents au 2nd tour ; aucun code d'extrême gauche ne l'est, ces candidatures ayant
été éliminées au 1er tour. Chaque choix était défendable isolément ; leur somme
produisait une asymétrie que personne n'avait voulue.

## 2. Les ancres de l'axe

> **Les pôles sont de la donnée, déclarée et datée, jamais un choix de dessin.**

`ancre_axe` vit dans le registre d'entités, avec son pôle et sa période de validité.
Le pipeline s'arrête si un pôle manque à la date d'agrégation ; il ne choisit jamais
un remplaçant.

**Et un pôle n'est pas un extrême.** `−1,00` est la médiane d'un groupe à une date,
pas la gauche. Tout écran qui laisse lire un axe absolu ment, quoi qu'en dise le
texte à côté.

## 3. Ce qui est publié, et ce qui est retenu

> **Les seuils sont des nombres écrits d'avance, jamais une appréciation au cas
> par cas.**

Effectif minimal, écart interquartile maximal, écart-type de rééchantillonnage
maximal. Une mesure retenue est retenue **par la règle**, et les chiffres qui la
déclenchent sont publiés avec le motif — c'est ce qui permet de contester la règle
plutôt que d'avoir à croire l'auteur.

## 4. Les mots

> **La liste des termes interdits est unique, et un contrôle la fait respecter.**

`scripts/lexique.sh` porte la liste canonique et s'exécute à chaque commit et en
intégration continue. Aucun axe de fiabilité, de crédibilité ou de véracité : le
projet dit **où**, jamais **si c'est bien**.

Un motif du lexique qui attrape la langue ordinaire est un défaut du motif, pas du
texte — il se resserre après avoir été passé sur l'arbre entier.

## 5. Le dessin

> **Une forme qui laisse lire ce que la mesure ne dit pas est fausse, même si tous
> les nombres sont exacts.**

`docs/design/garde-fous-visuels.md` en tient la liste, avec le contrôle de chacun.
Les trois qui tombent le plus souvent : aligner deux échelles disjointes, indexer une
teinte ou une taille sur une mesure de dispersion, et remettre sur l'axe une position
que le projet a retenue.

Une lecture **à froid** — quelqu'un qui n'a lu aucun document — vaut mieux que dix
relectures informées. Celle du 2026-08-28 a établi que le fait principal du site ne
passait pas, et pourquoi.

## 6. Ce qu'on ne mesure pas

> **Une entité qu'aucune source ne mesure est nommée, jamais placée.**

Des candidatures existent sans qu'aucune famille ne les couvre. Les inscrire sur un
axe demanderait de leur inventer une coordonnée. Elles sont dites absentes, avec ce
qui a été cherché — l'absence n'est lisible que si la recherche l'est.

## 7. Ce que ce document ne protège pas

Il ne protège pas contre une source biaisée. Une enquête d'experts porte les
conventions de ses répondants ; un nuancier administratif porte celles de son
administration. **La parade n'est pas de corriger la source — ce serait y substituer
notre propre jugement — mais d'en publier plusieurs, indépendantes, sans jamais les
fondre.** Là où elles s'accordent, l'accord vaut plus que chacune ; là où elles
divergent, la divergence est l'information.

Il ne protège pas non plus contre ce qu'on n'a pas pensé à mesurer. C'est la limite
la plus sérieuse, et elle n'a pas de règle : seulement l'obligation d'écrire ce que
le corpus couvre, et ce qu'il laisse dehors.
