# Test de lecture — Contrepoint

Rapport d'un lecteur qui arrive sans rien savoir. Partie 1 écrite avant toute lecture
de brief, de doc ou de code : seules les quatre captures ont été regardées.

---

## Partie 1 — la lecture à froid

### 1. De quoi parle ce site, en une phrase

C'est un tableau de positions politiques des partis français : il place les mêmes
partis sur plusieurs échelles différentes (les votes à l'Assemblée, une enquête
d'experts, un code administratif) et refuse explicitement de les fusionner en un
chiffre unique.

### 2. Trois choses que je tiens pour vraies après l'avoir parcouru

- « Il y a trois façons de mesurer, et le site les montre séparément — il ne fait
  jamais la moyenne. Il le répète partout. »
- « Les deux mesures qui donnent un chiffre classent les partis dans le même ordre :
  LFI le plus à gauche, le RN le plus à droite, sur les deux. Donc les deux méthodes
  se confirment l'une l'autre. »
- « Chaque chiffre est traçable : il y a des liens vers les fichiers sources, des
  empreintes à copier, une date d'arrêt des données, et un bloc "refaire le calcul"
  qui donne la recette. C'est fait par quelqu'un qui veut qu'on vérifie. »

### 3. Ce que je crois pouvoir faire avec ce site

Regarder où se situe un parti donné et selon quelle source, et surtout vérifier d'où
sort le chiffre. Ça ressemble à un instrument pour journaliste, chercheur ou
militant méfiant — quelqu'un qui veut citer une position en pouvant la sourcer. Ce
n'est pas un site pour se faire une opinion : il n'y a aucun commentaire, aucune
interprétation, aucun classement final. C'est une pièce à conviction, pas un article.

### 4. Les trois premières questions, dans l'ordre où elles viennent

1. Qu'est-ce que mesure exactement l'axe « votes nominatifs » ? Il va de −1,00 à
   +1,00 et LFI est à −1,00, le RN à +1,00 — donc c'est un axe gauche/droite ? Mais
   le texte dit précisément le contraire (« elles ne définissent ni la gauche ni la
   droite »). Alors c'est quoi ?
2. Pourquoi les deux listes n'ont-elles pas les mêmes partis ? Le premier bloc en a
   dix, le second huit, et ils ne se recouvrent pas (Renaissance et le PCF
   apparaissent dans l'enquête d'experts mais pas dans les votes ; l'inverse pour
   Ensemble pour la République, les Démocrates, UDDPLR).
3. Qui fait ce site, et pour dire quoi ? Aucun nom, aucun « à propos » visible, rien
   qui dise pourquoi ce travail existe.

### 5. Ce que je ne comprends pas

- **« UNITÉS MÉDIANES ANCRÉES »** et **« axe issu des votes, unités ancrées »**
  (bandeau « Trois familles, trois échelles », puis répété sous le graphique des
  votes nominatifs, puis en Ligne de preuve). Je lis les trois mots, je ne sais pas
  ce qu'est une unité ancrée ni sur quoi elle est ancrée. La phrase « Deux médianes
  de groupe fixent l'unité de l'axe » n'aide pas : je ne vois pas comment deux
  médianes fabriquent une unité.
- **Le bandeau du haut** : `DONNÉES ARRÊTÉES LE 2026-08-27   AN · XVIIᵉ LÉGISLATURE ·
  AU 2026-07-21`. Deux dates différentes collées, plus un sigle « AN » que je devine
  seulement au bout d'un moment. Je ne sais pas laquelle des deux dates compte.
- **La ligne « ligne de preuve »** dans le bloc du bas : deux longues empreintes
  hexadécimales, sans un mot pour dire ce qu'on est censé en faire.
- **`iterations_als 300`**, **`filtre_scrutins minorite_non_vide`**,
  **`scrutins_retenus 7979 / scrutins_ecartes 455`** : je vois les valeurs, je ne
  peux rien en faire. Notamment : pourquoi 455 scrutins sont-ils écartés ?
- **L'écart-type de rééchantillonnage à 0,00** pour LFI. Zéro exactement, c'est
  suspect, et rien ne me dit si c'est normal.

### 6. Ce que je crois avoir compris et dont je ne suis pas sûr

C'est la partie la plus longue, et c'est mauvais signe.

- **Que −1,00 / +1,00, c'est gauche/droite.** La page dit noir sur blanc que non. Mais
  LFI est à −1,00, le RN à +1,00, les partis sont rangés exactement dans l'ordre que
  j'attends, et l'axe est horizontal avec la gauche à gauche. Je *sais* que la page
  le nie et je continue à le lire comme ça. La dénégation ne bat pas l'image.
- **Que −1,00 et +1,00 sont des bornes atteintes, donc des maximums.** LFI et le RN
  sont pile aux extrémités. Je *crois* que c'est parce que ce sont les deux ancres
  qui définissent l'échelle (donc par construction, pas par mesure) — mais je le
  déduis du fait qu'ils tombent pile rond, pas parce que la page me l'a dit.
- **Que « position non publiée » veut dire « on a le chiffre mais on le cache ».**
  Le bloc LIOT affiche IQR 0,64 face à un maximum publiable de 0,25, et le bloc « Non
  inscrit » un effectif de 9 pour un minimum de 10. Donc la mesure existe et elle est
  jugée trop imprécise pour être montrée. C'est une lecture que je fais seul :
  personne ne me dit que c'est une règle décidée d'avance et non un cas par cas.
- **Que la « nuance administrative » n'est pas une mesure du tout**, juste une
  étiquette officielle (FI, RN, LR…). La page la présente comme la « troisième
  famille », à égalité avec les deux autres, alors qu'elle n'a aucun chiffre. Je ne
  sais pas si elle est là comme mesure ou comme contre-exemple.
- **Que les cinq entités du premier tableau sont l'intersection des deux familles.**
  Le titre dit « Deux familles, un même ordre » et « cinq rangs sur cinq » : je
  suppose que ce sont les seuls partis présents des deux côtés. Ce n'est écrit nulle
  part.
- **Que « Place publique » n'est mesurée par rien parce qu'elle est trop petite.** La
  page dit « absente des quatre sources d'identifiants ». Je traduis en « trop
  petite pour figurer », ce qui est peut-être faux.

### 7. Le premier chiffre que je retiens

**−1,00**, en face de La France insoumise. Il m'est resté parce qu'il est rond, qu'il
est premier de la première colonne, et qu'il est répété en gros dans le bloc de
preuve du bas. Ce que je crois qu'il signifie : « LFI est à l'extrême gauche, au
maximum de l'échelle ». C'est-à-dire, très probablement, à peu près le contraire de
ce que le site veut me faire lire — un rond de −1,00 pile est plus vraisemblablement
un point de référence choisi qu'une mesure obtenue.

### 8. Le téléphone (380 px)

- **Les deux graphiques à points se cassent.** L'axe se réduit à une bande étroite et
  la valeur (−1,00, +0,25…) passe *au-dessus* du point, sur la ligne du nom. Résultat :
  le point et son chiffre ne sont plus alignés sur la même ligne, et deux entités
  voisines se lisent en escalier. Sur « Ensemble pour la République », le point est à
  droite et le chiffre à droite aussi, une ligne plus haut — j'ai dû compter les
  lignes pour savoir à qui appartenait quoi.
- **Le tableau du haut tient**, mais les noms passent sur deux lignes (« La France
  insoumise », « Rassemblement national ») et l'en-tête aussi, ce qui gonfle chaque
  ligne au double.
- **Le bandeau de dates se casse en trois lignes** et devient un bloc gris illisible
  au-dessus du contenu — c'est la première chose qu'on voit sur un téléphone, et
  c'est la moins utile.
- **Les trois colonnes « trois familles » deviennent trois blocs empilés** : ça marche,
  mais on perd complètement l'effet de comparaison côte à côte, qui est justement
  l'argument du site. Sur mobile, « trois familles jamais moyennées » redevient une
  simple liste de trois paragraphes.
- **La note « les deux axes ne partagent ni origine ni largeur »** arrive en bas, en
  petit, après les deux graphiques — donc après que je me sois déjà fait mon idée en
  les comparant visuellement. Sur mobile c'est pire : les deux graphiques sont si
  loin l'un de l'autre qu'on ne les compare plus du tout.
- **Le scroll est très long.** Il faut passer beaucoup pour arriver au premier bloc
  qui explique quelque chose.

---

## Partie 2 — la confrontation

Briefs lus après coup : `ux-donnees.md`, `typo-wording.md`, `garde-fous-visuels.md`.

### Ce qui est passé

- **« Trois familles, jamais moyennées. »** C'est l'intention la mieux transmise. Ma
  partie 1, sans avoir rien lu : « il montre trois façons de mesurer séparément — il
  ne fait jamais la moyenne. Il le répète partout. » Le sous-titre est repris mot pour
  mot du brief typo (§5) et il fonctionne.
- **La nuance administrative sortie de la logique d'axe** (cas dur 3). Le bandeau de
  pastilles fait exactement l'effet recherché : j'ai écrit spontanément « elle n'a
  aucun chiffre », et je n'ai à aucun moment envisagé de la moyenner avec le reste.
  L'impossibilité de forme tient.
- **Place publique en note de pied** (cas dur 1). Aucune ligne vide, aucun mensonge de
  forme. Je l'ai lue comme une note, pas comme une mesure manquante — c'est réussi,
  même si j'en ai tiré une conclusion fausse (voir plus bas).
- **Aucun canal de fiabilité.** Rien dans ma partie 1 ne parle de sûr / douteux /
  fiable / bien mesuré. GV-05, GV-06 et GV-07 tiennent.
  Aucune gradation de crédibilité ne s'est formée dans ma lecture, pas même par
  accident. C'est le résultat le plus solide du test.
- **Les échelles disjointes** (GV-03). Elles le sont, et la page le dit. Ma partie 1 a
  bien enregistré que les deux listes n'ont ni les mêmes partis ni la même
  graduation — j'en ai fait une question, pas une confusion.
- **L'ordre identique des cinq entités** (« le fait le plus fort du jeu de données »).
  Le tableau de tête me l'a fait dire spontanément : « les deux méthodes se confirment
  l'une l'autre ». Passé — mais ce n'est pas la phrase des trois secondes.

### Ce qui n'est pas passé

- **La moustache d'IQR.** Le brief UX la classe catégorie 1, longueur à l'échelle,
  rapport de 7 entre LFI et EPR, « immédiatement visible ». **Elle n'existe pas dans
  la page.** À la place : `71 · 0,03`, deux nombres en rang 3 sous le nom, que j'ai
  lus comme du bruit — je n'ai pas su, en partie 1, que `0,03` était une dispersion.
  À noter : ce n'est pas un oubli, c'est un arbitrage. **GV-10 interdit explicitement
  ce que le brief UX prescrit** (« aucune longueur géométrique du SVG ne dérive de
  `dispersion` — l'IQR ne s'affiche qu'en `<text>` »). Les deux briefs se
  contredisent, la page a suivi le garde-fou, et personne ne l'a écrit. Le lecteur, lui,
  perd le seul canal qui rendait la dispersion perceptible.
- **L'intervalle ouvert de « Mesuré, non publié »** (cas dur 2). Le brief veut un
  dessin dont la longueur seule explique le retrait : « la raison du retrait est le
  dessin lui-même », LIOT à 199 px contre 65 px pour la moustache la plus large. **La
  page n'en dessine rien** : deux lignes de texte, `effectif 20 · IQR 0,64 · maximum
  publiable 0,25`. Résultat mesuré sur moi : j'ai compris qu'il y avait un seuil, mais
  je l'ai rangé dans « ce dont je ne suis pas sûr » — je ne savais pas si c'était une
  règle décidée d'avance ou un cas par cas. Le dessin, lui, aurait répondu sans mot.
- **Les 26 empreintes hors de l'écran principal** (catégorie 3 du brief UX : « 1 664
  caractères hexadécimaux sur l'écran principal ne servent personne »). **Elles sont
  revenues**, en bas de la page, en section « Ligne de preuve » dépliée. C'est le seul
  point où la page fait le contraire de ce qui est écrit. Ma partie 1 les a classées
  dans « ce que je ne comprends pas ».
- **`Voir la preuve`.** Le brief typo la désigne comme « la chaîne qui manque : le seul
  actif du projet et il est invisible ». Elle est toujours absente de la page. Ce qui
  la remplace — un mur de preuve déroulé en bas — est plus visible mais pas plus
  atteignable : à aucun moment je n'ai compris que la preuve se rattachait à *une*
  valeur que je pouvais désigner.
- **Les fonds alternés** (à retirer, brief UX §« Ce qu'on retire » n°3). Toujours là,
  une bande sur deux. Voir défaut 2.
- **La date une seule fois** (n°8, « la date répétée 26 fois »). Elle est encore dans
  les deux pôles, dans les cinq notes, dans les motifs. Le bandeau en porte deux
  différentes. Ma partie 1 : « je ne sais pas laquelle des deux dates compte. »
- **Incohérence de chiffres entre briefs et page.** `typo-wording.md` §5 écrit LIOT
  `IQR 0,687 pour un maximum de 0,25` et NI `IQR 0,623` ; `ux-donnees.md` écrit NI
  `IQR 1,24` ; la page affiche LIOT `0,64` / `0,6417` et NI `1,24`. Trois documents,
  trois valeurs pour LIOT. Un seul de ces nombres peut être juste, et c'est le genre de
  détail sur lequel un site de preuve se fait prendre.

### Ce que j'ai compris que personne n'avait voulu dire

Trois idées fausses, dont une grave.

1. **« −1,00 = extrême gauche, +1,00 = extrême droite. »** C'est l'assertion que le
   projet refuse le plus fermement — le brief typo lui consacre son plus long
   paragraphe (« Le piège traité ») et conclut que l'écrire serait « l'assertion la
   plus attaquable que le site pourrait produire ». **Le piège n'est pas traité : il
   est seulement non écrit.** La page ne met aucun mot de direction sous les pôles,
   et le lecteur les fournit. Un axe horizontal, LFI à gauche à −1,00 pile, le RN à
   droite à +1,00 pile, l'ordre attendu : l'image dit « gauche-droite » plus fort que
   la phrase « elles ne définissent ni la gauche ni la droite » ne dit le contraire.
   J'ai noté en partie 1 que je *savais* que la page le niait et que je continuais à
   le lire ainsi. Une dénégation textuelle ne bat pas une géométrie.
2. **« Place publique est trop petite pour figurer. »** La page dit « absente des
   quatre sources d'identifiants ». J'en ai fait un jugement de taille. Personne n'a
   voulu dire ça, et c'est presque une qualification d'organisation.
3. **« Écart-type de rééchantillonnage 0,00 pour LFI, c'est suspect. »** J'ai lu un
   zéro exact comme une anomalie du calcul, faute d'une seule ligne disant ce que
   mesure ce champ.

### La phrase des trois secondes

> « Ils sont massés aux deux bords : la moitié de l'axe des votes est vide au milieu,
> et l'enquête d'experts montre le même trou, au même endroit. »

**Non. Ma partie 1 ne la contient nulle part.** Pas une allusion : ni « massés », ni
« vide », ni « trou », ni « deux bords ». Je n'ai vu aucun regroupement, aucune
polarisation, aucun milieu désert. J'ai vu une liste ordonnée du plus négatif au plus
positif — un continuum, pas deux paquets.

Combien de temps il m'a fallu, et par quel chemin : **je ne l'ai pas trouvée du tout
en regardant.** Je ne l'ai lue qu'en ouvrant `ux-donnees.md`, et il m'a alors fallu
revenir aux chiffres du tableau (−0,84 puis +0,17) pour la vérifier arithmétiquement.
Autrement dit, le fait principal du jeu de données passe aujourd'hui par le calcul
mental, jamais par l'œil — exactement le reproche que le brief adressait à l'écran
précédent, déplacé d'un cran.

Pourquoi je ne l'ai pas vue, alors que le vide est réellement dessiné : parce que rien
ne m'a fait lire la **colonne des points**. Chaque ligne est un objet complet — nom à
gauche, point au milieu, valeur et dispersion à droite, fond alterné — et l'œil la
parcourt horizontalement, une ligne après l'autre. Le vide entre le PS et Les
Démocrates n'existe que si on lit verticalement. Le graphique est juste, et il se lit
comme un tableau.

---

## Partie 3 — le verdict

### Défaut 1 — l'axe se lit gauche-droite, et rien ne l'en empêche

**Ce que le lecteur perd :** il repart avec la seule affirmation que le site refuse de
produire, en croyant l'avoir lue sur le site. C'est le pire échec possible ici — pas
une information manquante, une information inventée avec l'autorité de la page.

**Où :** panneau « Votes nominatifs », les deux pôles `−1,00 / médiane LFI-NFP` et
`+1,00 / médiane RN`, plus l'ordre des dix lignes.

**Ce qui le corrigerait :** les deux pôles sont les seules valeurs rondes de la page,
et rien ne montre qu'elles le sont *par construction*. Marquer les deux lignes LFI-NFP
et RN comme **ancres** sur l'axe lui-même (le mot « ancre » est déjà dans le
vocabulaire du projet : `votes_an17_ancre_v1`), pour que l'œil voie que ces deux points
définissent l'échelle au lieu d'être mesurés dessus. Un lecteur qui voit deux ancres ne
lit plus deux extrêmes. Aucun mot de direction n'est ajouté, donc aucun garde-fou n'est
touché.

### Défaut 2 — le fait principal est invisible : la page se lit en lignes, pas en colonne

**Ce que le lecteur perd :** la phrase des trois secondes, entièrement. Il repart avec
« il y a un classement » au lieu de « il y a deux paquets et un trou ».

**Où :** panneau « Votes nominatifs ». Trois choses tirent l'œil à l'horizontale : le
fond alterné une ligne sur deux (que le brief UX demandait de retirer, n°3), le
sigle + effectif + IQR sous chaque nom, et la valeur répétée à droite de chaque ligne.

**Ce qui le corrigerait :** retirer les fonds alternés, comme prévu — c'est une
suppression, pas un ajout. Et ne pas espacer les dix lignes régulièrement quand les
données ne le sont pas : quatre lignes, un blanc, six lignes. Le trou dans les données
mérite un trou dans la page.

### Défaut 3 — la moitié basse est un mur de preuve que personne ne peut lire

**Ce que le lecteur perd :** rien d'utile, et beaucoup de temps. Mais il perd aussi la
preuve elle-même : noyée dans 1 664 caractères hexadécimaux, elle cesse d'être un
argument et devient une texture. Le seul actif du projet est présent et inexploitable.

**Où :** section « Ligne de preuve », toute la partie basse de la page, ainsi que les
blocs « Refaire le calcul » dépliés par défaut.

**Ce qui le corrigerait :** ce que le brief UX prescrivait déjà — replier. Le bloc
d'en-tête d'une preuve (valeur, échelle, observation, dispersion, date) est lisible et
suffit ; les empreintes et le bloc méthode se replient, et une seule chaîne visible par
bande — `Voir la preuve` — rattache une preuve à une valeur. C'est la chaîne manquante
identifiée par le brief typo, et elle vaut mieux que le mur.

### Ce site est-il exploitable ?

**Oui.** Pas brillamment, mais oui, et je m'appuie sur quatre constats de ma partie 1,
écrite avant d'avoir rien lu :

- j'ai pu énoncer **trois affirmations vraies** sur le jeu de données, sans aide ;
- je n'ai **retenu aucun chiffre faux**, et le chiffre que j'ai retenu (−1,00) est
  correct — c'est son *interprétation* qui dérape, pas sa lecture ;
- je suis arrivé **jusqu'à la source** : liens, dates, empreintes, méthode. Je pouvais
  vérifier ;
- **aucune gradation de fiabilité ne s'est formée dans ma tête.** Pas de « ce parti est
  mieux mesuré ». C'est l'écueil sur lequel ce genre de projet meurt, et il est évité.

Ce qui empêche de dire mieux que « oui » : le site **échoue à son propre test des trois
secondes**, et il laisse s'installer l'idée gauche-droite qu'il consacre trois phrases
à refuser. Un site de preuve qui produit involontairement l'assertion la plus attaquable
de son domaine reste exploitable, mais il est vulnérable exactement là où il se croit
protégé. Les défauts 1 et 2 se corrigent par des retraits et un marquage, pas par une
refonte : c'est la différence avec « inexploitable ».
