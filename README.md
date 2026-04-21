# 🎬 Letterboxd Scraper

Un outil en ligne de commande écrit en **Rust** pour scraper ta watchlist Letterboxd, la stocker en JSON, et obtenir des recommandations de films via **Gemini AI**.

---

## ✨ Fonctionnalités

| Commande | Description |
|---|---|
| `scrap <username>` | Scrape la watchlist d'un utilisateur Letterboxd |
| `count` | Affiche le nombre de films dans le JSON local |
| `info <titre>` | Affiche les infos d'un film |
| `random` | Tire un film au hasard dans la watchlist |
| `recommend "<envie>"` | Demande à Gemini AI 3 recommandations personnalisées |

---

## 🚀 Installation

### Prérequis

- [Rust](https://rustup.rs/) `>= 1.70`
- [Python](https://python.org) `>= 3.10`
- Une clé API Gemini — obtenue gratuitement sur [aistudio.google.com](https://aistudio.google.com)

### 1. Cloner le projet

```bash
git clone https://github.com/ton-username/letterboxd-scraper
cd letterboxd-scraper
```

### 2. Compiler le projet Rust

```bash
cargo build --release
```

### 3. Installer les dépendances Python

```bash
cd src
python3 -m venv .venv
source .venv/bin/activate
pip install google-genai
```

### 4. Configurer la clé API Gemini

```bash
export GEMINI_API_KEY="ta-clé-ici"
```

Pour la rendre permanente, ajoute cette ligne dans ton `~/.zshrc` ou `~/.bashrc` :

```bash
echo 'export GEMINI_API_KEY="ta-clé-ici"' >> ~/.zshrc
source ~/.zshrc
```

---

## 📖 Utilisation

### Scraper une watchlist

```bash
cargo run -- scrap neon_dreamer
```

Les films sont sauvegardés dans `watchlist.json` à la racine du projet. Les films déjà présents ne sont pas dupliqués — tu peux relancer la commande autant de fois que tu veux.

### Compter les films

```bash
cargo run -- count
# → 243 movies loaded
```

### Chercher un film

```bash
cargo run -- info "Annihilation"
# → Title : Annihilation (2018)
# → ID : 395249
# → link : https://letterboxd.com/film/annihilation/
# → Poster : https://a.ltrbxd.com/...
```

### Film aléatoire

```bash
cargo run -- random
```

### Recommandation par IA 🤖

```bash
cargo run -- recommend "un film qui fait peur mais pas trop"
cargo run -- recommend "quelque chose de très dynamique avec de l'action"
cargo run -- recommend "un film contemplatif pour un dimanche pluvieux"
```

Gemini analyse ta watchlist et te propose **3 films personnalisés** avec une explication pour chacun.

---

## 🗂️ Structure du projet

```
letterboxd-scraper/
├── src/
│   ├── main.rs          # CLI et orchestration des commandes
│   ├── models.rs        # Structures Movie et Watchlist
│   ├── scraper.rs       # Scraping HTTP et parsing HTML
│   ├── storage.rs       # Lecture / écriture du JSON
│   └── recommander.py   # Script de recommandation Gemini AI
├── watchlist.json       # Ta watchlist locale (généré automatiquement)
└── Cargo.toml
```

---

## 📦 Format du JSON

```json
{
  "movies": [
    {
      "film_id": "839160",
      "title": "Soft & Quiet",
      "year": 2022,
      "slug": "soft-quiet",
      "poster_url": "https://a.ltrbxd.com/resized/film-poster/.../839160-...-crop.jpg",
      "genres": []
    }
  ]
}
```

---

## ⚙️ Dépendances

### Rust (`Cargo.toml`)
| Crate | Rôle |
|---|---|
| `reqwest` | Requêtes HTTP |
| `scraper` | Parsing HTML / sélecteurs CSS |
| `serde` + `serde_json` | Sérialisation JSON |
| `clap` | Interface en ligne de commande |
| `rand` | Film aléatoire |

### Python
| Package | Rôle |
|---|---|
| `google-genai` | SDK Gemini AI |

---

## ⚠️ Quota API Gemini

La commande `recommend` utilise l'API Gemini en tier gratuit. Les limites actuelles sont :
- ~15 requêtes / minute
- ~500 requêtes / jour selon le modèle

Pour un usage personnel c'est largement suffisant. Le quota se réinitialise chaque nuit à minuit (heure du Pacifique).

---

## 📝 Licence

MIT
