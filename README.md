<p align="center" width="100%">
    <img width="33%" src="https://media.discordapp.net/attachments/1277269332728221977/1277311846768185364/Frame_46.png?ex=66ccb4db&is=66cb635b&hm=042ea7481d26fd583bc89604ad53eadb93c44f7463b3adcf6789d9eebf851ade&=&format=webp&quality=lossless">
</p>

# tamagokill
[![](https://dcbadge.limes.pink/api/server/discord.gg/Njh6vWPH2h)](discord.gg/Njh6vWPH2h)

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
- [nvm](https://github.com/nvm-sh/nvm) : Utile pour basculer sur les bonnes versions de node.js/npm, notamment si vous avez plusieurs projets sur votre ordi dans des versions différentes.

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

##### Première étape: utiliser une bonne version de node.

*Les versions éligibles écrites dans `web/package.json` dans la section `engine`.*
Le repository contient un `.nvmrc` qui permet à **nvm** de directement charger la bonne version 

```sh
# Vérifier sa version de node
node -v

# Basculer vers la bonne version de node
cd web

# Si vous n'avez pas encore installé la version de node pour ce projet
nvm install 

# Si vous l'avez déjà mais la version actuellement utilisée (retournée par node -v) n'est pas compatible avec ce projet
nvm use 


# A noter que nvm install marche comme nvm use mais download au préalable si besoin, dans le doute, vous pouvez juste retenir nvm install.

```

##### Deuxième étape: compiler et travailler sur le projet

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
