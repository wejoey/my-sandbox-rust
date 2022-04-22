# infos

## rustup

Permet de mettre à jour rust.

```bash
rustup check
rustup update
```

## cargo

Permet de gérer le projet.

Pour créer un projet template:

```bash
cargo new my-new-project
```

et dans le folder du projet où il y a le Cargo.toml:

```bash
cargo build
cargo run
```

Ca va builder et lancer l'exec du `my-new-project`.

Sinon, on peut toujours juste compiler ainsi:

```bash
rustc --out-dir ./out/hello-world.exe ./src/main.rs
```

## Debug avec vscode

vscode projets ouvre le /rust
ensuite réouvrir vscode à partir d'un folder/projet (ex: /rust/projet1) pour être capable de builder/debugger à partir du .vscode/launch.json

`F5` sur le fichier .rs
