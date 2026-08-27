# Règles de gestion

Une règle = une phrase vérifiable, un numéro stable, le document qui la fonde.
Ce document ne décide rien : il rassemble ce que les autres ont décidé, sous une
forme opposable.

**Usage.** Quand un test et une intention se contredisent, c'est cette liste qui
tranche. Si elle est muette, le conflit est une question de spécification et se
règle dans le document fondateur, pas dans le test
([tdd.md](tdd.md) §4, [plan-de-tests.md](brique0/plan-de-tests.md) §16).

Les numéros sont définitifs. Une règle abandonnée reste, marquée abandonnée, avec
la date et le document qui l'abandonne — un numéro ne se réattribue pas.

Abréviations des sources : **ADR0** = [adr/0000-perimetre-brique0.md](adr/0000-perimetre-brique0.md),
**ADR1** = [adr/0001-stack.md](adr/0001-stack.md), **MET** = [methode.md](methode.md),
**JUR** = [juridique.md](juridique.md), **DOD** = [definition-of-done.md](definition-of-done.md),
**ING** = [brique0/ingestion-votes.md](brique0/ingestion-votes.md),
**POS** = [brique0/positionnement.md](brique0/positionnement.md),
**REG** = [brique0/registre-entites.md](brique0/registre-entites.md),
**CON** = [brique0/contrats.md](brique0/contrats.md).

---

## A. Périmètre

| # | Règle | Fondement |
|---|---|---|
| RG-01 | Aucun média n'entre dans la brique 0 : les seules entités mesurées sont des partis, des coalitions et des groupes parlementaires. | ADR0 §1 |
| RG-02 | La v0 ne couvre que la XVIIe législature de l'Assemblée nationale ; le numéro de législature est un champ du modèle dès la v0, même à valeur unique. | ADR0 §3 |
| RG-03 | Le numéro de législature n'est jamais dérivé d'un libellé humain de la source, seulement du chemin de téléchargement. | ADR0 §3 |
| RG-04 | Aucune sortie ne présente une dérive, une évolution ou une comparaison entre deux dates ou deux législatures. | ADR0 §3, POS §7 |
| RG-05 | Trois familles de mesure existent — `votes`, `experts`, `administratif` — et aucune sortie ne contient de valeur agrégeant deux familles. | MET §3, CON §1 |
| RG-06 | Aucune valeur n'est projetée, recalibrée ou convertie d'une échelle de famille vers celle d'une autre. | POS §8 |
| RG-07 | Aucune source exigeant une inscription, une connexion ou une étape manuelle récurrente n'entre dans le pipeline. | ADR0 §8 |
| RG-08 | Aucun fichier de données tiers n'est redistribué par le dépôt ; seuls le sont ce que porte `entrees[]` d'une ligne de preuve — URL, producteur, date de dernière mise à jour de la source, citation exigée, empreinte d'archive, empreinte de contenu, date de récupération. | ADR0 §4, CON §2.1 |

## B. Ingestion des scrutins

| # | Règle | Fondement |
|---|---|---|
| RG-10 | Le codage est `pour = +1`, `contre = −1`, `abstention = 0`. | ING §5 |
| RG-11 | Un non-votant, quelle qu'en soit la cause, et un acteur absent des blocs d'un scrutin ne produisent **aucune cellule** ; il n'existe pas de code d'absence. | ING §5, MET §1 |
| RG-12 | Aucune donnée manquante n'est imputée, moyennée ni remplacée par zéro, à aucune étape. | ING §5 |
| RG-13 | Un scrutin n'entre dans la matrice que si `min(decompte.pour, decompte.contre) ≥ 1` ; c'est le seul filtre, et ce n'est pas un seuil de participation. | ING §6 |
| RG-14 | Les motions de censure sont écartées par l'application de RG-13, sans règle spéciale les nommant. | ING §7 |
| RG-15 | `nombreVotants` est publié par scrutin et affiché avec le décompte, jamais utilisé comme critère d'exclusion. | ING §6 |
| RG-16 | Le décompte des scrutins retenus et des scrutins écartés, avec le motif de chaque écart, est une sortie publiée. | ING §7, ROADMAP v0.1 |
| RG-17 | Le rattachement d'un député à un groupe respecte les périodes de validité : un changement de groupe ne réattribue pas les votes antérieurs au nouveau groupe. | MET §1, ING §8, DOD §9 |
| RG-18 | Une mise au point de vote est ingérée et jamais appliquée à la valeur de la cellule. | ING §3 |
| RG-19 | Le parseur absorbe les trois irrégularités mesurées de la source — un bloc `votant` nu au lieu d'un tableau, un tableau dont les éléments sont nuls, un identifiant enveloppé dans un objet `{"#text":…}` — dans trois adaptateurs, sans cas particulier ailleurs dans le code. | ADR1 §1.4, ING §4 |
| RG-20 | Un téléchargement est repris jusqu'à la taille annoncée par la source, son empreinte SHA-256 est consignée, et une empreinte qui change sans que la date de source change fait échouer l'exécution. | ADR1 §1.5 |

