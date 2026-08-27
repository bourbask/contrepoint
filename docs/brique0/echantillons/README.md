# Échantillons de scrutins — fixtures de test

Extraits de sources publiques en **Licence Ouverte / Open Licence version 1.0**.
Producteur : Assemblée nationale. Récupérés le **2026-08-27**.

Aucun fichier de ce répertoire n'est écrit à la main. Tout est reconstruit par
`extraire-echantillons.py` depuis les deux archives ci-dessous.

## Provenance

| Archive | URL | Taille | SHA-256 | MD5 publié par l'AN | `last-modified` |
|---|---|---|---|---|---|
| `Scrutins.json.zip` | `https://data.assemblee-nationale.fr/static/openData/repository/17/loi/scrutins/Scrutins.json.zip` | 26 317 479 o | `c5e405f1a715086b9325a585db80362e8e7e03b9d4178ea4e35b9009bdfcf59f` | `1f951dea5675556c5b675e5bdfeddba5` ✅ concorde | 2026-08-27 04:25:40 GMT |
| `AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip` | `https://data.assemblee-nationale.fr/static/openData/repository/17/amo/tous_acteurs_mandats_organes_xi_legislature/AMO30_tous_acteurs_tous_mandats_tous_organes_historique.json.zip` | 13 600 736 o | `bbecd01274d2bc9f46fcaa276b06868862ae7680131da3162e35b5cbef663061` | non publié sur la fiche | 2026-08-27 00:34:47 GMT |

Reconstruction :

```sh
unzip -q Scrutins.json.zip -d s17
unzip -q AMO30_....json.zip -d amo30
python3 docs/brique0/echantillons/extraire-echantillons.py s17 amo30 docs/brique0/echantillons
```

## Contenu

Les cinq scrutins sont **verbatim** — copie octet pour octet du fichier de
l'archive, sans troncature. Les deux index de référentiel sont dérivés et
portent un champ `_provenance` qui le dit.

| Fichier | Date | Votants | Ce qu'il couvre |
|---|---|---|---|
| `VTANR5L17V1.json` | 2024-10-08 | 197 | Premier scrutin de la législature. `typeVote.codeTypeVote = "MOC"` : une motion de censure n'enregistre que les `pour`, donc `contre = 0` et minorité vide par construction |
| `VTANR5L17V156.json` | 2024-10-26 | 163 | Cas nominal. `nombreVotants = pour + contre + abstentions`, `suffragesExprimes = pour + contre` |
| `VTANR5L17V5268.json` | 2026-01-29 | 35 | `votant` sérialisé en **objet nu** ; les **trois** valeurs de `causePositionVote` (`MG`, `PAN`, `PSE`) ; `parDelegation` aux deux valeurs |
| `VTANR5L17V2767.json` | 2025-06-27 | 23 | **Minorité vide** (`contre = 0`) → scrutin écarté ; et **mise au point** nominative |
| `VTANR5L17V6256.json` | 2026-04-16 | 110 | `organeRef = "PO0"` sur **tous** les blocs de groupe : référence pendante, aucun `organe/PO0.json` dans AMO30 |
| `organes-groupes-l17.json` | — | — | Les 14 groupes politiques de la XVIIe, avec `viMoDe.dateDebut` / `dateFin`. Dérivé |
| `mandats-gp-l17.json` | — | — | Cinq députés, un cas de rattachement chacun : mandat dupliqué, trois groupes successifs, retour dans le même groupe, désaccord ventilation / AMO30, présidence de l'Assemblée. Dérivé |

Les identifiants `acteurRef` sont conservés : ils sont publics, indispensables
pour tester la jointure, et ne portent aucune valeur de position. Aucune
coordonnée individuelle n'existe dans ce répertoire.

## Données à caractère personnel

Les identifiants d'acteurs et les mandats conservés ici sont indispensables pour
tester la jointure et ne portent aucune valeur de position. Ce sont des **données
à caractère personnel** au sens du RGPD et de la Licence Ouverte 2.0 ; leur
présence est signalée dans [LICENSE-DONNEES](../../../LICENSE-DONNEES) et dans
[docs/juridique.md](../../juridique.md). Aucune coordonnée individuelle n'existe
dans ce répertoire.
