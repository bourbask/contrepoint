# Ton et lexique

Spécification de rédaction du dépôt. Elle n'est pas une préférence de style :
elle est l'application du registre arbitré en
[ADR 0000 §5](adr/0000-perimetre-brique0.md) et du lexique contraint de
[juridique.md](juridique.md). Chaque règle est formulée pour qu'un relecteur
puisse répondre oui ou non sans discuter.

Ce document ne rouvre aucun arbitrage. Il en dérive les contrôles.

## 0. Deux régimes, deux portées

Les huit règles ont été écrites pour de la documentation et appliquées ensuite,
sans amendement, à des écrans destinés à des visiteurs. Le résultat était
illisible par conformité : T1 interdisait d'expliquer, T2 interdisait d'adresser
le lecteur, T5 plafonnait l'explication d'un écran entier à trois phrases, T7 lu
« avec son identifiant » faisait afficher `hors_source` tel quel. La portée est
scindée ici ; les règles qui protègent la mesure, elles, ne le sont pas.

**Régime interne.** S'applique à `docs/`, `README.md`, `ROADMAP.md`,
`CONTRIBUTING.md`, `CHANGELOG.md`, `atelier/`, `schemas/`, aux commentaires de
code de `pipeline/` et de `scripts/`, aux messages de commit, aux descriptions
de PR, aux libellés de données du registre d'entités et aux messages d'erreur du
pipeline. Un message d'erreur du pipeline est lu par un mainteneur : il relève
de ce régime. Règles applicables : §1 en entier, T2 compris.

**Régime public.** S'applique à toute chaîne affichée à un visiteur, c'est-à-dire
aux fichiers de `web/src/` et à eux seuls. Un libellé d'artefact n'est pas
concerné tant qu'il reste dans l'artefact : ce qui est jugé, c'est la chaîne que
le front met à l'écran. Règles applicables : §1, amendé par §1 bis.

La portée du régime public est déclarée une seule fois, dans
`scripts/lexique.sh` (`PUBLIC_CHEMINS`). Les deux listes ci-dessus la citent ;
elles ne la recopient pas.

Un chemin qui ne relève d'aucun des deux régimes relève du régime interne.

---

## 1. Les huit règles de forme

| # | Règle | Comment un relecteur la vérifie |
|---|---|---|
| T1 | **Registre : constat technique.** Descriptif, jamais évaluatif, jamais promotionnel. Expliquer un objet à qui ne le connaît pas est permis dans les deux régimes ; le juger ne l'est pas. | Aucune phrase ne peut être précédée de « je trouve que » sans devenir fausse. |
| T2 | **Aucune personne grammaticale.** Ni `nous`, `notre`, `nos`, ni `vous`, `votre`, `vos`, ni `on`, ni impératif adressé au lecteur. **Amendée en régime public : voir §1 bis.** | `./scripts/lexique.sh personne` sur le diff — le script porte la liste et exclut les chemins du régime public ; les citations entre guillemets se relisent. |
| T3 | **Présent pour la méthode et pour la règle. Passé pour la mesure faite.** Aucun futur d'intention. | « Sera fait », « devrait », « pourrait » sont absents ou portent une date et un identifiant d'incrément. |
| T4 | **Toute valeur porte sa date et son effectif.** Un nombre seul n'existe pas. | Chaque chiffre publié a une date à moins de 140 caractères de lui. |
| T5 | **Étiquette ≤ 40 caractères. Légende ou note ≤ 140 caractères, une phrase. Un bloc explicatif fait 3 phrases au plus.** Le plafond porte sur le bloc, non sur l'écran : un écran peut porter autant de blocs que d'objets à gloser. | Compte de caractères ; un point final unique dans une légende ; 3 points au plus par bloc. |
| T6 | **Comparatif interdit sans les deux chiffres.** « plus à droite que » est proscrit ; « 0,42 contre 0,18 » est retenu. | Tout mot en `plus…que` / `moins…que` / `-er` superlatif est suivi ou précédé de deux valeurs. |
| T7 | **L'absence est dite avec son motif en français, jamais comblée.** « Non mesuré », « Position non publiée », et le motif. Jamais `neutre`, `centre`, `n.d.`, `—`, ni case vide muette. Le `motif_code` reste dans l'artefact : il n'est jamais la chaîne affichée. | `grep` des quatre formes interdites dans les artefacts publiés (contrats.md I14, EXP-03) ; `./scripts/lexique.sh public` pour le code affiché. |
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

