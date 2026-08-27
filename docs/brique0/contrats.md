# Contrats de données

Roadmap v0.5 et v0.6. Ce document est l'interface entre le pipeline et le front,
et entre la brique 0 et les briques 1 à 3. Ce qui y est écrit ne se renomme pas
sans **majeure** au sens de [ADR 0000 §6](../adr/0000-perimetre-brique0.md).

**Contrat 0.2.0 — majeure.** `entrees[].items` gagne quatre propriétés
**requises** : `producteur`, `derniere_mise_a_jour`, `citation` et
`empreinte_contenu_sha256`. Rendre un champ obligatoire est une majeure au sens
de l'ADR 0000 §6 (§5, ligne « champ rendu obligatoire »), et la clé de
déduplication du §3 change de contenu : elle porte désormais l'empreinte de
**contenu** des entrées, plus celle de l'archive. Les trois `id` de la famille
`votes` du §2.6 sont donc recalculés ; ceux des familles `experts` et
`administratif` sont inchangés, leurs sources étant des fichiers uniques dont
les deux empreintes coïncident. Trois motifs, un seul contrat : la mention de
paternité exigée par la Licence Ouverte n'était pas portée par le contrat
(§2.1), l'empreinte d'archive n'atteste pas de la donnée (§2.8), et une source
peut exiger une citation par jeu de données (§2.1). `schema` reste
`contrepoint/preuve/1` : aucune ligne n'est publiée à ce jour — `logiciel.commit`
est nul partout (§10, blocage 3) et `data/preuves/positions.jsonl` n'existe pas
— donc aucune preuve déjà publiée n'est invalidée, et un `contrepoint/preuve/2`
n'aurait rien à faire cohabiter. La version du contrat suit la règle 3 de
l'ADR 0000 §6 : en `0.x`, une majeure incrémente le rang mineur, `0.1.0` →
`0.2.0`.

Schémas formels : [`schemas/`](../../schemas/). Toutes les valeurs d'exemple
sont réelles, reprises de [positionnement.md](positionnement.md),
[ingestion-votes.md](ingestion-votes.md) et
[registre-entites.md](registre-entites.md), avec leurs empreintes ; les tailles
citées sont mesurées sur les artefacts construits pour ce document, pas
estimées.

---

## 1. Trois artefacts, trois rôles

| Artefact | Chemin | Nature | Consommateur |
|---|---|---|---|
| **Registre de preuves** | `data/preuves/positions.jsonl` | JSONL en ajout seul, une ligne = une position mesurée | le dépôt, les tiers, la rejouabilité |
| **Manifeste** | `public/api/index.json` | Un objet, la liste des instantanés disponibles | le front, au chargement |
| **Instantané** | `public/api/instantanes/<id>.json` | Un graphe à une date | le front, à la sélection d'une date |
| **Éclat de preuves** | `public/api/preuves/<xx>.json` | Les lignes du registre référencées, groupées par préfixe d'identifiant | le front, au clic sur un marqueur |

| Schéma formel | Ce qu'il valide |
|---|---|
| `schemas/preuve-1.schema.json` | une ligne du registre, et par `$ref` chaque ligne servie au front |
| `schemas/manifeste-1.schema.json` | `public/api/index.json` |
| `schemas/instantane-1.schema.json` | un instantané, marqueurs compris |
| `schemas/eclat-preuves-1.schema.json` | un éclat de preuves |

JSON Schema 2020-12, `additionalProperties: false` partout : c'est la règle du
producteur strict du §5.1. Les cinq lignes du §2.6 sont la fixture de ce
contrôle — elles valident contre `preuve-1.schema.json` et leur `id` se recalcule
par la commande du §3. Ce que le schéma ne peut pas dire est énoncé dans sa
`description` et vérifié par le validateur du pipeline : l'ordre des clés (§7),
le calcul de `id` (§3), les invariants du §6.

Le registre de preuves est le contrat. Les trois autres en sont des projections
et ne portent **aucune valeur qui ne soit d'abord une ligne du registre** — un
marqueur sans ligne ne s'affiche pas (docs/definition-of-done.md, point 15).

Ce que le front télécharge pour afficher le graphe complet : **11,0 Ko** —
manifeste 928 octets, instantané 10 064 octets, 16 bandes, 32 marqueurs, mesurés
sur les artefacts construits pour ce document. Le registre entier ne descend
jamais dans le navigateur.

---

## 2. Le registre de preuves

### 2.1 La ligne

Une ligne = une valeur, pour une entité, par une méthode, sur une période
d'observation, à partir d'entrées empreintées. Ordre des clés imposé (§7).

| Champ | Type | Contrainte |
|---|---|---|
| `schema` | chaîne | `contrepoint/preuve/1`, littéral |
| `id` | chaîne | 64 hexadécimaux minuscules, identifiant de déduplication (§3) |
| `contrat` | chaîne | `^\d+\.\d+\.\d+$`, version du contrat de sortie qui a produit la ligne |
| `famille` | énumération | `votes` \| `experts` \| `administratif` |
| `entite` | chaîne | `id` du registre d'entités : `parti.*`, `coalition.*` ou `groupe.an17.*` |
| `valeur` | nombre \| `null` | position sur `echelle`, arrondie à `echelle.decimales` |
| `valeur_code` | chaîne \| `null` | valeur non numérique (code de nuance) |
| `echelle` | objet | `id`, `min`, `max`, `decimales`, `libelle` (§2.3) |
| `motif_code` | énumération \| `null` | obligatoire si `valeur` et `valeur_code` sont nuls (§2.4) |
| `motif` | chaîne \| `null` | ≤ 140 caractères, une phrase ; même condition que `motif_code` |
| `dispersion` | objet \| `null` | `effectif`, `iqr`, `ecart_type_reechantillonnage`. Jamais de minimum, de maximum ni de valeur extrême : une borne d'étendue est la coordonnée d'un membre identifiable |
| `observation` | objet | `debut`, `fin`, dates, **bornes incluses** — la période que la mesure décrit |
| `date_source` | date | date de publication ou `last-modified` de la source **déterminante** de la ligne. Ce n'est pas la mention de paternité : celle-ci est par entrée, `entrees[].derniere_mise_a_jour` |
| `date_calcul` | horodatage | RFC 3339 en UTC, seul champ dérivé d'une horloge (§8) |
| `methode` | objet | `id`, `version`, `parametres` (§2.5) |
| `epingles` | tableau | fonctions fixes externes épinglées : `{nom, version}`. Vide en brique 0 |
| `entrees` | tableau | ≥ 1 : `{source, url, producteur, derniere_mise_a_jour, citation, empreinte_sha256, empreinte_contenu_sha256, recupere_le}` (§2.8) |
| `logiciel` | objet | `version`, `commit` (40 hexadécimaux ou `null`) |

**Il n'existe pas de champ de position hors d'une ligne.** Toute valeur publiée
porte donc sa famille, son échelle, sa source et sa date, et il n'y a nulle part
où écrire un nombre qui n'appartienne à aucune famille. C'est la forme
structurelle de l'interdiction de moyenner les familles
(docs/methode.md §3) : ce n'est pas une règle de revue, c'est l'absence du
champ où l'écrire.

**La mention de paternité est portée par l'entrée, pas par le projet.** La
Licence Ouverte exige « sa source, a minima le nom du Producteur, et la date de
sa dernière mise à jour ». `source` est un code interne — `an_scrutins_17` n'est
le nom d'aucun producteur — et `recupere_le` est la date à laquelle Contrepoint a
téléchargé, pas celle à laquelle la source a mis à jour. Trois propriétés
requises portent donc l'obligation là où elle se vérifie :

| Propriété de `entrees[]` | Ce qu'elle porte |
|---|---|
| `producteur` | le nom du Producteur tel que la source le publie : `Assemblée nationale`, `Ministère de l'intérieur`, `Chapel Hill Expert Survey` |
| `derniere_mise_a_jour` | la date de dernière mise à jour **de cette source**, déclarée par elle |
| `citation` | la citation académique exigée par cette source, mot pour mot, ou `null` si elle n'en exige pas |

`date_source`, au niveau de la ligne, ne pouvait pas tenir ce rôle : une ligne
de la famille `votes` cite trois entrées mises à jour à trois moments
différents, et une date unique en écrase deux. La date de mise à jour est une
propriété de la source, donc de l'entrée.

**`citation` est une exigence de la source, pas une cession de droits.** Une
source peut n'accorder aucun droit de republication et exiger malgré tout d'être
citée ; le champ consigne cette exigence et rien de plus. Il est par jeu de
données parce que c'est ainsi que les sources le formulent : le fichier de
tendance 1999-2024 de CHES et l'enquête 2024 portent chacun sa propre ligne de
citation, et rien ne garantit qu'un jeu futur porte celle du précédent. Une
mention globale du projet aurait attribué à un jeu la citation d'un autre.

### 2.2 L'objet mesuré n'est pas le même selon la famille

