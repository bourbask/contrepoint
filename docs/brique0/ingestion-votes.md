# Ingestion des scrutins de l'Assemblée nationale

Périmètre : v0.1 de la brique 0, **XVIIe législature seule** (ADR 0000 §3).
Toutes les mesures de ce document ont été relevées le **2026-08-27** sur les
archives effectivement téléchargées, et sont reproductibles par les commandes
citées. Aucun nom de champ n'est deviné : chaque chemin JSON donné a été observé.

---

## 1. Les jeux de données

| Jeu | URL | Format | Taille | Contenu |
|---|---|---|---|---|
| Scrutins | `https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip` | ZIP de 8 434 fichiers JSON, un par scrutin | 26 317 479 o → 181 Mio | Position nominative de chaque député sur chaque scrutin public |
| Scrutins (variante) | `.../17/loi/scrutins/Scrutins.xml.zip` | ZIP XML | 30 795 168 o | Même contenu, arité portée par le schéma (§4) |
| Référentiel acteurs, mandats, organes | `https://data.assemblee-nationale.fr/static/openData/repository/17/amo/tous_acteurs_mandats_organes_xi_legislature/AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip` | ZIP, 3 119 acteurs + 10 813 organes + 59 déports | 13 600 736 o | Groupes politiques et mandats avec périodes de validité |

Fiche source : `https://data.assemblee-nationale.fr/travaux-parlementaires/votes`.

**Licence : Licence Ouverte / Open Licence version 1.0.** Vérifié en ouvrant
`https://data.assemblee-nationale.fr/content/download/28755/file/Licence_Ouverte.pdf`,
qui porte en clair « Cette licence est une version 1.0 de la Licence Ouverte ».
La page HTML n'affiche pas le numéro ; le PDF, si. Obligation exacte : *« Mentionner
la paternité de l'Information : sa source (a minima le nom du Producteur) et la
date de sa dernière mise à jour. »* Producteur : Assemblée nationale. La mention
retenue est donc `Assemblée nationale — Licence Ouverte v1.0 — données du <date
last-modified de l'archive>`. Ceci referme le `A VERIFIER` de l'ADR 0000 §4.

**Fréquence de mise à jour : quotidienne.** `last-modified` de l'archive
scrutins : `2026-08-27 04:25:40 GMT` ; le champ `date` de la fiche source dit
`2026-08-27 06:25:24` (heure de Paris). AMO30 : `2026-08-27 00:34:47 GMT`. Les
deux jeux sont reconstruits chaque nuit, à des heures différentes.

**La fiche source publie un MD5 par ressource.** Pour l'archive JSON :
`1f951dea5675556c5b675e5bdfeddba5`. Le fichier téléchargé le donne exactement.
C'est la réponse au piège de troncature relevé en ADR 0001 §1.5 : l'intégrité se
vérifie contre l'empreinte du producteur, pas seulement contre `content-length`.
AMO30 n'a pas de fiche équivalente ; pour lui, seul `content-length` et la reprise
`curl -C -` sont disponibles.

**Piège de libellé, confirmé.** La fiche du jeu 17 décrit son contenu comme
« Les scrutins AN […] pour la XV législature ». Le libellé est faux ; le chemin
`/repository/17/` et le champ `scrutin.legislature = "17"` de chaque fichier sont
autoritatifs. Ne jamais dériver le numéro de législature d'un libellé humain.

---

## 2. Le schéma réel d'un scrutin

Un fichier = un objet racine `scrutin`. Les valeurs numériques sont **toutes**
sérialisées en chaînes (`"nombreVotants": "163"`).