## 1 bis. Régime public — ce qui change

S'applique aux fichiers de `web/src/`, et à eux seuls. Ailleurs, §1 s'applique
tel quel.

Un seul amendement, et il ne porte pas sur la mesure :

**T2 amendée.** La deuxième personne à l'indicatif est autorisée, **une fois par
écran au plus**. « Cette page ne vous demande pas de la croire » est admise.
La première personne (`nous`, `notre`, `nos`) reste interdite, `on` reste
interdit, et **l'impératif adressé au lecteur reste interdit** : pas de
« cliquez », pas de « regardez », pas de « comparez ». Une deuxième occurrence
de `vous` sur le même écran est une occurrence de trop.

Contrôle : `./scripts/lexique.sh personne` n'examine pas les chemins du régime
public, et continue de les examiner partout ailleurs. Le plafond d'une
occurrence par écran n'est pas automatisable — un écran n'est pas un fichier —
et se relit. C'est une règle relue, pas une porte.

**Ce qui ne change pas, parce que ce sont les chiffres qui sont en jeu et non le
style :** T3 (temps), T4 (toute valeur porte sa date et son effectif), T6
(comparatif interdit sans les deux chiffres), T8 (rien de non vérifié n'est
affirmé). Le lexique contraint de §2 s'applique intégralement aux deux régimes.

**T7 en régime public.** Le motif est affiché en français — « Position non
publiée », « Non mesuré » — jamais son `motif_code`. Le code voyage dans
l'artefact et s'arrête à la fonction qui le traduit. Contrôle :
`./scripts/lexique.sh public`.

**T1 en régime public.** Expliquer est permis, et c'est le point de l'amendement.
Ce qui reste interdit est le jugement : une glose dit ce qu'une valeur mesure et
comment elle a été obtenue, jamais si elle est bonne.

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
| centre, neutre, n.d., case vide, tiret muet | **position non publiée** quand `motif_code` vaut `sous_seuil_de_publication` — la mesure existe, ses chiffres sont publiés avec le motif ; **non mesuré** quand aucune valeur n'existe (`hors_source`, `aucune_mesure`) |
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

Un concept, un nom, partout. La colonne **Forme canonique** est la seule forme
admise en régime interne, et la seule forme admise partout dans les données, les
clés et les identifiants.

La colonne **Forme d'écran** est ce que le régime public affiche à sa place. Un
tiret `—` signifie que le concept n'a pas à paraître à l'écran : s'il doit y
paraître un jour, la forme d'écran s'écrit ici d'abord. La forme d'écran ne
remplace jamais la forme canonique dans une clé de données.

### Objets de mesure