| Famille | Préfixe de `entite` imposé | Ce qui est mesuré |
|---|---|---|
| `votes` | `groupe.` | un **groupe parlementaire** dans une chambre, sur une législature |
| `experts` | `parti.` \| `coalition.` | un **parti** jugé par des politologues |
| `administratif` | `parti.` \| `coalition.` | les **candidatures** qu'une administration a codées |

Le contrôle est mécanique (§6, I4). Il interdit à la source de fait la plus
tentante des confusions : attribuer à un parti la position de vote d'un groupe
sans passer par le registre. Le rapprochement est fait au niveau du front, par
la règle de construction des bandes du §4.3, qui est écrite et vérifiable.

### 2.3 Les échelles sont nommées et closes

| `echelle.id` | `min` | `max` | `decimales` | Famille |
|---|---|---|---|---|
| `votes_lfi_rn` | −1,0000 | +1,0000 | 4 | `votes` |
| `ches_lrgen_0_10` | 0,00 | 10,00 | 2 | `experts` |
| `nuance_leg2024` | `null` | `null` | `null` | `administratif` |

`min` et `max` sont les bornes de **graduation**, et une valeur en dehors est un
refus bloquant, pas un dépassement toléré. Pour `votes_lfi_rn` les bornes sont
celles de l'ancrage : les 12 médianes de groupe mesurées tiennent dans
[−1,0000 ; +1,0000] par construction (positionnement.md §6), et un groupe qui en
sortirait signalerait que les deux ancres ne couvrent plus le spectre — cas où
le pipeline doit s'arrêter et un humain trancher, jamais publier.

**L'identifiant d'échelle encode l'ancrage.** Inverser la convention de signe ne
modifie pas `votes_lfi_rn`, cela crée `votes_rn_lfi` : les lignes déjà publiées
restent interprétables, ce que l'ADR 0000 §6 exige d'une majeure. Deux marqueurs
de familles différentes ne partagent jamais un `echelle.id` (§6, I6).

### 2.4 L'absence est dite, avec son code

`valeur` et `valeur_code` nuls ⟹ `motif_code` et `motif` non nuls, et
réciproquement interdits sinon. Même convention que le registre d'entités
(registre-entites.md §3.6, règle V9).

| `motif_code` | Ce qu'il dit | Cas réel |
|---|---|---|
| `hors_source` | l'entité n'existe pas dans cette source | UDR, absent de CHES 2024 : créé après le terrain |
| `sous_seuil_de_publication` | la mesure existe, elle n'est pas publiable | LIOT, IQR 0,687 pour un maximum de 0,25 |
| `source_indeterminee` | la source ne tranche pas | Les Écologistes : `ECO` ou `VEC`, non tranché |
| `source_non_recuperable` | la source existe et n'est pas accessible par script | grille de nuances, annexe PDF Légifrance en 403 |

Le code existe parce que le front doit distinguer sans lire la prose : une
mesure retenue puis non publiée occupe une bande (§4.3), une entité absente
d'une source n'en occupe pas. `sous_seuil_de_publication` est la seule
occurrence où `dispersion` est renseignée sans `valeur` : les chiffres qui
justifient la non-publication sont publiés, la valeur non.

### 2.5 `methode`

| Champ | Contrainte |
|---|---|
| `id` | `votes_rang1_ancre` \| `ches_lrgen` \| `nuance_constatee` |
| `version` | semver ; **tout changement susceptible de déplacer une valeur l'incrémente** |
| `parametres` | objet, clés `^[a-z0-9_]+$`, valeurs chaîne, nombre, booléen ou tableau de chaînes |

`parametres` porte ce qui détermine le résultat et rien d'autre. Pour
`votes_rang1_ancre` : les deux ancres, le codage, le filtre de scrutins, le
nombre d'itérations, les décomptes retenus et écartés. La règle sur
`version` n'est pas cosmétique : elle est ce qui empêche l'invariant I8 de
transformer une correction de méthode en ligne silencieusement rejetée par la
déduplication.

### 2.6 Cinq lignes réelles

Reformatées ici sur plusieurs lignes pour la lecture ; dans le fichier, une
ligne est une ligne. Empreintes d'archive réelles, mesurées le 2026-08-27
(`aa767a2a…` Scrutins L17, `bbecd012…` AMO30, `1c1ec053…` CHES 2024,
`bcc062b9…` nuances 2024, `991c389d…`
`data/registre/partis.exemple.json`), empreintes de contenu calculées par la
méthode du §2.8. L'ancienne valeur `c5e405f1…` de l'archive des scrutins **ne se
reproduit pas** : deux récupérations indépendantes le 2026-08-27 donnent
`aa767a2a…` pour 26 317 479 octets et un `last-modified` du 2026-08-27
(verification-2026-08-27.md §0), une troisième le confirme.

Les `id` sont ceux que produit la commande du §3 sur ces lignes, vérifiés un par
un. Le §2.6 étant la fixture du contrôle de schéma, un `id` faux y serait pire
qu'absent.

```json
{"schema":"contrepoint/preuve/1","id":"66b931a3da7ed525ff81d68e361f276c5a9ddc50ac39042a0a9cbb39be66bbbb","contrat":"0.2.0","famille":"votes","entite":"groupe.an17.lfi-nfp","valeur":-1.0000,"valeur_code":null,"echelle":{"id":"votes_lfi_rn","min":-1.0000,"max":1.0000,"decimales":4,"libelle":"Votes XVIIe législature, unités médianes LFI-NFP à RN"},"motif_code":null,"motif":null,"dispersion":{"effectif":73,"iqr":0.0470,"ecart_type_reechantillonnage":0.0000},"observation":{"debut":"2024-10-08","fin":"2026-07-21"},"date_source":"2026-08-27","date_calcul":"2026-08-27T00:00:00Z","methode":{"id":"votes_rang1_ancre","version":"1.0.0","parametres":{"ancre_droite":"groupe.an17.rn","ancre_gauche":"groupe.an17.lfi-nfp","codage":"pour=+1;contre=-1;abstention=0;non_votant=manquant;absent=manquant","filtre_scrutins":"minorite_non_vide","iterations_als":300,"scrutins_ecartes":455,"scrutins_retenus":7979}},"epingles":[],"entrees":[{"source":"registre_partis","url":"https://raw.githubusercontent.com/bourbask/contrepoint/v0.1.0/data/registre/partis.json","producteur":"Contrepoint","derniere_mise_a_jour":"2026-08-27","citation":null,"empreinte_sha256":"991c389d34573dc2dbeebd022145b9b85030fc5ace1576e536891d5c8ce19308","empreinte_contenu_sha256":"991c389d34573dc2dbeebd022145b9b85030fc5ace1576e536891d5c8ce19308","recupere_le":"2026-08-27"},{"source":"an_scrutins_17","url":"https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip","producteur":"Assemblée nationale","derniere_mise_a_jour":"2026-08-27","citation":null,"empreinte_sha256":"aa767a2a05f25e38badca738af3535cb9ab89b5fa95d0810a60af05eab1e4721","empreinte_contenu_sha256":"c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c","recupere_le":"2026-08-27"},{"source":"an_organe","url":"https://data.assemblee-nationale.fr/static/openData/repository/17/amo/tous_acteurs_mandats_organes_xi_legislature/AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip","producteur":"Assemblée nationale","derniere_mise_a_jour":"2026-08-27","citation":null,"empreinte_sha256":"bbecd01274d2bc9f46fcaa276b06868862ae7680131da3162e35b5cbef663061","empreinte_contenu_sha256":"0f49c00a8227d6cb8e658d374bacfec35238fe4e2dd6305f7df6ac4f515c5de6","recupere_le":"2026-08-27"}],"logiciel":{"version":"0.1.0","commit":null}}
```

```json
{"schema":"contrepoint/preuve/1","id":"cb2887271d7427e2a486d50e8a636b109ac41fe6146f6f439a8633417c345199","contrat":"0.2.0","famille":"votes","entite":"groupe.an17.rn","valeur":1.0000,"valeur_code":null,"echelle":{"id":"votes_lfi_rn","min":-1.0000,"max":1.0000,"decimales":4,"libelle":"Votes XVIIe législature, unités médianes LFI-NFP à RN"},"motif_code":null,"motif":null,"dispersion":{"effectif":129,"iqr":0.0520,"ecart_type_reechantillonnage":0.0000},"observation":{"debut":"2024-10-08","fin":"2026-07-21"},"date_source":"2026-08-27","date_calcul":"2026-08-27T00:00:00Z","methode":{"id":"votes_rang1_ancre","version":"1.0.0","parametres":{"ancre_droite":"groupe.an17.rn","ancre_gauche":"groupe.an17.lfi-nfp","codage":"pour=+1;contre=-1;abstention=0;non_votant=manquant;absent=manquant","filtre_scrutins":"minorite_non_vide","iterations_als":300,"scrutins_ecartes":455,"scrutins_retenus":7979}},"epingles":[],"entrees":[{"source":"registre_partis","url":"https://raw.githubusercontent.com/bourbask/contrepoint/v0.1.0/data/registre/partis.json","producteur":"Contrepoint","derniere_mise_a_jour":"2026-08-27","citation":null,"empreinte_sha256":"991c389d34573dc2dbeebd022145b9b85030fc5ace1576e536891d5c8ce19308","empreinte_contenu_sha256":"991c389d34573dc2dbeebd022145b9b85030fc5ace1576e536891d5c8ce19308","recupere_le":"2026-08-27"},{"source":"an_scrutins_17","url":"https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip","producteur":"Assemblée nationale","derniere_mise_a_jour":"2026-08-27","citation":null,"empreinte_sha256":"aa767a2a05f25e38badca738af3535cb9ab89b5fa95d0810a60af05eab1e4721","empreinte_contenu_sha256":"c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c","recupere_le":"2026-08-27"},{"source":"an_organe","url":"https://data.assemblee-nationale.fr/static/openData/repository/17/amo/tous_acteurs_mandats_organes_xi_legislature/AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip","producteur":"Assemblée nationale","derniere_mise_a_jour":"2026-08-27","citation":null,"empreinte_sha256":"bbecd01274d2bc9f46fcaa276b06868862ae7680131da3162e35b5cbef663061","empreinte_contenu_sha256":"0f49c00a8227d6cb8e658d374bacfec35238fe4e2dd6305f7df6ac4f515c5de6","recupere_le":"2026-08-27"}],"logiciel":{"version":"0.1.0","commit":null}}
```