## C. Estimation de l'axe des votes

| # | Règle | Fondement |
|---|---|---|
| RG-25 | L'estimateur est un rang 1 avec constante par scrutin, calculé sur les seules cellules observées. | POS §4 |
| RG-26 | L'initialisation ne dépend que des données, jamais d'un indice de ligne ni d'un générateur pseudo-aléatoire ; aucune graine n'existe. | POS §4 |
| RG-27 | Une permutation des lignes de la matrice ne déplace aucune position publiée. | POS §5 |
| RG-28 | Aucune réduction flottante parallèle n'est employée sur le chemin déterministe. | ADR1 §1.6, POS §4 |
| RG-29 | Le signe et l'échelle de l'axe sont fixés par une unique transformation affine ancrée sur deux médianes de groupe nommées : médiane de l'ancre gauche = −1, médiane de l'ancre droite = +1. | POS §5 |
| RG-30 | Les deux ancres sont déclarées dans le registre d'entités comme identifiants de groupe avec période de validité, jamais en dur dans le code. | POS §5, ADR1 §8 |
| RG-31 | Si une ancre est absente à la date d'agrégation, le pipeline échoue bruyamment et ne choisit jamais d'ancre de remplacement. | POS §5 |
| RG-32 | Le second axe est calculé, sert aux contrôles de séparation, et n'est jamais publié comme position ni étiqueté. | POS §4, POS §10 |
| RG-33 | Les valeurs sont arrondies au nombre de décimales de leur échelle avant écriture ; la garantie d'identité octet pour octet porte sur l'artefact arrondi. | POS §5, ADR1 §1.7 |

## D. Agrégation et publication des positions

| # | Règle | Fondement |
|---|---|---|
| RG-40 | La position d'un groupe est la médiane des positions de ses membres, avec choix déterministe de la valeur centrale en effectif pair. | POS §5, POS §6 |
| RG-41 | Aucune coordonnée individuelle de député n'apparaît dans le registre de preuves, dans un artefact publié ni dans l'interface. | ADR0 §2, DOD §9 |
| RG-42 | La dispersion publiée est l'écart interquartile et l'écart-type de rééchantillonnage, jamais une variance et jamais une étendue : une borne d'étendue est la coordonnée d'un membre identifiable. | POS §6, JUR |
| RG-43 | Toute valeur publiée est accompagnée de son effectif retenu et de sa date. | ADR0 §5 |
| RG-44 | Un groupe n'est publié que si les trois conditions tiennent : IQR ≤ 0,25 en unités ancrées, écart-type de rééchantillonnage ≤ 0,05, et au moins 10 députés ayant exprimé au moins 200 votes. | POS §6 |
| RG-45 | Un groupe qui échoue à RG-44 s'affiche « non mesuré » avec sa raison et ses chiffres de dispersion, jamais une valeur assortie d'un avertissement. | POS §6, ADR0 §5 |
| RG-46 | Le seuil de 200 votes ne porte que sur l'inclusion d'un député dans la médiane de son groupe, jamais sur la constitution de la matrice. | POS §6 |
| RG-47 | Si le premier axe n'atteint pas le critère de séparation documenté, la famille `votes` s'affiche comme non mesurée et la sortie est publiée avec deux familles. | ADR0 §1, ADR0 §8 |
| RG-48 | Une absence de donnée est dite avec son code ; elle n'est jamais rendue par une case muette, par « neutre » ou par « centre ». | ADR0 §5, CON §2.4 |

