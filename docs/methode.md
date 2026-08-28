# Méthode

Chaîne de traitement, maillon par maillon. Chaque maillon est déterministe :
même entrée, même sortie, bit pour bit.

---

## Ce que « déterministe » exclut, et ce qu'il n'exclut pas

**Exclu : la génération de texte.** Aucun résumé, aucune synthèse rédigée,
aucune reformulation. C'est la source d'erreur non vérifiable, et c'est aussi
inutile ici : l'objectif est que le lecteur se fasse son avis, donc l'outil lui
montre les titres et les mots réellement employés par les rédactions.

**Non exclu : le calcul vectoriel.** Un embedding est une fonction fixe — pas de
génération, donc pas d'hallucination possible, et un résultat reproductible à
l'identique. Ce n'est pas de l'« IA qui réfléchit », c'est de l'algèbre
linéaire. Si un embedding est utilisé, le modèle et sa version sont épinglés et
consignés dans le registre de preuves.

Conséquences pratiques : aucune clé d'API, aucun coût par exécution, tests hors
ligne complets, et un résultat que n'importe qui peut reproduire depuis les
sources brutes.

---

## Brique 0 — positionner les acteurs

### 1. Matrice de votes

> **cellule**, **bloc**, **acteur**, **corpus**, **densité**, **porte** : chacun de ces mots a un sens exact et un seul, fixé par [glossaire.md](glossaire.md).

Scrutins nominatifs de l'Assemblée nationale, open data, JSON.

Codage `pour / contre / abstention`. L'**absence est une donnée manquante**, pas
une position — l'assimiler à une abstention fabrique un centrisme artificiel
chez les députés peu présents.

**Aucun seuil de participation.** L'objection est légitime — un scrutin peu
suivi renseigne sur les présents — mais elle a été mesurée et ne tient pas :
aucune rupture de la distribution ne désigne de valeur, et un seuil relevé
dégrade la séparation des blocs au lieu de l'améliorer. Un seul filtre, qui est
une définition et non un seuil : un scrutin sans minorité enregistrée n'entre pas
dans la matrice — `min(pour, contre) ≥ 1` — sa variance étant nulle. 455 scrutins
écartés sur 8 434, dont les 23 motions de censure en totalité, l'article 49-2 ne
faisant voter qu'un camp ; 7 979 retenus. `nombreVotants` est publié par scrutin
avec le décompte, jamais employé comme porte
([ADR 0003](adr/0003-arbitrages-de-coherence.md) §2).

Rattachement député → groupe **avec période de validité**. Le groupe retenu est
celui du **bloc de ventilation du scrutin** : il est daté par construction et ne
demande aucune jointure. Le référentiel AMO30 sert de recours pour les blocs
`organeRef: "PO0"`, de source des périodes de validité des groupes, et de
contrôle croisé ([ADR 0003](adr/0003-arbitrages-de-coherence.md) §1). Les
changements de groupe en cours de mandature sont fréquents et cassent
silencieusement toute agrégation qui les ignore.

### 2. Axe issu des votes

Estimation de position sur les **seules cellules observées** : moindres carrés
alternés de rang 1, avec une constante par scrutin qui absorbe le fait qu'un
scrutin est majoritairement pour ou contre. L'ACP et l'analyse des
correspondances sont écartées : elles exigent une matrice complète là où les
trois quarts des cellules sont manquantes, et l'absence n'est pas une position.
Signe et échelle fixés en une seule transformation affine, par les médianes des
deux groupes de repère ramenées à −1 et +1 — sans quoi l'orientation de l'axe
change d'une exécution à l'autre.

Méthode volontairement légère. Les estimateurs de référence de la littérature
(IRT bayésien, W-NOMINATE) sont plus corrects sur les données manquantes ; ils
deviennent justifiés si le premier axe s'avère instable, et pas avant.

**Limite structurelle.** La discipline de vote est quasi totale à l'Assemblée.
L'axe sépare donc nettement les blocs et très mal les individus — contrairement
au Congrès américain d'où la méthode est empruntée. Les votes positionnent les
**partis**. Le positionnement fin d'un député isolé n'est pas défendable et
n'est pas affiché comme tel. La dispersion intra-groupe est publiée pour que la
limite soit visible : **écart interquartile et écart-type de rééchantillonnage**.
Jamais la variance, illisible sur un axe sans unité ; jamais l'étendue, dont les
deux bornes sont les coordonnées de deux membres identifiables du groupe
([ADR 0003](adr/0003-arbitrages-de-coherence.md) §3).

### 3. Familles de mesure indépendantes

