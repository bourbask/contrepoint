# Definition of Done — PR vers `develop`

Arbitré par [docs/adr/0000-perimetre-brique0.md](adr/0000-perimetre-brique0.md).
Une PR n'est pas finie parce qu'elle marche : elle est finie quand le
déterminisme, la traçabilité et la conformité au lexique sont **démontrés dans
la PR**. Une case cochée sans sortie montrable ne compte pas.

---

## Avant d'ouvrir la PR

1. **Branche synchronisée.** `git pull origin develop` dans la branche de travail, conflits résolus, puis seulement la PR. Intégration par merge, **jamais par rebase**, sur aucune branche.
2. **Une PR, un objet.** Un item de roadmap ou un correctif. Pas de fourre-tout : une PR qui touche à la fois l'ingestion et le rendu se scinde.
3. **Commits** `--no-gpg-sign`. Aucune mention d'attribution d'IA nulle part — message de commit, description de PR, commentaire, doc, fichier de données.

## Déterminisme

4. **Deux exécutions consécutives du pipeline touché donnent des sorties identiques.** L'empreinte des artefacts produits figure dans la description de la PR. Une sortie qui bouge sans que l'entrée bouge est un bug bloquant, pas une tolérance.
5. **Rejouabilité si le registre de preuves est touché.** Reconstruction complète depuis les sources brutes → fichier identique octet pour octet. L'empreinte avant/après est dans la description.
6. **Aucune source de non-déterminisme introduite** : pas d'horloge dans une valeur calculée (les dates sont des données d'entrée), pas d'itération sur un ensemble non ordonné, pas de graine aléatoire non fixée, pas de dépendance à l'ordre du système de fichiers.

## Tests

7. **Tests hors ligne, zéro réseau, zéro clé, zéro token.** La suite passe machine débranchée.
8. **Toute donnée réseau est une fixture figée, commitée**, accompagnée de son URL et de sa date de récupération. Fixtures issues de sources en Licence Ouverte uniquement — aucune fixture CHES dans le dépôt (voir ADR §4).
9. **Invariants qui doivent avoir un test dédié** dès que la PR touche le code concerné :
   - absent ≠ abstention — le test échoue si l'absence est traitée comme une position ;
   - le seul filtre de scrutins est la minorité non vide (`min(pour, contre) ≥ 1`) et non un seuil de participation, et le décompte des scrutins écartés est exposé avec son motif (RG-13, brique0/ingestion-votes.md §6) ;
   - le rattachement député → groupe respecte les périodes de validité (un changement de groupe en cours de mandature ne réattribue pas rétroactivement les votes antérieurs) ;
   - le signe et l'échelle de l'axe sont stables d'une exécution à l'autre, fixés par la transformation affine ancrée sur les deux médianes de groupe déclarées (RG-29) ;
   - aucune coordonnée individuelle de député n'apparaît dans une sortie publiée.
10. **Logique non triviale sans test = PR non finie.** Un one-liner n'a pas besoin de test.

## Conformité éditoriale et juridique

11. **Lexique.** Zéro occurrence des termes interdits de docs/juridique.md et de docs/ton.md §2 dans le diff — code, identifiants, données, chaînes d'interface, docs — hors les tableaux de juridique.md et de ton.md eux-mêmes. Vérification à lancer sur le diff :

    ```
    git diff origin/develop... | grep -inE 'fiabilit|crédibilit|credibilit|véracit|veracit|désinformation|desinformation|fake ?news|infox|biais d|partial|militant|classement'
    ```

12. **Aucun axe, champ ni variable interne à pôle dépréciatif.** Y compris sous un nom technique anodin. La règle vaut en interne, pas seulement à l'affichage.
13. **Ton conforme à l'ADR §5 et à [docs/ton.md](ton.md)** pour toute chaîne visible : aucune personne, limites de longueur respectées, aucun comparatif sans chiffre, absence de donnée dite et non comblée. Les contrôles à passer sur le diff sont dans ton.md §7.
14. **Aucune génération de texte** dans le produit. Un embedding est admis s'il est épinglé en version et consigné dans la ligne de preuve.

## Traçabilité

15. **Toute valeur affichée trace vers une ligne du registre de preuves.** Une valeur sans preuve ne s'affiche pas, et aucune estimation ne prend sa place.
16. **Aucune constante en dur** qui devrait vivre dans le registre d'entités : nom de parti, code de nuance, identifiant CHES, appartenance de groupe.
17. **Registre d'entités : relecture ligne par ligne exigée.** C'est le seul fichier où elle est obligatoire — une erreur d'appariement s'y propage dans toutes les briques.
18. **Fichier de données jamais modifié à la main** sans script reproductible commité dans la même PR.

## Documentation et version

19. **La méthode change → docs/methode.md change dans la même PR.** Pas de PR de rattrapage documentaire.
20. **Le contrat de sortie change → la version est incrémentée selon l'ADR §6**, et le journal des changements dit ce qui rompt.
21. **Nouvelle dépendance → une ligne de justification** dans la description, et une réponse à « la bibliothèque standard le fait-elle ? ». Une dépendance ajoutée sans cette ligne est refusée.
22. **Une source ajoutée → une ligne dans docs/sources.md** avec son format, sa licence, sa date de vérification.
23. **Rien de non vérifié affirmé.** Ce qui n'a pas pu l'être est marqué `A VERIFIER` avec la façon de le vérifier.

---

## Non fini, quoi qu'en dise l'auteur

- Un `TODO` laissé sans issue ouverte en face.
- Une case de ROADMAP.md cochée sans la sortie visible que l'incrément promet.
- Un test désactivé ou marqué à sauter sans motif écrit et sans date de reprise.
- Une empreinte de déterminisme absente de la description quand la PR touche le pipeline.
- Une valeur affichée dont la preuve n'existe pas encore.