```
scrutin.uid                              "VTANR5L17V156"
scrutin.numero                           "156"          (1 → 8434, aucun trou)
scrutin.legislature                      "17"
scrutin.dateScrutin                      "2024-10-26"   (seule date, pas d'heure)
scrutin.organeRef                        "PO838901"     (l'Assemblée)
scrutin.sessionRef / seanceRef            SCR5A2025O1 / RUANR5L17S2025IDS28689
scrutin.lieuVote                         "Hémicycle" (8 409) | "Salons" (25)
scrutin.typeVote.codeTypeVote            "SPO" (8 339) | "SPS" (72) | "MOC" (23)
scrutin.sort.code                        "rejeté" (5 585) | "adopté" (2 849)
scrutin.titre / objet.libelle            texte libre
scrutin.objet.dossierLegislatif.dossierRef   présent dans 2 608 fichiers, sinon null
scrutin.modePublicationDesVotes          "DecompteNominatif" (8 434 / 8 434)
scrutin.syntheseVote.nombreVotants       "163"
scrutin.syntheseVote.suffragesExprimes   "159"
scrutin.syntheseVote.decompte.{nonVotants,pour,contre,abstentions,nonVotantsVolontaires}
scrutin.ventilationVotes.organe.groupes.groupe[]     un bloc par groupe politique
        .organeRef                       "PO845401"
        .nombreMembresGroupe             "125"
        .vote.positionMajoritaire        "pour"
        .vote.decompteVoix.{nonVotants,pour,contre,abstentions,nonVotantsVolontaires}
        .vote.decompteNominatif.pours.votant[]
        .vote.decompteNominatif.contres.votant[]
        .vote.decompteNominatif.abstentions.votant[]
        .vote.decompteNominatif.nonVotants.votant[]
scrutin.miseAuPoint.{pours,contres,abstentions,nonVotants,nonVotantsVolontaires}
scrutin.miseAuPoint.dysfonctionnement.{pour,contre,abstentions,nonVotants,nonVotantsVolontaires}
```

**La position d'un député n'est pas un champ : c'est le nom du bloc qui le
contient.** Il n'existe aucune valeur `"pour"` / `"contre"` attachée au votant.
Un votant est :

```json
{ "acteurRef": "PA793238", "mandatRef": "PM842426",
  "parDelegation": "true", "numPlace": "073" }
```

et sa position est celle du tableau `pours` / `contres` / `abstentions` /
`nonVotants` où il figure. Un votant de `nonVotants` porte un champ de plus :

```json
{ "acteurRef": "PA721908", "mandatRef": "PM843467", "parDelegation": "false",
  "numPlace": "402", "causePositionVote": "PAN" }
```

`causePositionVote` **n'apparaît que dans `nonVotants`** : 23 383 occurrences,
zéro dans les trois autres blocs sur 1 247 093 cellules. C'est le seul champ
d'énumération de tout le jeu.

Extrait réel dans `echantillons/VTANR5L17V156.json` (verbatim, non tronqué).

### Cohérences vérifiées sur les 8 434 fichiers

| Invariant | Résultat |
|---|---|
| `nombreVotants = pour + contre + abstentions` | vrai 8 434 / 8 434 |
| `suffragesExprimes = pour + contre` | vrai 8 434 / 8 434 |
| longueur de la liste nominative = `decompteVoix` correspondant | vrai pour les 4 positions, 101 208 blocs / 101 208 |
| somme des `nonVotants` nominatifs = somme des `decompteVoix.nonVotants` | 23 383 = 23 383 |
| un acteur apparaît au plus une fois par scrutin | vrai — **zéro** doublon sur 1 270 476 cellules |

Conséquence utile : **l'abstention compte dans `nombreVotants`, le non-votant
non.** `nombreVotants` est donc directement un compte de participation exprimée.

### Les groupes

Le groupe apparaît deux fois, et il faut choisir.

1. Dans le scrutin : `ventilationVotes.organe.groupes.groupe[].organeRef`. Daté
   par construction — c'est le groupe du député **le jour du vote**, tel que
   l'Assemblée l'a publié. 13 identifiants distincts observés sur la législature.
2. Dans AMO30 : mandats `typeOrgane = "GP"`, `legislature = "17"`, avec
   `dateDebut` / `dateFin`. 648 députés en portent au moins un.

**Le rattachement retenu est celui du scrutin** (§7). AMO30 sert au libellé et
aux périodes de validité des groupes, et de recours pour les 146 blocs `PO0`.

Les 14 groupes politiques de la législature — 13 référencés par au moins un scrutin, plus AD qui n'apparaît dans aucun
(`echantillons/organes-groupes-l17.json`) :

| `uid` | `libelleAbrev` | `viMoDe.dateDebut` | `viMoDe.dateFin` |
|---|---|---|---|
| PO840056 | NI | 2024-07-01 | — |
| PO845520 | AD | 2024-07-18 | 2024-09-11 |
| PO845454 | DEM | 2024-07-18 | — |
| PO845425 | DR | 2024-07-18 | — |
| PO845439 | ECOS | 2024-07-18 | — |
| PO845407 | EPR | 2024-07-18 | — |
| PO845514 | GDR | 2024-07-18 | — |
| PO845470 | HOR | 2024-07-18 | — |
| PO845413 | LFI-NFP | 2024-07-18 | — |
| PO845485 | LIOT | 2024-07-18 | — |
| PO845401 | RN | 2024-07-18 | — |
| PO845419 | SOC | 2024-07-18 | — |
| PO847173 | UDR | 2024-09-12 | 2025-09-04 |
| PO872880 | UDDPLR | 2025-09-05 | — |