| Famille | Ce qu'elle capte |
|---|---|
| Votes nominatifs | Comportement **révélé** — ce qu'ils font |
| Enquêtes d'experts (CHES) + programmes (Manifesto/RILE) | Position **déclarée et expertisée** — ce qu'ils disent, et comment le champ académique les situe |
| Nuancier du ministère de l'Intérieur | Classification **administrative** — comment l'État les étiquette |

**Elles ne sont jamais moyennées.** Un chiffre unique fusionnant les trois
détruirait la seule information intéressante : leur écart. Un parti dont les
votes n'ont pas bougé mais que l'administration a reclassé est exactement le cas
que l'outil doit rendre visible.

Le nuancier n'est pas une vérité de référence : c'est une classification
gouvernementale, révisée par circulaire, contestée devant le Conseil d'État. Il
est stocké comme un point de mesure daté, avec la référence de la circulaire et
l'issue des recours.

### 4. Registre d'entités

Identifiants stables de partis, réconciliant groupe parlementaire AN,
identifiant CHES, code Manifesto et code nuance.

**Groupe parlementaire ≠ parti**, la distinction est portée par le modèle.
Périodes de validité pour les fusions, scissions et changements de nom.

Fichier versionné, lisible et corrigeable à la main. Une erreur ici se propage
dans toutes les briques : c'est le seul endroit où la relecture manuelle est
justifiée.

### 5. Registre de preuves

JSONL en ajout seul. Une ligne par position mesurée, portant : entité, valeur,
méthode, source, **date de la source**, date de calcul.

Idempotent, dédupliqué par identifiant, rejouable — la reconstruction complète
depuis les sources brutes doit redonner le même fichier. C'est ce qui rend une
position vérifiable par un tiers, et c'est ce qui rend l'historique des dérives
gratuit.

---

## Brique 1 — presse écrite

### 6. Regroupement des articles en sujets

TF-IDF ou BM25, similarité cosinus, clustering agglomératif, fenêtre glissante
de 48 h.

Renforcement par recouvrement d'entités nommées : les noms propres sont le
signal fort du « même événement », bien plus que le vocabulaire général.

### 7. Extraction d'acteurs — gazetteer, pas de NER statistique

Appariement contre une liste fixe issue de la brique 0 : députés, sénateurs,
partis, syndicats, ministères, avec leurs variantes de graphie.

Un modèle de reconnaissance d'entités serait plus couvrant et moins auditable.
Ici l'auditabilité primait : chaque appariement se vérifie ligne par ligne, et
une erreur se corrige dans un fichier de données plutôt que dans un modèle.

### 8. Espace de citation

Matrice `rédaction × acteur` de l'attention accordée. ACP, premier axe. Il
**émerge des données** — aucune étiquette n'est saisie à la main. Signe fixé par
deux points de repère.

**Citer n'est pas approuver.** Une rédaction cite ses adversaires pour les
attaquer. L'axe mesure l'attention, ce qui répond au besoin — il indique de qui
on entendra parler — mais il ne mesure pas l'adhésion et ne porte jamais le mot
« biais ». Nom retenu : **espace de citation**.

### 9. Biais de sélection

Comptage pur : quels sujets sont couverts par qui, et surtout par qui ils ne le
sont pas. C'est la mesure la plus simple, la plus solide et la plus difficile à
contester. Elle ne demande que les titres et les dates.

### 10. Divergences chiffrées

Extraction des quantités par motifs, normalisation des unités et des ordres de
grandeur, regroupement des nombres désignant la même grandeur au sein d'un
sujet.

Sortie : les deux valeurs et leurs sources. Aucun commentaire. Deux chiffres
incompatibles pour le même fait sont auto-explicatifs.

### 11. Divergences de cadrage

Log-odds ratio à prior de Dirichlet informatif — Monroe, Colaresi & Quinn
(2008), méthode conçue exactement pour comparer le vocabulaire de deux corpus
politiques de tailles inégales.

Sortie : les termes qui distinguent réellement la couverture d'un groupe de
rédactions de celle d'un autre. Ce sont **leurs** mots, extraits
statistiquement, pas une paraphrase — plus honnête et plus vérifiable qu'un
résumé.

### 12. Affichage

Titres réels côte à côte, écarts mis en évidence, liens vers les articles. Le
travail d'interprétation reste au lecteur, ce qui est le but.

---

## Stockage et droit d'auteur

L'exception de fouille de textes et de données (directive 2019/790, transposée à
l'article L.122-5-3 du code de la propriété intellectuelle) permet l'analyse.
Elle ne permet pas de constituer une base de textes intégraux republiable.

Conception imposée par cette contrainte, et plus légère au passage :
- métriques dérivées et compteurs conservés durablement ;
- corps d'article conservé le temps du calcul, puis écarté ;
- citations courtes uniquement, avec lien vers la source ;
- respect des opt-out déclarés par les éditeurs.
