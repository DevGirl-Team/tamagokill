# tamagokill
Tamagochi, mais version Mirror universe ; Le but n'est pas de le tenir en vie, mais de le tuer. Projet open-source avec la communauté sur Twitch. Pour fêter les 4k followers.

## Préparation de l'environnement de développement

### Dev shell nix

Si vous utilisez [nix](https://nixos.org/download/) (Linux ou MacOS), vous pouvez utiliser le shell de développement pour avoir un environnement de développement cohérent.

Dans votre terminal, à la racine du projet, exécutez:
```bash
nix-shell
```

Si vous avez [activé les flakes](https://nixos.wiki/wiki/Flakes), vous pouvez aussi exécuter:
```bash
nix develop
```

Une fois dans le devshell, vous aurez accès à la bonne version de `node` et `rust` et toutes les dépendances nécessaires (par exemple `cargo-watch` pour le hot reload de l'api, et `gitflow` pour gérer vos branches).

## Workflow Gitflow
https://www.atlassian.com/fr/git/tutorials/comparing-workflows/gitflow-workflow
http://danielkummer.github.io/git-flow-cheatsheet/
