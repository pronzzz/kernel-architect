# The Kernel Architect

![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Status](https://img.shields.io/badge/Status-In%20Development-yellow.svg)

**The Kernel Architect** is an immersive, terminal-based simulation game where you explore, analyze, and hack into procedurally generated digital clusters. Built with Rust and Ratatui, it combines the aesthetics of a retro cyberpunk interface with modern TUI capabilities.

## 🌟 Features

- **Procedural Generation**: Explore unique, cellular-automata-generated server clusters (`pcg` module).
- **Interactive TUI**: Smooth, responsive interface powered by `ratatui` and `crossterm`.
- **Hacking Mechanics**: Compromise nodes to gain control of system resources.
- **Dual Modes**: 
  - **Local Mode**: Play directly in your terminal.
  - **SSH Server Mode**: Host the game as an SSH service for remote play.
- **Dynamic State Management**: Robust state handling for exploration, combat, and inventory systems.

## 🚀 Getting Started

### Prerequisites

You need [Rust and Cargo](https://rustup.rs/) installed on your system.

### Installation

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/pronzzz/kernel-architect.git
    cd kernel-architect
    ```

2.  **Build the project**:
    ```bash
    cargo build --release
    ```

## 🎮 How to Play

### Local Mode

Run the game directly in your terminal:

```bash
cargo run --release
```

### Server Mode

Start the SSH game server (defaults to port 2026):

```bash
cargo run --release -- --server
```

Clients can then connect via regular SSH (authentication configuration required in `net` module).

### Controls

| Key | Action |
| :--- | :--- |
| **Arrow Keys** | Move cursor/player through the cluster |
| **Enter** | Hack the current node / Interact |
| **Q** | Quit the application |

## 🛠️ Architecture

The project follows a modular architecture:

- `src/main.rs`: Entry point and main event loop.
- `src/state.rs`: Core game state definition (`App` struct).
- `src/tui.rs`: Terminal initialization and restoration logic.
- `src/ui/`: UI rendering components (Widgets).
- `src/pcg/`: Procedural Content Generation algorithms (Cellular Automata).
- `src/net/`: Networking and SSH server implementation.

For a deeper dive into the system's design, check out the [Detailed Guide](docs/GUIDE.md).

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
