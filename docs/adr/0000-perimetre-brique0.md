# ADR 0000 — Périmètre et arbitrages de la brique 0

Statut : **acté**. Date : 2026-08-27.

Cet ADR tranche les points laissés ouverts par ROADMAP.md et docs/methode.md.
Il n'ouvre aucune option. Ce qui n'y figure pas comme « dedans » est dehors.

---

## 1. Périmètre de la v0 livrable

### Contexte

ROADMAP.md découpe la brique 0 en sept incréments (v0.1 → v0.7) sans dire
lesquels constituent une première livraison publiable. Sans cette ligne, chaque
incrément appelle le suivant et rien ne sort. La contrainte est une personne,
zéro budget, pas de temps libre : la v0 doit être atteignable et se défendre
seule.

### Décision

**Dedans.** Un graphe à une seule date, trois familles de marqueurs, les partis
de la XVIIe législature.

| Incrément | Portée retenue en v0 |
|---|---|
| v0.1 | Scrutins nominatifs, **XVIIe législature uniquement** |
| v0.2 | Axe issu des votes, agrégé **au parti**, dispersion intra-groupe publiée |
| v0.3 | **CHES seulement** (fichier de tendance 1999→2024) + **nuancier** (circulaire, recours) |
| v0.4 | Registre d'entités **restreint aux partis présents dans la XVIIe** |
| v0.5 | Registre de preuves complet — c'est le contrat, il n'est pas réductible |
| v0.6 | Graphe **statique** : une bande par parti, trois marqueurs, clic → preuve |
| v0.7 | Publication, tests hors ligne, page de méthode, procédure de correction |

**Dehors, et ça attend.**

| Écarté de la v0 | Raison |
|---|---|
| Manifesto Project / RILE | Inscription gratuite mais **manuelle** requise ; viole la contrainte « rien qui exige une action manuelle récurrente ». Réintégré le jour où un accès scriptable est obtenu |
| ParlGov | Apport marginal en v0 (résultats électoraux, compositions gouvernementales) alors qu'aucun écran ne les consomme |
| Wikidata | Sert les briques 1-3 (propriété des médias). Aucun usage en brique 0 |
| Curseur temporel (v0.6) | Une seule législature : il n'y a rien à faire glisser. Un curseur sur un point unique est une promesse fausse |
| Vue des dérives (v0.6) | Même raison. Une dérive exige deux dates de mesure du **même** dispositif |
| XVIe et XVe législatures | Voir §3 |
| Positionnement individuel des députés | Voir §2 |

**Sortie visible de la v0** : une page, N bandes de partis, chaque marqueur
cliquable vers sa ligne de preuve, plus le décompte des scrutins retenus et
écartés avec le motif. Si le premier axe des votes ne satisfait pas les conditions de
publication documentées (brique0/positionnement.md §6), la colonne « votes » est **vide** et la v0 sort
quand même avec deux familles.

### Conséquences

- La v0 n'a pas d'axe temporel. Aucun texte d'interface ne peut suggérer une évolution.
- Le registre de preuves porte dès la v0 son schéma définitif : c'est le seul endroit où l'anticipation est justifiée, parce que le rétrofit y coûte la réécriture de l'historique.
- Trois lignes de ROADMAP.md sont repoussées hors v0 sans être annulées (Manifesto, ParlGov, curseur/dérives).

---

## 2. Partis seulement, ou partis + députés

### Contexte

docs/methode.md établit que la discipline de vote à l'Assemblée est quasi
totale : l'axe issu des scrutins sépare les blocs et non les individus. Le
positionnement fin d'un député isolé n'est pas défendable. Mais le député est
l'unité d'observation obligatoire de la matrice — sans lui, pas d'ACP.

### Décision

**Députés ingérés, jamais publiés individuellement.** Le député existe dans le
pipeline comme ligne de la matrice et comme membre d'un groupe avec période de
validité. Sa coordonnée sur l'axe n'entre **ni dans le registre de preuves, ni
dans l'interface, ni dans un fichier de sortie**. Ce qui est publié au niveau
du groupe : la position agrégée, la dispersion intra-groupe, l'effectif retenu.

### Conséquences

- Le tiers qui veut la coordonnée individuelle la recalcule : le code et la matrice sont publics. La reproductibilité est intacte, le produit ne la met pas en vitrine.
- Cette règle est le principal rempart contre l'accusation de ciblage de personnes énoncée dans docs/juridique.md. Elle n'est pas négociable au motif que « ce serait intéressant à voir ».
- Le gazetteer que les briques 1-3 consommeront contient les **noms** des députés (appariement de chaînes) sans aucune valeur de position attachée.

---

## 3. Législatures couvertes au départ

### Contexte

Vérifié le 2026-08-27 : la XVIIe législature, ouverte après les législatives des
30 juin et 7 juillet 2024, est en cours ; les prochaines élections générales sont
au plus tard en juin 2029. Les archives de scrutins sont servies par des chemins
distincts selon la législature (codes HTTP relevés le 2026-08-27) :

