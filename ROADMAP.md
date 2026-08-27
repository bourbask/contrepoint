# Roadmap

Ordonnée par dépendance, pas par appétit. Chaque étape se termine par quelque
chose qui se montre — la seule défense fiable contre l'enlisement dans le milieu
difficile.

Créée le 2026-08-27.

---

## Brique 0 — Les acteurs

**But : un graphe des partis français où chaque position est datée, sourcée, et
affichée simultanément selon trois méthodes indépendantes.**

> **Cette liste est cochée à chaque PR.** Une feuille de route qui ne l'est pas
> n'est ni un tableau de bord ni un périmètre : elle a dérivé une fois, quatre
> lignes contredisant l'ADR 0003 pendant plusieurs jours. Ce qui est barré est
> hors périmètre, avec le motif ; ce qui est coché est fait et vérifiable.
> Le suivi fin des tâches vit sur le tableau du dépôt, pas ici.

Aucun média n'intervient. Aucun risque juridique : personnalités publiques,
votes publics, jeux de données académiques cités. Utile seule.

### v0.1 — la matrice de votes

- [ ] Ingestion des scrutins nominatifs, AN open data (JSON, licence Etalab)
- [ ] Matrice `député × scrutin`, codage pour / contre / abstention / absent
- [ ] **Absent ≠ abstention.** L'absence est une donnée manquante, jamais une position
- [ ] **Aucun seuil de participation** — mesuré injustifiable, aucune rupture ne désigne de valeur et un seuil relevé dégrade la séparation des blocs ([ADR 0003](docs/adr/0003-arbitrages-de-coherence.md) §2)
- [ ] Filtre unique : un scrutin sans minorité enregistrée n'entre pas dans la matrice — `min(pour, contre) ≥ 1`. **455 scrutins écartés sur 8 434**, dont les **23 motions de censure** en totalité (l'article 49-2 ne fait voter qu'un camp) ; **7 979 retenus**
- [ ] `nombreVotants` publié par scrutin, avec le décompte — jamais employé comme porte d'entrée dans la matrice
- [ ] Rattachement des députés à leur groupe **avec période de validité** (les changements de groupe en cours de mandature sont fréquents) : le groupe vient du **bloc de ventilation du scrutin**, daté par construction ; AMO30 sert de recours pour les blocs `organeRef: "PO0"` et de source des périodes de validité ([ADR 0003](docs/adr/0003-arbitrages-de-coherence.md) §1)

Sortie visible : le décompte des scrutins retenus et écartés, et pourquoi.

### v0.2 — l'axe issu des votes

- [ ] Estimation de position **sur les seules cellules observées** : moindres carrés alternés de rang 1 avec constante par scrutin. L'ACP et l'analyse des correspondances sont écartées — elles exigent une matrice complète, et 77 % des cellules sont manquantes ([ADR 0003](docs/adr/0003-arbitrages-de-coherence.md), [docs/methode.md](docs/methode.md) §2)
- [ ] Fixation du signe de l'axe par deux points de repère connus, pour que « gauche » soit stable d'une exécution à l'autre
- [ ] Agrégation au niveau du parti
- [ ] Dispersion intra-groupe publiée, pas cachée : **écart interquartile et écart-type de rééchantillonnage**. Jamais la variance — illisible sur un axe sans unité — jamais l'étendue : un minimum et un maximum sont les coordonnées de deux membres identifiables ([ADR 0003](docs/adr/0003-arbitrages-de-coherence.md) §3)

**Limite méthodologique à énoncer sur le site, pas à découvrir plus tard :** la
discipline de vote est quasi totale à l'Assemblée. L'axe issu des scrutins
sépare donc très bien les **blocs**, et très mal les **individus** — contrairement
au cas américain dont la méthode est empruntée. Les votes servent à positionner
les partis. Le positionnement fin d'un député isolé n'est pas défendable et ne
sera pas affiché comme tel.

### v0.3 — les autres familles de mesure