```json
{"schema":"contrepoint/preuve/1","id":"f625c63528e9b8ac7f78e0a1258b19eedd92d5143438ebe94858c935a4d35f56","contrat":"0.2.0","famille":"votes","entite":"groupe.an17.liot","valeur":null,"valeur_code":null,"echelle":{"id":"votes_lfi_rn","min":-1.0000,"max":1.0000,"decimales":4,"libelle":"Votes XVIIe législature, unités médianes LFI-NFP à RN"},"motif_code":"sous_seuil_de_publication","motif":"Dispersion interne au-delà du seuil publié : IQR 0,687 pour un maximum de 0,25.","dispersion":{"effectif":25,"iqr":0.6870,"ecart_type_reechantillonnage":0.0176},"observation":{"debut":"2024-10-08","fin":"2026-07-21"},"date_source":"2026-08-27","date_calcul":"2026-08-27T00:00:00Z","methode":{"id":"votes_rang1_ancre","version":"1.0.0","parametres":{"ancre_droite":"groupe.an17.rn","ancre_gauche":"groupe.an17.lfi-nfp","codage":"pour=+1;contre=-1;abstention=0;non_votant=manquant;absent=manquant","filtre_scrutins":"minorite_non_vide","iterations_als":300,"scrutins_ecartes":455,"scrutins_retenus":7979}},"epingles":[],"entrees":[{"source":"registre_partis","url":"https://raw.githubusercontent.com/bourbask/contrepoint/v0.1.0/data/registre/partis.json","producteur":"Contrepoint","derniere_mise_a_jour":"2026-08-27","citation":null,"empreinte_sha256":"991c389d34573dc2dbeebd022145b9b85030fc5ace1576e536891d5c8ce19308","empreinte_contenu_sha256":"991c389d34573dc2dbeebd022145b9b85030fc5ace1576e536891d5c8ce19308","recupere_le":"2026-08-27"},{"source":"an_scrutins_17","url":"https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip","producteur":"Assemblée nationale","derniere_mise_a_jour":"2026-08-27","citation":null,"empreinte_sha256":"aa767a2a05f25e38badca738af3535cb9ab89b5fa95d0810a60af05eab1e4721","empreinte_contenu_sha256":"c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c","recupere_le":"2026-08-27"},{"source":"an_organe","url":"https://data.assemblee-nationale.fr/static/openData/repository/17/amo/tous_acteurs_mandats_organes_xi_legislature/AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip","producteur":"Assemblée nationale","derniere_mise_a_jour":"2026-08-27","citation":null,"empreinte_sha256":"bbecd01274d2bc9f46fcaa276b06868862ae7680131da3162e35b5cbef663061","empreinte_contenu_sha256":"0f49c00a8227d6cb8e658d374bacfec35238fe4e2dd6305f7df6ac4f515c5de6","recupere_le":"2026-08-27"}],"logiciel":{"version":"0.1.0","commit":null}}
```

```json
{"schema":"contrepoint/preuve/1","id":"ae4508151abc6386ee763aaf75aadb77c24010315a7810db689102cdff6cddb3","contrat":"0.2.0","famille":"experts","entite":"parti.rn","valeur":8.82,"valeur_code":null,"echelle":{"id":"ches_lrgen_0_10","min":0.00,"max":10.00,"decimales":2,"libelle":"CHES 2024, variable lrgen, échelle 0 à 10"},"motif_code":null,"motif":null,"dispersion":null,"observation":{"debut":"2024-01-01","fin":"2024-12-31"},"date_source":"2026-08-04","date_calcul":"2026-08-27T00:00:00Z","methode":{"id":"ches_lrgen","version":"1.0.0","parametres":{"colonne":"lrgen","pays":6,"vague":"2024"}},"epingles":[],"entrees":[{"source":"ches_2024","url":"https://github.com/chesdata/chesdata.github.io/releases/download/ches-europe/CHES_2024_final_v2.csv","producteur":"Chapel Hill Expert Survey","derniere_mise_a_jour":"2026-08-04","citation":"Rovny, Jan, Jonathan Polk, Ryan Bakker, Liesbet Hooghe, Seth Jolly, Gary Marks, Marco Steenbergen, and Milada Anna Vachudova. 2025. \"The 2024 Chapel Hill Expert Survey on political party positioning in Europe: Twenty-five years of party positional data.\" Electoral Studies 97 (October). doi:10.1016/j.electstud.2025.102981","empreinte_sha256":"1c1ec0532afa2a0a13317122cbbe40eb9ff35425191892d1fff24fbef6acc6a8","empreinte_contenu_sha256":"1c1ec0532afa2a0a13317122cbbe40eb9ff35425191892d1fff24fbef6acc6a8","recupere_le":"2026-08-27"},{"source":"registre_partis","url":"https://raw.githubusercontent.com/bourbask/contrepoint/v0.1.0/data/registre/partis.json","producteur":"Contrepoint","derniere_mise_a_jour":"2026-08-27","citation":null,"empreinte_sha256":"991c389d34573dc2dbeebd022145b9b85030fc5ace1576e536891d5c8ce19308","empreinte_contenu_sha256":"991c389d34573dc2dbeebd022145b9b85030fc5ace1576e536891d5c8ce19308","recupere_le":"2026-08-27"}],"logiciel":{"version":"0.1.0","commit":null}}
```

```json
{"schema":"contrepoint/preuve/1","id":"1d824226ce54d92799e6649907e633772b78fdb7ce993cc2a468c1ff5dfed9b0","contrat":"0.2.0","famille":"administratif","entite":"coalition.nfp","valeur":null,"valeur_code":"UG","echelle":{"id":"nuance_leg2024","min":null,"max":null,"decimales":null,"libelle":"Code de nuance attribué à une candidature, législatives 2024"},"motif_code":null,"motif":null,"dispersion":null,"observation":{"debut":"2024-06-30","fin":"2024-07-07"},"date_source":"2024-07-10","date_calcul":"2026-08-27T00:00:00Z","methode":{"id":"nuance_constatee","version":"1.0.0","parametres":{"colonne":"Nuance candidat","reference_grille":"IOMA2415630C du 2024-06-11","tour":"2"}},"epingles":[],"entrees":[{"source":"registre_partis","url":"https://raw.githubusercontent.com/bourbask/contrepoint/v0.1.0/data/registre/partis.json","producteur":"Contrepoint","derniere_mise_a_jour":"2026-08-27","citation":null,"empreinte_sha256":"991c389d34573dc2dbeebd022145b9b85030fc5ace1576e536891d5c8ce19308","empreinte_contenu_sha256":"991c389d34573dc2dbeebd022145b9b85030fc5ace1576e536891d5c8ce19308","recupere_le":"2026-08-27"},{"source":"nuance_leg2024","url":"https://static.data.gouv.fr/resources/elections-legislatives-des-30-juin-et-7-juillet-2024-resultats-definitifs-du-2nd-tour/20240710-170728/resultats-definitifs-par-circonscription.csv","producteur":"Ministère de l'intérieur","derniere_mise_a_jour":"2024-07-10","citation":null,"empreinte_sha256":"bcc062b9264dcf3dc1eac5463acf1e8743171b86385923e7f770fd9028271083","empreinte_contenu_sha256":"bcc062b9264dcf3dc1eac5463acf1e8743171b86385923e7f770fd9028271083","recupere_le":"2026-08-27"}],"logiciel":{"version":"0.1.0","commit":null}}
```

Ce que ces cinq lignes couvrent : une valeur d'ancre exacte, une seconde ancre
de signe opposé, une non-publication motivée avec sa dispersion, une valeur sur
une échelle de source tierce, une valeur non numérique portée par une coalition.