## E. Registre d'entités

| # | Règle | Fondement |
|---|---|---|
| RG-50 | Un groupe parlementaire et un parti sont deux entités distinctes du modèle ; l'une n'est jamais employée à la place de l'autre. | MET §4, REG §1 |
| RG-51 | Toute entité, relation, appartenance et identifiant externe porte une période de validité. | MET §4, REG §4 |
| RG-52 | Un `id` d'entité est immuable : rien n'est renommé ni supprimé ; une entité qui cesse d'exister reçoit une date de fin. | REG §7 |
| RG-53 | Le registre n'accepte aucune clé absente de son schéma — producteur strict. | REG §6, CON §5.1 |
| RG-54 | Un marqueur de votes ne rejoint la bande d'un parti que si la composition du groupe désigne exactement un parti et qu'aucun autre groupe valide à la même date ne le désigne ; sinon le groupe reçoit sa propre bande. | CON §4.3 |
| RG-55 | Aucune constante en dur ne remplace une donnée du registre : nom de parti, sigle, code de nuance, identifiant CHES, appartenance de groupe. | DOD §16 |
| RG-56 | Le registre d'entités est le seul fichier dont la relecture humaine ligne par ligne est obligatoire avant merge. | DOD §17, REG §7 |
| RG-57 | Un fichier de données n'est jamais modifié à la main sans script reproductible commité dans la même PR. | DOD §18 |

## F. Registre de preuves et contrat de sortie

| # | Règle | Fondement |
|---|---|---|
| RG-60 | Le registre de preuves est un JSONL en ajout seul : aucune ligne n'est modifiée ni supprimée. | MET §5, ADR0 §6, CON §2.7 |
| RG-61 | Une valeur qui change est ré-émise en nouvelle ligne ; aucun recalcul silencieux n'écrase une valeur publiée. | ADR0 §6 |
| RG-62 | Chaque ligne porte la version du contrat de sortie qui l'a produite et sa propre date de calcul. | ADR0 §6, CON §2.1 |
| RG-63 | Il n'existe aucun champ de position hors d'une ligne de preuve : toute valeur publiée porte sa famille, son échelle, sa source et sa date. | CON §2.1 |
| RG-64 | Une ligne dont `valeur` et `valeur_code` sont nuls porte obligatoirement `motif_code` et `motif`, et réciproquement. | CON §2.4 |
| RG-65 | Une valeur hors des bornes de graduation de son échelle est un refus bloquant, jamais un dépassement toléré. | CON §2.3 |
| RG-66 | Deux marqueurs de familles différentes ne partagent jamais un identifiant d'échelle. | CON §2.3, CON §6 |
| RG-67 | Toute fonction fixe externe intervenant dans un calcul est épinglée en version et consignée dans la ligne de preuve. | ADR0 §6, MET |
| RG-68 | Chaque ligne cite au moins une entrée avec sa source, son URL, son empreinte SHA-256 et sa date de récupération. | CON §2.1 |
| RG-69 | Toute valeur affichée trace vers une ligne du registre ; un marqueur sans ligne ne s'affiche pas, et aucune estimation ne comble un manque. | DOD §15, CON §1, ROADMAP |
| RG-70 | Les lignes servies au front sont identiques octet pour octet à celles du registre ; aucune preuve n'est reformatée pour l'affichage. | CON §4.4 |
| RG-71 | Le front ne code en dur ni la liste des familles, ni celle des échelles, ni celle des motifs — consommateur tolérant. | CON §5.1 |
| RG-72 | Devant un schéma majeur inconnu, le front ne rend aucun marqueur de cet artefact et ne l'annonce jamais comme « non mesuré ». | CON §5.2 |
| RG-73 | La date d'arrêt affichée se dérive de la date de calcul maximale du registre, sans saisie manuelle. | ADR0 §6, CON §4.2 |
| RG-76 | Chaque entrée d'une ligne de preuve porte le nom du producteur de sa source, tel que cette source le publie, et la date de dernière mise à jour **de cette source**. `producteur` est le nom d'une **organisation** : une source dont le producteur déclaré est une personne physique n'entre pas dans le pipeline sous son nom — elle porte celui de son institution, ou elle est écartée. La mention de paternité exigée par la Licence Ouverte est portée par l'entrée ; elle n'est jamais un texte global à maintenir, et `source`, qui est un code interne, n'en tient pas lieu. | LICENSE-DONNEES, ING §1, CON §2.1, CON §6 I21 |
| RG-77 | Chaque entrée porte deux empreintes : celle de l'archive téléchargée, qui atteste du téléchargement et sert le contrôle contre l'empreinte publiée par la source, et celle de son contenu, qui atteste de la donnée. Seule l'empreinte de contenu entre dans la clé de déduplication ; une republication à contenu identique ne ré-émet donc aucune ligne. | CON §2.8, CON §3, CON §6 I22, verification-2026-08-27 §0 |
| RG-78 | L'empreinte de contenu d'une archive est le SHA-256 de la concaténation des contenus de tous ses fichiers réguliers, triés par chemin relatif en **ordre d'octets** — jamais selon la collation d'une locale, qui donne une autre valeur sur la même archive. Pour une source d'un seul fichier, c'est le SHA-256 de ce fichier. | CON §2.8, verification-2026-08-27 §0 |
| RG-79 | Une entrée dont la source exige une citation la porte mot pour mot dans `citation` ; une source qui n'en exige pas porte `null`. Aucune citation n'est reformulée, abrégée, ni remplacée par une mention globale du projet : la liste opposable des sources à citation est celle de [sources.md](sources.md). | LICENSE-DONNEES, CON §2.1, CON §6 I23, sources.md |