- [ ] CHES, toutes les vagues 1999 → 2024 (axes gauche-droite, économique, sociétal, UE)
- ~~Manifesto Project — score RILE issu des programmes~~ — **hors v0** ([ADR 0000](docs/adr/0000-perimetre-brique0.md) §1) : l'accès exige une inscription manuelle, ce qui viole la contrainte « rien qui exige une action manuelle récurrente ». Reste une source d'identifiants dans le registre, établie à la main et datée
- [ ] Nuancier du ministère de l'Intérieur, avec la date de la circulaire et l'issue des recours
- ~~ParlGov pour les résultats électoraux et les compositions gouvernementales~~ — **hors v0** : aucun écran ne les consomme. Reste une table de correspondance employée hors pipeline

### v0.4 — le registre d'entités *(le vrai travail, et le contrat des briques suivantes)*

- [x] Identifiants stables de partis, réconciliant : groupe parlementaire AN, `party_id` CHES, code Manifesto, code nuance Intérieur
- [x] **Groupe parlementaire ≠ parti.** La distinction est explicite dans le modèle, pas contournée
- [x] Périodes de validité : fusions, scissions, changements de nom
- [x] Registre versionné, lisible à la main, corrigeable par pull request

C'est la brique que les briques 1 à 3 consommeront. Se tromper ici se paie
partout ensuite.

### v0.5 — le registre de preuves

- [ ] JSONL en ajout seul, une ligne par position mesurée
- [ ] Chaque ligne porte : entité, valeur, méthode, source, date de la source, date de calcul
- [ ] Idempotent, dédupliqué par identifiant
- [ ] Rejouable : la reconstruction complète depuis les sources brutes donne le même fichier

Même mécanisme que le `trend_ledger.jsonl` du pipeline de veille, y compris la
possibilité de reconstituer l'historique par minage des révisions git.

### v0.6 — le graphe

- [ ] Une bande par parti, trois marqueurs — votes, experts, administration — jamais moyennés
- ~~Curseur temporel~~ — **hors v0** : une seule législature. Le manifeste porte déjà une liste d'instantanés, aucun champ ne changera le jour où le curseur arrivera
- [ ] Clic sur une position → les preuves, la méthode, le lien vers la source brute
- ~~Vue des dérives~~ — **hors périmètre, pas seulement hors v0** : aucun artefact ne porte d'écart entre deux dates (invariant I18). Deux législatures s'affichent côte à côte, jamais superposées

### v0.7 — publication

- [x] Dépôt public, cron hebdomadaire — **exécuté le 2026-08-27**, archives déposées en release. GitHub Pages : reste à basculer la source sur Actions
- [x] Suite de tests hors ligne, zéro jeton, zéro réseau
- [ ] Page de méthode et procédure de correction accessibles depuis chaque écran
- [ ] Entrée « Contrepoint » dans la navbar du site personnel, en redirection
- [x] Aucune promotion

---

## Brique 1 — Presse écrite

**But : sur un sujet, voir la couverture des angles qu'on ne lit pas, et les
points où les rédactions se contredisent.**

Ancrée sur la brique 0.

- [ ] Ingestion RSS, spectre complet des rédactions à flux exploitable
- [ ] Regroupement des articles en sujets : TF-IDF/BM25, cosinus, agglomératif, fenêtre 48 h, renforcé par le recouvrement d'entités nommées
- [ ] Extraction d'acteurs par **gazetteer** issu de la brique 0 — appariement de chaînes, auditable ligne par ligne, pas de modèle statistique
- [ ] Espace de citation : matrice `rédaction × acteur` → ACP → axe émergent
- [ ] Biais de sélection : quels sujets couverts par qui, et surtout par qui pas
- [ ] Divergences chiffrées : extraction et normalisation des quantités, regroupement par grandeur désignée
- [ ] Divergences de cadrage : log-odds ratio à prior de Dirichlet informatif (Monroe, Colaresi & Quinn 2008) — les termes distinctifs sont les leurs, pas une paraphrase
- [ ] Digest hebdomadaire
- [ ] Requête ponctuelle sur une URL, réutilisant le même index
- [ ] Récupération du corps d'article : métriques dérivées conservées, **texte intégral jamais stocké**

