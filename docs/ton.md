# Ton et lexique

Spécification de rédaction du dépôt. Elle n'est pas une préférence de style :
elle est l'application du registre arbitré en
[ADR 0000 §5](adr/0000-perimetre-brique0.md) et du lexique contraint de
[juridique.md](juridique.md). Chaque règle est formulée pour qu'un relecteur
puisse répondre oui ou non sans discuter.

Portée : toute chaîne visible du produit, tout document de `docs/`, `README.md`,
`ROADMAP.md`, `CONTRIBUTING.md`, les messages de commit, les descriptions de PR,
les libellés de données du registre d'entités, les messages d'erreur du
pipeline. Un message d'erreur est une chaîne visible.

Ce document ne rouvre aucun arbitrage. Il en dérive les contrôles.

---

## 1. Les huit règles de forme

| # | Règle | Comment un relecteur la vérifie |
|---|---|---|
| T1 | **Registre : constat technique.** Descriptif, jamais évaluatif, jamais pédagogique, jamais promotionnel. | Aucune phrase ne peut être précédée de « je trouve que » sans devenir fausse. |
| T2 | **Aucune personne grammaticale.** Ni `nous`, `notre`, `nos`, ni `vous`, `votre`, `vos`, ni `on`, ni impératif adressé au lecteur. | `grep -nwiE 'nous\|notre\|nos\|vous\|votre\|vos\|on'` sur le diff, hors citation entre guillemets. |
| T3 | **Présent pour la méthode et pour la règle. Passé pour la mesure faite.** Aucun futur d'intention. | « Sera fait », « devrait », « pourrait » sont absents ou portent une date et un identifiant d'incrément. |
| T4 | **Toute valeur porte sa date et son effectif.** Un nombre seul n'existe pas. | Chaque chiffre publié a une date à moins de 140 caractères de lui. |
| T5 | **Étiquette ≤ 40 caractères. Légende ou note ≤ 140 caractères, une phrase. Un seul bloc explicatif par écran, 3 phrases au plus.** | Compte de caractères ; un point final unique dans une légende. |
| T6 | **Comparatif interdit sans les deux chiffres.** « plus à droite que » est proscrit ; « 0,42 contre 0,18 » est retenu. | Tout mot en `plus…que` / `moins…que` / `-er` superlatif est suivi ou précédé de deux valeurs. |
| T7 | **L'absence est dite avec son code, jamais comblée.** « Non mesuré » et son motif. Jamais `neutre`, `centre`, `n.d.`, `—`, ni case vide muette. | `grep` des quatre formes interdites dans les artefacts publiés (contrats.md I14, EXP-03). |
| T8 | **Rien de non vérifié n'est affirmé.** Ce qui n'a pas pu l'être porte `A VERIFIER` et la façon de le vérifier. | Toute affirmation factuelle a une source citée, ou le marqueur. |

### Proscrit en sus du lexique

