# Cycle de travail imposé — rouge, vert, refactor

S'applique à tout code du dépôt : pipeline, validateur, front, scripts de CI.
Complète [docs/definition-of-done.md](definition-of-done.md), qui dit quand une
PR est finie ; ce document dit dans quel ordre elle y arrive.

La liste des tests à écrire et leur ordre sont dans
[docs/brique0/plan-de-tests.md](brique0/plan-de-tests.md).

---

## 1. Le cycle

**Rouge.** Écrire le test. L'exécuter. **Le voir échouer, et lire le message
d'échec.** Un test qui n'a jamais été rouge ne prouve rien : il peut être vert
parce qu'il n'affirme rien, parce qu'il porte sur une branche jamais atteinte, ou
parce qu'il a été écrit après le code qu'il devait contraindre. L'échec observé
est la seule preuve que le test est branché.

Le message d'échec compte autant que l'échec. Un test dont le message est
`assertion failed: left == right` sur deux tableaux de 8 000 éléments ne sert à
personne à 23 h. Le message nomme l'invariant, pas la comparaison.

**Vert.** Écrire le minimum qui rend le test vert. Pas la version générale, pas
le cas suivant, pas le champ qui servira au cycle d'après. Le code spéculatif est
du code non testé qui a l'air testé.

**Refactor.** Réorganiser à tests verts, sans en modifier aucun. Si un
remaniement exige de toucher un test, ce n'est pas un remaniement : c'est un
changement de comportement, et il suit le §4.

Un cycle porte sur **un** test, ou sur le petit groupe de tests d'un même
invariant. Un cycle qui rend douze tests verts d'un coup n'a montré aucun d'entre
eux rouge pour la bonne raison.

---

## 2. Interdit

**Modifier un test pour le faire passer.** C'est l'interdiction centrale. Le test
est la spécification exécutable ; le corriger pour arranger le code inverse le
sens de la relation. Trois formes déguisées, toutes visées :

- élargir une tolérance ;
- restreindre l'entrée du test jusqu'à ce que le cas gênant en sorte ;
- affaiblir une assertion (`assert!(x.is_some())` là où l'attendu était une valeur).

**Supprimer un test devenu rouge.** Un test rouge est une information. La seule
réponse admise est : corriger le code, ou reconnaître que la spécification change
et suivre le §4.

**Marquer un test à sauter sans motif écrit et sans date de reprise.** Déjà
énoncé par definition-of-done.md ; répété ici parce que c'est le contournement le
plus rapide et le plus silencieux.

**Accepter un instantané sans lire le diff.** `insta` rend l'acceptation d'une
nouvelle référence plus rapide que sa lecture. Un instantané accepté sans lecture
transforme un test de non-régression en enregistreur de régressions. La
description de la PR dit ce qui a changé dans l'instantané et pourquoi.

**Écrire le code d'abord et le test après, « parce que c'était plus simple ».**
Le test écrit après passe du premier coup, ce qui n'apprend rien, et il est écrit
en regardant l'implémentation, donc il teste ce que le code fait au lieu de ce
qu'il devait faire.

**Faire d'une exécution du pipeline la source d'une valeur attendue sans la
lire.** Les valeurs de référence sont produites par l'implémentation, puis
**lues, contrôlées et figées** — jamais recopiées d'une sortie parce qu'elle
existe. Une référence non lue épingle le premier bug rencontré.

---

## 3. Obligatoire

- **Le test d'abord, y compris pour un correctif.** Un bug corrigé sans test rouge d'abord est un bug qui reviendra sans prévenir. Le test reproduit le défaut, puis le correctif le rend vert.
- **Un test, un invariant.** Un test qui échoue pour deux raisons ne dit pas laquelle. Les variantes fautives du registre d'entités en sont l'exemple : une par règle.
- **Aucun réseau, aucune clé, aucun jeton dans la suite de niveau 1.** Elle passe machine débranchée.
- **Aucun tirage aléatoire dans un test.** Les permutations et les cas sont des tables fixes. Une suite déterministe ne tire pas au sort ce qu'elle vérifie ; un test qui échoue un jour sur dix est un test qui finit désactivé.
- **Aucune horloge dans un test.** La date est une entrée injectée. Un test qui lit l'horloge échoue un 29 février ou à minuit, et personne ne sait pourquoi.
- **Les tolérances vivent dans un seul module** et sont citées, jamais recopiées (plan-de-tests.md §4).
- **Le test de conception s'écrit avant le code qu'il contraint.** Les tests marqués `[C]` du plan de tests ne naissent d'aucun bug observé : écrits après, ils ne sont jamais écrits.

---

## 4. Quand un test doit légitimement changer

Cela arrive : la méthode change, une mesure invalide une spécification, une
tolérance était mal placée. La procédure, dans cet ordre :

1. **Le motif est écrit dans la description de la PR**, avec la mesure ou l'arbitrage qui l'exige. « Le test était trop strict » n'est pas un motif ; « la mesure du §X donne 60,8 % et non 2,1 % » l'est.
2. **La documentation change dans la même PR** — docs/methode.md si la méthode bouge, l'ADR concerné si l'arbitrage bouge (definition-of-done.md §19). Pas de PR de rattrapage documentaire.
3. **Le changement de test est seul dans son commit**, séparé du code qu'il autorise. Un diff où le test et le code changent ensemble ne permet pas de voir lequel a cédé.
4. **La version du contrat de sortie est incrémentée** si l'attendu publié bouge (ADR 0000 §6).
5. **Un test supprimé est justifié par la disparition de son objet**, pas par son coût. Si l'objet existe encore, le test reste.

Cas particulier de l'instantané : mettre à jour une référence figée est un
changement de test au sens de ce paragraphe, même quand l'outil le rend trivial.

---

## 5. Ce que le cycle ne remplace pas

Le TDD garantit qu'un comportement spécifié est vérifié. Il ne garantit pas que
la spécification est bonne. Trois choses restent hors de sa portée, et sont
couvertes ailleurs :

| Ce que le cycle ne voit pas | Où c'est couvert |
|---|---|
| Une spécification fausse, correctement implémentée et correctement testée | La mesure sur les sources réelles, avant d'écrire la spec — c'est le rôle des documents de `docs/brique0/` |
| Une erreur d'appariement dans le registre d'entités | La relecture humaine ligne par ligne, seule obligatoire du projet (definition-of-done.md §17) |
| Une valeur de référence figée sur un bug du premier jour | La lecture de la référence avant de la figer (§2), et la porte 1 du plan de tests §15 |

---

## 6. Résumé opposable

| Question | Réponse |
|---|---|
| Le test a-t-il été vu rouge ? | Sinon, il ne compte pas |
| Le code écrit dépasse-t-il le test ? | Alors il est spéculatif, il sort |
| Un test a-t-il changé dans cette PR ? | Alors le motif, la doc et un commit séparé sont exigés |
| Un test est-il sauté ? | Alors motif écrit et date de reprise, sinon la PR n'est pas finie |
| Une tolérance a-t-elle bougé ? | Alors le motif est dans la description |
| Un instantané a-t-il été accepté ? | Alors le diff a été lu et la PR dit ce qui a changé |