`positionPolitique` est `null` pour les 14 : aucune étiquette d'orientation ne
vient de la source. C'est cohérent avec l'exigence que l'axe émerge des votes.
`couleurAssociee` est renseigné (ex. `#313567` pour PO845401) et n'est pas
utilisé : une couleur de groupe est une convention éditoriale de l'Assemblée.

---

## 3. Le piège des non-votants

C'est le point où une erreur de codage fabrique un centrisme artificiel. Les
catégories présentes dans les données, et elles seules :

| Catégorie | Où elle se lit | Occurrences | Nature |
|---|---|---|---|
| Pour | bloc `pours` | 555 568 | position exprimée |
| Contre | bloc `contres` | 620 352 | position exprimée |
| Abstention | bloc `abstentions` | 71 173 | position exprimée |
| Non-votant, cause `MG` | bloc `nonVotants` | 9 991 | empêchement institutionnel |
| Non-votant, cause `PAN` | bloc `nonVotants` | 7 508 | empêchement institutionnel |
| Non-votant, cause `PSE` | bloc `nonVotants` | 5 884 | empêchement institutionnel |
| Absence | **nulle part** | 4 144 152 cellules | donnée manquante |

### Ce que valent les trois causes

Les trois codes ne sont documentés par aucune liste publiée sur la fiche source.
Ils ont été décodés **en croisant les acteurs concernés avec leurs mandats
AMO30**, pas en développant l'acronyme :

| Code | Acteurs distincts | Mandat porté par tous, sur la période concernée | Lecture |
|---|---|---|---|
| `PAN` | **1** (PA721908) | `BUREAU` / `codeQualite = "Président"`, 2024-07-18 → en cours | Président de l'Assemblée nationale |
| `PSE` | **11** | `BUREAU` / `"Président"` ou `"Vice-Président"` | Président de séance |
| `MG` | **39** | `GOUVERNEMENT` / `"membre"` couvrant les dates | Membre du Gouvernement |

`A VERIFIER` : ces trois libellés sont **inférés des données**, pas lus dans une
énumération officielle. Confirmation possible en ouvrant le schéma XSD du
référentiel (`http://schemas.assemblee-nationale.fr/referentiel`) ou en
interrogeant le service de données de l'Assemblée. Non bloquant : le codage
retenu ne dépend pas du libellé, seulement du fait que les trois causes sont des
empêchements et non des positions — ce que le croisement des mandats établit.

### Les catégories qui n'existent pas dans les données

- **Il n'y a pas de non-participation volontaire nominative.** Le champ
  `decompteVoix.nonVotantsVolontaires` existe, et c'est un leurre : il vaut
  **exactement `decompteVoix.abstentions` dans les 101 208 blocs de groupe**, et
  `"0"` dans les 8 434 `syntheseVote.decompte`. Ce n'est pas une cinquième
  catégorie, c'est un doublon du compte d'abstentions. Le lire comme une
  catégorie double-compte les abstentions et invente une position qui n'est pas
  dans la source. Aucune liste nominative `nonVotantsVolontaires` n'existe sous
  `decompteNominatif` — seulement sous `miseAuPoint`, une fois sur 8 434.
- **Il n'y a pas de code d'absence.** Un député absent ne produit aucune ligne.
  L'absence se déduit d'une différence d'ensembles, jamais d'une lecture.
- **La délégation de vote n'est pas un empêchement.** `parDelegation = "true"`
  sur 191 629 cellules, soit 15,4 % des cellules exprimées, répartie sur les trois positions exprimées
  (`pour` 85 723, `contre` 95 236, `abstention` 10 669) et sur **une** cellule de
  `nonVotants`. Un vote par délégation est enregistré sous l'`acteurRef` du
  délégant, avec son `mandatRef` : c'est sa position, exprimée en son nom. Elle
  est codée comme les autres.

### Coût mesuré de l'erreur de codage

Le même estimateur (rang 1 sur cellules observées, constante par scrutin, signe
ancré sur LFI-NFP) a été ajusté deux fois sur les 7 979 scrutins retenus : une
fois avec l'absence et le non-votant traités en données manquantes, une fois avec
le non-votant codé `0` comme une abstention.

Les cinq lignes les plus déplacées, anonymisées : ni `acteurRef`, ni nom, ni
fonction — la position individuelle ne se rattache à aucun identifiant, y
compris dans un exemple (ADR 0000 §2, ton.md §5). Le groupe, le volume de
non-votants et la cause suffisent à l'argument.

