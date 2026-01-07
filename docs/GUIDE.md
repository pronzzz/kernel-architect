# The Kernel Architect: Detailed Guide

This document serves as a comprehensive guide to understanding the internal mechanics, architecture, and future roadmap of **The Kernel Architect**.

## System Overview

The Kernel Architect is designed as a simulation running on a virtual "kernel". The player acts as an architect or intruder manipulating the system's core clusters.

### 1. The World Generation (PCG)

The game world uses **Cellular Automata** to generate organic-looking "clusters" of nodes.
- **Location**: `src/pcg/automata.rs`
- **Logic**: 
  - A grid is initialized with random noise.
  - Automata rules (like Game of Life) are applied for several iterations to smooth out the noise into cohesive structures.
  - Nodes are assigned types (Void, Data, Firewall, etc.) based on their position and connectivity.

### 2. State Management

The game state is central to the application, managed in `src/state.rs`.
- **`App` Struct**: Holds the `Cluster`, player position, logs, and current `GameState`.
- **Game States**:
  - `Exploration`: Real-time movement on the map.
  - `KernelDebug`: A pause/menu mode for inspecting deeper stats.
  - `LogicCombat`: (Planned) Turn-based combat against system defenses.

### 3. Networking (SSH Server)

Unlike standard terminal games, The Kernel Architect can run as an SSH server (`src/net/ssh_server.rs`).
- Uses `russh` to handle SSH connections.
- Each connection spawns a new instance of the TUI logic, effectively allowing remote play via `ssh user@host -p 2026`.

### 4. User Interface (UI)

Rendering is handled by `ratatui`.
- **Layout**: Split into Header (Status), Body (Map + Info), and Footer (Console Logs).
- **Widgets**: Custom widgets in `src/ui/components.rs` handle the specific rendering of the map grid and status bars.

## Gameplay Mechanics

### Hacking
Hacking is the primary interaction. Standing on a non-void node and pressing `Enter` triggers a hack.
- **Process**: Checks node integrity. If > 0, the hack attempts to compromise it.
- **Result**: Successful hacks zero the integrity and mark the node as `Compromised`.
- **Feedback**: Results are logged to the in-game console.

### Movement
- Simple grid-based movement.
- Collision detection ensures players stay within bounds and (in the future) respect solid walls.

## Roadmap

- [ ] **Logic Combat**: Implement the card-based/logic-puzzle combat system when encountering Firewalls.
- [ ] **Inventory System**: Allow collection of data fragments and tools.
- [ ] **Multiplayer**: Expand the SSH server to allow multiple architects to inhabit the same cluster.