| Concept | Forme canonique | Forme d'écran |
|---|---|---|
| Les trois familles | **famille de mesure** ; identifiants `votes`, `experts`, `administratif` | d'où vient la mesure |
| Résultat publié | **position**, **positionnement daté** | position, au … |
| Axe des scrutins | **axe issu des votes** | position tirée des scrutins publics |
| Échelle de cet axe | **unités ancrées**, ancre gauche = médiane LFI-NFP = −1, ancre droite = médiane RN = +1 | unités ancrées : médiane LFI-NFP à −1,00, médiane RN à +1,00 |
| Étendue de l'échelle publiée | **amplitude ancrée**, 2,0 par construction. C'est la seule amplitude publiable | — |
| Étendue de l'axe avant ancrage | **amplitude brute**, propre à un ajustement — 2,6 sur le tableau d'exploration de l'ADR 0001 §1.3. Jamais « amplitude » nu : le mot sans qualificatif a déjà désigné les deux | — |
| Enquête d'experts | **CHES**, échelle 0–10, variable `lrgen`, **vague** 2024 (jamais « édition ») | enquête Chapel Hill, vague 2024 |
| Classification administrative | **nuance administrative**, issue du **nuancier du ministère de l'Intérieur** | code de nuance du ministère de l'Intérieur |
| Dispersion d'un groupe | **dispersion intra-groupe** = {écart interquartile (IQR), écart-type de rééchantillonnage} | écart entre les membres du groupe *(avec les deux chiffres)* |
| Critère de publication de l'axe | **critère de séparation des axes** (`s2/s1`) | — |
| Ce qui autorise la publication d'un groupe | **conditions de publication** — les trois de positionnement.md §6 (IQR, écart-type de rééchantillonnage, effectif). Jamais « seuil de variance expliquée » | — |
| Cycle de vie d'un groupe | **créé le …**, **actif du … au …**, **dissous le …**. Jamais « naît », « vit », « meurt » | créé le …, dissous le … |
| Brique 1, à venir | **espace de citation** | espace de citation |

**Divergence assumée avec les documents fondateurs.** ROADMAP.md v0.2 et
methode.md §2 écrivent « variance intra-groupe » et « part de variance
expliquée ». Ces deux formes sont remplacées ici et dans `docs/brique0/` parce
que positionnement.md §6 les a mesurées non publiables telles quelles ; le
remplacement est épinglé par le test AGR-03 et listé en plan-de-tests.md §16.
Les deux fichiers fondateurs ne sont pas modifiés par une passe de rédaction :
ils le sont par la PR de méthode qui portera le changement
(definition-of-done.md §19).

### Entités

| Concept | Forme canonique | Forme d'écran |
|---|---|---|
| Personne morale durable | **parti** (`parti.*`) | parti |
| Alliance électorale datée | **coalition** (`coalition.*`) | coalition |
| Organe d'une chambre pour une législature | **groupe parlementaire** (`groupe.an17.*`) | groupe parlementaire |
| Sigle d'un groupe | `libelleAbrev` de la source, **en capitales** : NI, AD, DEM, DR, ECOS, EPR, GDR, HOR, LFI-NFP, LIOT, RN, SOC, UDR, UDDPLR | le sigle en capitales, le nom complet à côté |
| Législature | **XVIIe législature** en prose, `17` en donnée | XVIIe législature |

`libelleAbrege` (`Dem`, `EcoS`, `UDR` pour `PO872880`) n'est jamais employé comme
sigle ni comme clé : il diverge de `libelleAbrev` sur trois groupes.

### Codage des votes

| Concept | Forme canonique | Forme d'écran |
|---|---|---|
| Position exprimée | **pour** (+1), **contre** (−1), **abstention** (0) | pour, contre, abstention |
| Empêchement institutionnel | **non-votant**, causes `MG`, `PAN`, `PSE` — **donnée manquante**, jamais une position | n'a pas pu voter *(jamais compté comme une position)* |
| Député hors des blocs | **absence** — **donnée manquante** | — |
| Cellule écrite / non écrite | **cellule observée** / **cellule absente**. Jamais « valeur manquante imputée », jamais « masque » | — |
| Déclaration postérieure au vote | **mise au point de vote** — ingérée, jamais appliquée | — |
| Seul filtre de scrutins | **minorité non vide** (`min(pour, contre) ≥ 1`). Ce n'est pas un seuil de participation ; aucun seuil de participation n'existe | scrutins où au moins un député a voté dans chaque sens |

### Artefacts