| Rang | Groupe | Non-votants | Cause | x, absence manquante | x, absence = abstention | Écart |
|---|---|---|---|---|---|---|
| 1 | EPR | 8 341 | PAN 7 508, PSE 833 | −0,833 | −0,116 | **+0,717** |
| 2 | HOR | 915 | MG 639, PSE 276 | +0,635 | +0,099 | −0,536 |
| 3 | RN | 776 | PSE 776 | +1,125 | +0,544 | **−0,581** |
| 4 | EPR | 1 304 | PSE 665, MG 639 | +0,234 | −0,127 | −0,360 |
| 5 | EPR | 644 | MG 644 | +0,153 | −0,150 | −0,303 |

Sur une amplitude inter-groupes de 2,6, le mauvais codage déplace la ligne la
plus touchée de **0,72**, soit 28 % de l'axe, et ramène la troisième à mi-chemin
du centre. Le mécanisme est celui annoncé dans `methode.md` : les zéros s'accumulent chez les députés
institutionnellement empêchés de voter, et les tirent vers le milieu.

**Et l'erreur ne se voit pas au niveau du groupe.** Corrélation des deux
solutions : **0,9969**. Les moyennes de groupe bougent de moins de 0,04. En
revanche l'écart-type intra-groupe de DR passe de **0,151 à 0,227** (+50 %) —
c'est-à-dire que le seul chiffre que la roadmap exige de publier pour rendre la
limite de la méthode visible est le premier corrompu. Une erreur invisible sur
la sortie affichée et visible sur l'indicateur de qualité : elle passerait une
relecture. D'où l'exigence d'un test dédié (`definition-of-done.md` §9).

### Les mises au point : ingérées, jamais appliquées

`scrutin.miseAuPoint` recense les déclarations postérieures au scrutin par
lesquelles un député indique que le vote enregistré à son nom n'était pas celui
qu'il voulait. **1 442 scrutins sur 8 434 (17,1 %) en portent au moins une**, pour
3 043 entrées nominatives :

| Vote de la machine | Mise au point déclarée | Entrées |
|---|---|---|
| pour | contre | 745 |
| contre | pour | 716 |
| **aucun** | pour | 404 |
| **aucun** | contre | 342 |
| contre | non-votant | 221 |
| pour | non-votant | 167 |
| pour / contre | abstention | 233 |
| abstention | pour / contre | 148 |
| autres | | 67 |

**Décision : la matrice enregistre le vote de la machine, jamais la mise au
point.** Deux raisons, aucune de commodité. D'abord l'Assemblée elle-même ne
modifie pas le résultat du scrutin : `sort.code` et `syntheseVote` restent ceux
du vote enregistré, et appliquer la mise au point produirait une matrice
incohérente avec la source qu'elle prétend reproduire. Ensuite 746 entrées
concernent un député **qui n'a pas voté du tout** : les appliquer comblerait une
absence par une intention déclarée, ce que `methode.md` interdit explicitement.
La mise au point relève du déclaratif — la deuxième famille de mesure, pas la
première.

Elle est néanmoins ingérée et conservée comme attribut du scrutin, pour que le
choix soit auditable et l'écart chiffrable par un tiers.

---

## 4. Trois pièges de parsing

Le JSON est une transcription automatique de XML ; l'arité et le type sont
perdus. Comptages sur les 8 434 fichiers.

**a. Un tableau à un élément devient un objet nu.** `decompteNominatif.*.votant`
est un tableau dans 95 180 blocs, un objet nu dans **35 671** (27,3 %). Un
désérialiseur typé sur `Vec<Votant>` échoue sur plus d'un quart du corpus, et
échoue préférentiellement là où un groupe n'a qu'un seul votant dans une
position — donc sur les scrutins les plus serrés. Fixture :
`echantillons/VTANR5L17V5268.json`.

**b. Le même niveau de schéma, deux sérialisations.** Dans `miseAuPoint` :

| Champ | Forme observée |
|---|---|
| `pours`, `contres` | `null` (7 772 / 7 751) ou objet nu (662 / 683) — **jamais** un tableau |
| `abstentions`, `nonVotants`, `nonVotantsVolontaires` | **toujours** un tableau, dont les éléments sont `null` quand il n'y a rien (8 270 / 8 042 / 8 433) |

Un tableau `[null, null]` est la forme vide de trois champs et la forme
impossible des deux autres. L'adaptateur « un-ou-plusieurs » doit donc aussi
filtrer les éléments nuls, pas seulement envelopper les scalaires.

