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

### Découverte : l'empreinte consignée ne se reproduit pas

| | Valeur |
|---|---|
| Empreinte obtenue, 1re récupération | `aa767a2a05f25e38badca738af3535cb9ab89b5fa95d0810a60af05eab1e4721` |
| Empreinte obtenue, 2de récupération | `aa767a2a…` — **identique** |
| Empreinte consignée dans `contrats.md` §2.6 | `c5e405f1a715086b9325a585db80362e8e7e03b9d4178ea4e35b9009bdfcf59f` |

Deux récupérations indépendantes le même jour donnent le même octet, donc
l'archive **est** stable à cet instant. La valeur consignée dans les cinq lignes
de preuve de référence ne se reproduit pas, et rien ne permet de dire si elle
provient d'une récupération incomplète ou d'une republication de la source dans
la journée.

**Conséquence de conception.** Une empreinte d'archive atteste du conteneur, pas
de la donnée : une republication à contenu identique change les octets du ZIP
(horodatages d'entrées, ordre, niveau de compression) sans que rien ait bougé.
Un pipeline qui détecte le changement sur cette seule empreinte ré-émettrait des
lignes de preuve sans cause.

Empreinte du **contenu**, stable par construction, obtenue par concaténation des
fichiers triés par chemin :

```sh
find scrutins -name '*.json' | sort | xargs cat | sha256sum
# 503255ac0b39eb28ae623368c0f21f6ec30df0893d091119c7c4efbf030c2f40
```

À consigner **en plus** de l'empreinte d'archive : la première prouve ce qui a
été téléchargé, la seconde prouve ce qui a été calculé.

---

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
