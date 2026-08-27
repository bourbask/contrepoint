# Contraintes de publication

Ce document existe parce qu'un projet comparable est mort en France pour des
raisons qui n'étaient pas techniques.

---

## Le précédent

Le Décodex du *Monde* a fermé le **29 août 2022**. Le journal a par ailleurs été
condamné en appel le **28 février 2024** sur ses classements.

Ce n'est pas l'idée qui a échoué, c'est la forme : un média étiquetant nommément
d'autres médias sur un axe de **fiabilité**, c'est-à-dire un axe dont un pôle est
dépréciatif. C'est cette forme précise qui ouvre la porte à l'action en
dénigrement ou en diffamation. Elle explique aussi le vide actuel du marché
français.

## Les cinq règles qui en découlent

**1. Des mesures, jamais des jugements.**
« Cette rédaction accorde son attention majoritairement à des acteurs dont les
votes se situent ici » est un calcul vérifiable. « Cette rédaction est partiale »
est une opinion. Seule la première est publiée.

**2. Aucun axe à pôle dépréciatif.**
Un axe gauche-droite est descriptif : aucune extrémité n'est une insulte. Un axe
de fiabilité, de crédibilité ou de véracité est intrinsèquement dépréciatif d'un
côté. Contrepoint n'en produit aucun, jamais, même en interne.

**3. Reproductibilité totale.**
Méthode publiée, code public, données brutes accessibles, calcul rejouable par
un tiers. Un chiffre reproductible se défend devant un juge ; une appréciation ne
se défend pas. C'est la raison principale du choix d'un pipeline entièrement
déterministe — la prudence juridique et l'exigence technique convergent ici sur
la même architecture.

**4. Correction visible.**
Procédure de signalement accessible depuis chaque écran, correction appliquée et
tracée. Une réclamation fondée est un bug à corriger, et le dire publiquement
coûte moins cher que le contester.

**5. Lexique contraint.**
Le vocabulaire est une décision d'architecture, pas une question de style.

| Interdit | Retenu |
|---|---|
| biais d'un média | espace de citation, attention accordée |
| fiabilité, crédibilité | *rien — aucune mesure de ce type n'existe dans le projet* |
| désinformation, fake news, infox | *jamais employé* |
| média orienté, partial, militant | position mesurée sur l'axe, à telle date |
| ce média ment | les valeurs citées divergent : X selon A, Y selon B |
| classement | positionnement daté |

---

## Droit d'auteur et fouille de textes

L'exception de fouille de textes et de données — directive (UE) 2019/790,
transposée à l'article **L.122-5-3** du code de la propriété intellectuelle —
autorise l'analyse automatisée. Elle n'autorise pas la constitution d'une base de
textes intégraux republiable.

Règles de stockage, énoncées dans [methode.md](methode.md) :
- métriques dérivées conservées durablement ;
- corps d'article conservé le temps du calcul, puis écarté ;
- citations courtes uniquement, avec lien vers la source ;
- opt-out déclarés par les éditeurs respectés.

---

## Données personnelles

Le projet n'observe aucun comportement de lecture, ne stocke aucune donnée
d'utilisateur, n'a pas de comptes. Il n'y a donc pas de traitement de données
personnelles au sens du RGPD côté utilisateurs.

Les personnalités politiques mesurées le sont sur la base de leurs actes publics
en qualité d'élus — votes nominatifs publiés par l'Assemblée nationale, prises de
position publiques. Aucune donnée relevant de la vie privée n'entre dans le
projet, et le projet n'a aucun usage d'une telle donnée.

---

## Ce que le projet ne fera pas

Contrepoint mesure des positions publiques d'organisations et d'élus à partir de
sources publiques. Il ne sert pas à cibler des personnes.

Concrètement, et sans exception : pas de dossier sur un individu, pas de
compilation d'éléments à charge, pas de donnée non publique, pas de mise en
relation destinée à nuire à quelqu'un. Un outil de mesure dont l'auteur serait
soupçonné de s'en servir contre des personnes perd le seul actif qu'il possède —
sa crédibilité — et offre à ses adversaires l'unique argument capable de le faire
tomber sans avoir à discuter sa méthode.

Cette limite est une contrainte de conception, au même titre que l'absence d'axe
de fiabilité.
