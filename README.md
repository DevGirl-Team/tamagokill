# tamagokill
Tamagochi, mais version Mirror universe ; Le but n'est pas de le tenir en vie, mais de le tuer. Projet open-source avec la communauté sur Twitch. Pour fêter les 4k followers.

## Roadmap

- [x] Protection du repo Github
- [x] Créer le Wiki - Doc technique
- [x] Mise en place semver et Conventional Commits dans Git flow
- [x] Mise en place du monolithe
- [x] COC (Code of Conduct)
- [x] Mise en place des templates pour Issues
- [x] Mise en place de la doc avec [Docusaurus](https://docusaurus.io/)
- [x] Ajout CONTRIBUTING.md
- [ ] Intégrer le crate (code + doc) du générateur de tamago sprite
- [ ] Ajout d'un manuel pour les ADR dans CONTRIBUTING.md
- [ ] Hosting (Shiftek ou Vercel ?)
- [ ] Transformer les fichiers de config en tables d'authorité
- [ ] CI/CD
- [ ] Lancement du projet en prod

## La stack
- **Client Web** - SvelteKit
- **API** - Axum
- **DB** - PostgreSQL

## Features du projet
![Frame 1](https://github.com/DevGirl-Team/tamagokill/assets/15716589/24ae9fe2-0938-43f3-9917-47ddace803b7)

## Requirements

- [Docker](https://www.docker.com/) : Nécessaire pour run l'environnement complete (ou partiel) dans des containers. Cela permet d'assurer que l'environnement reste aussi similaire que possible d'une machine à l'autre.
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) : Nécessaire pour compiler l'API en Rust hors des containers. Cargo gère aussi les packages et dépendances de l'API.
- [Node.js & npm](https://nodejs.org/en) : Nécessaire pour compiler et run localement la Web UI de Tamagokill

## Getting started

Vous pouvez compiler et executer les parties du projets localement directement sur votre ordi (Manually) ou dans les conteneurs (Docker)

### Manually
#### API
```sh
# Pour compiler puis lancer l'API rust

cd api
cargo run

# Une fois compilée & lancée, l'API tourne par défaut à l'url "localhost:8080".
# Vérifiez dans le fichier /api/src/main.rs pour voir les routes accessibles
```

#### Web
```sh
# Pour compiler puis lancer l'UI

cd web
npm install
npm run dev

# L'UI est par défaut à l'url "localhost:5173"
```


### Docker
#### Dev environment
```sh
# Build & run une application multicontainer de l'ensemble du projet en environnement de dev
docker compose -f compose.dev.yaml up --build --watch -d
```

#### Production environment
```sh
# Build & run une application multicontainer de l'ensemble du projet en environnement de prod
docker compose up -d
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
