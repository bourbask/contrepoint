# Correspondance wiki ↔ dépôt

Base de la synchronisation automatique du wiki. Le dépôt est la source de vérité ;
le wiki en est un miroir. Une page marquée **miroir** ne doit jamais être éditée
dans le wiki : elle est régénérée depuis son fichier source.

| Page du wiki | Fichier du dépôt | Nature | Transformations appliquées à la recopie |
|---|---|---|---|
| `Home.md` | *aucun* | **propre au wiki** | Écrite pour le wiki. Page d'accueil : de quoi il s'agit, ce que l'outil ne fait pas, où aller. À relire quand `README.md` change de fond |
| `Comprendre-le-projet.md` | `docs/utilisation.md` | **miroir enrichi** | Liens relatifs convertis ; tableau des trois familles ajouté ; schéma Mermaid de la chaîne « votes → positions → graphe » ajouté (§2) avec la section « ce que ce calcul ne dit pas » ; §1 reformulé pour un lecteur non technique. Aucun fait modifié |
| `Methode.md` | `docs/methode.md` | **miroir** | Liens relatifs convertis ; renvoi ajouté vers `[[Comprendre-le-projet]]` §2 |
| `Contraintes-de-publication.md` | `docs/juridique.md` | **miroir adapté** | Liens relatifs convertis. Nom du responsable de traitement remplacé par « l'auteur du projet, nommément désigné dans le fichier du dépôt » (aucun nom de personne physique dans le wiki). Tableau du lexique et deux phrases reformulés pour rester conformes à `scripts/lexique.sh`, dont le dépôt exempte `docs/juridique.md` mais pas le wiki. Aucun fait modifié |
| `Architecture.md` | `docs/architecture.md` | **miroir** | Liens relatifs convertis |
| `Feuille-de-route.md` | `ROADMAP.md` | **miroir** | Liens relatifs convertis ; avertissement de tête renforcé (document le plus volatile) ; la ligne « hors périmètre » sur les scores dépréciatifs reformulée pour rester conforme à `scripts/lexique.sh`. **À resynchroniser à chaque modification de `ROADMAP.md`** |
| `Contribuer.md` | `CONTRIBUTING.md`, `.github/ISSUE_TEMPLATE/` | **propre au wiki** | Page d'orientation courte. Ne recopie pas `CONTRIBUTING.md` : elle y renvoie |
| `_Sidebar.md` | *aucun* | **propre au wiki** | Navigation, du plus accessible au plus technique. À mettre à jour à chaque ajout ou retrait de page |

## Règles de synchronisation

1. **Sens unique : dépôt → wiki.** Jamais l'inverse. Une modification faite dans
   le wiki est écrasée sans avertissement.
2. **Chaque page miroir porte son avertissement de tête**, avec le lien vers son
   fichier source. Formulation unique, reprise à l'identique ; `Feuille-de-route.md`
   en porte une version renforcée.
3. **Aucun lien relatif ne survit.** Les `docs/…`, `../adr/…` et autres chemins
   relatifs des fichiers sources sont convertis en URL complètes vers
   `https://github.com/bourbask/contrepoint/blob/main/…`, ou en liens de wiki
   `[[Nom-De-Page]]` quand la page miroir existe.
4. **Le wiki est hors intégration continue.** `scripts/lexique.sh` ne le vérifie
   pas : les motifs sont appliqués à la main. Les exemptions du mode `docs`
   (`docs/juridique.md`, `docs/ton.md`, `CHANGELOG.md`) **ne s'appliquent pas** au
   wiki — un miroir d'un fichier exempté doit être reformulé.
5. **Aucun nom de personne physique, aucune coordonnée individuelle de député,
   aucun chemin absolu, aucune adresse de courriel.** Le wiki est aussi public
   que le dépôt.
6. **Aucun fait n'est modifié à la recopie.** Une contradiction repérée dans un
   fichier source se signale dans le dépôt et se corrige là ; elle se recopie
   fidèlement en attendant.