**Tailles mesurées.** 2320, 2315, 2416, 1833 et 1685 octets, dont 1345, 1345, 1345, 1165 et 934
pour `entrees` — soit 58 % d'une ligne de la famille `votes`. Mesure faite sur
les lignes ci-dessus par `python3 -c 'import sys;[print(len(l.rstrip(chr(10)).encode()))
for l in sys.stdin]'`. Les chiffres antérieurs — 1 823, 1 817, 1 919, 1 148 et
1 321 — ne se reproduisaient pas sur les lignes publiées ici, qui mesuraient
1 783, 1 778, 1 879, 1 151 et 1 324 octets avant cette majeure ; ils sont
corrigés.

La v0 émet 35 lignes (32 marqueurs plus 3 absences documentées), soit de l'ordre
de 74 Ko à la moyenne de ces cinq lignes. Le coût est assumé : une ligne de preuve
se lit seule, sans catalogue à côté, et c'est ce qui la rend citable. *Plafond
nommé* : si le registre devient volumineux, `entrees[].url` se résout par un
catalogue commité indexé sur l'empreinte d'archive, et `entrees` retombe à
`{source, empreinte_sha256, empreinte_contenu_sha256}` — mineure, sans effet sur
`id` puisque seule l'empreinte de contenu entre dans la clé. `producteur`,
`derniere_mise_a_jour` et `citation` ne se délèguent à un catalogue qu'à la
condition que ce catalogue soit publié avec les artefacts : ce sont des
obligations de licence, pas des commodités.

### 2.7 Ajout seul

Le fichier n'est jamais réécrit. Une exécution **ajoute** les lignes dont l'`id`
n'est pas déjà présent, dans l'ordre `(famille, entite, methode.id, id)`. La
conséquence est vérifiable et l'est (I13) : le fichier antérieur est un préfixe
**octet pour octet** du nouveau. Aucun tri global, aucune réindexation, aucune
suppression — une valeur qui bouge est une ligne de plus, jamais une ligne
modifiée (ADR 0000 §6, règle 2).

### 2.8 Les deux empreintes d'une entrée

Une empreinte d'archive atteste du **conteneur**, pas de la donnée. Une
republication à contenu identique change les octets d'un ZIP — horodatages
d'entrées, ordre des entrées, niveau de compression — sans que la donnée ait
bougé. Un pipeline qui décide d'une ré-émission sur cette seule empreinte
ré-émettrait des lignes de preuve sans cause : c'est le défaut que la clé du §3
ne doit pas porter. Chaque entrée porte donc deux empreintes, et le contrat dit
laquelle sert à quoi.

| Propriété | Ce qu'elle atteste | Ce qu'elle décide |
|---|---|---|
| `empreinte_sha256` | ce qui a été téléchargé, octet pour octet | l'intégrité du téléchargement, et le contrôle contre l'empreinte publiée par la source. **Hors de la clé du §3** |
| `empreinte_contenu_sha256` | ce qui a été calculé — la donnée, indépendamment de son emballage | la ré-émission d'une ligne. **Seule des deux dans la clé du §3** |

**Méthode de calcul de `empreinte_contenu_sha256`.** Trois règles, sans marge
d'interprétation :

1. Décompresser l'archive dans un répertoire quelconque.
2. Lister **tous les fichiers réguliers** obtenus, par chemin **relatif à la
   racine de l'archive**, et les trier en **ordre d'octets** — `LC_ALL=C`, pas
   la collation de la locale.
3. SHA-256 de la concaténation de leurs contenus, dans cet ordre, sans
   séparateur.

Pour une source d'un **seul** fichier — un CSV, un JSON — le contenu est le
fichier : `empreinte_contenu_sha256` = `empreinte_sha256`. C'est le cas de CHES,
du nuancier et du registre d'entités, et c'est la raison pour laquelle les `id`
des familles `experts` et `administratif` sont inchangés par cette majeure.

```sh
unzip -oq Scrutins.json.zip -d s17
cd s17 && find . -type f | sed 's|^\./||' | LC_ALL=C sort \
  | tr '\n' '\0' | xargs -0 cat | sha256sum
# c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c   8 434 fichiers
```

La méthode est reproductible par un tiers et stable quel que soit l'outil de
décompression : aucun décompresseur ne modifie ni le nom relatif ni le contenu
d'une entrée, et les trois seules libertés que prend un ZIP — ordre des entrées,
horodatages, niveau de compression — sont éliminées, la première par le tri, les
deux autres parce que seuls les contenus sont concaténés.

**Le tri est la partie fragile, et elle est mesurée.** `find … | sort` sans
`LC_ALL=C` trie selon la collation de la locale : la même archive donne
`503255ac0b39eb28ae623368c0f21f6ec30df0893d091119c7c4efbf030c2f40` sous
`fr_FR.UTF-8` et `c8457f34…` en ordre d'octets, parce que
`VTANR5L17V1.json`, `VTANR5L17V10.json` et `VTANR5L17V100.json` ne s'ordonnent
pas de la même façon dans les deux régimes. Mesuré le 2026-08-27 sur l'archive
des scrutins. La valeur retenue est celle de l'ordre d'octets, seule
reproductible hors de la locale d'un poste : `LC_ALL=C` fait partie de la
spécification, pas du confort. Le préfixe du répertoire d'extraction ne compte
pas — le tri porte sur les chemins relatifs, et les deux formes ont été
vérifiées égales.

**Ce que l'empreinte d'archive contrôle, et pourquoi elle ne peut pas être une
porte.** Mesuré le 2026-08-27 : la source sert **deux constructions du même
contenu**, et laquelle répond dépend du serveur atteint.

| Récupération | Taille | SHA-256 | MD5 | `last-modified` |
|---|---|---|---|---|
| Première récupération, deux fois de suite | 26 317 479 | `aa767a2a…` | `910e6022…` | 04:25:40 GMT |
| Plus tard le même jour | 26 317 479 | `c5e405f1…` | `1f951dea…` | 10:25:39 GMT |

Taille identique à l'octet, `etag` et `last-modified` distincts. Les deux
archives décompressées donnent 8 434 fichiers, **zéro différence** (`diff -rq`),
et la **même empreinte de contenu** `c8457f34…`. Le MD5 publié par la fiche suit
celle des deux constructions que son propre serveur voit — il n'y a donc aucune
divergence à trancher, et les deux valeurs sont exactes.

Conséquence directe, et c'est la justification empirique du §3 : une empreinte
d'archive **n'est pas une propriété de la donnée**. Un pipeline qui déciderait
d'une ré-émission dessus réécrirait le registre entier à chaque exécution, au
hasard du serveur. Elle est consignée à titre documentaire ; **le contrôle qui
décide est la taille annoncée, puis l'empreinte de contenu**. Le MD5 du
producteur n'est pas une porte.

Protocole et tableaux : [verification-2026-08-27.md](verification-2026-08-27.md) §0.

---

## 3. L'identifiant de déduplication

```
cle = famille ␟ entite ␟ observation.debut ␟ observation.fin
      ␟ methode.id ␟ methode.version ␟ canonique(methode.parametres)
      ␟ empreintes de CONTENU des entrees — entrees[].empreinte_contenu_sha256 —
        triées par ordre lexicographique, jointes par ","

id  = sha256(cle en UTF-8), 64 hexadécimaux minuscules
```

`␟` est U+001F. `canonique()` sérialise l'objet en JSON compact, clés triées par
point de code, sans espace. Le séparateur est un caractère de contrôle
précisément parce qu'il ne peut apparaître dans aucun champ : deux clés
différentes ne peuvent pas produire la même chaîne.

**Ce qui est dans la clé, et pourquoi.** Tout ce qui détermine la valeur :
l'objet mesuré, la période décrite, la méthode et ses paramètres, et les
empreintes de **contenu** de toutes les entrées — dont celle du registre
d'entités. Une correction d'appariement dans le registre change son contenu,
donc son empreinte de contenu, donc l'`id`, donc ré-émet les lignes concernées.
C'est exactement ce que l'ADR 0000 §6 décrit comme un patch qui ré-émet des
preuves.

**Pourquoi l'empreinte de contenu et pas celle de l'archive.** Le critère est
« tout ce qui détermine la valeur ». Les octets d'un ZIP ne déterminent aucune
valeur : ils changent à chaque reconstruction nocturne du jeu de l'Assemblée
nationale, à contenu éventuellement identique (§2.8). Les garder dans la clé
ferait de chaque republication une ré-émission de toutes les lignes `votes`,
sans qu'aucune valeur n'ait bougé — et une ré-émission parasite est le défaut
exact que cette majeure ferme. L'empreinte de contenu, elle, ne bouge que si la
donnée bouge. Elle porte donc la clé, et l'empreinte d'archive reste dans la
ligne comme trace du téléchargement.

**Effet mesuré du changement de clé.** Les trois lignes de la famille `votes` du
§2.6 changent d'`id` : leurs entrées d'archive sont des ZIP dont l'empreinte de
contenu diffère de l'empreinte d'archive. Les deux lignes des familles `experts`
et `administratif` gardent le **même** `id` : leurs sources sont des fichiers
uniques, pour lesquels les deux empreintes coïncident par définition. La clé n'a
donc pas changé de sens, elle a changé de témoin.

**Ce qui n'y est pas, et pourquoi.**

