# Utilisation

Document écrit pour un lecteur qui n'a rien à programmer. Il dit à quoi sert
l'outil, comment se lit ce qu'il affiche, et comment se signale une erreur.

---

## 1. À quoi ça sert

Un parti politique peut être situé « à gauche » ou « à droite » de trois façons
qui ne disent pas la même chose :

- par **ce que ses députés votent** à l'Assemblée nationale ;
- par **ce que des politologues en disent**, dans une enquête d'experts internationale (CHES) ;
- par **la case que l'administration lui attribue** — la « nuance » du ministère de l'Intérieur, utilisée pour compter les résultats électoraux.

Contrepoint affiche les trois côte à côte, pour chaque parti, à une date donnée.
Il n'en fait jamais la moyenne. Quand les trois ne concordent pas, l'écart est
ce qu'il y a à voir : un parti dont les votes n'ont pas changé mais que
l'administration a reclassé est exactement le cas que l'outil rend visible.

Ce que l'outil ne fait pas : dire qui a raison, apprécier la qualité de
quiconque, situer un député en particulier, résumer ou commenter.
Il n'y a aucun texte rédigé par une machine : chaque nombre affiché est le
résultat d'un calcul qu'un tiers peut refaire depuis les fichiers publics.

## 2. Comment se lit le graphe

Une ligne horizontale par parti — appelée **bande**. Sur chaque bande, jusqu'à
trois marqueurs de formes distinctes, un par famille de mesure : votes, experts,
administration. La forme et le libellé portent l'information ; la couleur seule
ne porte jamais rien, pour rester lisible sans distinguer les couleurs.

**L'axe des votes n'a pas d'unité.** Il est calé sur deux groupes nommés de
l'Assemblée : la position médiane du groupe LFI-NFP vaut exactement −1, celle du
groupe RN vaut exactement +1, et tout le reste se place entre les deux. Un
marqueur à +0,70 se lit « entre le centre et l'extrémité droite de cette
Assemblée », et rien de plus. Une distance sur cet axe ne se convertit pas en
distance idéologique, et deux graphes de deux Assemblées différentes ne se
superposent pas.

**L'échelle des experts est différente**, de 0 à 10, celle de l'enquête CHES.
**Le marqueur administratif n'est pas un nombre** mais un code de nuance. Trois
graduations distinctes sur la même bande : c'est voulu, et c'est ce qui empêche
de les additionner.

**Une case peut afficher « non mesuré ».** Ce n'est jamais un vide poli et ce
n'est jamais « au centre » : la raison est écrite à côté. Quatre raisons
existent — le parti n'est pas dans cette source, la mesure existe mais n'est pas
publiable, la source ne tranche pas, la source n'est pas récupérable
automatiquement. Le cas le plus fréquent de la deuxième raison : un groupe
parlementaire trop hétérogène pour qu'une position moyenne veuille dire quoi que
ce soit — les non-inscrits, par exemple, ne sont pas un parti.

**Aucun député n'apparaît avec sa propre position.** Les votes des députés sont
la matière du calcul, mais la discipline de vote à l'Assemblée est telle que le
chiffre obtenu pour un député isolé mesure surtout son taux de présence. Il n'est
donc pas publié, et cette limite est un choix documenté, pas un oubli.

**Un bandeau donne la date d'arrêt des données.** Si le calcul automatique
s'interrompt, le site conserve son dernier état valide et affiche cette date.
Un chiffre du site est toujours daté, jamais « actuel ».

## 3. Comment se lit une preuve

Chaque marqueur est cliquable. Le clic ouvre **la ligne de preuve** qui a produit
ce marqueur : une ligne de fichier, conservée telle quelle, jamais reformatée
pour l'affichage. Elle contient, dans cet ordre de lecture utile :

| Ce qui est écrit | Ce que ça permet de vérifier |
|---|---|
| l'entité mesurée et la famille de mesure | ce qui est mesuré, et selon laquelle des trois méthodes |
| la valeur, et l'échelle sur laquelle elle se lit | qu'aucun nombre ne flotte sans graduation |
| la période observée, du premier au dernier jour | sur quoi porte la mesure |
| la date de la source, et la date du calcul | l'âge de la donnée, distinct de l'âge du calcul |
| la méthode, sa version et ses paramètres | comment le nombre a été obtenu |
| chaque fichier d'entrée : son nom, son adresse, son empreinte, la date de récupération | que le calcul a bien été fait sur le fichier public que la source publiait ce jour-là |
| la dispersion interne du groupe, quand elle existe | à quel point les membres du groupe s'accordent |
| la version du logiciel et le commit qui a produit la ligne | de retrouver le code exact |

L'**empreinte** est une longue suite de chiffres et de lettres calculée à partir
du fichier téléchargé. Deux fichiers identiques donnent la même empreinte, un
octet de différence en donne une toute autre. C'est ce qui permet à un tiers de
prouver qu'il a bien le même fichier de départ.

Ces lignes sont **ajoutées, jamais modifiées**. Une valeur corrigée est une
nouvelle ligne ; l'ancienne reste lisible, avec sa date. L'historique complet est
donc gratuit et vérifiable, et une capture d'écran ancienne reste explicable.

## 4. Comment signaler une erreur

Une réclamation fondée est un bug à corriger. Deux voies, la première suffit.

**Ouvrir un signalement** sur <https://github.com/bourbask/contrepoint/issues>.
Utile dans le message, et suffisant :

1. **quoi** — le parti ou le groupe concerné, et lequel des trois marqueurs ;
2. **ce qui est affiché** — la valeur lue, et la date du bandeau ;
3. **ce qui devrait être affiché**, et **d'où ça vient** : une adresse publique consultable — texte de circulaire, page de l'Assemblée, publication de l'institut d'enquête. Une correction « de mémoire » n'est pas exploitable, parce qu'elle n'est pas vérifiable par un tiers.

**Proposer la correction directement**, pour qui sait se servir de git : voir
[../CONTRIBUTING.md](../CONTRIBUTING.md) et, pour le registre d'entités,
[brique0/registre-entites.md](brique0/registre-entites.md) §7.

### Ce qui se passe ensuite

| Nature du signalement | Traitement |
|---|---|
| Erreur d'appariement, de codage, de seuil, de libellé | Correction, nouvelle version de correctif, lignes de preuve concernées ré-émises. Rien n'est effacé |
| Désaccord sur la méthode elle-même | Réponse par la source et la méthode citées dans la ligne de preuve. La méthode est publique et discutable ; elle ne change pas sans mesure à l'appui |
| Demande d'ajouter un axe dont un pôle est dépréciatif | Refusée. Aucun axe de ce type n'existe dans le projet, ni à l'affichage ni en interne — voir le lexique de [juridique.md](juridique.md) |
| Demande d'afficher la position d'un député nommément | Refusée. La méthode ne la soutient pas (ADR 0000 §2) |

### Une limite à connaître avant de signaler

La nuance administrative n'est pas une vérité de référence : c'est une
classification décidée par circulaire, parfois contestée devant le Conseil
d'État. Elle est stockée comme un point de mesure daté, avec la référence de la
circulaire et l'issue des recours. Un désaccord avec une nuance est un désaccord
avec l'administration qui l'a attribuée, pas avec un calcul de Contrepoint — et
c'est pour cela qu'elle est affichée à côté des deux autres mesures plutôt que
fondue avec elles.
