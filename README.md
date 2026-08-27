# Contrepoint

Cartographier qui se situe où dans le paysage politique et médiatique français,
à partir de mesures reproductibles et de preuves datées — pour pouvoir croiser
plusieurs angles sur une même information sans dépendre d'un seul son de cloche.

> Statut : **conception**. Aucun code de produit n'est implémenté. Les chiffres
> cités dans la documentation ne sont pas des estimations : ils viennent de
> mesures réellement exécutées sur les archives publiques, hors pipeline, dont le
> protocole et les commandes sont consignés dans
> [docs/brique0/verification-2026-08-27.md](docs/brique0/verification-2026-08-27.md).
> Ce dépôt contient pour
> l'instant la méthode, la roadmap et les arbitrages.

## Le problème

Un lecteur régulier finit par épouser la position de ses sources sans s'en
apercevoir. Le basculement ne se voit que par accident, le jour où un article ou
une interview hors de son spectre habituel crée assez de recul pour rendre le
biais visible. Ce recul-là est fortuit, donc rare, donc inefficace.

Contrepoint le rend systématique et bon marché : sur un fait donné, montrer
comment il est traité par des sources d'orientations différentes, y compris
celles qu'on ne lit jamais, et laisser le lecteur juger.

## Le principe

**Aucun arbitre.** Il n'existe pas de source d'information neutre — les
publications gouvernementales n'en sont pas davantage. Contrepoint ne désigne
donc jamais qui a raison. Il rend le **désaccord** lisible : quand six
rédactions couvrent le même fait et citent des chiffres différents, l'écart
*est* l'information.

**Aucun jugement, uniquement des mesures.** Chaque position affichée est un
calcul reproductible sur une source publique, horodaté, avec la preuve
attachée. « Cette rédaction accorde son attention majoritairement à des acteurs
dont les votes se situent ici » est une mesure vérifiable. « Cette rédaction est
partiale » est une opinion. On ne produit que la première.

**Aucune moyenne des sources.** Quand plusieurs méthodes situent une même entité
à des endroits différents, on les affiche côte à côte au lieu de les fondre. Le
delta entre la façon dont un parti vote, la façon dont les politologues le
situent et la façon dont l'administration le classe est le produit, pas un
défaut à corriger.

**Tout est daté.** Une position sans date ne veut rien dire. Le corollaire est
gratuit : l'historique révèle les dérives d'orientation dans le temps.

## Déterminisme

Tout le pipeline est programmatique. Pas de génération de texte, pas de LLM
rédacteur, pas de résumé synthétisé. Même entrée, même sortie, bit pour bit.

Ce n'est pas une posture : un résultat reproductible se défend, une prose
générée ne se défend pas. Et l'objectif de l'outil est que le lecteur se fasse
son avis — donc on lui montre les mots des rédactions, pas les nôtres.

Détail maillon par maillon : [docs/methode.md](docs/methode.md).

## Découpage

Une brique par nature de source, ordonnées par dépendance. Chacune a sa spec,
son plan et son implémentation. Voir [ROADMAP.md](ROADMAP.md).

| Brique | Objet | État |
|---|---|---|
| 0 | Les acteurs — partis et députés | en conception |
| 1 | Presse écrite | à concevoir |
| 2 | YouTube | à concevoir |
| 3 | TV / radio | à concevoir |

La brique 0 est l'ancrage : sans axe des acteurs, aucune mesure sur les médias
n'a de référentiel. Elle est aussi utile seule.

## Arbitrages actés

| Décision | Raison |
|---|---|
| Usage personnel d'abord, ouverture ensuite | L'outil doit servir son auteur avant de servir un public |
| Contraintes de publication respectées dès la V1 | Rétrofitter la rigueur sur un corpus déjà constitué coûte plus cher que la tenir dès le départ |
| Digest hebdomadaire, pas quotidien | Un rythme quotidien crée une obligation ; l'hebdo laisse le temps à la couverture de se déployer |
| Requête ponctuelle en complément | Le besoin réel survient en lisant un article précis. Le même index sert les deux |
| Aucune mesure du comportement de lecture | L'outil pousse déjà les angles absents. Observer l'utilisateur n'ajoute presque rien et coûte des données personnelles |
| 100 % déterministe, zéro token | Reproductibilité, défendabilité, coût nul, tests hors ligne |
| Dépôt public dédié, redirection depuis la navbar du site perso | Le projet a son identité propre |

## Contraintes

Zéro budget, une personne, pas de temps libre. Ce qui exige une action manuelle
récurrente n'existe pas : le pipeline tourne seul ou ne tourne pas.

Cadre juridique et lexique imposé : [docs/juridique.md](docs/juridique.md).
Sources de données et leur état : [docs/sources.md](docs/sources.md).

## Le nom

Le contrepoint fait sonner ensemble plusieurs voix indépendantes sur un même
sujet, et tire sa valeur de leur différence plutôt que de leur fusion à
l'unisson. C'est la thèse du projet.

## Langue

Documentation et interface en français : le domaine est entièrement français
(rédactions, partis, droit) et le public l'est nécessairement.