| Exclu | Raison |
|---|---|
| `valeur`, `valeur_code`, `motif` | sans quoi deux valeurs différentes auraient deux `id` différents et la déduplication ne dédupliquerait rien |
| `date_calcul` | c'est le seul champ d'horloge ; l'y mettre rendrait chaque exécution intégralement ré-émise |
| `logiciel.commit`, `logiciel.version` | un commit qui ne change aucune valeur ne doit rien ré-émettre. Le levier sémantique est `methode.version`, pas le dépôt |
| `contrat` | une majeure de contrat qui ne déplace aucune valeur ne ré-émet rien |
| `entrees[].url` | l'URL d'une source peut être redirigée sans que la donnée change ; l'empreinte de contenu, non |
| `entrees[].empreinte_sha256` | les octets d'une archive changent à chaque republication, à contenu identique. L'y laisser ré-émettrait des lignes sans cause : c'est le défaut que le §2.8 ferme |
| `entrees[].producteur` | un producteur qui change de dénomination ne déplace aucune valeur. La mention de paternité est une obligation de licence, pas un déterminant de la mesure |
| `entrees[].derniere_mise_a_jour` | une source qui republie à contenu identique avance sa date sans que la donnée bouge — même défaut que l'empreinte d'archive. Ce qu'elle atteste est déjà attesté, plus étroitement, par l'empreinte de contenu |
| `entrees[].citation` | une exigence juridique de la source ; elle ne détermine aucune valeur, et sa correction ne doit pas ré-émettre une mesure juste |

Le risque que cette exclusion crée est réel et il est fermé par un invariant :
une correction de méthode livrée **sans** incrémenter `methode.version` produit
le même `id` avec une autre valeur, et la ligne serait avalée par la
déduplication. I8 refuse ce cas, bruyamment.

Recalcul d'un `id` à la main, pour vérifier une ligne publiée :

```sh
printf '%s\x1f%s\x1f%s\x1f%s\x1f%s\x1f%s\x1f%s\x1f%s' \
  votes groupe.an17.rn 2024-10-08 2026-07-21 votes_rang1_ancre 1.0.0 \
  '{"ancre_droite":"groupe.an17.rn","ancre_gauche":"groupe.an17.lfi-nfp","codage":"pour=+1;contre=-1;abstention=0;non_votant=manquant;absent=manquant","filtre_scrutins":"minorite_non_vide","iterations_als":300,"scrutins_ecartes":455,"scrutins_retenus":7979}' \
  '0f49c00a8227d6cb8e658d374bacfec35238fe4e2dd6305f7df6ac4f515c5de6,991c389d34573dc2dbeebd022145b9b85030fc5ace1576e536891d5c8ce19308,c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c' \
  | sha256sum
# cb2887271d7427e2a486d50e8a636b109ac41fe6146f6f439a8633417c345199
```

Le dernier argument est la liste des `entrees[].empreinte_contenu_sha256`, triée,
et non celle des `empreinte_sha256` : `0f49c00a…` AMO30, `991c389d…` registre
d'entités, `c8457f34…` archive des scrutins. Les cinq `id` du §2.6 ont été
vérifiés par cette commande, un par un, avant d'être écrits.

---

## 4. Les fichiers statiques du front

### 4.1 Manifeste — `public/api/index.json`

| Champ | Contrainte |
|---|---|
| `schema` | `contrepoint/manifeste/1` |
| `contrat` | version du contrat de sortie |
| `schemas` | tableau des schémas présents dans les artefacts publiés |
| `date_arretee` | maximum des `date_calcul` des lignes référencées. **Dérivé, jamais saisi** |
| `licence` | `Licence Ouverte / Open Licence (Etalab)` |
| `mention_paternite` | `Assemblée nationale — Licence Ouverte v1.0 — données du 2026-08-27`. **Dérivée**, jamais saisie : `producteur` et `derniere_mise_a_jour` de l'entrée amont des lignes référencées (§2.1) |
| `familles` | tableau `{id, libelle, echelle}` — la légende, close, dans l'ordre d'affichage |
| `instantanes` | tableau `{id, chambre, legislature, date, url, empreinte_sha256, octets, bandes}` |
| `preuves` | `{racine, eclats, fonction}` — où sont les preuves et comment en dériver le chemin |

Le manifeste ne porte **aucune valeur mesurée**. Il porte de quoi choisir un
fichier et de quoi afficher le bandeau de date exigé par ROADMAP.md v0.7, dérivé
sans saisie humaine (ADR 0000 §6, conséquences).

Le manifeste réel de la v0, 928 octets — la longueur ne change pas avec le
passage de `contrat` à `0.2.0`, les deux chaînes ayant cinq caractères. En
revanche `instantanes[0].empreinte_sha256` porte l'empreinte d'un instantané dont
le contenu vient de changer, les `preuve` de ses marqueurs ayant été recalculés :
la valeur `c23fc935…` ci-dessous est **`A VERIFIER`**, à recalculer par
`sha256sum` sur l'instantané reconstruit (§10).

```json
{"schema":"contrepoint/manifeste/1","contrat":"0.2.0","schemas":["contrepoint/preuve/1","contrepoint/instantane/1","contrepoint/eclat-preuves/1"],"date_arretee":"2026-08-27T00:00:00Z","licence":"Licence Ouverte / Open Licence (Etalab)","mention_paternite":"Assemblée nationale — Licence Ouverte v1.0 — données du 2026-08-27","familles":[{"id":"votes","libelle":"Votes nominatifs","echelle":"votes_lfi_rn"},{"id":"experts","libelle":"Enquête d'experts","echelle":"ches_lrgen_0_10"},{"id":"administratif","libelle":"Nuance administrative","echelle":"nuance_leg2024"}],"instantanes":[{"id":"an17-2026-07-21","chambre":"AN","legislature":"17","date":"2026-07-21","url":"instantanes/an17-2026-07-21.json","empreinte_sha256":"c23fc935f89647fabff052b92497beeb3fd7518a4e7fee207d1eb1e8a379d50c","octets":10064,"bandes":16}],"preuves":{"racine":"preuves/","eclats":256,"fonction":"deux premiers caractères hexadécimaux de l'id"}}
```

Le manifeste ne porte **pas** les citations exigées par les sources : une
citation est une propriété de l'entrée qui l'exige (§2.1), et la recopier au
niveau du manifeste attribuerait à un jeu la citation d'un autre. Le front la lit
sur la ligne de preuve, avec le reste de la mention de paternité.
`mention_paternite` reste la légende de l'amont dont la licence exige une mention
au point de réutilisation, et elle est dérivée des lignes, pas écrite.

En v0, `instantanes` a **une** entrée. Le curseur temporel de la roadmap, hors
v0, est exactement « plus d'une entrée » : le front lit le manifeste, propose les
dates disponibles, et ne télécharge que l'instantané choisi. Aucun champ ne
change le jour où il arrive. Ce qu'il n'aura jamais : un champ portant un écart
entre deux instantanés — la comparaison de deux dates n'est pas défendable
(positionnement.md §7), et le contrat ne fournit pas l'emplacement pour
l'écrire.

### 4.2 Instantané — `public/api/instantanes/<id>.json`

| Champ | Contrainte |
|---|---|
| `schema` | `contrepoint/instantane/1` |
| `contrat` | version du contrat |
| `id` | `^[a-z0-9-]+$`, ex. `an17-2026-07-21` |
| `chambre`, `legislature` | `AN`, `17` |
| `date` | date de référence de l'agrégation — les groupes dont la période ne la couvre pas sont absents (ingestion-votes.md §8) |
| `date_arretee` | `date_calcul` maximale des lignes référencées par cet instantané |
| `ancrage` | `{famille, ancre_gauche, ancre_droite, note}` — `note` ≤ 140 caractères |
| `bandes` | tableau `{id, libelle, marqueurs}` |
| `sans_mesure` | tableau `{entite, libelle, motif_code, motif}` |

Un marqueur : `famille`, `echelle`, `valeur`, `valeur_code`, `libelle` (≤ 40
caractères), `motif_code`, `motif`, `dispersion` réduite à `{effectif, iqr}`,
et `preuve` — l'`id` de la ligne du registre.

`ancrage` est de la donnée et non du texte d'interface parce que c'est lui qui
rend la superposition de deux instantanés fausse : l'échelle est ancrée sur deux
groupes de **cette** législature. La légende s'écrit depuis ce champ.

Deux bandes de l'instantané réel, indentées ici pour la lecture — une bande de
parti à trois marqueurs, et une bande de groupe dont le seul marqueur porte sa
non-publication :

```json
{"id":"parti.lfi","libelle":"La France insoumise","marqueurs":[
 {"famille":"votes","echelle":"votes_lfi_rn","valeur":-1.0000,"valeur_code":null,"libelle":"Votes du groupe LFI-NFP","motif_code":null,"motif":null,"dispersion":{"effectif":73,"iqr":0.0470},"preuve":"66b931a3da7ed525ff81d68e361f276c5a9ddc50ac39042a0a9cbb39be66bbbb"},
 {"famille":"experts","echelle":"ches_lrgen_0_10","valeur":0.82,"valeur_code":null,"libelle":"CHES 2024, lrgen","motif_code":null,"motif":null,"dispersion":null,"preuve":"…"},
 {"famille":"administratif","echelle":"nuance_leg2024","valeur":null,"valeur_code":"FI","libelle":"Nuance 2024","motif_code":null,"motif":null,"dispersion":null,"preuve":"…"}]}
```

