# 🎯 Wordle in Rust

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen)](https://www.rust-lang.org/) [![GitHub Pages](https://img.shields.io/badge/GitHub-Pages-blue)](https://atdeters.github.io/ft_wordle/)

Welcome to our implementation of the popular New York Times game **Wordle** — now playable right in your browser!  

[🎮 **Play the Game Now**](https://atdeters.github.io/ft_wordle/)

---

## 🕹️ Gameplay Preview

![Gameplay Placeholder](screenshot.gif)  
*Replace this with an actual GIF or screenshot of the game in action.*

---

## 📚 About This Project

This project was created as part of a **42 Rush** — a fast-paced, two-person coding challenge with a limited timeframe:

**Friday 6:42 PM → Sunday 11:42 PM**  

The goal? Work in a team, push your limits, experiment with new languages, and have fun building something from scratch.  

To up the challenge, we decided to:

- Implement the game in **Rust**, a language new to both of us.  
- Use the **Macroquad** library, enabling compilation to **WebAssembly (WASM)**.  
  - Combined with a lightweight HTML wrapper, this allows the game to run directly in the browser.  
  - Deployment is automated via **GitHub Actions**, so every push updates the live version on GitHub Pages.

---

## 🛠️ Lessons Learned

This Rush gave us insights into:

- **Rust syntax, ownership, and borrowing**  
- **Game development in Rust**: loops, input handling, and rendering with Macroquad  
- **Compiling Rust to WebAssembly** for browser deployment  
- **Teamwork under tight deadlines**  

It was a challenging but **highly rewarding experience**, showing how the 42 system builds **language-agnostic skills** and helps us adapt quickly to new environments.

---

## ⚠️ Disclaimer for 42 Students

If you’re doing this Rush too:  

**Please don’t copy our code.**  

- There’s nothing to gain from it.  
- The real fun is in **building your own program and learning along the way**.  
- Experiment, create something unique, and enjoy the process.

---

## 💻 Run Locally

If you want to try the game locally, follow these steps:

1. **Clone the repository:**

git clone https://github.com/atdeters/ft_wordle.git
cd ft_wordle

markdown
Copy code

2. **Install Rust** (if not already installed):

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

markdown
Copy code

3. **Build the WebAssembly binary:**

cargo build --release --target wasm32-unknown-unknown

markdown
Copy code

4. **Serve the game locally** (using Python, for example):

Python 3
python3 -m http.server 8080

markdown
Copy code

5. **Open your browser** and go to:

http://localhost:8080/index.html

powershell
Copy code

You should now be able to play Wordle in your browser using the locally compiled WASM file.