**c. `uid` est parfois une chaîne, parfois un objet.** `organe.uid` est
`"PO845401"`. `acteur.uid` dans AMO30 est
`{"@xmlns:xsi": …, "@xsi:type": "IdActeur_type", "#text": "PA721908"}`. Il faut
un adaptateur « chaîne ou objet enveloppé xsi ».

La variante XML ne souffre pas de (a) ni (b) par construction. `A VERIFIER` :
parser `Scrutins.xml.zip` et compter les cas particuliers restants. Non bloquant,
les trois adaptateurs sont écrits une fois.

---

## 5. Le codage retenu

| Bloc source | Valeur dans la matrice |
|---|---|
| `pours.votant[]` | **+1** |
| `contres.votant[]` | **−1** |
| `abstentions.votant[]` | **0** |
| `nonVotants.votant[]`, quelle que soit la cause | **manquant** |
| acteur absent de tous les blocs du scrutin | **manquant** |

Une seule valeur manquante, deux origines, aucune distinction. Le pipeline n'a
pas de « code d'absence » : il produit une liste de triplets
`(acteurRef, uid_scrutin, valeur)` et rien d'autre. Une cellule non émise n'a pas
de valeur à mal interpréter — c'est ce qui rend l'invariant « absent ≠
abstention » structurel plutôt que défensif.

**Traitement des données manquantes : aucun.** Pas d'imputation, pas de moyenne
de ligne, pas de zéro de remplissage. `0` est réservé à l'abstention observée, et
l'estimateur travaille sur les cellules observées seulement — contrainte
d'architecture actée en ADR 0001 §1.2. La densité de la matrice retenue est de
**23,2 %** ; les 76,8 % restants ne sont pas des zéros et ne le deviennent nulle
part dans le code.

`0` pour l'abstention et `manquant` pour l'absence sont deux choses que rien ne
distingue une fois la matrice écrite. C'est pourquoi la valeur manquante est une
**cellule absente** et non un `0.0` accompagné d'un masque : un masque se perd,
une ligne non écrite ne se perd pas.

---

## 6. Le filtre de participation

### Ce que disent les données de participation

Distribution de `syntheseVote.nombreVotants` sur 8 434 scrutins, pour 577 sièges :

| min | p10 | p25 | médiane | p75 | p90 | max | moyenne |
|---|---|---|---|---|---|---|---|
| 16 | 59 | 89 | **133** | 192 | 246 | 574 | 147,9 |

La médiane est à 23 % des sièges. Le corpus s'effondre dès que le
seuil est relevé : ≥ 100 → 68,6 % ; ≥ 150 → 42,5 % ; ≥ 200 → 22,7 % ; ≥ 300 → 4,3 %.

### Le seuil ne peut pas être justifié par une rupture, parce qu'il n'y en a pas

L'objection de la roadmap est que « sans seuil, l'axe mesure surtout qui était
présent ». Elle est vérifiable. Pour chaque scrutin est calculée la **distance de
composition** : la moitié de la somme des écarts absolus entre la part de chaque
groupe parmi les votants et sa part de l'effectif déclaré
(`nombreMembresGroupe`, somme médiane = 577). Zéro = les votants sont un
échantillon exactement proportionnel de l'Assemblée.

| Tranche de `nombreVotants` | n | Distance médiane | p90 |
|---|---|---|---|
| [0, 50) | 390 | 0,258 | 0,419 |
| [50, 80) | 1 256 | 0,221 | 0,337 |
| [80, 100) | 999 | 0,198 | 0,277 |
| [100, 120) | 988 | 0,196 | 0,272 |
| [120, 150) | 1 220 | 0,183 | 0,274 |
| [150, 200) | 1 669 | 0,186 | 0,290 |
| [200, 250) | 1 121 | 0,149 | 0,242 |
| [250, 300) | 432 | 0,131 | 0,183 |
| [300, 577] | 359 | 0,091 | 0,158 |

La distorsion est réelle et **strictement monotone, sans coude**. Elle ne
descend jamais sous 0,09 : même les scrutins les plus suivis ne sont pas des
échantillons proportionnels. Il n'existe donc aucune valeur que les données
désignent. Tout seuil de participation serait un arbitrage déguisé en mesure.

### Ce que le seuil coûte, mesuré

L'axe a été ajusté sur sept corpus. Corrélation de Pearson des coordonnées
individuelles avec le corpus de référence (minorité non vide, aucun seuil de
participation) :

