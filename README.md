# 🔡 42 Rush: Wordle

[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](https://rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?logo=webassembly&logoColor=fff)](https://webassembly.org/)
[![HTML](https://img.shields.io/badge/HTML-%23E34F26.svg?logo=html5&logoColor=white)](https://en.wikipedia.org/wiki/HTML)
[![Netlify Status](https://api.netlify.com/api/v1/badges/bbf5fc36-369f-4434-9e78-af0cafc7da37/deploy-status)](https://app.netlify.com/projects/42wordle/deploys)

Welcome to our implementation of the popular New York Times game [**Wordle**](https://www.nytimes.com/games/wordle/index.html) — now playable right in your browser!  

🎮 [**Play the Game Now**](https://atdeters.github.io/wordle/)

---

![Gameplay](docs/playthrough.gif)  

---

## 📚 About This Project

This project was created as part of a **42 Rush** — a fast-paced, two-person coding challenge
hosted by [**42 Vienna**](https://www.42vienna.com/en/about-42/) with a limited timeframe:

**Friday 6:42 PM → Sunday 11:42 PM**  

The goal? Work in a team, push your limits, experiment with new languages, and have fun building something from scratch.  

To up the challenge, we decided to:

- Implement the game in **Rust**, a language new to both of us.  
- Use the [**Macroquad**](https://macroquad.rs/) library, enabling compilation to [**WebAssembly (WASM)**](https://webassembly.org/).  
  - Combined with a lightweight HTML wrapper, this allows the game to run directly in the browser.  
  - Deployment is automated via **GitHub Actions**, so every push updates the live version on GitHub Pages.

---

## 🛠️ Lessons Learned

This Rush gave us insights into:

- **Rust syntax, ownership, and borrowing**  
- **Game development in Rust**: game loop, input handling, and rendering with **Macroquad**  
- **Compiling Rust to WebAssembly** for browser deployment  
- **Implementing sound effects** into the game with interactive logic  
- **Adding a reveal Animation**  
- **Teamwork under tight deadlines**  

It was a challenging but **highly rewarding experience**, showing how the 42 system builds **language-agnostic skills** and helps us adapt quickly to new environments.

---

## ⚠️ Disclaimer for other 42 Students

If you’re doing this Rush too:  

**Please don’t copy our code.**  

- There’s nothing to gain from it.  
- The real fun is in **building your own program and learning along the way**.  
- Experiment, create something unique, and enjoy the process.

---

## 🙏 Thanks

A big thank you to **The New York Times** for the inspiration behind Wordle, and to everyone who helped us along the way during this project. Your guidance, support, and ideas made this Rush even more enjoyable!
