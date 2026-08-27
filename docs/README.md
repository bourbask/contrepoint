# Index de la documentation

## Par où commencer

| Lecteur | Ordre |
|---|---|
| Curieux, non technique | [utilisation.md](utilisation.md) |
| Qui veut juger la méthode | [methode.md](methode.md), [brique0/positionnement.md](brique0/positionnement.md), [juridique.md](juridique.md) |
| Qui veut écrire du code | [regles-de-gestion.md](regles-de-gestion.md), [architecture.md](architecture.md), [tdd.md](tdd.md), [definition-of-done.md](definition-of-done.md) |
| Qui veut écrire du texte | [ton.md](ton.md), [juridique.md](juridique.md) |

## Racine du dépôt

| Document | Contenu |
|---|---|
| [../README.md](../README.md) | Le problème, le principe, le découpage en briques, les arbitrages actés |
| [../ROADMAP.md](../ROADMAP.md) | Incréments ordonnés par dépendance, hors-périmètre délibéré, risques |
| [../CONTRIBUTING.md](../CONTRIBUTING.md) | Branches, format des PR, Conventional Commits, interdiction de signature GPG |
| [../LICENSE](../LICENSE) | AGPL-3.0-only — le code |
| [../LICENSE-DONNEES](../LICENSE-DONNEES) | Licence Ouverte 2.0 — données, registres, documentation ; mention de paternité exigée |
| [../schemas/](../schemas/) | Les quatre schémas JSON du contrat de sortie : preuve, manifeste, instantané, éclat de preuves |

## Transverse

| Document | Contenu |
|---|---|
| [utilisation.md](utilisation.md) | À quoi sert l'outil, comment se lit le graphe, comment se lit une preuve, comment se signale une erreur |
| [regles-de-gestion.md](regles-de-gestion.md) | Les règles métier numérotées et testables, avec leur fondement. Tranche en cas de conflit entre un test et une intention |
| [architecture.md](architecture.md) | Composants, frontières, flux de données, et ce que chaque composant ne fait pas |
| [methode.md](methode.md) | La chaîne de traitement maillon par maillon, briques 0 à 1, et ce que « déterministe » exclut |
| [sources.md](sources.md) | Les sources de données, leur format, leur licence, leur état vérifié ; celles écartées et pourquoi |
| [juridique.md](juridique.md) | Le précédent Décodex, les cinq règles de publication, le lexique contraint, fouille de textes, données personnelles |
| [ton.md](ton.md) | Le registre de rédaction, le lexique contraint étendu, les termes canoniques imposés à tous les documents, les tournures bannies et les contrôles sur le diff |
| [tdd.md](tdd.md) | Le cycle rouge / vert / refactor, ce qui est interdit, et la procédure quand un test doit changer |
| [definition-of-done.md](definition-of-done.md) | Les 23 points qu'une PR vers `develop` doit démontrer, et ce qui n'est pas fini quoi qu'en dise l'auteur |

## Arbitrages (ADR)

| Document | Contenu |
|---|---|
| [adr/0000-perimetre-brique0.md](adr/0000-perimetre-brique0.md) | Périmètre de la v0, députés non publiés individuellement, législatures, licence, ton éditorial, politique de version, risques acceptés et refusés |
| [adr/0001-stack.md](adr/0001-stack.md) | Mesures faites sur les sources réelles, trois piles comparées, recommandation Rust + React/TS, plan de mise en place en huit étapes |
| [adr/0002-arborescence-et-chaine-de-publication.md](adr/0002-arborescence-et-chaine-de-publication.md) | Emplacement du code et des artefacts, artefacts versionnés, chaîne de version et de publication |

## Brique 0 — les acteurs

| Document | Contenu |
|---|---|
| [brique0/ingestion-votes.md](brique0/ingestion-votes.md) | Schéma réel des scrutins, piège des non-votants, pièges de parsing, codage retenu, filtre, décompte des scrutins écartés, rattachement aux groupes |
| [brique0/positionnement.md](brique0/positionnement.md) | Pourquoi l'ACP et l'IRT sont écartés, spécification de l'estimateur, ancrage du signe et de l'échelle, agrégation, règle de non-publication |
| [brique0/registre-entites.md](brique0/registre-entites.md) | Identifiants réels de chaque source, modèle de données, périodes de validité, cas durs, règles de validation, procédure de correction |
| [brique0/contrats.md](brique0/contrats.md) | Le registre de preuves et les trois artefacts du front, champ par champ, avec versionnement des schémas et invariants vérifiés |
| [brique0/plan-de-tests.md](brique0/plan-de-tests.md) | Les tests à écrire, leur ordre, les trois tolérances numériques, les portes de couverture bloquantes |
| [brique0/echantillons/README.md](brique0/echantillons/README.md) | Les fixtures de scrutins, leur provenance, leur empreinte, et le script qui les reconstruit |