| Concept | Forme canonique | Forme d'écran |
|---|---|---|
| `data/preuves/positions.jsonl` | **registre de preuves**, une **ligne de preuve** par valeur | — |
| `data/registre/partis.json` | **registre d'entités** | — |
| `public/api/index.json` | **manifeste** | — |
| `public/api/instantanes/<id>.json` | **instantané** | — |
| `public/api/preuves/<xx>.json` | **éclat de preuves** | d'où vient ce chiffre |
| Empreinte d'un fichier | **empreinte SHA-256** | empreinte SHA-256 |
| Bornes de temps | **période de validité**, bornes incluses | du … au … |
| Contrat versionné | **contrat de sortie** (majeure / mineure / patch, ADR 0000 §6) | — |

### Licence

Deux objets distincts, jamais confondus :

| Objet | Forme canonique | Forme d'écran |
|---|---|---|
| Amont Assemblée nationale | **Licence Ouverte / Open Licence 1.0**, producteur : Assemblée nationale | Licence Ouverte / Open Licence 1.0, Assemblée nationale |
| Sorties du projet — code | **AGPL-3.0-only** | AGPL-3.0-only |
| Sorties du projet — données, registres, documentation | **Licence Ouverte 2.0** (`etalab-2.0`) | Licence Ouverte 2.0 |
| Nuancier via data.gouv.fr | **Licence Ouverte 2.0** (`license = lov2`) | Licence Ouverte 2.0 |

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
| Législature | `XVIIe` en prose, `17` en donnée | XVIIe législature |
| Octets | `o` pour octet, `Mio` pour mébioctet, `Mo` pour mégaoctet — les trois distingués |

---

## 7. Contrôles opposables

Sur le diff d'une PR, avant de la déclarer finie (definition-of-done.md §11
et §13) :

```sh
# 1. Lexique interdit — la liste vit dans scripts/lexique.sh et nulle part
#    ailleurs (definition-of-done.md §11). Une copie inline de plus est une
#    copie de plus qui divergera : la quatrième refusait « biais de sélection »,
#    terme retenu du projet, et signalait « synthèse » dans methode.md §9,
#    « syntheseVote », « fiche de synthèse n° 56 » et « indice de ligne ».
./scripts/lexique.sh docs
./scripts/lexique.sh code

# 1 bis. T7 en régime public — aucun code d'absence à l'écran. La liste des
#    codes est dérivée de schemas/preuve-2.schema.json, jamais recopiée.
./scripts/lexique.sh public

# 2. Personne grammaticale (T2). Même raison qu'au contrôle 1 : le motif et la
#    portée vivent dans le script. Il exclut les chemins du régime public, où
#    §1 bis autorise la deuxième personne, et n'examine que la prose et le
#    commentaire de code — `on` attrapait la clé `on:` des workflows et le
#    « on » anglais des citations académiques des artefacts de preuve.
./scripts/lexique.sh personne

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

Les contrôles 1, 1 bis et 2 **sont** l'intégration continue : c'est le script
qu'elle lance, pas une transcription. Les contrôles 4 et 6 sont automatisables à
leur tour ; le contrôle 2 produit encore des faux positifs sur les citations, et
le contrôle 3 se relit.
Un faux positif se justifie dans la description de la PR, il ne se supprime pas
du grep.

---

## 8. Ce que ce document ne tranche pas

- Le contenu des décisions : il vit dans les ADR.
- Les seuils numériques : ils vivent dans `docs/brique0/`.
- La forme canonique des fichiers de données : elle vit dans
  [brique0/contrats.md](brique0/contrats.md) §7 et
  [brique0/registre-entites.md](brique0/registre-entites.md) §6 règle V23.
- Le nombre d'occurrences de `vous` par écran : §1 bis le plafonne à une, et
  aucun contrôle ne le mesure — un écran n'est pas un fichier. C'est une règle
  relue, pas une porte, et ce document le dit plutôt que de le laisser croire.
- Le fait qu'un bloc explicatif explique sans juger (T1 en régime public) :
  aucun grep ne distingue une glose d'un jugement. Relu, pas contrôlé.

Un désaccord entre ce document et un ADR se règle en faveur de l'ADR, et ce
document est corrigé dans la même PR.