```json
{"id":"groupe.an17.liot","libelle":"LIOT","marqueurs":[
 {"famille":"votes","echelle":"votes_lfi_rn","valeur":null,"valeur_code":null,"libelle":"Votes du groupe LIOT","motif_code":"sous_seuil_de_publication","motif":"Dispersion interne au-delà du seuil publié : IQR 0,687 pour un maximum de 0,25.","dispersion":{"effectif":25,"iqr":0.6870},"preuve":"f625c63528e9b8ac7f78e0a1258b19eedd92d5143438ebe94858c935a4d35f56"}]}
```

Les deux `preuve` ci-dessus sont les `id` recalculés du §2.6 : un `id` fait
64 caractères hexadécimaux quelle que soit sa valeur, la taille de l'instantané
est donc inchangée, son empreinte non.

**Taille mesurée : 10 064 octets** pour 16 bandes et 32 marqueurs, la XVIIe
législature complète. Le front charge un fichier et affiche le graphe.

### 4.3 Règle de construction des bandes

Déterministe, calculée par le pipeline, jamais par le front :

1. Une bande par entité `parti.*` ou `coalition.*` portant au moins un marqueur avec une valeur.
2. Le marqueur `votes` d'un groupe rejoint la bande du parti que sa `composition` désigne, **si et seulement si** elle en désigne exactement un et qu'aucun autre groupe valide à la même date ne désigne ce parti. Le libellé du marqueur nomme le groupe mesuré : `Votes du groupe DR`.
3. Sinon le groupe reçoit sa propre bande : `ECOS` (deux partis déclarés, ECOLO 33 et PCF 5), `EPR` et `DEM` (composition vide, `ESBMP` ne la donne pas), `LIOT` et `NI`.
   Le libellé d'une bande est le `nom` du registre s'il tient en 40 caractères (ADR 0000 §5), sinon le `sigle` — cas de `LIOT`, dont le nom en compte 48.
4. Une bande est aussi créée pour un groupe dont le seul marqueur porte `motif_code = sous_seuil_de_publication` : la mesure existe, sa non-publication est le résultat et il s'affiche.
5. Une entité dont **aucun** marqueur ne porte de valeur et dont aucun ne porte `sous_seuil_de_publication` n'a pas de bande : elle est listée dans `sans_mesure`. Cas réel : Place publique, absente des quatre sources d'identifiants et de la grille de nuances.

Résultat en v0 : 9 bandes de parti, 5 bandes de groupe, 2 bandes de coalition,
1 entité sans mesure. La règle 2 est ce qui empêche une bande de parti de porter
deux marqueurs `votes`, et la règle 3 est ce qui empêche d'attribuer à ECOLO les
votes de 5 députés communistes (registre-entites.md §5.1).

### 4.4 Éclats de preuves — `public/api/preuves/<xx>.json`

`<xx>` = les **deux premiers caractères** de l'`id`. Le chemin se dérive de
l'identifiant du marqueur : aucun index, aucune requête préalable, un clic
télécharge un fichier. Au plus 256 éclats, contenu = un tableau de lignes du
registre.

Les lignes y sont **identiques octet pour octet** à celles du registre (I15) :
le front n'affiche jamais une preuve reformatée. En v0, un clic charge de 1,7 à
2,4 Ko — les lignes du §2.6 mesurent de 1 685 à 2 416 octets depuis que chaque
entrée porte sa mention de paternité et ses deux empreintes. Les éclats ne contiennent que les lignes référencées par au moins un
instantané du manifeste ; l'historique complet vit dans le dépôt, pas dans le
navigateur.

---

## 5. Versionnement des schémas

Trois versions, trois objets. Les confondre est la première erreur à ne pas
commettre.

| Version | Portée | Où elle vit |
|---|---|---|
| `schema` | identité structurelle d'un artefact, `contrepoint/<nom>/<majeur>` | chaque ligne, chaque fichier |
| `contrat` | contrat de sortie au sens ADR 0000 §6 | chaque ligne, chaque fichier |
| `methode.version` | l'estimateur | dans `methode` |

| Changement | `schema` | `contrat` | `id` | Front |
|---|---|---|---|---|
| Ajout d'un champ optionnel | inchangé | mineure | inchangé | l'ignore |
| Ajout d'une valeur d'énumération (`famille`, `motif_code`, `echelle.id`) | inchangé | mineure | inchangé pour l'existant | la rend sans la connaître (§5.2) |
| Ajout d'une source, d'une législature, d'une famille | inchangé | mineure | nouvelles lignes | nouvel instantané, nouveau marqueur |
| Méthode qui déplace les valeurs, schéma constant | inchangé | mineure | **change** | nouvelles preuves, mêmes écrans |
| Nouvelle convention de signe | inchangé | **majeure** | change | nouvel `echelle.id`, les anciennes lignes restent lisibles |
| Champ supprimé, renommé, resémantisé | **`/2`** | **majeure** | change | refuse et le dit (§5.2) |
| Champ rendu obligatoire | **`/2`** | **majeure** | inchangé | idem |
| Champ rendu obligatoire dans `entrees[]`, **avant toute publication de ligne** | inchangé | **majeure** | inchangé si le champ n'entre pas dans la clé | l'ignore |
| Changement du témoin de la clé de déduplication — empreinte d'archive remplacée par empreinte de contenu | inchangé | **majeure** | **change** pour les sources dont les deux empreintes diffèrent | nouvelles preuves, mêmes écrans |
| Retrait d'une famille | inchangé | **majeure** | — | marqueur absent, pas « non mesuré » |

Les deux dernières lignes sont celles de la majeure `0.2.0`. Le `schema` reste
`contrepoint/preuve/1` parce qu'aucune ligne n'est publiée : la règle « champ
rendu obligatoire ⟹ `/2` » existe pour qu'un lecteur d'une ligne déjà publiée ne
soit jamais mis en défaut, et il n'y a aucun lecteur à protéger. Le jour où une
ligne est publiée, cette exception disparaît.

Un `schema` majeur ne se rétropropage pas : le registre étant en ajout seul,
`contrepoint/preuve/1` et `contrepoint/preuve/2` **cohabitent** dans le même
fichier. Un lecteur traite chaque ligne selon le `schema` qu'elle déclare. C'est
la raison pour laquelle `schema` est sur la ligne et pas dans un en-tête de
fichier : un JSONL en ajout seul n'a pas d'en-tête à mettre à jour.

### 5.1 Producteur strict, consommateur tolérant

- **Producteur.** Refus d'écrire une clé absente du schéma, comme la règle V1 du registre d'entités. C'est ce qui empêche l'apparition silencieuse d'un champ de valorisation (docs/definition-of-done.md, point 12).
- **Consommateur.** Le front ignore toute clé inconnue, et **ne code en dur ni la liste des familles, ni celle des échelles, ni celle des `motif_code`**. Il rend un marqueur par entrée de `marqueurs`, avec la graduation de son échelle et le libellé qu'elle porte.

L'asymétrie est justifiée : le registre d'entités est édité à la main, donc la
liste blanche stricte est son garde-fou ; les artefacts de sortie sont produits
par un programme, et la tolérance du consommateur est ce qui permet d'ajouter
une famille de mesure sans redéployer le front. Cette règle a un effet de bord
utile : un front qui ne connaît pas la liste des familles ne peut pas les
moyenner, puisqu'il ne sait pas ce qu'il additionnerait.

### 5.2 Ce que le front fait devant un schéma qu'il ne connaît pas

Le manifeste déclare `schemas`. Le front compare à sa propre liste.

- Majeur inconnu pour un artefact : le front affiche **une ligne** disant que ces données exigent une version plus récente, et **ne rend aucun marqueur** issu de cet artefact.
- Il n'affiche **jamais** « non mesuré » à cette occasion : une incompatibilité de version n'est pas une absence de donnée, et les confondre ferait passer un défaut d'outil pour un résultat.
- Il ne rend jamais un artefact partiellement compris.

---

## 6. Invariants vérifiés à chaque exécution

Bloquants. Un artefact qui viole une règle n'est pas corrigé, il est refusé.

### Traçabilité

- **I1** Toute ligne porte `entrees` non vide, chaque entrée avec `source` de la liste close, `url`, `empreinte_sha256` et `empreinte_contenu_sha256` de 64 hexadécimaux, et `recupere_le`. Aucune position sans source empreintée.
- **I2** `date_source` présente, `≤ date_calcul`, et `≤ recupere_le` de chaque entrée. Aucune position sans source datée.
- **I3** Toute `source` d'`entrees` également déclarée dans le registre d'entités y porte la **même** URL et la **même** empreinte. Divergence = refus : soit la source a bougé, soit un fichier a été édité à la main.
- **I4** `entite` existe dans le registre, sa période couvre `observation.fin`, et son préfixe correspond à la famille (§2.2). Aucun identifiant d'entité inconnu du registre.
- **I5** Les deux ancres de `methode.parametres` existent dans le registre comme groupes valides à `observation.fin`. Absence = arrêt du pipeline, jamais substitution automatique (positionnement.md §5).

