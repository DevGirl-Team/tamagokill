# tamagokill
Tamagochi, mais version Mirror universe ; Le but n'est pas de le tenir en vie, mais de le tuer. Projet open-source avec la communauté sur Twitch. Pour fêter les 4k followers.

## Getting started

### Manually
#### API
```
cargo run
```

#### Web
```
npm install
npm run dev
```


### Docker
#### Dev environment
```
docker compose -f compose.dev.yaml up --build --watch
```

#### Production environment
```
docker compose up
```

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