| Corpus | Scrutins | Corrélation | Ordre des groupes |
|---|---|---|---|
| aucun filtre | 8 434 | 1,0000 | identique |
| minorité non vide | 7 979 | référence | référence |
| \+ `nombreVotants` ≥ 50 | 7 635 | 1,0000 | identique |
| \+ ≥ 100 | 5 535 | 0,9996 | DEM et NI permutent, à +0,10 tous les deux |
| \+ ≥ 150 | 3 451 | 0,9979 | idem |
| \+ ≥ 200 | 1 846 | 0,9923 | idem |
| \+ ≥ 300 | 339 | **0,9497** | **RN passe devant DR ; LIOT passe à droite du centre** |

Le seuil de participation n'améliore rien et, poussé, **dégrade**. À ≥ 300, DR
(+0,82) et RN (+0,79) se confondent et LIOT traverse le zéro : sur 339 scrutins,
l'axe cesse de séparer les blocs de droite. Le corpus complet, lui, produit un
ordre stable et un écart DR / RN de 0,41.

### Décision

**Aucun seuil de participation.** Un seul filtre, et il n'est pas un seuil mais
une définition : **un scrutin sans minorité enregistrée n'entre pas dans la
matrice**, parce que sa variance est nulle et qu'il ne contribue à aucun axe.
Condition : `min(decompte.pour, decompte.contre) ≥ 1`.

`nombreVotants` est **publié par scrutin dans le registre de preuves** et affiché
avec le décompte, au lieu de servir de porte. La roadmap demandait un seuil
documenté ; la mesure dit que le seuil justifiable est l'absence de seuil, et
c'est cela qui est documenté.

---

## 7. Décompte des scrutins retenus et écartés

Sortie visible de la v0.1.

| | Scrutins | Part |
|---|---|---|
| Total, XVIIe législature, du 2024-10-08 au 2026-07-21 | **8 434** | 100 % |
| Écartés — minorité vide, scrutin public | 432 | 5,1 % |
| Écartés — minorité vide, motion de censure | 23 | 0,3 % |
| **Retenus** | **7 979** | **94,6 %** |

Les 23 motions de censure sont écartées **en totalité**, et par construction :
l'article 49 alinéa 2 de la Constitution ne fait voter que les députés favorables
à la censure, donc `contre = 0` dans les 23 fichiers. Les votes les plus visibles
de la législature ne portent aucune information de position gauche-droite, parce
que l'institution n'enregistre qu'un seul camp. C'est à dire, pas à découvrir
plus tard.

Matrice résultante :

