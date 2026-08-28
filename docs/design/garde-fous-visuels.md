# Garde-fous visuels

Ce que la refonte graphique n'a pas le droit de dessiner, et comment on le
vérifie sans relire.

**Portée.** Le document ne décrit aucune maquette et n'interdit aucun style. Il
énonce des **formes** interdites — une géométrie, une échelle, une teinte, un
glyphe — et, pour chacune, le contrôle qui la refuse dans un fichier. Un
interdit dont le contrôle est « relecture » se fera violer : la colonne
« mécanisable » dit honnêtement lesquels le sont.

**Ce qui existe déjà.** Cinq contrôles mordent aujourd'hui et couvrent une
partie de la liste : `scripts/lexique.sh` (termes), `scripts/securite.sh`
(motifs sur l'arbre entier), `scripts/portes-de-ci.sh invariants` (I12, I13 sur
les artefacts), `web/src/contrat.test.ts` (EXP-01 → EXP-08, sur le modèle de
dessin **et** sur le SVG rendu), `web/scripts/verifier-artefacts.mjs` (schémas).
Les contrôles proposés ci-dessous s'ajoutent à ces fichiers ; aucun n'exige un
nouveau script, sauf mention.

**Deux règles d'atelier, non négociables** (RG-108, RG-109) : une porte
déclarée sans travail en face dans `.github/workflows/ci.yml` est réputée
inexistante, et tout nouveau test `EXP-nn` doit être déclaré dans
`docs/brique0/plan-de-tests.md` §13 **et** écrit dans `web/src/contrat.test.ts`,
sinon `scripts/portes-de-ci.sh identifiants` rougit. Prochain identifiant
libre : **EXP-09**.

---

## 1. Le tableau