| Jeu | URL | Code |
|---|---|---|
| Scrutins XVII | `https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip` | 200 |
| Scrutins XVI | `https://data.assemblee-nationale.fr/static/openData/repository/16/loi/scrutins/Scrutins.json.zip` | 200 |
| Scrutins XV, même schéma d'URL | `.../repository/15/loi/scrutins/Scrutins.json.zip` | **404** — passe par les archives, chemin différent |

**Piège relevé sur la source** : la fiche du jeu 17 est libellée « pour la XV
législature » sur data.assemblee-nationale.fr. Le libellé est faux, le chemin
`/repository/17/` est autoritatif. Ne jamais dériver le numéro de législature du
libellé humain.

### Décision

**XVIIe législature seule en v0.** La XVIe est le premier lot d'extension : même
schéma d'URL, même format, coût quasi nul, donc mineure et pas v0. La XVe exige
un second chemin d'ingestion (archives) et n'est pas planifiée.

### Conséquences

- Le registre d'entités n'a qu'un jeu de groupes parlementaires à réconcilier au départ. Les fusions et scissions antérieures à 2024 sont modélisables mais non peuplées.
- Le modèle porte le numéro de législature dès la v0, même avec une seule valeur. Ajouter la dimension après coup casserait le registre de preuves, donc majeure — inacceptable pour un gain nul.
- **Interdiction dérivée** : aucun affichage de dérive, d'évolution ou de comparaison inter-législature en v0.

---

## 4. Licence

### Contexte

L'amont est en Licence Ouverte / Open Licence (Etalab), imposant l'attribution.
*A VERIFIER* : la page `https://data.assemblee-nationale.fr/licence-ouverte-open-licence`
n'affiche pas le numéro de version dans son corps ; il faut ouvrir le PDF ou le
RTF qu'elle lie pour lire « 2.0 » et la mention de paternité exacte exigée. À
faire avant publication, la mention exacte étant une obligation de licence.

Le projet a deux natures d'actif : du code, et des fichiers de données dérivés.
Le risque spécifique du projet est le fork silencieux qui greffe un axe de
fiabilité sur la méthode et l'attribue à Contrepoint.

### Décision

- **Code : AGPL-3.0-only.** Aucune licence n'interdit un usage ; l'AGPL force le fork déployé comme service à publier son code, donc rend la déviation **visible** au lieu de silencieuse. C'est la seule protection réaliste de l'actif du projet, qui est sa méthode.
- **Données dérivées, registres et documentation : Licence Ouverte / Open Licence (Etalab).** Alignée sur l'amont AN, donc aucun débat de compatibilité. ODbL 1.0 est écartée : son partage à l'identique sur toute base dérivée décourage exactement les réutilisateurs dont le projet a besoin (chercheurs, rédactions).
- **Aucune redistribution de fichier tiers.** CHES, nuancier et tout jeu futur sont téléchargés par script à l'exécution ; le dépôt contient l'URL, la date de récupération, l'empreinte, jamais la copie. *A VERIFIER* : les conditions de redistribution de CHES 2024 ne sont pas documentées publiquement — vérifiable en lisant le codebook et les *terms of use* sur chesdata.eu. Cette décision rend la vérification non bloquante.

### Conséquences

- Deux fichiers de licence à la racine, et un en-tête de licence par répertoire de données. Coût accepté.
- Les fixtures de test sont des extraits de sources publiques en Licence Ouverte uniquement. Aucune fixture CHES commitée : le test qui en dépend est marqué et sauté hors réseau.
- Un contributeur ne peut pas relicencier le code en permissif. C'est voulu.

---

## 5. Ton éditorial du produit

### Contexte

docs/juridique.md impose un lexique. Un lexique ne suffit pas : la même mesure
peut être présentée en constat ou en insinuation avec un vocabulaire strictement
conforme. Le registre doit donc être arbitré, et il l'est ici. Sa mise en application
détaillée est dans [docs/ton.md](../ton.md).

### Décision

| Dimension | Choix |
|---|---|
| Registre | Constat technique. Descriptif, jamais évaluatif, jamais pédagogique |
| Personne | **Aucune.** Pas de « nous », pas de « vous », pas d'impératif adressé au lecteur. L'interface énonce, elle n'accompagne pas |
| Temps | Présent pour la méthode. Toute valeur porte sa date explicite |
| Étiquette | ≤ 40 caractères |
| Légende, note contextuelle | ≤ 140 caractères, une phrase |
| Bloc explicatif d'écran | ≤ 3 phrases, un seul par écran. Le texte long vit dans la page de méthode |
| Comparatifs | Interdits sans chiffres. « plus à droite que » est proscrit ; « 0,42 contre 0,18 » est retenu |
| Chiffres | Jamais seuls : toujours avec leur effectif et leur date |
| Proscrit en sus du lexique | Superlatifs, adverbes d'intensité, points d'exclamation, questions rhétoriques, métaphores, verbes d'intention prêtés à une entité |
| Absence de donnée | Dite, jamais comblée. « Non mesuré » et non « neutre », « centre » ou une case vide muette |