Superlatifs, adverbes d'intensité (`très`, `extrêmement`, `particulièrement`,
`largement`, `réellement`, `clairement`), points d'exclamation, questions
rhétoriques, métaphores — y compris les métaphores biologiques appliquées à une
organisation (`un parti naît`, `un groupe meurt`, `une source est morte`) —,
verbes d'intention prêtés à une entité (`ce groupe a voulu`, `l'administration
cherche à`), émojis et pictogrammes décoratifs.

Un verbe d'intention se remplace par l'acte constaté : non pas « un groupe a jugé
utile de faire compter les présents » mais « un groupe a demandé un scrutin
public ».

---

## 2. Le lexique contraint

Reprise du tableau de [juridique.md](juridique.md), étendue aux termes apparus
depuis. La colonne de gauche ne figure nulle part dans le dépôt, clés de données
et identifiants internes compris — la règle vaut en interne, pas seulement à
l'affichage (definition-of-done.md §12).

| Interdit | Retenu |
|---|---|
| biais d'un média, biais de couverture | espace de citation, attention accordée |
| fiabilité, crédibilité, véracité, qualité d'une source | *rien — aucune mesure de ce type n'existe dans le projet* |
| désinformation, fake news, infox, intox | *jamais employé* |
| média orienté, partial, militant, extrémiste, radical, modéré | position mesurée sur l'axe, à telle date |
| ce média ment, ce parti ment | les valeurs citées divergent : X selon A, Y selon B |
| classement, palmarès, notation | positionnement daté |
| score, note, indice, cote | valeur, position |
| position consolidée, synthèse, moyenne des familles, position globale, consensus | *aucun objet de ce nom n'existe ; les trois familles ne sont jamais moyennées* |
| vrai / faux positionnement, position correcte | position issue de telle famille, à telle date |
| centre, neutre, n.d., case vide | non mesuré, avec son `motif_code` |
| variance intra-groupe *(comme quantité publiée)* | dispersion intra-groupe : écart interquartile, écart-type de rééchantillonnage |
| étendue, minimum, maximum d'un groupe | *(rien — une borne d'étendue est la coordonnée d'un membre identifiable)* |
| part de variance expliquée *(comme critère de publication)* | critère de séparation des axes, `s2/s1` |
| tests de sanité, sanity check | contrôles de garde-fou |
| hash, checksum | empreinte SHA-256 |
| dataset | jeu de données |
| trend file *(hors nom de fichier cité)* | fichier de tendance CHES |
| backend, frontend | pipeline, front |
| wording | rédaction |
| API *(pour le contrat de sortie)* | contrat de sortie |

Les six noms de champ `moyenne`, `consolide`, `synthese`, `score_global`,
`indice`, `note` sont refusés par le validateur, pas seulement par la relecture
(contrats.md I12, plan-de-tests.md PRE-10).

---

## 3. Termes canoniques

Un concept, un nom, partout. La colonne de droite est la seule forme admise.

### Objets de mesure

| Concept | Forme canonique |
|---|---|
| Les trois familles | **famille de mesure** ; identifiants `votes`, `experts`, `administratif` |
| Résultat publié | **position**, **positionnement daté** |
| Axe des scrutins | **axe issu des votes** |
| Échelle de cet axe | **unités ancrées**, ancre gauche = médiane LFI-NFP = −1, ancre droite = médiane RN = +1 |
| Enquête d'experts | **CHES**, échelle 0–10, variable `lrgen`, **vague** 2024 (jamais « édition ») |
| Classification administrative | **nuance administrative**, issue du **nuancier du ministère de l'Intérieur** |
| Dispersion d'un groupe | **dispersion intra-groupe** = {écart interquartile (IQR), écart-type de rééchantillonnage} |
| Critère de publication de l'axe | **critère de séparation des axes** (`s2/s1`) |
| Ce qui autorise la publication d'un groupe | **conditions de publication** — les trois de positionnement.md §6 (IQR, écart-type de rééchantillonnage, effectif). Jamais « seuil de variance expliquée » |
| Cycle de vie d'un groupe | **créé le …**, **actif du … au …**, **dissous le …**. Jamais « naît », « vit », « meurt » |
| Brique 1, à venir | **espace de citation** |

**Divergence assumée avec les documents fondateurs.** ROADMAP.md v0.2 et
methode.md §2 écrivent « variance intra-groupe » et « part de variance
expliquée ». Ces deux formes sont remplacées ici et dans `docs/brique0/` parce
que positionnement.md §6 les a mesurées non publiables telles quelles ; le
remplacement est épinglé par le test AGR-03 et listé en plan-de-tests.md §16.
Les deux fichiers fondateurs ne sont pas modifiés par une passe de rédaction :
ils le sont par la PR de méthode qui portera le changement
(definition-of-done.md §19).

### Entités

| Concept | Forme canonique |
|---|---|
| Personne morale durable | **parti** (`parti.*`) |
| Alliance électorale datée | **coalition** (`coalition.*`) |
| Organe d'une chambre pour une législature | **groupe parlementaire** (`groupe.an17.*`) |
| Sigle d'un groupe | `libelleAbrev` de la source, **en capitales** : NI, AD, DEM, DR, ECOS, EPR, GDR, HOR, LFI-NFP, LIOT, RN, SOC, UDR, UDDPLR |
| Législature | **XVIIe législature** en prose, `17` en donnée |

`libelleAbrege` (`Dem`, `EcoS`, `UDR` pour `PO872880`) n'est jamais employé comme
sigle ni comme clé : il diverge de `libelleAbrev` sur trois groupes.

### Codage des votes

| Concept | Forme canonique |
|---|---|
| Position exprimée | **pour** (+1), **contre** (−1), **abstention** (0) |
| Empêchement institutionnel | **non-votant**, causes `MG`, `PAN`, `PSE` — **donnée manquante**, jamais une position |
| Député hors des blocs | **absence** — **donnée manquante** |
| Cellule écrite / non écrite | **cellule observée** / **cellule absente**. Jamais « valeur manquante imputée », jamais « masque » |
| Déclaration postérieure au vote | **mise au point de vote** — ingérée, jamais appliquée |
| Seul filtre de scrutins | **minorité non vide** (`min(pour, contre) ≥ 1`). Ce n'est pas un seuil de participation ; aucun seuil de participation n'existe |

### Artefacts

| Concept | Forme canonique |
|---|---|
| `data/preuves/positions.jsonl` | **registre de preuves**, une **ligne de preuve** par valeur |
| `data/registre/partis.json` | **registre d'entités** |
| `public/api/index.json` | **manifeste** |
| `public/api/instantanes/<id>.json` | **instantané** |
| `public/api/preuves/<xx>.json` | **éclat de preuves** |
| Empreinte d'un fichier | **empreinte SHA-256** |
| Bornes de temps | **période de validité**, bornes incluses |
| Contrat versionné | **contrat de sortie** (majeure / mineure / patch, ADR 0000 §6) |

### Licence

Deux objets distincts, jamais confondus :

| Objet | Forme canonique |
|---|---|
| Amont Assemblée nationale | **Licence Ouverte / Open Licence 1.0**, producteur : Assemblée nationale |
| Sorties du projet — code | **AGPL-3.0-only** |
| Sorties du projet — données, registres, documentation | **Licence Ouverte 2.0** (`etalab-2.0`) |
| Nuancier via data.gouv.fr | **Licence Ouverte 2.0** (`license = lov2`) |

---

## 4. Tournures bannies et leur remplacement

| Banni | Remplacement |
|---|---|
| il est important de noter que | *supprimer, énoncer le fait* |
| il convient de, il faut noter que, à noter que | *supprimer* |
| en effet, par ailleurs, en outre, force est de constater | *supprimer, ou une conjonction* |
| on peut dire que, on constate que, on remarque que | *le constat, sans « on »* |
| un test qu'on désactive | un test qui finit désactivé |
| ce n'est pas X, et c'est le bon choix | *énoncer les raisons, sans le jugement* |
| la comparaison honnête, le verdict | la comparaison, la décision |
| tests de sanité | contrôles de garde-fou |
| l'ordre est celui qu'on attend | l'ordre obtenu correspond à l'ordre gauche-droite décrit par la littérature |
| suffisent largement | suffisent à *(avec le chiffre)* |
| bien séparés, très demandé, réellement dangereux | séparés *(avec le chiffre)*, demandé *(ou supprimer)*, à risque |
| une structure presque aussi grosse | une structure de poids comparable |
| les plus gros groupes | les groupes les plus nombreux |
| ce groupe est composé pour être hétérogène | ce groupe réunit des députés de *n* partis déclarés |
| la source est morte | la source ne répond plus (code HTTP, date) |
| un groupe naît, vit, meurt | un groupe est créé le …, dissous le … |
| le validateur tourne en CI | le validateur s'exécute en intégration continue |
| ce média est partial | l'espace de citation de cette rédaction se situe à … , au … |
| ce parti est extrémiste | position 0,98 en unités ancrées, au 2026-08-27 |
| le député X est plus modéré | la position estimée du député est plus proche de zéro que son comportement de vote observé |
| la position réelle du parti | la position issue de la famille *f*, à telle date |
| notre axe, notre méthode | l'axe, la méthode |
| nous ne disposons pas de | aucun instrument équivalent n'existe |
| ✅ ❌ 🚀 et tout pictogramme | un mot |
| réellement observé / attribué / présent / téléchargé | observé / attribué / présent / téléchargé |
| la meilleure clé, le meilleur estimateur | la clé retenue, l'estimateur retenu *(avec la mesure qui le retient)* |
| un groupe fantôme, un votant fantôme | un groupe sans effectif réel, un votant inexistant |
| on croit tenir la jointure | ce qui donne l'impression que la jointure est là |
| ces deux abrégés sauveraient par accident | ces deux abrégés évitent la fusion par accident |
| ne souffre pas de ces ambiguïtés | n'a pas ces ambiguïtés |
| dès qu'on relève le seuil | dès que le seuil est relevé |
| Ce qu'on perd | Ce qui est perdu |
| on ne renomme pas, on ne supprime pas | aucun renommage, aucune suppression |
| ce qui est justement / précisément ce qu'il y a à voir | ce qu'il y a à voir |
| le chemin est mort | le chemin ne répond plus (code HTTP, date) |
| un litige tuerait le projet | un litige arrête le projet |
| une source qui pourrit | une source qui cesse de répondre |
| site mort | site à l'arrêt |

---

## 5. Nommer une entité, nommer une personne

**Une entité publiée est un parti, une coalition ou un groupe parlementaire.**
Jamais une personne (ADR 0000 §2).

Une personne physique peut être nommée dans la documentation **uniquement** pour
décrire un fait de mandat public — appartenance à un groupe, dates, fonction —
et **jamais** avec une valeur de position, une coordonnée, un rang ou un
qualificatif. Une position ne se rattache à aucun nom, à aucune fonction et à
aucun `acteurRef`, y compris à titre d'exemple pédagogique dans un document
interne : le document est public.

Quand un exemple mesuré porte sur des individus, il s'écrit avec le groupe,
l'effectif et l'écart, sans identifiant ni fonction.

---

## 6. Écriture des nombres et des dates

| Point | Règle |
|---|---|
| Décimales | virgule décimale en prose (`0,42`), point en JSON (`0.42`) |
| Milliers | espace insécable étroite (`8 434`) |
| Signe | `−` (U+2212) en prose, `-` en JSON |
| Dates | `YYYY-MM-DD` partout, y compris en prose |
| Pourcentages | espace avant `%` (`23,2 %`) |
| Intervalles | `[0,10 ; 0,90]` |
| Législature | `XVIIe` en prose, `17` en donnée |
| Octets | `o` pour octet, `Mio` pour mébioctet, `Mo` pour mégaoctet — les trois distingués |

---

## 7. Contrôles opposables

Sur le diff d'une PR, avant de la déclarer finie (definition-of-done.md §11
et §13) :

```sh
# 1. Lexique interdit — hors le tableau de juridique.md et de ton.md
git diff origin/develop... \
  | grep -inE 'fiabilit|crédibilit|credibilit|véracit|veracit|désinformation|desinformation|fake ?news|infox|biais d|partial|militant|classement|palmarès|score|indice|consensus|synthese|synthèse|note globale'