## G. Déterminisme

| # | Règle | Fondement |
|---|---|---|
| RG-80 | Le produit ne contient aucune génération de texte ; un embedding est admis seulement sous RG-67, et aucun n'est utilisé en brique 0. | MET, ADR0 §6, DOD §14 |
| RG-81 | Deux exécutions consécutives sur la même entrée produisent des artefacts d'empreintes identiques. | DOD §4 |
| RG-82 | La reconstruction complète depuis les sources brutes redonne le registre de preuves octet pour octet. | MET §5, DOD §5 |
| RG-83 | Aucune valeur calculée ne lit l'horloge : les dates sont des entrées, et `date_calcul` est la seule exception, déclarée. | DOD §6, CON §8.1 |
| RG-84 | Aucune itération sur un ensemble non ordonné, aucune dépendance à l'ordre du système de fichiers. | DOD §6 |
| RG-85 | Aucun tirage aléatoire et aucune lecture d'horloge dans un test ; les cas sont des tables fixes. | tdd.md §3 |
| RG-86 | La suite de tests de niveau 1 passe sans réseau, sans clé et sans jeton. | DOD §7, tdd.md §3 |
| RG-87 | Toute donnée réseau utilisée par un test est une fixture figée et commitée, avec son URL et sa date de récupération, issue d'une source en Licence Ouverte. | DOD §8 |

## H. Ton, lexique et publication

| # | Règle | Fondement |
|---|---|---|
| RG-90 | Aucun axe, champ, score ou variable, même interne et même sous un nom technique anodin, n'a de pôle dépréciatif. | JUR règle 2, DOD §12 |
| RG-91 | Les termes proscrits du lexique de JUR n'apparaissent nulle part : code, identifiants, données, chaînes d'interface, documentation. | JUR règle 5, DOD §11 |
| RG-92 | Aucune chaîne d'interface n'emploie de personne — ni « nous », ni « vous », ni impératif adressé au lecteur. | ADR0 §5 |
| RG-93 | Une étiquette d'interface tient en 40 caractères, une légende ou une note contextuelle en 140 caractères et une phrase. | ADR0 §5 |
| RG-94 | Un écran porte au plus un bloc explicatif, de trois phrases au maximum. | ADR0 §5 |
| RG-95 | Un comparatif sans chiffres est proscrit ; la comparaison s'écrit avec les deux valeurs. | ADR0 §5 |
| RG-96 | La procédure de signalement d'erreur est accessible depuis chaque écran. | JUR règle 4, ROADMAP v0.7 |
| RG-97 | La couleur n'est jamais le seul porteur d'une information ; chaque marqueur est atteignable au clavier et porte un libellé accessible. | ADR1 §7 |
| RG-98 | En cas d'échec du pipeline, le site conserve sa dernière sortie valide et affiche sa date d'arrêt, plutôt que de se taire. | ROADMAP, ADR1 §7 |

