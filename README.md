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
- [ ] Intégrer le crate (code + doc) du générateur de tamago sprite https://github.com/ferdodo/procedural-tamago-sprite
- [ ] Doc seaORM pour l'API
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
docker compose -f compose.dev.yaml up --build --watch -d
```

#### Production environment
```
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