Exemple conforme : *Position issue des votes, 2026-08-27. 214 scrutins retenus
sur 389. Dispersion intra-groupe : écart interquartile 0,11 sur 71 députés.*

### Conséquences

- Ces limites de caractères sont une contrainte technique, pas une préférence de style. Elles sont dérivées en règles vérifiables dans docs/ton.md.
- Le ton exclut toute page d'accueil promotionnelle. Cohérent avec « aucune promotion » (ROADMAP.md v0.7).
- Le lexique et ce registre sont vérifiables par grep. Voir docs/definition-of-done.md.

---

## 6. Politique de version

### Contexte

SemVer parle d'API. Ici le contrat public n'est pas une API : c'est **le registre
de preuves** — son schéma, ses identifiants d'entités, la convention de signe de
l'axe. Une preuve publiée doit rester interprétable.

### Décision

La version porte sur le contrat de sortie, pas sur le code.

| Niveau | Ce qui le déclenche |
|---|---|
| **Majeure** | Rupture qui invalide une preuve déjà publiée : champ supprimé, renommé ou resémantisé ; identifiant de parti modifié ; **inversion de la convention de signe de l'axe** ; retrait d'une famille de mesure ; ajout d'une dimension au grain de mesure (législature, chambre) |
| **Mineure** | Ajout sans rupture : nouvelle source, nouvelle législature, nouvelle famille, champ optionnel. Également : changement de méthode d'estimation qui déplace les valeurs à schéma constant |
| **Patch** | Correction sans changement de méthode ni de schéma : appariement d'entité erroné, scrutin mal codé, seuil mal appliqué, rendu, texte d'interface |

Règles dures, non négociables :

1. **Chaque ligne du registre de preuves porte la version qui l'a produite**, et sa propre date de calcul. Sans ce champ, la politique de version est décorative.
2. **Aucun recalcul silencieux.** Une valeur qui bouge est **ré-émise** en nouvelle ligne. Les lignes antérieures ne sont ni modifiées ni supprimées — le registre est en ajout seul.
3. Version `0.x` jusqu'à la première publication hors usage personnel. Avant, une mineure peut rompre ; le fait de rompre est écrit dans le journal des changements.
4. Toute fonction fixe externe utilisée dans un calcul (modèle d'embedding, bibliothèque d'ACP dont le résultat dépend de l'implémentation) est **épinglée en version et consignée dans la ligne de preuve**. Changer cette version est une mineure au minimum. Aucun embedding n'est utilisé en brique 0.

### Conséquences

- Corriger un appariement dans le registre d'entités est un patch mais ré-émet des lignes de preuve. C'est normal et attendu.
- Le bandeau « données arrêtées le … » exigé par ROADMAP.md se dérive de la dernière date de calcul du registre. Aucune saisie manuelle.

---

## 7. Definition of Done d'une PR vers `develop`

Décision détaillée dans **[docs/definition-of-done.md](../definition-of-done.md)**, qui fait
partie de cet ADR. Principe : une PR n'est finie que si le déterminisme, la
traçabilité vers une preuve et la conformité au lexique sont **démontrés dans la
PR**, pas affirmés.

---

## 8. Les trois risques acceptés, les trois refusés

### Acceptés

| Risque | Pourquoi il est tenable |
|---|---|
| **Le premier axe des votes est peu informatif ou instable sur une seule législature** | Il est mesurable : le critère de séparation des axes (`s2/s1`) et la dispersion intra-groupe sont publiés. Hors des conditions de publication documentées, la famille « votes » s'affiche comme non mesurée et la v0 sort avec deux familles. Le risque devient un résultat |
| **Abandon à mi-chemin, chiffres périmés cités comme actuels** | Mitigé par construction : bandeau de date dérivé automatiquement du registre, et jeu de données utilisable seul, site à l'arrêt. Un corpus figé mais daté est supérieur au silence |
| **Nuancier incomplet ou en retard** — source par circulaire, pas jeu de données stable | Case affichée comme non mesurée. Jamais interpolée, jamais reportée d'une circulaire antérieure sans le dire. Une famille partiellement peuplée reste lisible |

### Refusés

| Risque | Pourquoi il est exclu par construction |
|---|---|
| **Afficher une position individuelle de député** | La méthode ne la soutient pas (discipline de vote) et c'est le vecteur direct de l'accusation de ciblage de personnes. Aucun gain d'usage ne compense |
| **Toute source exigeant une étape manuelle récurrente** — inscription, connexion, parser HTML fragile | Contredit la contrainte fondatrice. Conséquence assumée : Manifesto Project reste dehors, ParlGov reste dehors, et une source qui cesse de répondre est retirée explicitement, avec son code HTTP et sa date, au lieu d'être laissée en place |
| **Redistribuer un fichier de données tiers dont la redistribution n'est pas explicitement autorisée** | Un litige de licence arrête le projet plus vite qu'un litige sur la méthode, et sans argument à opposer. Le dépôt distribue le script de téléchargement, l'URL, la date et l'empreinte. Jamais la copie |