| | Valeur |
|---|---|
| Scrutins | 7 979 |
| Députés avec au moins une position exprimée | **641** (642 sur le corpus complet — un député dont l'unique vote était dans un scrutin unanime) |
| Cellules observées | 1 188 035 |
| Densité | **23,2 %** |
| Cellules `nonVotants`, jamais codées | 23 383 |
| Positions par député : médiane | 1 801 |
| p10 / p90 | 430 / 3 388 |
| Députés sous 10 positions | 3 |
| Députés sous 100 positions | 16 |

**7 979 scrutins et 1,19 million de cellules suffisent à un estimateur de rang 1.** La conclusion « corpus trop faible » ne s'applique pas ici. La
limite réelle est ailleurs : l'axe explique **60,8 % du résidu** après constante
par scrutin (recompté le 2026-08-27, voir
[verification-2026-08-27.md](verification-2026-08-27.md)), mais il l'explique en
séparant les blocs et non les individus. Le corpus n'est pas le facteur
limitant ; la discipline de vote l'est.

Les 3 députés sous 10 positions exprimées (2, 4 et 8 votes) reçoivent une
coordonnée dénuée de sens. Elle n'est pas publiée — ADR 0000 §2 l'interdit pour
tous — mais elle entre dans la moyenne de leur groupe. `A VERIFIER` : mesurer
l'effet d'un plancher par député sur la moyenne et l'écart-type des groupes
concernés, avant de décider d'en poser un. C'est une décision de la v0.2, pas de
l'ingestion.

---

## 8. Rattachement député → groupe, avec périodes de validité

### Où est la donnée, et laquelle vaut

**Le groupe retenu est celui du bloc `ventilationVotes` du scrutin lui-même.**
Trois raisons, toutes vérifiables sur le fichier :

- il est daté par construction — le bloc est dans le fichier du scrutin, dont
  `dateScrutin` est la date ;
- il ne demande aucune jointure, donc aucune fenêtre de validité à interpréter ;
- il est ce que l'Assemblée a publié ce jour-là.

AMO30 sert à trois choses : le libellé et la période de validité du groupe, la
résolution des blocs `PO0`, et le contrôle croisé.

### Ce que la jointure par mandat coûterait, mesuré

Index construit sur les mandats AMO30 `typeOrgane = "GP"`, `legislature = "17"`,
puis comparé au groupe du bloc pour les 1 270 476 cellules :

| Comparaison | Cellules |
|---|---|
| Identique | 1 250 505 (98,4 %) |
| **Plusieurs mandats GP valides à la date** | **17 716 (1,4 %)** |
| **Groupe différent** | **2 255 (0,2 %)** |

Les 2 255 désaccords vont tous dans le même sens : AMO30 dit encore `NI`
(PO840056) là où la ventilation dit un groupe constitué. Exemple réel :
PA642725, scrutin `VTANR5L17V10` du 2024-10-22 — ventilation `PO845425` (DR),
mandat AMO30 `PO840056` (NI). Le mandat de non-inscrit porte une `dateFin` en
retard sur la constitution du groupe. Une jointure par mandat classerait ces
votes chez les non-inscrits.

Les 17 716 cellules ambiguës viennent de **18 chevauchements de périodes** sur
648 députés. Ils sont de deux natures :

- des **mandats dupliqués à périodes identiques** — PA267285 porte deux fois
  `PO845425` du 2024-07-19 à `null` ;
- de vraies `dateFin` concurrentes — PA642868 porte `PO845470` du 2025-01-25 au
  2026-03-31 et `PO845470` du 2025-01-25 au 2026-04-06.

Règle de dédoublonnage, appliquée quand AMO30 est consulté : regrouper par
`(organeRef, dateDebut)` et retenir la `dateFin` maximale, `null` valant
« en cours ». Après cela, aucun chevauchement ne subsiste entre deux
`organeRef` différents. Fixture : `echantillons/mandats-gp-l17.json`.

### `votant.mandatRef` ne donne pas le groupe

Piège coûteux : le votant porte un `mandatRef`, ce qui donne l'impression que la
jointure est là.
Vérifié sur les 1 270 476 cellules — `mandatRef` pointe **toujours** un mandat
`typeOrgane = "ASSEMBLEE"`, `legislature = "17"`, c'est-à-dire le mandat de
député, jamais le mandat de groupe. 1 270 476 / 1 270 476, aucune exception.
`mandatRef` sert à identifier le siège, pas l'appartenance.

### La référence pendante `PO0`

**146 blocs de groupe portent `organeRef = "PO0"`.** Aucun fichier
`organe/PO0.json` n'existe dans AMO30 : la référence est pendante. Elle se
concentre sur **14 scrutins** (0,17 %), dont 13 du seul 2024-12-02 et
`VTANR5L17V6256` du 2026-04-16 : dans ces 14 fichiers, **tous** les blocs de
groupe ont perdu leur `organeRef`, pas seulement un.

Traitement : pour un bloc `PO0`, le groupe est résolu par le mandat GP AMO30 des
votants du bloc à la date du scrutin. Vérifié sur les 146 blocs — la résolution
est **unanime dans chaque bloc** (tous les votants d'un bloc relèvent du même
groupe), et 10 blocs sont vides de tout votant, donc sans objet. Aucun bloc
ambigu. Fixture : `echantillons/VTANR5L17V6256.json`.

Un bloc `PO0` non résolu est une erreur bloquante du pipeline, jamais un groupe
« inconnu » qui remonterait dans une agrégation.

### La fenêtre où le rattachement serait à risque n'existe pas ici

633 députés sur 648 relèvent de plus d'un groupe sur la législature — presque
toujours le passage de non-inscrit au groupe constitué le 2024-07-18 ou le
2024-07-19. Une jointure « dernier groupe connu » attribuerait aux non-inscrits
tout vote antérieur au 18 juillet 2024.

Or **le premier scrutin de la XVIIe législature est daté du 2024-10-08**
(`VTANR5L17V1`, motion de censure), et les numéros vont de 1 à 8 434 sans trou.
Aucun scrutin n'a eu lieu pendant la fenêtre de non-inscription. Le risque est
réel dans le modèle, nul dans les données de la v0 — ce qui n'autorise pas à
retirer les périodes de validité du modèle : la XVIe législature, premier lot
d'extension, n'a pas la même chance.

### Agrégation au groupe

Les groupes n'ont pas la même durée de vie que la législature : UDR est actif du
2024-09-12 au 2025-09-04, UDDPLR est créé le 2025-09-05, AD est dissous avant le
premier scrutin. Une agrégation « dernier groupe observé » laisse UDR avec
**1 membre** — celui dont le dernier vote tombe avant la dissolution — et fabrique
un groupe qui ne correspond à aucun effectif réel.

Règle : l'agrégation est faite **à une date de référence explicite**, portée par
la ligne de preuve, et n'inclut que les groupes dont la période de validité
couvre cette date. Un groupe éteint à la date de référence n'est pas affiché
avec un effectif résiduel : il n'est pas affiché.

---

## 9. Cache et rejouabilité

Le même calcul doit redonner le même résultat hors ligne. Trois artefacts, trois
rôles.

**a. Le cache d'archives.** Une archive téléchargée est stockée sous
`<empreinte>/<nom>` avec un fichier voisin décrivant la récupération : URL,
`content-length` annoncé, octets reçus, `last-modified`, `etag`, MD5 publié par
le producteur quand il existe, SHA-256 calculé, horodatage de récupération.
Immuable. Le pipeline ne retélécharge jamais une archive dont l'empreinte est
déjà présente.

Obligations de téléchargement, imposées par les défauts constatés du serveur :

1. Reprise `Range` tant que les octets reçus diffèrent de `content-length` — le
   serveur ferme la connexion en cours de transfert sans erreur (ADR 0001 §1.5).
2. Comparaison au MD5 publié sur la fiche source lorsqu'il existe. Écart →
   arrêt, pas d'avertissement.
3. SHA-256 calculé et consigné dans le registre de preuves. Une archive dont
   l'empreinte est inconnue n'entre pas dans le pipeline.

**b. La matrice normalisée.** Sortie déterministe de l'ingestion : les triplets
`(acteurRef, uid_scrutin, valeur)` triés par `(uid_scrutin, acteurRef)` — ordre
lexicographique sur les identifiants, jamais l'ordre du système de fichiers, qui
a livré `VTANR5L17V5646` avant `VTANR5L17V2136` sur la machine de mesure. Plus
le décompte retenus / écartés avec motif, et par scrutin `dateScrutin`,
`nombreVotants`, `typeVote.codeTypeVote`, le drapeau de mise au point.

Cet artefact est une fonction pure des deux SHA-256 d'entrée et de la version du
code d'ingestion. Il porte les trois dans son en-tête, ce qui rend le cache
invalidable sans horloge.

**c. Ce qui n'entre jamais dans un calcul.** L'horloge. Les dates du pipeline
sont des données d'entrée : `dateScrutin` du fichier, `last-modified` de
l'archive, date de récupération consignée dans le cache. La date de calcul
apparaît dans la ligne de preuve, jamais dans une valeur calculée.

**d. Tests hors ligne.** Les fixtures de `echantillons/` couvrent les cinq
pièges (objet nu, `PO0`, minorité vide, mise au point, trois causes de
non-votant) et sont reconstructibles par
`echantillons/extraire-echantillons.py`. La suite passe machine débranchée : le
réseau n'est touché que par l'étape de téléchargement, qui est la seule à ne pas
avoir de test hors ligne et à devoir échouer bruyamment.

**e. Ce que la rejouabilité ne couvre pas.** Les archives de l'Assemblée sont
reconstruites chaque nuit et **rétroactivement modifiables** : une mise au point
enregistrée en 2026 apparaît dans le fichier d'un scrutin de 2024. Reproduire un
résultat de 2026 en 2027 exige l'archive de 2026, pas celle du jour. Le cache la
conserve ; le registre de preuves porte son SHA-256. Sans cela, « rejouable »
voudrait dire « recalculable sur des données qui ont changé », ce qui n'est pas
la même chose.

---

## Commandes de vérification

```sh
# archives et empreintes
curl -C - -O https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip
md5sum Scrutins.json.zip      # doit valoir le MD5 de la fiche source
sha256sum Scrutins.json.zip
unzip -q Scrutins.json.zip -d s17 && ls s17/json | wc -l   # 8434

curl -C - -O 'https://data.assemblee-nationale.fr/static/openData/repository/17/amo/tous_acteurs_mandats_organes_xi_legislature/AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip'

# licence : la version n'est que dans le PDF
curl -sL https://data.assemblee-nationale.fr/content/download/28755/file/Licence_Ouverte.pdf \
  | pdftotext - - | grep -i 'version 1.0'

# fixtures
python3 docs/brique0/echantillons/extraire-echantillons.py s17 amo30 docs/brique0/echantillons
```

Les comptages de ce document (formes de sérialisation, causes de non-votant,
distances de composition, corrélations entre corpus) ont été produits par des
scripts jetables sur les archives ci-dessus. Ils sont réécrits comme tests du
pipeline en v0.1, où ils cessent d'être jetables.
