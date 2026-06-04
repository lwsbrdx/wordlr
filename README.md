# wordlr

Clone du Wordle en français, jouable directement dans le terminal.

## Aperçu

Devinez le mot du jour en 6 essais. Chaque jour, un nouveau mot de 5 lettres tiré d'un dictionnaire français. Les tuiles changent de couleur pour indiquer à quel point votre proposition est proche du mot secret.

- **Vert** — lettre à la bonne place
- **Orange** — lettre dans le mot, mais pas à cette position
- **Gris** — lettre absente du mot

## Installation

### Homebrew (macOS / Linux)

```sh
brew tap lwsbrdx/tap
brew install wordlr
```

### Depuis les sources

Prérequis : [Rust](https://rustup.rs/) (edition 2024)

```sh
git clone https://github.com/lwsbrdx/wordlr
cd wordlr
cargo run --release
```

## Commandes

| Mode   | Touche      | Action                        |
|--------|-------------|-------------------------------|
| Normal | `i`         | Passer en mode saisie         |
| Normal | `?`         | Afficher l'aide               |
| Normal | `q`         | Quitter                       |
| Insert | lettres     | Saisir une lettre             |
| Insert | `Backspace` | Effacer la lettre précédente  |
| Insert | `Enter`     | Valider le mot                |
| Insert | `Esc`       | Revenir en mode Normal        |

## Statistiques

Les parties sont sauvegardées localement et accessibles à la prochaine ouverture :

- Nombre de parties jouées
- Taux de victoire
- Série actuelle et meilleure série
- Distribution des performances par nombre d'essais

## Stack technique

- **[ratatui](https://ratatui.rs/)** — rendu TUI
- **[serde](https://serde.rs/) / serde_json** — persistance des stats en JSON
- **[chrono](https://docs.rs/chrono)** — mot du jour basé sur la date
- **[dirs](https://docs.rs/dirs)** — stockage dans le répertoire data de l'OS

Les stats sont stockées dans :
- **macOS/Linux** : `~/.local/share/wordlr/games.json`
- **Windows** : `%APPDATA%\wordlr\games.json`

## Soutenir le projet

☕ [buymeacoffee.com/lwsbrdx](https://buymeacoffee.com/lwsbrdx)