### Cohérence de la ligne

- **I6** `valeur` et `valeur_code` ne sont jamais tous deux non nuls. Tous deux nuls ⟹ `motif_code` et `motif` non nuls ; l'un non nul ⟹ `motif_code` et `motif` nuls.
- **I7** `echelle.id` appartient à la liste close, `valeur` tient dans `[min, max]` quand ils sont non nuls, et est écrite avec exactement `decimales` décimales. Deux familles ne partagent jamais un `echelle.id`.
- **I8** `id` recalculé depuis la ligne = `id` déclaré. Et deux lignes de même `id` portent la **même** `valeur`, `valeur_code` et `motif` — sinon une méthode a été modifiée sans incrémenter `methode.version` : refus.
- **I9** Ancrage exact : la ligne du groupe `ancre_gauche` porte `−1.0000`, celle de `ancre_droite` `+1.0000`, à 10⁻¹² près avant arrondi.
- **I10** Règle de non-publication appliquée : IQR > 0,25, écart-type de rééchantillonnage > 0,05 ou effectif < 10 ⟹ `valeur` nulle et `motif_code = sous_seuil_de_publication` (positionnement.md §6).

### Ce que le contrat interdit par construction

- **I11** Aucun nombre atteignable depuis `bandes[]` en dehors d'un `marqueurs[]`, hors `effectif`. Il n'existe donc pas d'emplacement pour une valeur agrégeant deux familles : aucune moyenne entre familles de mesure n'est représentable.
- **I12** Aucune clé ni valeur d'énumération dont le nom suggère une agrégation inter-familles — `moyenne`, `score`, `synthese`, `global`, `consensus`, `indice`. Vérifié par expression rationnelle sur les clés de tous les artefacts.
- **I13** Aucune coordonnée individuelle : aucun `entite` ne correspond à `^PA[0-9]+$` ni à `depute*`, et `grep -E '\bPA[0-9]{4,}\b'` sur les artefacts publiés est vide (ADR 0000 §2).
- **I14** Aucun terme du lexique interdit de docs/juridique.md dans un artefact publié, clés comprises.
- **I15** Le fichier antérieur du registre est un préfixe **octet pour octet** du nouveau. Ajout seul démontré, pas affirmé.
- **I16** Tout `preuve` d'un marqueur existe dans un éclat publié, la ligne y est identique octet pour octet à celle du registre, et tout éclat publié est référencé par au moins un marqueur — aucun orphelin dans les deux sens.
- **I17** `date_arretee` du manifeste et de chaque instantané = maximum des `date_calcul` des lignes référencées. Aucune valeur saisie.
- **I18** Aucun champ, dans aucun artefact, ne porte un écart, un ratio ou une flèche entre deux instantanés, deux dates ou deux législatures (positionnement.md §7).
- **I19** Aucun champ d'artefact publié ne porte une valeur extrême, un minimum, un maximum, un rang ou un quantile d'ordre autre que Q1/Q2/Q3 sur une distribution de positions individuelles. Une borne d'étendue est la coordonnée d'un membre identifiable du groupe : I13 ne l'attrape pas, un nombre n'ayant pas de préfixe `PA`.
- **I20** Aucune valeur de chaîne d'un artefact publié n'excède 200 caractères, `url`, empreintes et `entrees[].citation` exceptées — cette dernière plafonnée à 400 caractères, celle de CHES en comptant 322. L'exception est étroite et justifiée : une référence bibliographique exigée par une source n'est pas un texte de tiers republié, et le champ n'accepte rien d'autre (I23). C'est la forme opposable de l'interdiction de constituer une base de textes intégraux republiable (L.122-5-3 CPI), et elle rend le champ « corps d'article » inajoutable sans majeure explicite.

### Paternité, empreintes et citation

- **I21** Toute entrée porte `producteur` non vide et `derniere_mise_a_jour` ≤ `recupere_le`. Aucune entrée sans nom de producteur ni sans date de mise à jour de la source : la mention de paternité exigée par la Licence Ouverte est structurelle, pas un texte à maintenir. `producteur` n'est jamais égal à `source` — un code interne n'est pas un nom. `producteur` est le nom d'une **organisation** : une source dont le producteur déclaré est une personne physique n'entre pas dans le pipeline sous son nom, elle porte celui de son institution ou elle est écartée (RG-76).
- **I22** Toute entrée porte `empreinte_contenu_sha256`, et c'est la **seule** des deux empreintes qui entre dans la clé du §3. Contrôle de non-régression : deux lignes identiques par ailleurs, dont les `empreinte_sha256` diffèrent et les `empreinte_contenu_sha256` coïncident, ont le **même** `id` — l'archive a été republiée, la donnée non, rien n'est ré-émis. Pour une source d'un seul fichier, les deux empreintes sont égales ; une inégalité y est un refus.
- **I23** Une entrée dont la `source` figure dans la liste des sources à citation de [sources.md](../sources.md) porte `citation` non nulle, identique **caractère pour caractère** à la citation que cette source publie ; toute autre entrée porte `citation: null`. Une citation reformulée, abrégée ou déplacée vers une mention globale est un refus. L'exception que ce champ ouvre à RG-110 est de même portée : des noms d'auteurs d'une référence exigée, jamais un nom rattaché à une valeur mesurée.

I11, I12 et I18 sont la forme mécanique de règles éditoriales. Elles sont
énoncées ici parce qu'un contributeur pressé les casse en premier, et parce
qu'une règle vérifiée par grep coûte moins cher qu'une règle vérifiée par
relecture.

---

## 7. Forme canonique

Deux exécutions ne peuvent donner le même octet que si la sérialisation est
fixée. Elle l'est.

| Point | Règle |
|---|---|
| Encodage | UTF-8 sans BOM, accents littéraux, **aucune séquence `\u`** pour un caractère imprimable |
| Fin de ligne | LF, une fin de ligne finale, aucun espace en fin de ligne |
| JSONL | une ligne par objet, aucun espace après `:` ni `,`, clés dans l'ordre déclaré au §2.1 |
| JSON du front | même compacité, mêmes ordres de clés ; la lisibilité passe par l'outillage, pas par l'indentation |
| Nombres | notation décimale, jamais d'exposant, jamais de `+`, décimales fixées par `echelle.decimales`, y compris les zéros terminaux (`-1.0000`) |
| Entiers | `effectif`, décomptes, `pays` : sans point décimal |
| Tableaux | `entrees` triée par `empreinte_sha256` **croissante** — l'empreinte d'archive est unique par téléchargement, donc l'ordre est total, ce que l'empreinte de contenu ne garantit pas : deux entrées de contenu identique ne s'ordonneraient plus. Le tri de la clé du §3, lui, porte sur les empreintes de contenu et ne dépend pas de l'ordre du tableau — `epingles` par `nom`, `bandes` par valeur du marqueur `votes` puis par `id`, `marqueurs` dans l'ordre de `familles` du manifeste |
| Objets | jamais triés à l'écriture : l'ordre des clés est celui du schéma |

Les zéros terminaux ne survivent pas à un aller-retour par un analyseur JSON
générique : `-1.0000` relu puis réécrit donne `-1.0`. La forme canonique
n'engage donc que le **producteur**, et le front reformate à l'affichage depuis
`echelle.decimales` — raison d'être de ce champ.

---

## 8. Non-régression du déterminisme

### 8.1 La date de calcul est une entrée, pas une horloge

Le pipeline **ne lit jamais l'horloge**. `date_calcul` est fournie par
l'environnement, et son absence est une erreur, pas un défaut à combler :

```sh
CONTREPOINT_DATE_CALCUL=2026-08-27T00:00:00Z contrepoint construire --sortie /tmp/a
```

Cela ferme le point 6 de docs/definition-of-done.md sans exception à négocier :
il n'y a pas de champ à retirer avant de comparer, et le contrôle est un
`sha256sum`.

### 8.2 Les trois contrôles de l'intégration continue

```sh
# 1. Idempotence — même entrée, même date injectée, même octet
CONTREPOINT_DATE_CALCUL=$D contrepoint construire --sortie /tmp/a
CONTREPOINT_DATE_CALCUL=$D contrepoint construire --sortie /tmp/b
diff -r /tmp/a /tmp/b                                  # vide

# 2. Isolement de l'horloge — seule la date de calcul bouge
CONTREPOINT_DATE_CALCUL=2027-01-01T00:00:00Z contrepoint construire --sortie /tmp/c
contrepoint diff-hors-date_calcul /tmp/a /tmp/c        # vide
#    et : mêmes id, même nombre de lignes, mêmes valeurs

# 3. Ajout seul — l'existant est un préfixe du nouveau
head -c $(stat -c%s data/preuves/positions.jsonl) /tmp/a/positions.jsonl \
  | cmp - data/preuves/positions.jsonl                 # identiques
```