## I. Version et licence

| # | Règle | Fondement |
|---|---|---|
| RG-100 | La version porte sur le contrat de sortie, pas sur le code ; majeure, mineure et patch sont définies par ADR0 §6. | ADR0 §6 |
| RG-101 | Une inversion de la convention de signe de l'axe est une majeure et crée un nouvel identifiant d'échelle, les lignes antérieures restant interprétables. | ADR0 §6, CON §2.3 |
| RG-102 | Un changement du contrat de sortie s'accompagne, dans la même PR, de l'incrément de version et du journal de ce qui rompt. | DOD §20 |
| RG-103 | Une modification de la méthode s'accompagne, dans la même PR, de la modification de MET. | DOD §19 |
| RG-104 | Une source ajoutée s'accompagne d'une ligne dans [sources.md](sources.md) avec son format, sa licence et sa date de vérification. | DOD §22 |
| RG-105 | Une dépendance ajoutée s'accompagne d'une justification écrite répondant à « la bibliothèque standard le fait-elle ? ». | DOD §21 |
| RG-106 | Le code est publié sous AGPL-3.0-only, les données, registres et documentation sous Licence Ouverte 2.0. | ADR0 §4 |
| RG-107 | Une affirmation non vérifiée est marquée `A VERIFIER` avec la façon de la vérifier, ou n'est pas écrite. | DOD §23 |

## J. Personnes physiques et textes de tiers

Ajoutées à la suite des relectures juridique et qualité du 2026-08-27.

| # | Règle | Fondement |
|---|---|---|
| RG-74 | Aucun champ d'un artefact publié ni d'un fichier de `data/` ne stocke un texte de tiers de plus de 200 caractères. Le corps d'un article, d'une transcription ou d'un document source n'est jamais persisté : il est traité en mémoire puis écarté. **Exception unique, ouverte le 2026-08-27 avec le contrat 0.2.0** : `entrees[].citation`, plafonnée à 400 caractères, qui porte la mention légale exigée par une source. Une référence bibliographique n'est pas un texte de tiers republié, et le champ n'accepte rien d'autre. | JUR, MET, L.122-5-3 CPI, CON §6 I20 et I23 |
| RG-75 | Un titre, un chapô ou une citation courte de tiers s'affiche avec le lien vers la source et n'est jamais stocké au-delà de la fenêtre de calcul déclarée. | JUR, MET |
| RG-110 | Aucun identifiant de personne, aucun nom de personne physique et aucune coordonnée individuelle n'apparaît dans un artefact de `public/api/` ni dans `data/`. Les fixtures de test en contiennent et sont signalées comme telles. **Exception unique, ouverte le 2026-08-27 avec le contrat 0.2.0** : les noms d'auteurs contenus dans `entrees[].citation`, lorsque la source exige la reproduction mot pour mot de sa référence bibliographique (I23). Ce ne sont ni des identifiants, ni des coordonnées, ni des personnes mesurées : aucune valeur du projet ne leur est rattachée, et la mention est une obligation de licence, pas une donnée collectée. Aucun autre champ n'admet un nom de personne physique. | JUR, ADR0 §2, CON §6 I23 |
| RG-111 | Aucune colonne nominative d'un fichier de résultats électoraux n'est ingérée. Une nuance administrative n'est jamais rattachée à une personne physique, dans aucun fichier du projet, y compris intermédiaire ou de cache. | JUR, ADR0 §2 |
| RG-112 | Aucun identifiant de schéma, d'échelle, de famille, de méthode ou de motif ne contient le nom, le sigle ou l'identifiant d'une entité mesurée. | JUR règle 1, ADR0 §6 |
| RG-113 | Toute étiquette reproduisant une classification de tiers porte le nom de son auteur en tête. Contrepoint n'affiche jamais une catégorie de tiers sous une forme qui pourrait être lue comme sienne. | JUR règle 1 |
