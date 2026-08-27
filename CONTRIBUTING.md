# Contribuer

Dépôt solo. Les règles ci-dessous ne sont pas là pour coordonner une équipe,
elles sont là pour qu'un artefact publié reste explicable six mois plus tard.

Elles complètent, sans les répéter :

- [docs/tdd.md](docs/tdd.md) — le cycle rouge / vert / refactor, obligatoire pour tout code ;
- [docs/definition-of-done.md](docs/definition-of-done.md) — quand une PR est finie ;
- [docs/regles-de-gestion.md](docs/regles-de-gestion.md) — les règles métier, référence en cas de conflit entre un test et une intention.

---

## 1. Branches

| Branche | Rôle |
|---|---|
| `main` | publication. Contient exactement ce qui est déployé, et rien d'autre |
| `develop` | intégration. Branche par défaut du dépôt |
| `<type>/<sujet>` | travail. Créée depuis `develop`, courte, un seul objet |

Aucun push direct sur `main` ni sur `develop`, y compris par l'auteur. La
protection de branche est appliquée par [`scripts/setup-github.sh`](scripts/setup-github.sh).

Nommage des branches de travail : le type du commit qui la résume, puis un sujet
en minuscules et tirets — `feat/ingestion-scrutins`, `fix/codage-non-votants`,
`docs/regles-de-gestion`.

## 2. Deux natures de PR, et une seule chacune

**PR de travail → `develop`.** Une fonctionnalité complète ou un correctif. Pas
de fourre-tout : une PR qui touche l'ingestion et le rendu se scinde. La
Definition of Done s'y applique intégralement — déterminisme démontré,
traçabilité vers une preuve, conformité au lexique.

**PR `develop` → `main`.** Une montée de version, et rien de plus. Elle porte un
numéro `X.Y.Z` neuf, décidé selon l'[ADR 0000 §6](docs/adr/0000-perimetre-brique0.md)
— la version porte sur le contrat de sortie, pas sur le code — et son merge est
suivi d'un tag `vX.Y.Z` sur `main`. Sa description dit ce qui rompt. Aucun commit
de fonctionnalité n'entre dans cette PR.

Avant d'ouvrir une PR : `git pull origin develop` dans la branche de travail,
conflits résolus. **Intégration par merge, jamais par rebase**, sur aucune
branche.

## 3. Commits

Conventional Commits, en français, sujet à l'impératif et sans point final :

```
<type>(<portée>)<!>: <sujet>
```

Types employés : `feat`, `fix`, `docs`, `test`, `refactor`, `chore`, `ci`.
Portées usuelles : `ingestion`, `matrice`, `estimateur`, `agregation`,
`registre`, `preuves`, `export`, `front`.

Le `!` marque une rupture du contrat de sortie, et le corps du commit porte alors
un pied `BREAKING CHANGE:` qui nomme ce qui devient invalide.

```
feat(ingestion): écarter les scrutins sans minorité enregistrée
fix(agregation): respecter les périodes de validité des groupes
test(estimateur): figer l'ancrage sur deux médianes de groupe
```

Un changement de test légitime est **seul dans son commit**, séparé du code qu'il
autorise (docs/tdd.md §4).

## 4. Signature et attribution

- **Jamais de signature GPG** : `git commit --no-gpg-sign`. Le dépôt fixe déjà `commit.gpgsign = false`.
- **Aucune mention d'attribution à un assistant ou à une IA**, nulle part : message de commit, description de PR, commentaire de code, documentation, fichier de données.

## 5. Corriger une donnée

Un fichier de données ne se modifie jamais à la main sans script reproductible
commité dans la même PR (definition-of-done.md, point 18). Le registre d'entités
a sa procédure propre, plus stricte, dans
[docs/brique0/registre-entites.md](docs/brique0/registre-entites.md) §7 :
une PR d'un seul objet, la source citée dans la ligne modifiée, relecture ligne
par ligne, `id` immuable.

Un signalement d'erreur venu de l'extérieur suit la même voie : voir
[docs/utilisation.md](docs/utilisation.md).

## 6. Licence des contributions

Le code contribué est publié sous AGPL-3.0-only, les données et la documentation
sous Licence Ouverte 2.0. Ouvrir une PR vaut acceptation de ces termes. Le code
n'est pas relicenciable en permissif — c'est un arbitrage acté (ADR 0000 §4).
