# Direction artistique — assemblage des trois rendus

Page assemblée : `atelier/rendus/site.html`. Rendus regardés :
`site.png` (760, clair), `site-sombre.png` (760, sombre), `site-etroit.png`
(380, clair), `site-bas.png` (bas de page, 760, clair — la page fait 6 720 px
de haut, les 3 000 px de la commande n'en montrent que la moitié).

Ordre de la page : en-tête → thèse → légende → partition → preuve → pied.

---

## 1. Le système

Un seul bloc `:root`, recopié tel quel de `site.html`. Toute valeur d'espacement,
de filet ou de corps employée dans la page vient de là ; il n'y a pas de valeur
libre dans les règles, sauf les quatre constantes de disposition du SVG (§1.4).

### 1.1 Teintes

```css
/* support */
--papier:#e8eaee;  --papier-systeme:#f5f6f8;
--encre:#191e2c;   --encre-pale:#5c6373;  --regle:#b7bec9;

/* famille — trois, et trois seulement (GV-05) */
--f-votes:#191e2c; --f-experts:#2b5490; --f-nuance:#3f6b3c;
--f-aucune:#5c6373;
```

```css
@media (prefers-color-scheme:dark){:root{
  --papier:#12151b;  --papier-systeme:#1a1e27;
  --encre:#e3e7ee;   --encre-pale:#939bab;  --regle:#39404e;
  --f-votes:#e3e7ee; --f-experts:#8fb2e4; --f-nuance:#8bbd84;
  --f-aucune:#939bab;
}}
```

Une teinte de famille n'apparaît qu'à trois endroits, et nulle part ailleurs :
le filet de 2 px en tête d'un bloc de légende ou d'une colonne de tableau, le
remplissage d'une tête de marqueur, le filet gauche et l'intitulé d'un panneau
de preuve. Aucun sélecteur portant une couleur ne nomme une entité (GV-13, C13).

### 1.2 Rythme

```css
--e1:.4rem; --e2:.8rem; --e3:1.2rem; --e4:1.8rem; --e5:2.8rem; --e6:4rem;
--gouttiere:1.25rem; --largeur:64rem; --mesure:44rem;
```

`--e5` sépare deux sections, `--e4` deux sous-blocs, `--e3` un titre de son
corps, `--e2` deux lignes d'un même bloc, `--e1` une étiquette de sa valeur.
`--e6` n'est employé que par le pied et le bas de page. `--mesure` plafonne
toute prose, y compris les motifs et les constats du panneau de preuve.

### 1.3 Filets

```css
--filet:1px solid var(--regle);
--filet-famille:2px solid var(--famille,var(--regle));
```

Deux épaisseurs, pas trois. Le 1 px sépare (rubriques, lignes de tableau,
cadres, sections d'un panneau) ; le 2 px désigne une famille et n'existe qu'en
tête d'un bloc de légende, d'une colonne de tableau, ou à gauche d'un panneau
de preuve. La variable `--famille` est posée sur le bloc, jamais dans la règle :
c'est ce qui rend le filet coloré réutilisable sans dupliquer la règle.
Le pointillé du brief de preuve (`2px dashed`, bords des notes d'atelier)
disparaît avec les notes d'atelier (§3.6).

### 1.4 Corps

```css
--t-marque:2rem;    /* 32 px, serif 500 — une occurrence */
--t-these:1.0625rem;/* 17 px, serif 400 — thèse et blocs explicatifs */
--t-entite:1.125rem;/* 18 px, serif 500 — rang 1, le nom d'entité */
--t-valeur:1rem;    /* 16 px, mono 500, tabular-nums — rang 2, ce qui se compare */
--t-appareil:.875rem;/* 14 px — rang 3, le plancher : rien n'existe en dessous */
```

Dans le SVG les mêmes rangs sont écrits en pixels, `web/src/Partition.tsx`
n'ayant pas de `viewBox` : 18 / 16 / 14 px. Les quatre constantes de
disposition restent celles de la partition (`H` 46 px large, 76 px étroit ;
gouttière 42 % ; colonne droite 92 px ; bascule étroite à 560 px).

---

## 2. Ce qui divergeait, et ce qui a été retenu

| Point | Enveloppe | Partition | Preuve | Retenu |
|---|---|---|---|---|
| Largeur de page | 64 rem | 64 rem | 60 rem | **64 rem** |
| Marge haute de page | 2,5 rem | 1,6 rem | 2,25 rem | **`--e5`, 2,8 rem** |
| Gouttière | 1,25 rem | 1,25 rem | 1,1 rem | **1,25 rem** |
| Écart entre sections | 2,6 rem | 1,4 rem | 3 rem | **`--e5`** |
| Rubrique | `h2`, filet 1 px | — | `.rubrique`, filet 1 px | **`h2` / `h3`, même règle** |
| Teinte `experts` | `--crayon-rouge` | `--crayon-bleu` | — | **`--f-experts`, bleu** |
| Teinte `nuance` | `--crayon-bleu` | `--crayon-vert` | `--crayon-bleu` | **`--f-nuance`, vert** |
| Filet de famille | 2 px | — | 3 px à gauche | **2 px partout** |
| Prose plafonnée à | 42–44 rem | 42–46 rem | 44 rem | **`--mesure`, 44 rem** |

`--crayon-rouge` est **supprimé de la palette**. Trois familles, trois teintes :
si le rouge reste défini, il finira par servir, et le seul emploi qu'il trouvera
est celui que GV-08 refuse — une couleur d'alerte sur les deux lignes non
publiées. La clé de l'en-tête porte désormais les trois glyphes de famille
(carré plein `--f-votes`, losange `--f-experts`, carré ouvert `--f-nuance`) :
la marque est la légende.

---

## 3. Les arbitrages

### 3.1 La moustache d'IQR reste retirée

Le brief UX la classe en catégorie 1 et la décrit comme immédiatement lisible.
Elle l'est. Elle est aussi fausse.

Le projet publie `mediane` et `iqr`. Il ne publie ni Q1 ni Q3 — et ne peut pas :
ADR 0003 §3 rappelle qu'une borne de distribution est la coordonnée d'un membre
identifiable. Une moustache centrée sur la médiane et longue d'un IQR dessine
donc l'intervalle [médiane − IQR/2, médiane + IQR/2], **qui n'a jamais été
mesuré** : elle affirme que la dispersion est symétrique autour de la médiane,
ce que la donnée ne dit pas et que le pipeline n'a pas calculé. Ce n'est pas une
simplification de lecture, c'est une quantité fabriquée par le rendu — GV-01.
Le fait qu'elle soit belle et qu'elle « dise littéralement ce qu'elle mesure »
aggrave le cas : ce qu'elle donne à lire est plus précis que ce qui existe.

L'IQR reste en texte, en rang 3, accolé à l'effectif : `71 · 0,03`.

### 3.2 L'intervalle ouvert de la bande « mesuré, non publié » tombe avec elle

Même raisonnement, et un interdit explicite en plus. Le rendu de la partition
dessinait LIOT et NI par un intervalle ouvert dont **la longueur valait l'IQR à
l'échelle de l'axe** (199 px pour 0,64). GV-10 l'écrit sans réserve : « aucune
longueur géométrique du SVG ne dérive de `dispersion` — l'IQR ne s'affiche qu'en
`<text>` ». Garder cette barre après avoir retiré la moustache pour cause de
géométrie inventée aurait été incohérent : c'est la même encre, tirée du même
nombre, à la même échelle.

La bande devient donc **du texte, pas du dessin** : nom d'entité en rang 1,
`Position non publiée` en rang 2 dans la gouttière droite, les chiffres du refus
en rang 3 (`effectif 20 · IQR 0,64 · maximum publiable 0,25`), le motif daté en
italique. Aucune graduation, aucun tick, aucune portée, aucune abscisse : rien
à aligner, donc rien à mésinterpréter.

Ce qui est perdu, et qu'il faut assumer : l'effet visé par le cas dur 2 du brief
UX — « la raison du retrait est le dessin lui-même ». Elle est maintenant dans
la phrase, avec ses deux nombres en regard. Le brief typo l'avait déjà anticipé
(ton.md T6 : le motif porte son chiffre et son seuil).

### 3.3 « Position non publiée » l'emporte sur « non mesuré »

Retenu dans les quatre emplacements où la question se pose : la bande sous la
partition, le cadran du panneau de preuve, la comparaison des états, le
`aria-label` du marqueur.

`docs/ton.md` T7 impose encore « Non mesuré ». Appliqué à LIOT, c'est faux :
l'IQR vaut 0,6417, il est calculé, consigné, publié avec son motif ; ce n'est
pas la mesure qui manque, c'est la position que le projet refuse de publier.
Le modèle sépare déjà les deux cas par `motif_code`. Le dessin les sépare
maintenant aussi. **Les deux documents doivent suivre**, et voici la forme exacte
à y porter :

- `docs/ton.md`, ligne T7 — remplacer la formulation de la règle par :

  > **L'absence est dite avec son code, jamais comblée.** Deux formes, et deux
  > seulement : « Position non publiée » quand `motif_code` vaut
  > `sous_seuil_de_publication` — la mesure existe, ses chiffres sont publiés
  > avec le motif ; « Non mesuré » quand aucune valeur n'existe (`hors_source`).
  > Jamais `neutre`, `centre`, `n.d.`, `—`, ni case vide muette.

- `docs/glossaire.md` §3, entrée `position`, table des quatre concepts —
  ajouter une cinquième ligne :

  | **position non publiée** | mesure calculée dont la position n'est pas publiée, la dispersion ou l'effectif étant hors des conditions de publication ; s'énonce avec le chiffre mesuré et le seuil en regard | **2** (LIOT, NI, instantané `an17-2026-07-21`) |

  et, sous « Ce que ce n'est pas », ajouter : « Une position non publiée n'est
  pas une position manquante : la ligne de preuve existe et se vérifie. »

Le dessin a raison avant le document ; l'écart est écrit ici pour qu'il ne
passe pas pour un oubli. Tant que la PR de rédaction n'est pas passée,
`site.html` est en avance sur `docs/ton.md`, et c'est délibéré.

### 3.4 Place publique a sa place

L'entité existe : `parti.place-publique`, dans `instantane.sans_mesure`, motif
`hors_source`. La graphiste de la partition ne l'a pas vue parce qu'elle n'a lu
que `bandes[]`, où elle n'est pas — et où elle ne doit pas être.

Elle apparaît deux fois dans la page assemblée, jamais sur un axe :

1. sous les deux panneaux gradués, rubrique **« Entités sans mesure — 1 »** :
   nom en rang 1, `Aucune famille ne porte de valeur` en rang 2, motif daté en
   rang 3. C'est le cas dur 1 du brief UX, appliqué tel quel ;
2. en quatrième panneau de preuve, avec le relevé des cinq sources consultées —
   c'est ce qui transforme un silence en constat daté.

Le champ « Conséquence à l'écran » du panneau d'atelier est retiré : il décrivait
la maquette au lecteur du brief, pas la donnée.

### 3.5 Une teinte par famille, arbitrée une fois

GV-05 exige une teinte par famille et une seule ; il ne dit pas laquelle. §3 des
garde-fous renvoie l'arbitrage à un humain, une fois, à l'adoption de la palette.
C'est fait : `votes` prend l'encre (c'est la famille du projet, calculée par
lui), `experts` le bleu (source tierce, mesurée, graduée), `nuance` le vert
(source administrative, non graduée). Le vert n'est jamais employé pour un
nombre : il ne porte que des codes de nuance, qui ne se comparent pas.

### 3.6 Les notes d'atelier ne montent pas dans la page

Les rendus de la preuve portaient huit blocs `note-atelier` (« Lecture. »,
« L'empreinte. », « Le pli. », « Pourquoi le vide n'est pas une prise de
position. ») et une bande de comparaison des quatre états. Ce sont d'excellentes
notes de brief et des violations de GV-15 / RG-94 dans un site : un bloc
explicatif par écran, trois phrases au plus, aucune adresse au lecteur. Elles
restent dans `atelier/rendus/preuve.html`, elles ne sont pas dans `site.html`.

---

## 4. Ce que je n'ai pas tranché seul

1. **Deux blocs explicatifs sur la page.** RG-94 / ton.md T5 en autorisent un
   par écran. La page en porte deux : la thèse de concordance (3 phrases) et
   l'ancrage des deux médianes (3 phrases), chacun ≤ 3 phrases, chacun posé au
   -dessus de ce qu'il explique et séparé de l'autre par ~1 200 px de
   défilement. J'ai supprimé la troisième phrase de la thèse, qui redisait
   « jamais moyennées » déjà présente deux fois. Reste à dire si « un écran »
   se compte en page ou en fenêtre ; ce n'est pas une question de dessin.

2. **La hauteur de la page : 6 720 px à 760 px de large, 9 000 et plus à
   380 px.** Le brief UX visait ~900 px pour la partition seule — c'est tenu
   (les deux panneaux gradués occupent un peu plus de 1 000 px à 760 px de large,
   contre 1 456 px pour dix axes séparés).
   Le reste est le panneau de preuve déplié, montré ici parce qu'une page
   statique n'a pas d'interaction. En production il est fermé et la page tient.
   Les trois captures demandées (3 000 / 3 400 px) coupent donc la page ;
   `site-bas.png` montre le reste.

3. **`Nuance 2024` répété.** Le brief typo §4 retire l'intitulé de famille
   répété par voix, mais GV-16 exige que le producteur précède le code au DOM.
   J'ai gardé une mention unique en sous-titre du bandeau (`Nuance 2024 · Code
   de nuance, sans échelle graduée.`) et le producteur nommé en tête du panneau
   de preuve. Sept pastilles nues sous un titre de section satisfont GV-16 à la
   lettre mais pas à l'œil de quelqu'un qui arrive par une ancre : à vérifier
   avec le contrôle d'ordre des nœuds, que je ne peux pas écrire ici.

4. **`Voir la preuve`.** Le brief typo §5 la déclare manquante et nécessaire —
   « le seul actif du projet, et il est invisible ». Elle n'est pas dans la page
   assemblée : la page statique montre les panneaux dépliés, il n'y a rien à
   ouvrir. La chaîne devra être posée en rang 3 à droite de chaque bande quand
   la partition redeviendra interactive. Ce n'est pas un arbitrage, c'est un
   reste à faire.

5. **La longueur des motifs.** Le brief typo plafonne à 95 caractères pour
   cause de texte SVG non repliable. Les motifs sont maintenant en HTML et se
   replient : la contrainte tombe pour eux, elle reste pour tout ce qui est
   écrit dans le SVG. Deux plafonds différents dans la même page est un piège ;
   je ne sais pas s'il vaut mieux garder 95 partout par prudence.

---

## 5. Interdits touchés de près

- **GV-10** — respecté en retirant l'intervalle d'IQR (§3.2). C'est le seul
  endroit où l'assemblage m'a mis en face d'un interdit et où j'ai retiré du
  dessin plutôt que de le contourner.
- **GV-03** — le décalage des deux axes (insertion 19 %, largeur 68 %) est
  conservé tel quel, et la note qui le déclare délibéré est sous le graphe.
  C'est le seul « défaut d'alignement » de la page qu'il ne faut pas corriger.
- **GV-09** — deux barres de graduation par échelle, aux bornes déclarées,
  aucun tick intermédiaire, aucun zéro. Le bandeau de nuances n'a pas de portée.
- **GV-05 / GV-06** — une teinte et un glyphe par famille, gabarit constant :
  `circle r=5` pour `votes`, losange de 12 px pour `experts`, aucun autre.
  Rien n'est indexé sur l'effectif ni sur l'IQR.
- **GV-15** — un `note-atelier` de moins par section (§3.6).
- **Le `<rect>` sans `fill`** — les règles de remplissage du SVG sont scopées à
  `.partition` et la tête de marqueur reprend `currentColor` avec une
  spécificité supérieure. Vérifié à l'image : aucun aplat noir, dans les deux
  thèmes. La clé de l'en-tête, elle, garde ses `fill` en attribut — c'est
  pourquoi les règles générales ne doivent pas être écrites sur `rect, circle,
  path, line` nus : elles gagnent contre un attribut de présentation et
  effacent le logo. Défaut rencontré, corrigé, consigné.