# 2. Personne grammaticale
git diff origin/develop... | grep -nwiE 'nous|notre|nos|vous|votre|vos|on'

# 3. Prose de remplissage et adverbes d'intensité
git diff origin/develop... \
  | grep -inE 'il est important|il convient de|il faut noter|à noter que|en effet,|force est de|très |extrêmement|particulièrement|largement|clairement|réellement|évidemment'

# 4. Absence comblée
git diff origin/develop... | grep -inE '"(neutre|centre|n\.d\.|nd)"|non renseigné'

# 5. Longueurs — étiquettes ≤ 40, légendes et remarques ≤ 140
#    porté par les règles V21 du registre et EXP-04 de l'export

# 6. Sigles de groupe : forme capitale uniquement
git diff origin/develop... | grep -nE '\b(Dem|EcoS|Ecos)\b'
```

Les contrôles 1, 4 et 6 sont automatisables en intégration continue ; les
contrôles 2 et 3 produisent des faux positifs sur les citations et se relisent.
Un faux positif se justifie dans la description de la PR, il ne se supprime pas
du grep.

---

## 8. Ce que ce document ne tranche pas

- Le contenu des décisions : il vit dans les ADR.
- Les seuils numériques : ils vivent dans `docs/brique0/`.
- La forme canonique des fichiers de données : elle vit dans
  [brique0/contrats.md](brique0/contrats.md) §7 et
  [brique0/registre-entites.md](brique0/registre-entites.md) §6 règle V23.

Un désaccord entre ce document et un ADR se règle en faveur de l'ADR, et ce
document est corrigé dans la même PR.
