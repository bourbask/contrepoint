# Contraintes de publication

Ce document existe parce qu'un projet comparable est mort en France pour des
raisons qui n'étaient pas techniques.

---

## Le précédent

Le Décodex du *Monde* a fermé le **2022-08-29**. Le tribunal de commerce de
Paris (15e chambre, jugement du **2023-06-19**) a condamné la société éditrice
du *Monde* sur le fondement de la **concurrence déloyale par dénigrement**, pour
avoir qualifié un site tiers dans le Décodex.

`A VERIFIER` : la date d'appel du **2024-02-28** circule mais n'a pu être
attestée que par le site de la partie gagnante. Référence exacte de l'arrêt —
chambre, numéro de RG, dispositif — à retrouver sur Judilibre ou Doctrine avant
toute publication. **Le raisonnement ci-dessous n'en dépend pas** : il dépend de
la forme de l'axe, pas de l'issue d'une instance.

Ce n'est pas l'idée qui a échoué, c'est la forme : un média étiquetant nommément
d'autres médias sur un axe de **fiabilité**, c'est-à-dire un axe dont un pôle est
dépréciatif. Deux voies d'action s'ouvrent alors — le **dénigrement**, qui
suppose un rapport de concurrence commerciale, et la **diffamation**, qui n'en
suppose aucun. Contrepoint n'étant ni commercial ni éditeur de presse, la
première le vise mal ; la seconde le vise pleinement. Les règles ci-dessous sont
écrites contre la seconde.

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

**Côté utilisateurs.** Aucun compte, aucun témoin de connexion, aucune mesure
d'audience, aucune observation du comportement de lecture. Le site est un
ensemble de fichiers statiques. Aucun traitement de données personnelles
d'utilisateur n'est opéré par le projet. Les journaux techniques de l'hébergeur
relèvent de la politique de cet hébergeur.

**Côté données traitées.** Le pipeline traite des données personnelles :
identifiants d'acteurs de l'Assemblée nationale, noms de députés, positions de
vote nominatives. Une position politique inférée relève des **catégories
particulières** de l'article 9.1 du RGPD. Le fait qu'elle soit publique ne l'en
fait pas sortir — c'est une exception qui l'autorise, et il faut l'invoquer.

- **Responsable de traitement** : Kevin Bourbasquet. Contact : les issues du dépôt.
- **Base légale** : article 6.1.f — intérêt légitime à l'information du public
  sur l'activité d'élus, mis en balance avec des données que les personnes
  concernées ont elles-mêmes rendues publiques dans l'exercice d'un mandat.
- **Levée de l'interdiction de l'article 9.1** : article **9.2.e** — données
  manifestement rendues publiques par la personne concernée. Un vote nominatif
  est publié par l'Assemblée nationale en application de son règlement, à
  l'initiative et sous le contrôle du député.
- **Minimisation** : aucune coordonnée individuelle n'est publiée (ADR 0000 §2).
  Les identifiants d'acteurs restent internes au pipeline et aux fixtures de
  test ; aucun artefact publié n'en contient (invariant I13).
- **Exercice des droits** : par issue publique. Une donnée inexacte est corrigée
  par la procédure de [utilisation.md](utilisation.md). Un droit d'opposition
  portant sur un vote nominatif se dirige vers l'Assemblée nationale, productrice
  de la donnée ; Contrepoint la retire de ses artefacts si la source la retire
  des siens.
- **Hors périmètre absolu** : aucune donnée de vie privée, aucune donnée hors
  exercice du mandat, aucun profil de personne physique.

**Fixtures.** `docs/brique0/echantillons/` contient des identifiants d'acteurs et
des noms de députés issus de l'open data de l'Assemblée nationale. Leur présence
est signalée ici et dans [LICENSE-DONNEES](../LICENSE-DONNEES), en application de
la clause « Données à caractère personnel » de la Licence Ouverte 2.0.

Nier la qualification serait le pire choix disponible : cela priverait le projet
de l'exception qui le protège et offrirait un argument gratuit à un plaignant.

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