Le contrôle 1 attrape l'itération sur un ensemble non ordonné, la graine non
fixée, l'ordre du système de fichiers et la réduction flottante parallèle
(ADR 0001 §1.6). Le contrôle 2 attrape ce que le contrôle 1 masque : une lecture
d'horloge cachée qui aurait contaminé un `id` ou une valeur. Le contrôle 3
attrape la réécriture d'une ligne publiée.

**Ce que le contrôle 2 doit voir, et rien de plus** : les champs `date_calcul`
des lignes ajoutées, et les `date_arretee` dérivés. Un `id` qui bouge quand la
date bouge est un bug de la clé du §3 ; c'est le test le plus utile de tout ce
document.

### 8.3 Rejouabilité, et ce qu'elle ne couvre pas

Reconstruction complète depuis les archives du cache, à empreintes identiques :
les lignes reconstruites doivent être **identiques octet pour octet** aux lignes
commitées portant ces mêmes empreintes. La comparaison est restreinte à ces
lignes, et c'est nécessaire : les archives de l'Assemblée sont reconstruites
chaque nuit et rétroactivement modifiables (ingestion-votes.md §9e). Reproduire
un résultat exige l'archive qui l'a produit, que le cache conserve et dont
`entrees[].empreinte_sha256` porte l'empreinte. La comparaison, elle, se fait sur
`entrees[].empreinte_contenu_sha256` : c'est ce champ qui définit « ces mêmes
empreintes », puisque c'est lui qui porte la clé (§3). Une archive du cache
republiée à contenu identique reste donc utilisable, et une reconstruction qui
part d'une archive dont seuls les octets diffèrent doit retrouver les **mêmes**
lignes, `id` compris.

Une reconstruction qui produit une ligne **absente** du fichier commité, à
empreintes d'entrée identiques, est un bug bloquant. Une reconstruction qui
produit une ligne de plus **avec une nouvelle empreinte d'entrée** est le
fonctionnement normal : la source a bougé.

### 8.4 Ce qui n'est pas garanti bit pour bit

Les flottants intermédiaires. L'égalité mesurée après ancrage est de 1,6·10⁻¹⁵
sur 642 positions (positionnement.md §5) ; la promesse porte sur l'artefact
écrit, arrondi à `echelle.decimales` avant écriture. La formulation de
l'ADR 0001 §1.7 est déjà celle-là. Corollaire : l'arrondi est appliqué **une
fois**, à l'écriture de la ligne de preuve, et les projections du front recopient
la ligne au lieu de rearrondir.

---

## 9. Ce que l'arrivée des médias ne changera pas

Ce contrat est écrit pour survivre aux briques 1 à 3 sans réécriture. Ce qui
bouge alors, et à quel niveau :

| Ajout | Effet sur le contrat |
|---|---|
| Une entité de nature `media` | `entite` accepte un nouveau préfixe : mineure. Le registre d'entités gagne une `nature`, `id` reste immuable |
| La famille « espace de citation » | nouvelle valeur de `famille`, nouvel `echelle.id`, nouvelle `methode.id` : mineure. Aucun champ nouveau |
| Un embedding dans une méthode | `epingles` porte déjà `{nom, version}` : rien à ajouter (ADR 0000 §6, règle 4) |
| Une mesure de comptage, sans échelle continue | `valeur_code` ou une échelle bornée déclarée : mineure |
| Plusieurs dates de mesure par famille | plusieurs instantanés dans le manifeste : c'est le mécanisme du curseur, déjà là |

Ce qui exigerait une majeure, et qu'il faut donc décider maintenant : ajouter une
dimension au grain de mesure. `entite` + `observation` + `famille` +
`methode` est le grain, et la législature y est portée par l'`entite`
(`groupe.an17.*`) et par l'instantané. C'est la raison pour laquelle le numéro de
législature est dans le modèle dès la v0 alors qu'il n'a qu'une valeur
(ADR 0000 §3).

Ce qui n'arrivera jamais, quelle que soit la brique : un champ portant une valeur
qui agrège deux familles. Il n'y a pas d'endroit pour l'écrire (I11), et cela est
délibéré.

---

## 10. Points de blocage et `A VERIFIER`

**Blocage 0 — `docs/brique0/echantillons/README.md` porte l'empreinte
`c5e405f1…`** pour l'archive des scrutins, avec la mention « concorde » face au
MD5 `1f951dea…` de la fiche source. Ni l'une ni l'autre ne se reproduit
(§2.8). Ce fichier est hors du périmètre de cette majeure ; sa correction est un
patch à part, et les fixtures qu'il décrit ne sont pas invalidées pour autant —
elles sont extraites d'une archive dont l'empreinte est désormais connue comme
non reproduite.

**Blocage 1 — le registre d'entités ne déclare pas les ancres.** Le §3 de
positionnement.md exige que `groupe.an17.lfi-nfp` et `groupe.an17.rn` soient
stockés comme ancres d'axe avec période de validité, et l'invariant I5 en dépend.
`contrepoint/registre-partis/1` n'a pas ce champ, et sa règle V1 refuse toute clé
inconnue : l'ajouter est une modification du schéma du registre, à trancher avant
la v0.2. En l'état, I5 ne vérifie que l'existence et la validité du groupe à la
date, pas qu'il a été **déclaré** comme ancre.

**Blocage 2 — le nombre de décimales de `dispersion`.** Les chiffres de
dispersion sont écrits avec les décimales de l'échelle de position (4 pour
`votes_lfi_rn`). C'est cohérent pour l'IQR et l'étendue, qui sont dans les mêmes
unités. Ce serait faux pour une dispersion exprimée dans une autre unité, cas
qu'aucune méthode actuelle ne produit. À trancher le jour où une méthode en
produit une, et pas avant.

**Blocage 3 — `logiciel.commit` est nul dans les exemples.** Le dépôt n'a pas
encore de version publiée. Le champ est prévu, la valeur viendra ; un `null`
publié en v0 est acceptable puisqu'il est hors de la clé de déduplication et
n'est qu'une trace.

| `A VERIFIER` | Comment |
|---|---|
| Période de terrain exacte de CHES 2024. La convention retenue est « une source qui ne déclare qu'une année donne `observation` = bornes de l'année », d'où `2024-01-01` → `2024-12-31`. Si le codebook publie des dates de terrain, elles s'y substituent — ce qui change l'`id` et ré-émet les lignes `experts` | Lire le codebook `CHES.2024.Codebook.pdf` |
| CHES publie-t-il une dispersion par parti exploitable comme `dispersion` ? Aujourd'hui `null` | Lire le codebook, colonne d'écart-type de la vague 2024 |
| Empreinte et URL définitives de `data/registre/partis.json`. Les exemples portent celles de `partis.exemple.json`, `991c389d34573dc2dbeebd022145b9b85030fc5ace1576e536891d5c8ce19308` | `sha256sum data/registre/partis.json` à la première publication du registre réel |
| ~~`date_source` de CHES~~ **tranché le 2026-08-27** : la ressource `CHES_2024_final_v2.csv` de la version `ches-europe` porte une date de dépôt, `2026-08-04T17:28:50Z`. C'est cette date qui est retenue pour `date_source` et pour `entrees[].derniere_mise_a_jour`, à la place de la date de récupération | `curl -sSL https://api.github.com/repos/chesdata/chesdata.github.io/releases \| python3 -c "import json,sys;[print(a['name'],a['updated_at']) for r in json.load(sys.stdin) for a in r['assets']]"` |
| Empreinte de l'instantané `an17-2026-07-21` déclarée dans le manifeste, `c23fc935…`. Les `preuve` de ses marqueurs ont changé avec la clé du §3 : la valeur publiée n'est plus la bonne, sa **taille** l'est toujours (10 064 octets, un `id` faisant 64 caractères quelle que soit sa valeur) | `sha256sum public/api/instantanes/an17-2026-07-21.json` à la première construction |
| ~~Le MD5 publié par la fiche source ne correspondrait pas à l'archive servie~~ **Refermé le 2026-08-27** : la source sert deux constructions du même contenu, aux octets et aux MD5 distincts mais au contenu identique (`diff -rq` vide, même empreinte de contenu). Les deux MD5 sont exacts, chacun pour sa construction. L'empreinte d'archive et le MD5 du producteur sont documentaires ; le contrôle qui décide est la taille puis l'empreinte de contenu (§2.8) | — |
| `entrees[].derniere_mise_a_jour` du registre d'entités. La valeur écrite, `2026-08-27`, est celle de `partis.exemple.json` ; celle du registre réel viendra avec lui | `git log -1 --format=%cs -- data/registre/partis.json` à la première publication |
| Empreinte de contenu de l'archive AMO30, `0f49c00a8227d6cb8e658d374bacfec35238fe4e2dd6305f7df6ac4f515c5de6`, mesurée le 2026-08-27 sur 13 991 fichiers pour une archive de 13 600 736 octets. Le jeu est reconstruit chaque nuit : la valeur est datée, pas permanente | Méthode du §2.8, à rejouer à chaque exécution du pipeline |
