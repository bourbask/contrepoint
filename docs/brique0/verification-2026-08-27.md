# Vérification du 2026-08-27 — recomptage sur l'archive complète

Trois chiffres se contredisaient entre documents. Aucun arbitrage n'était
possible sans recompter : ce document consigne le protocole, les commandes et les
résultats.

Tout ce qui suit a été exécuté sur l'archive complète de la XVIIe législature, le
2026-08-27, avec Python 3 et NumPy 2.5.2.

---

## 0. Récupération

```sh
U=https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip
curl -sI "$U" | grep -iE 'content-length|last-modified'
#   content-length: 26317479
#   last-modified: Thu, 27 Aug 2026 04:25:40 GMT

for i in $(seq 6); do
  curl -sS -C - -o Scrutins.json.zip "$U"
  [ "$(stat -c%s Scrutins.json.zip)" -eq 26317479 ] && break
done
sha256sum Scrutins.json.zip
unzip -oq Scrutins.json.zip -d scrutins
find scrutins -name '*.json' | wc -l   # 8434
```

Le téléchargement a abouti du premier coup, taille exacte. La reprise reste
obligatoire : le serveur tronque par intermittence (ADR 0001 §1.5).

### Découverte : l'archive est servie en plusieurs constructions d'un même contenu

Trois récupérations le même jour ont donné **deux archives d'octets différents** :

| Récupération | Taille | SHA-256 | MD5 | `last-modified` |
|---|---|---|---|---|
| Première récupération, deux fois de suite | 26 317 479 | `aa767a2a…` | `910e6022…` | 04:25:40 GMT |
| Seconde récupération, plus tard le même jour | 26 317 479 | `c5e405f1…` | `1f951dea…` | 10:25:39 GMT |

Taille identique à l'octet, `etag` et `last-modified` distincts, et le MD5 publié
sur la fiche source suit celle des deux constructions que son propre serveur
voit. Signature d'un répartiteur devant plusieurs serveurs portant deux
constructions du même jeu.

**Le contenu, lui, est identique.** Les deux archives décompressées donnent
8 434 fichiers, **zéro différence** (`diff -rq`), et la même empreinte de
contenu :

```sh
find <racine> -name '*.json' | LC_ALL=C sort | xargs cat | sha256sum
# c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c
```

**`LC_ALL=C` n'est pas décoratif.** Sans lui, `sort` suit la locale : sous
`fr_FR.UTF-8` la ponctuation est ignorée dans la collation, donc
`VTANR5L17V100.json` précède `VTANR5L17V10.json`, qui précède
`VTANR5L17V1.json` — l'ordre inverse de l'ordre d'octets. La même archive rend
alors `503255ac…` au lieu de `c8457f34…`. Une version antérieure de ce document
portait la valeur dépendante de la locale : elle est corrigée, et la
spécification impose désormais `LC_ALL=C`.

**Conséquences de conception, toutes vérifiées ici :**

1. **L'empreinte d'archive n'est pas une propriété de la donnée.** Un pipeline
   qui détecte le changement dessus ré-émettrait l'intégralité du registre de
   preuves à chaque exécution, au hasard du serveur qui répond. C'est le défaut
   que le ticket #20 nommait, et il est ici démontré et non supposé.
2. **L'empreinte de contenu est stable** à travers les constructions. C'est elle
   qui entre dans la clé de déduplication ; l'empreinte d'archive en sort.
3. **Le MD5 publié par la source n'est pas un contrôle d'intégrité utilisable
   comme porte.** Il varie avec la construction servie. Il est consigné à titre
   documentaire ; le contrôle qui décide est la taille annoncée, puis l'empreinte
   de contenu.
4. La reprise sur troncature reste obligatoire, pour une raison distincte : le
   serveur ferme la connexion en cours de transfert sans erreur.

### Reprise du 2026-08-27 par `scripts/recuperer-sources.sh`

Première exécution du script, sur les deux sources réelles. Les deux empreintes
de contenu et les deux tailles du présent document sont retrouvées à
l'identique, par un chemin de calcul indépendant.

| Source | Octets annoncés et reçus | Empreinte d'archive | Empreinte de contenu | Fichiers | `last-modified` |
|---|---|---|---|---|---|
| Scrutins | 26 317 479 | `aa767a2a…` | `c8457f346220b5b7fb673bd1f273ef8c3296b7ff2769524bf5024c9d95c7e65c` | 8 434 | 2026-08-27T10:25:39Z |
| AMO30 | 13 600 736 | `bbecd012…` | `0f49c00a8227d6cb8e658d374bacfec35238fe4e2dd6305f7df6ac4f515c5de6` | 13 991 | 2026-08-27T00:34:47Z |

**Écart avec le tableau ci-dessus, et il compte.** Ce document appariait la
construction `aa767a2a…` au `last-modified` 04:25:40 GMT et la construction
`c5e405f1…` au 10:25:39 GMT. L'exécution a reçu `aa767a2a…` — MD5
`910e6022…`, conforme à l'appariement archive/MD5 — servi avec le
`last-modified` **10:25:39 GMT**. Le `last-modified` n'est donc pas davantage
une propriété d'une construction que l'empreinte d'archive ne l'est du contenu :
seul le couple archive/MD5 reste apparié, et la date annoncée flotte entre les
serveurs du répartiteur.

Conséquence sur la porte de RG-20, et elle tient : le contrôle porte sur
l'empreinte de **contenu** à date de source constante. Un `last-modified` qui
change sans que le contenu change ouvre une nouvelle clé et n'échoue pas ; un
contenu qui change sous une date inchangée échoue. Le défaut symétrique — un
contenu modifié servi sous une date elle aussi modifiée — n'est pas détectable,
et il n'a pas à l'être : une date nouvelle est une annonce.