**Piège nommé d'avance : citer n'est pas approuver.** Une rédaction cite ses
adversaires pour les attaquer. L'axe mesure l'attention accordée, ce qui suffit à
l'usage anti-bulle — il indique de qui on entendra parler. Il ne sera jamais
étiqueté « biais ». Nom retenu : **espace de citation**.

---

## Brique 2 — YouTube

**But : cartographier le canal d'où vient l'essentiel de l'actualité consommée,
et que rien d'existant ne couvre.**

- [ ] Flux RSS de chaîne (gratuits, stables)
- [ ] Transcripts — sous-titres automatiques, qualité variable, à mesurer avant de construire dessus
- [ ] Réutilisation telle quelle des étapes acteurs / espace de citation / cadrage de la brique 1
- [ ] Traiter les chaînes comme une nature de source distincte : un vidéaste n'est pas une rédaction, l'agrégation ne les mélange pas

---

## Brique 3 — TV / radio

**But : compléter le tableau, et le rendre lisible par quelqu'un qui ne lit pas la presse en ligne.**

- [ ] Pages d'actualité écrites des rédactions audiovisuelles, parsers dédiés
- [ ] Accepter le coût de maintenance HTML, ou renoncer explicitement à une source plutôt que la laisser pourrir en silence

---

## Hors périmètre, délibérément

- Noter des journalistes nommément. Risque maximal, apport minimal.
- Un score de fiabilité, de crédibilité ou de véracité. Aucun axe à pôle négatif : c'est ce qui transforme une mesure en diffamation.
- Résumer un article. L'outil montre les titres et les écarts, le lecteur lit.
- Comptes utilisateurs, commentaires, modération. Chacun impose une charge récurrente.
- Réseaux sociaux. Pas d'accès gratuit stable.
- Aucune détection ni prédiction de désinformation, jamais.

---

## Risques

| Risque | Note |
|---|---|
| Abandon | Une personne, pas de temps libre. Un jeu de données de positionnement figé à mi-chemin est pire qu'absent : ses chiffres périmés continuent d'être cités comme actuels. Le site doit afficher « données arrêtées le … » plutôt que se taire, et le jeu de données rester utilisable seul, site mort. |
| Registre d'entités | Une erreur d'appariement parti / groupe / code se propage dans toutes les briques. C'est le point où la relecture manuelle est justifiée. |
| Sparsité des scrutins | Peu de scrutins publics, participation faible — médiane de 133 votants pour 577 sièges. Le risque « l'axe mesure surtout qui était présent » a été mesuré et écarté : la corrélation des coordonnées est de 1,0000 entre corpus complet et corpus filtré à 50 votants, et c'est le seuil élevé qui casse l'ordre des blocs ([ADR 0003](docs/adr/0003-arbitrages-de-coherence.md) §2). |
| Discipline de vote | Fait remonter le groupe, pas l'idéologie individuelle. Limite à afficher, pas à masquer. |
| Flux RSS tronqués | Beaucoup de rédactions ne servent que titre et chapô. Les divergences chiffrées ont besoin du corps de texte. |
| Parsers HTML | La ligne de maintenance qui tue les projets solo. Raison de l'ordre des briques. |
| Attaque juridique | La méthode publique et les preuves traçables sont la défense. Une attaque fondée est une correction à faire ; une attaque de principe se répond par les sources. |
| Réputation de l'auteur | L'unique actif du projet. Un outil qui prétend mesurer objectivement ne survit pas à un auteur soupçonné de s'en servir contre des personnes. |

---

## Une règle permanente

Aucune assertion qui ne soit un calcul reproductible sur une source publique
citée et datée. Quand la mesure n'existe pas, on n'affiche rien — on n'affiche
jamais une estimation à sa place.