| # | Interdit | Règle qui le fonde | Forme graphique tentante | Contrôle | Mécanisable |
|---|---|---|---|---|---|
| **GV-01** | **Un nombre qui n'est pas une valeur publiée.** Aucune quantité affichée ne peut être calculée par le rendu. | RG-05 ; MET §3 (« elles ne sont jamais moyennées ») ; CON §6 I11 (« aucun nombre atteignable depuis `bandes[]` hors d'un `marqueurs[]` ») | Un « consensus », une moyenne des trois, un écart « votes − experts », un pourcentage d'accord, un total en pied de bande | **C1 — provenance des nombres.** Extraire tous les nombres des nœuds `<text>` du SVG rendu ; chacun doit être, caractère pour caractère, soit `formaterValeur(marqueur.valeur)`, soit un `valeur_code`, soit une borne déclarée du manifeste, soit un `effectif`/`iqr` de `dispersion`. Tout nombre orphelin = échec. Test `EXP-09` | **Oui** — le rendu est déjà produit hors ligne par `renderToStaticMarkup` dans la suite existante |
| **GV-02** | **Une barre, une jauge ou un segment qui combine deux familles.** | RG-05 ; RG-06 ; CON §6 I11, I12 | Une barre empilée, un « ruban » reliant les trois têtes d'une même entité, un dégradé allant du marqueur `votes` au marqueur `experts` | **C5 — inventaire d'encre.** Liste close des primitives admises dans un `<g class="systeme">` : `rect.fond`, `path.accolade`, `line.portee`, `rect.cible`, une tête par marqueur, les `<text>` prévus. Toute autre balise ou classe = échec. En prime, aucun élément dont `x1`/`x2` n'appartient pas aux bornes d'**une seule** échelle. Test `EXP-09` | **Oui** |
| **GV-03** | **Aligner sur les mêmes pixels deux échelles disjointes.** | RG-06 (« aucune valeur projetée, recalibrée ou convertie d'une échelle vers celle d'une autre ») ; POS §8 ; CON §4.1 (bornes **déclarées**, jamais dérivées) | Un axe unique pleine largeur, deux échelles normalisées en 0–100 %, deux graduations partageant origine et pôle haut, un axe « harmonisé » | **C2 — plages disjointes, étendues.** Le test actuel n'exige que `debut`, `fin` et le milieu distincts. Le durcir : pour chaque couple d'échelles, l'ensemble `{debut + t·(fin − debut), t ∈ 0, ¼, ⅓, ½, ⅔, ¾, 1}` doit être disjoint deux à deux, à 2 px près. Extension de `EXP-01` | **Oui** |
| **GV-04** | **Deux têtes d'une même entité au même pixel.** Défaut déjà survenu : `parti.lfi`, `votes` −1,00 et `experts` 0,82, tombaient à la même abscisse — lisible « les deux méthodes s'accordent exactement ». | RG-06 ; CON §2.3 ; défaut consigné dans `web/src/graphe.ts` et `EXP-01` | Élargir la mise en page, mettre `PAS` à 0, « aligner les axes proprement » | **C3 — écart minimal de tête.** `EXP-01` teste `new Set(x).size === x.length` : deux têtes à 0,01 px d'écart passent et sont visuellement confondues. Exiger `|xᵢ − xⱼ| ≥ 10` px (largeur de tête) pour deux voix d'un même système, à toute largeur testée : 320, 460, 640, 1024 | **Oui** |
| **GV-05** | **Suggérer une fiabilité par la couleur ou l'opacité.** | JUR règle 2 (« aucun axe à pôle dépréciatif… même en interne ») ; RG-90 ; RG-91 ; le Décodex est mort de cette forme, pas de l'idée | Une rampe chaud/froid, une opacité indexée sur l'IQR, un gris « peu sûr », un halo, une bordure rouge sur une bande dispersée | **C6 — teintes par rang.** Rastériser (`scripts/apercu.sh`) puis échantillonner le pixel central de chaque tête aux coordonnées que `disposer()` donne : le nombre de triplets RGB distincts doit être **exactement** le nombre de familles, et deux têtes de même famille doivent être bit pour bit de la même couleur, quelle que soit leur bande, leur valeur ou leur dispersion. Nouvelle porte, travail `garde-fous` de `ci.yml` | **Oui** — voir §2, le rendu figé actuel n'est pas rastérisable en l'état |
| **GV-06** | **Suggérer une fiabilité par la taille, l'épaisseur ou le glyphe.** | JUR règle 2 ; RG-90 ; RG-97 (« la couleur n'est jamais le seul porteur ») | Tête proportionnelle à l'effectif, trait épaissi quand l'IQR est faible, une étoile, un badge, un point d'exclamation, un cadenas, un « ✓ » | **C12 — gabarit constant** : toutes les têtes d'une même famille ont une boîte englobante identique dans le SVG, indépendante de la valeur, de l'effectif et de l'IQR. **C5bis — glyphes clos** : la liste des attributs `d` et des primitives de tête est close (`FORMES`, 5 entrées) ; tout `d` hors liste = échec | **Oui** |
| **GV-07** | **Suggérer une fiabilité par l'ordre.** Un tri est un classement, et « classement » est proscrit au profit de « positionnement daté ». | JUR règle 5 (lexique) ; RG-90 ; CON §4.3 (les bandes sont construites par le pipeline, **jamais par le front**) | Trier les bandes par valeur `votes`, par effectif, par « qualité de mesure » ; reléguer en bas les bandes non mesurées | **C7 — ordre du DOM.** La suite des `id` des `<g class="systeme">` dans le SVG est égale, dans l'ordre, à celle de `instantane.bandes[].id`. Idem pour l'ordre des voix, qui suit `manifeste.familles`. Test `EXP-10` | **Oui** |
| **GV-08** | **Faire d'une absence une donnée.** Une mesure non publiée pour dispersion excessive n'est ni un zéro, ni un centre, ni une case vide. | RG-48 ; RG-12 ; CON §2.4 ; ADR0 §5 (« non mesuré », jamais « neutre » ni « centre ») | Une tête creuse posée à 0, un pointillé vers le centre, un aplat gris sur toute la portée, une case vide, « n.d. », une interpolation entre les deux autres familles | **C4 — gouttière étanche.** Déjà tenu côté modèle par `EXP-01`. L'étendre au **SVG** : pour toute voix d'état ≠ `mesuree`, aucun élément du groupe `marqueur` n'a d'abscisse ≥ `min(echelle.debut) − 10`. Puis au **raster** : sur la ligne de pixels de cette voix, aucune encre non-fond entre `debut` et `fin` de chaque échelle. Extension `EXP-01` + porte raster | **Oui** |
| **GV-09** | **Dessiner une portée graduée sous une famille qui n'en a pas.** Le nuancier n'a ni `min`, ni `max`, ni `decimales`. | CON §2.3 (`nuance_leg2024` : trois `null`) ; RG-06 ; RG-113 (une classification de tiers porte le nom de son auteur en tête) | Placer les codes de nuance sur l'axe gauche-droite « pour comparer », leur inventer une graduation, afficher `FI` seul en pastille | **C10 — graduations déclarées.** Pour chaque échelle rendue : exactement deux barres de graduation, aux abscisses de ses bornes déclarées, et aucune portée tracée sous une voix `sans_graduation`. Aucune barre intermédiaire, **aucun zéro** — un tick « 0 » sur l'axe des votes fabrique le centre que RG-48 interdit. Test `EXP-11` | **Oui** |
| **GV-10** | **Publier une coordonnée individuelle, ou la rendre devinable.** | ADR0 §2 ; RG-41 ; RG-42 ; CON §6 I13, I19 ; ADR 0003 §3 (« un minimum et un maximum **sont** les coordonnées de deux membres identifiables ») | Une boîte à moustaches, un violon, un nuage de points des membres, un « rug », une silhouette de distribution, une barre d'étendue min–max autour de la médiane | **C5 — inventaire d'encre** : le nombre de têtes dessinées dans un système est **exactement** `bande.marqueurs.length` ; toute encre supplémentaire est un refus. Et : aucune longueur géométrique du SVG ne dérive de `dispersion` — l'IQR ne s'affiche qu'en `<text>`. Côté données, `portes-de-ci.sh invariants` (I13) et le motif `dispersion` de `securite.sh` tiennent déjà | **Oui** |
| **GV-11** | **Donner une fausse précision.** Deux décimales sont publiées, et l'arrondi est délibéré : au dix-millième, la médiane d'un groupe à effectif impair **est** la coordonnée d'un député nommable. | CON §2.3 (« deux décimales, et non quatre » ; GDR −0,9111, DEM 0,1729, UDR 0,9884) ; RG-33 ; CON §6 I7 | Écrire 4 décimales dans une infobulle, graduer l'axe au dixième pour « aider à lire », une tête d'un pixel de large, un dégradé continu qui laisse interpoler | **C1** (tout nombre affiché est un nombre publié) **+ C8 — quantum visible** : la largeur de tête doit rester **supérieure** à un quantum d'échelle, `(fin − debut) × 10^−decimales` — 2,6 px sur l'échelle des votes à 640 px de large, pour une tête de 8,4 px. Une tête plus fine que le quantum promet une résolution que la mesure ne porte pas. **Défaut latent à corriger** : `graphe.ts` formate l'IQR avec `grille?.decimales ?? 4` — 4 décimales dès qu'une famille sans graduation portera une dispersion | **Oui** |
| **GV-12** | **Comparer deux instantanés ou deux législatures.** L'ancrage est propre à un instantané : deux instantanés ne se superposent pas. | RG-04 ; ADR0 §3 (« interdiction dérivée ») ; POS §7 ; CON §4.1 (« aucun champ portant un écart entre deux instantanés ») ; CON §4.2 (`ancrage` est de la donnée « parce que c'est lui qui rend la superposition fausse ») ; CON §6 I18 | Une flèche « évolution », un marqueur fantôme de la date précédente, une sparkline, un curseur temporel, deux graphes côte à côte sur un axe commun, « depuis 2024 » | **C9 — une seule date.** L'ensemble des dates trouvées dans les nœuds `<text>` du rendu est un singleton, égal à `instantane.date`. Plus : aucun `<defs><marker>`, aucun `<polygon>`, aucun `marker-end` dans le SVG — une flèche a besoin d'une pointe. Extension `EXP-02` | **Oui** |
| **GV-13** | **Reprendre la couleur officielle d'un parti.** | RG-97 ; EXP-08 (« `couleurAssociee` de l'AN est disponible et tentant : c'est une convention éditoriale de l'Assemblée, pas une donnée du projet ») ; RG-55 | Colorer chaque bande à la couleur du parti, un fond bleu à droite et rouge à gauche, un dégradé politique derrière les portées | **C13 — sélecteurs par rang.** Dans `styles.css`, aucune règle portant une couleur ne peut être sélectionnée par une entité : refuser tout sélecteur contenant `parti`, `groupe`, `coalition`, `bande`, `[data-entite`, ou un attribut de valeur. Les seuls porteurs de teinte restent `.marqueur--N`. **C11 — fond uniforme** : sur le raster, une ligne de pixels traversant un système ne porte au plus que deux couleurs de fond (`--papier`, `--papier-systeme`) ; un dégradé horizontal est refusé. Motif `linearGradient|radialGradient` ajouté à `securite.sh` sur `web/` | **Oui** |
| **GV-14** | **Un axe, un champ ou un tri à pôle dépréciatif sous un nom anodin.** | JUR règle 2 ; RG-90 ; RG-91 ; RG-112 | Une variable CSS `--confiance`, une classe `.douteux`, un `data-qualite`, un tri « du plus net au plus flou », un libellé « robustesse » | **Extension de `scripts/lexique.sh`** : ajouter aux motifs du mode `code` les formes graphiques du même registre — `jauge|gauge|badge|etoile|star-|palmar|ranking|classement|robustesse|confiance|qualite[_-]?(mesure|donnee)|barre unique|score composite`. La liste canonique reste dans ce seul fichier (RG-91) | **Oui**, sur le mot ; **non** sur l'intention — une rampe de teintes sans mot n'a pas de motif : c'est **C6** qui l'attrape, pas le lexique |
| **GV-15** | **Ajouter du texte : un chapô, un didacticiel, une adresse au lecteur.** | RG-92 (aucune personne) ; RG-93 (étiquette ≤ 40, légende ≤ 140, une phrase) ; RG-94 (un bloc explicatif par écran, trois phrases au plus) ; ADR0 §5 | Une accroche de page d'accueil, une infobulle pédagogique, « découvrez », « vous pouvez comparer », un pictogramme d'aide bavard | **C14 — limites sur le DOM rendu.** `EXP-04` teste les artefacts ; l'étendre aux nœuds de texte **du rendu** : ≤ 140 caractères, une phrase, et un seul bloc explicatif de ≤ 3 phrases par écran. Plus un motif de personne : `\b(nous|vous|notre|votre|découvrez|comparez|cliquez)\b` dans les chaînes d'interface. Test `EXP-12` | **Oui**, avec faux positifs à arbitrer sur les impératifs |
| **GV-16** | **Afficher une classification de tiers comme si elle était de Contrepoint.** | RG-113 ; JUR règle 1 ; MET §3 (« le nuancier n'est pas une vérité de référence ») | Un badge `FI` nu sur la bande, une pastille de couleur sans auteur, une colonne « nuance » sans mention du ministère | Le libellé du marqueur (`Nuance 2024`) doit précéder le code dans l'ordre du DOM et rester présent au rendu ; sa `preuve` doit être atteignable au clavier. Contrôle d'ordre des nœuds, pas de sens | **Partiellement** — l'ordre se vérifie, la lisibilité de l'attribution non |

---

## 2. Notes de mise en œuvre

**Le raster n'est pas branchable en l'état.** `web/src/fixtures/rendu-fige.html`
est un **fragment SVG nu**, sans feuille de style : rastérisé par
`scripts/apercu.sh`, il rend tout en noir — mesuré, la partition sort en aplat
noir de 640 px. Les contrôles **C6** et **C11** exigent une page portant
`styles.css`. Le moins coûteux : rastériser la page construite plutôt que la
fixture, ou envelopper le fragment figé dans un gabarit minimal qui lie la
feuille. Sans cela, la porte raster affirmerait le vert sur une image qui ne
montre pas ce qu'on croit vérifier.

**Le rendu figé (`EXP-07`) n'est pas un garde-fou de méthode.** Il dit qu'une
sortie a changé, jamais qu'elle est fausse — le plan de tests l'écrit déjà — et
il se régénère par `FIGER=1`. Il sert de filet, il ne remplace aucun contrôle
nommé ci-dessus.

**Les contrôles de contenu tiennent déjà, ceux de forme non.** I12, I13, I18,
I19 portent sur les **artefacts JSON** : ils empêchent le pipeline de publier une
moyenne, une coordonnée ou un écart. Aucun ne regarde le SVG. Toute la colonne
« contrôle » ci-dessus existe parce qu'un graphiste ne touche pas au JSON : il
dessine à partir de valeurs licites, et c'est le dessin qui ment.

---

## 3. Ce qui n'est pas mécanisable, et qu'il faut relire

Dire qu'un contrôle existe là où il n'y en a pas est pire que l'absence.

- **La connotation d'une teinte.** C6 garantit qu'une famille a une couleur et
  une seule ; il ne dit rien du fait que cette couleur soit rouge, ni de ce que
  le rouge dit. Arbitrage humain, une fois, à l'adoption de la palette.
- **La hiérarchie visuelle.** Rien ne mesure qu'une famille domine par sa taille
  de titre, sa position en tête ou son contraste. Trois familles disjointes
  peuvent être rendues avec une manifestement principale.
- **Ce qu'un glyphe évoque.** C5bis vérifie une liste close de formes ; il ne
  sait pas qu'un triangle plein se lit comme un avertissement.
- **Le sens d'une phrase conforme.** C14 compte les caractères et les phrases.
  « Les valeurs divergent fortement » tient en 140 caractères, ne contient aucun
  terme du lexique, et reste une appréciation.
- **La lisibilité réelle.** Un axe juste et illisible reste un échec ; aucun test
  ne le voit. C'est la raison d'être de `scripts/apercu.sh`, qui donne à
  regarder — il ne conclut pas.

---

## 4. Les trois interdits les plus exposés

Classés par la probabilité qu'un graphiste **de bonne foi** les viole en
cherchant à bien faire.

### 1. GV-03 — aligner les deux échelles sur les mêmes pixels

C'est la violation qui a **déjà eu lieu** dans ce dépôt, et elle est tentante
pour la meilleure des raisons : deux graduations décalées de 28 px à chaque
bout, ce n'est pas beau. Un œil de designer voit un défaut d'alignement et le
corrige — le réflexe professionnel est exactement le geste interdit. Le résultat
est une concordance fabriquée entre `votes` (−1 à +1) et `experts` (0 à 10),
c'est-à-dire la conversion d'échelle que RG-06 refuse, obtenue sans écrire une
seule ligne de calcul. Le décalage n'est pas un compromis de mise en page : il
**est** le contrôle, et il doit être documenté comme tel dans la maquette, sinon
il sera nettoyé à la première passe de finition.

### 2. GV-08 — la pause posée sur l'axe

Une tête sans valeur reléguée à gauche, hors graduation, se lit comme un défaut
d'alignement de plus. La correction naturelle — « remets-la sur l'axe, en creux,
avec la mention en italique » — publie une position sur une entité nommée
précisément là où la donnée refuse de la publier. La mention ne rattrape rien :
l'œil lit la position avant la légende. Et le geste paraît généreux, puisqu'il
« montre l'information manquante » au lieu de la cacher. LIOT et NI sont les
deux cas réels du jeu ; l'un porte un IQR de 0,687 pour un maximum de 0,25.

### 3. GV-06 — la taille de tête indexée sur l'effectif

Encoder l'effectif dans le diamètre du marqueur est un classique de la
visualisation, enseigné comme une bonne pratique, et il utilise une donnée
**réellement publiée** : rien dans le contrat ne l'interdit littéralement. Il
fabrique pourtant l'axe que JUR règle 2 refuse — gros égale sûr, petit égale
douteux —, sans employer un seul mot du lexique et sans qu'aucun contrôle
existant ne le voie. C'est la forme exacte dont le projet est mort ailleurs :
une gradation de crédibilité — que le projet n'admet jamais, sous aucune forme —
obtenue par une variable visuelle, sur des organisations nommées.