Aucune reprise n'a été nécessaire ce jour-là : les deux archives sont arrivées
complètes du premier coup. La boucle de reprise n'est donc pas exercée par cette
exécution, elle l'a été par les mesures de l'ADR 0001 §1.5.

MD5 relevés, à titre documentaire : scrutins `910e6022c9eba71f932df42267778c46`,
AMO30 `4d74d2b6179eb4879d0aa31afd1b2f97`. AMO30 ne publie aucun MD5 sur sa
fiche : celui-ci est calculé localement et ne se compare à rien.

## 1. Volumétrie — confirme les documents

| Grandeur | Recompté | Documenté | |
|---|---|---|---|
| Fichiers de scrutin | 8 434 | 8 434 | ✅ |
| Cellules observées, corpus complet | 1 247 093 | 1 247 093 | ✅ |
| Acteurs distincts | 642 | 642 | ✅ |
| Scrutins retenus, `min(pour, contre) ≥ 1` | 7 979 | 7 979 | ✅ |
| Scrutins écartés | 455 | 455 | ✅ |
| Cellules après filtre | 1 188 035 | 1 188 035 | ✅ |
| Acteurs après filtre | 641 | 641 | ✅ |
| Densité | 0,2323 | 0,232 | ✅ |

Causes de non-participation relevées, et seulement celles-là :
`MG` 9 991 · `PAN` 7 508 · `PSE` 5 884. **Aucun code d'absence, aucune
non-participation volontaire nominative** — ce qui confirme que
`nonVotantsVolontaires` du décompte agrégé est un leurre.

---

## 2. Contradiction tranchée — scrutins à `organeRef: "PO0"`

`ingestion-votes.md` annonçait 14 scrutins dont 13 le 2024-12-02.
`positionnement.md` annonçait 14 et en énumérait 15.

**Les deux étaient faux sur la ventilation, justes sur le total.**

| Date | Scrutins |
|---|---|
| 2024-12-02 | **12** |
| 2025-04-07 | 1 |
| 2026-04-16 | 1 |
| **Total** | **14** |

---

## 3. Contradiction tranchée — gain du terme de rang 1

L'ADR 0001 annonçait **2,1 %**, `positionnement.md` **59,1 %**. Modèle ajusté sur
les cellules observées des 7 979 scrutins retenus, codage `pour = +1`,
`contre = −1`, `abstention = 0`, non-votant et absent traités en donnée
manquante. Moindres carrés alternés, 300 itérations, initialisation déterministe
sans générateur aléatoire.

| Modèle | Somme des carrés résiduels |
|---|---|
| Autour de la moyenne globale | 1 108 825,4 |
| Constante par scrutin | 939 865,3 |
| Constante par scrutin + rang 1 | 368 791,6 |

| Quantité | Valeur |
|---|---|
| Part expliquée par la constante seule | 15,2 % de la variance totale |
| **Gain du rang 1 sur le résidu** | **60,8 %** |
| Gain du rang 1 sur la variance totale | 51,5 % |

**`positionnement.md` avait raison** ; l'écart de 59,1 à 60,8 tient à la
convergence de l'estimateur. **L'ADR 0001 se trompait d'un facteur trente**, et
la phrase qu'il en tirait — « l'axe ne résume qu'une petite part du comportement
de vote » — était destinée au site. Corrigée.

La limite de séparation, elle, tient : l'axe explique le comportement de vote en
séparant les **blocs**, pas les individus.

---

## 4. Positions par groupe, après ancrage affine

Ancrage sur les deux médianes extrêmes, ramenées à −1 et +1. Rattachement par le
dernier groupe observé dans les scrutins — approximation suffisante pour un
contrôle, la spécification retenue restant `positionnement.md` §6.

| Groupe | n | Médiane ancrée | IQR |
|---|---|---|---|
| `PO845413` | 72 | −1,0000 | 0,028 |
| `PO845439` | 38 | −0,9830 | 0,028 |
| `PO845514` | 18 | −0,9095 | 0,095 |
| `PO845419` | 70 | −0,8366 | 0,053 |
| `PO845485` | 25 | +0,1590 | 0,655 |
| `PO845454` | 41 | +0,1647 | 0,184 |
| `PO845407` | 115 | +0,2495 | 0,221 |
| `PO840056` | 10 | +0,3576 | 0,681 |
| `PO845470` | 43 | +0,3846 | 0,156 |
| `PO845425` | 63 | +0,6838 | 0,118 |
| `PO872880` | 17 | +0,9885 | 0,037 |
| `PO845401` | 128 | +1,0000 | 0,037 |

Trois lectures :

**L'ordre se reproduit** à partir des seuls votes, sans qu'aucune étiquette
n'entre dans le calcul.

**Les effectifs confirment `positionnement.md`**, à une unité près, et non ceux
du tableau d'exploration de l'ADR 0001 — qui annonçait par exemple 91 et 122 là
où l'on trouve 115 et 128.

**Deux groupes se distinguent par leur dispersion** : `PO845485` (IQR 0,655) et
`PO840056` (IQR 0,681), contre 0,03 à 0,22 pour les autres. Ce ne sont pas des
groupes idéologiques, et la règle de non-publication les écarte — c'est
exactement le cas qu'elle est faite pour attraper.

---

## Reproduire

Les scripts de ce recomptage ne sont pas versionnés : ils seront remplacés par le
pipeline lui-même, qui doit retrouver ces valeurs. Ils constituent la cible de
non-régression du cycle correspondant. Le protocole ci-dessus suffit à les
réécrire, et les chiffres attendus sont dans les tableaux.
