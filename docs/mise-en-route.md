# Mise en route d'un clone et du dépôt GitHub

Ce que `scripts/setup-github.sh` fait, ce qu'il ne peut pas faire, et dans quel
ordre. Un réglage non scripté et non consigné est un réglage qui n'existe pas :
il est refait de mémoire, différemment, et il se découvre en panne.

État lu le 2026-08-27 par `gh api`, dépôt `bourbask/contrepoint`. Les états
changent : chaque ligne porte la commande qui la relit.

---

## 1. Ordre d'exécution, avant la première PR

Le script pose la protection de branches. Une fois posée, `main` et `develop`
refusent le push direct et exigent la vérification `ci-ok`. Il faut donc que
`.github/workflows/ci.yml` existe **et** ait publié au moins une fois un contexte
nommé `ci-ok`, sinon toute PR reste bloquée sur une vérification qui n'arrive
jamais.

1. `ci.yml` est sur `develop`, et une exécution a publié `ci-ok`.
2. `./scripts/setup-github.sh` — depuis un clone, `gh` authentifié.
3. Vérifier : `gh api repos/bourbask/contrepoint/branches/develop/protection --jq '.required_status_checks.contexts'` rend `["ci-ok"]`.
4. Première PR.

Si la protection a été posée trop tôt, elle se retire avec
`gh api -X DELETE repos/bourbask/contrepoint/branches/develop/protection`, puis
se repose après.

**`ci-ok` est la seule vérification exigée.** Tout nouveau travail de `ci.yml`
s'ajoute à ses `needs`, jamais à `CHECKS` : un travail gardé par `hashFiles` ne
publie aucun contexte quand il saute, et une protection qui l'attendrait
bloquerait toutes les PR.

## 2. Ce que le script fait

| Réglage | Comment | Relire |
|---|---|---|
| Crochet local `core.hooksPath` | `git config core.hooksPath .githooks` | `git config --get core.hooksPath` |
| Branche par défaut `develop` | `gh repo edit --default-branch` | `gh api repos/bourbask/contrepoint --jq .default_branch` |
| Protection de `main` et `develop` | `PUT .../branches/<b>/protection` | `gh api repos/bourbask/contrepoint/branches/main/protection` |
| Source de publication des Pages sur Actions | `POST .../pages` avec `build_type=workflow`, `PUT` si le site existe | `gh api repos/bourbask/contrepoint/pages --jq .build_type` |
| Alertes de vulnérabilité | `PUT .../vulnerability-alerts` | `gh api -i .../vulnerability-alerts` — 204 activé, 404 désactivé |
| Correctifs de sécurité automatiques | `PUT .../automated-security-fixes` | `gh api .../automated-security-fixes --jq .enabled` |
| Étiquettes | `gh label create` / `edit` | `gh label list` |

Le script est idempotent et se relance sans effet de bord. `DRY_RUN=1` affiche
sans appliquer.

## 3. Ce que le script ne peut pas faire

**`core.hooksPath` sur les autres clones.** C'est une configuration locale, elle
n'est pas clonée. Chaque poste, et chaque arbre de travail secondaire, la pose
lui-même :

```sh
git config core.hooksPath .githooks
```

Sans elle, `.githooks/pre-commit` n'est jamais appelé, et les contrôles de
lexique et de sécurité redeviennent consultatifs.

**L'environnement `github-pages`.** Il est créé par GitHub lors de la
configuration des Pages, avec sa politique de branche. Le script ne le fabrique
pas : un environnement créé à la main naît sans politique, et `deploy.yml`
publierait alors depuis n'importe quelle branche. Vérifier après coup :

```sh
gh api repos/bourbask/contrepoint/environments --jq '[.environments[].name]'
```

`A VERIFIER` — au 2026-08-27, `total_count` vaut 0 et
`gh api repos/bourbask/contrepoint/pages` rend 404 : les Pages ne sont pas
encore configurées, donc l'environnement n'existe pas. Relancer le script puis
relire les deux commandes ci-dessus.

**Les montées de version Dependabot.** Elles ne s'activent par aucun appel :
`.github/dependabot.yml` suffit, **à condition d'être sur la branche par
défaut**, qui est `develop`. Rien à scripter, mais rien ne le dit non plus
ailleurs.

**L'ordre du §1.** C'est une procédure, pas un réglage.

## 4. `A VERIFIER`

| Point | Comment le vérifier |
|---|---|
| `POST /repos/.../pages` aboutit avec le jeton `gh` courant (portées `repo`, jeton OAuth `gho_`) | Lancer le script et lire le code de retour ; en cas de 403, la portée `admin:repo_hook` ou un jeton personnel avec `repo` complet est exigée |
| L'environnement `github-pages` apparaît après configuration des Pages, avec une politique limitée à `main` | `gh api repos/bourbask/contrepoint/environments/github-pages --jq .deployment_branch_policy` |
| `automated-security-fixes` reste activé après `PUT` | `gh api repos/bourbask/contrepoint/automated-security-fixes --jq .enabled` doit rendre `true` |
