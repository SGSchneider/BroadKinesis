# BroadKinesis

BroadKinesis is a desktop workflow platform for streamers and live content creators who want to manage tools, actions, and automation from one local control surface.

The product is currently in its early foundation stage: the desktop shell and base navigation are in place, and the platform is being structured around a reusable automation model for OBS, Twitch, YouTube, and operational workflow commands.

## Why BroadKinesis

Streaming setups often combine multiple tools and services at once: chat, overlays, scene switching, alerts, metadata, automation, and platform-specific status checks. BroadKinesis is designed to centralize those moving parts into a single desktop application that feels lightweight, local-first, and controllable without needing to juggle a dozen browser tabs or external tools.

## Product vision

BroadKinesis aims to become a creator operations hub with:

- a clear command center for actions and automation
- support for stream-related integrations and service connections
- reusable variables and command flows
- a simple, low-friction configuration experience
- a native desktop experience optimized for performance and reliability

## Current status

The project is in an early product stage and includes the foundation for the desktop experience:

- custom undecorated window controls for minimize, maximize, and close
- a collapsible navigation sidebar for key app areas
- a dark-mode-aware interface that follows system preferences
- a Rust-backed configuration layer for Twitch, OBS, and YouTube settings
- Tauri integration for a native desktop app shell

The main feature modules are still being developed. The dashboard pages currently act as structural placeholders for the upcoming product workflows.

## Architecture

BroadKinesis uses a layered architecture that separates interface, desktop shell, and integration logic.

```text
Frontend layer (React + TypeScript)
        |
        | Tauri bridge / native commands
        v
Desktop runtime layer (Tauri + Rust)
        |
        +-- local configuration storage
        +-- OBS integration hooks
        +-- Twitch and YouTube connection support
```

### Frontend

The current frontend is centered in the `src` directory:

- [src/App.tsx](src/App.tsx) creates the main shell and handles theme behavior
- [src/components/Dashboard.tsx](src/components/Dashboard.tsx) contains the app navigation and page routing structure
- [src/components/Navbar.tsx](src/components/Navbar.tsx) implements the custom title bar and native window controls
- [src/components/TransparentButton.tsx](src/components/TransparentButton.tsx) provides the reusable UI control styling
- [src/App.css](src/App.css) defines the base design system and dark/light theme variables
- [src/main.tsx](src/main.tsx) boots the React application

### Native and integration layer

The native portion lives in the `src-tauri` folder:

- [src-tauri/src/main.rs](src-tauri/src/main.rs) is the app entry point and includes the Linux WebKit startup workaround
- [src-tauri/src/lib.rs](src-tauri/src/lib.rs) defines the initial app configuration models and local JSON file helper functions
- [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json) configures the desktop window, frontend build flow, and app metadata
- [src-tauri/Cargo.toml](src-tauri/Cargo.toml) declares the Rust dependencies, including Tauri and the OBS WebSocket library

This layer is intentionally designed to host connection logic, local persistence, and future automation actions without crowding the frontend UI.

## Tech stack

- Tauri 2 for the desktop runtime
- Rust 2021 for native logic and local configuration
- React 19 + TypeScript for the product interface
- Vite 8 for development and bundling
- Tailwind CSS 4 for styling and layout
- Lucide React for UI icons
- JSON-based configuration storage
- Biome for tooling and code quality checks

## Key product areas

The app is organized around several product domains that will eventually become full feature modules:

- Actions: execution flows the user can trigger
- Commands: reusable command definitions and sequences
- Variables: dynamic values used across workflows
- Connections: service and integration setup for OBS, Twitch, and YouTube
- Preferences: app behavior, settings, and local configuration

## Repository structure

```text
.
├── public/                  # Static frontend assets
├── src/                     # React application source
│   ├── assets/
│   ├── components/
│   ├── App.tsx
│   ├── App.css
│   └── main.tsx
├── src-tauri/               # Rust + Tauri backend
│   ├── src/
│   ├── capabilities/
│   ├── icons/
│   ├── Cargo.toml
│   └── tauri.conf.json
├── index.html
├── package.json
├── vite.config.ts
├── tsconfig.json
├── tsconfig.node.json
├── README.md
├── .gitignore
└── public/
```

## Getting started

### Prerequisites

- Node.js and npm
- Rust and Cargo
- Tauri desktop dependencies for your operating system

### Run locally

```bash
npm install
npm run tauri dev
```

This starts the desktop application with the Vite frontend and the native Tauri shell.

### Build for production

```bash
npm run build
npm run tauri build
```

The frontend is configured to use port `1420`, matching the Tauri dev setup.

## Roadmap

The near-term roadmap is focused on turning the shell into a functioning creator automation app:

1. expose typed Tauri commands for config read/write operations
2. add connection management for OBS, Twitch, and YouTube
3. implement action and command execution logic
4. add reusable variables and state flow across actions
5. improve preferences and connection diagnostics in the UI
6. build out dedicated product pages for each workflow module

## Summary

BroadKinesis is being built as a local-first desktop product for creators who want streamlined control across their live-streaming tools and automation flows. The current repository establishes the product foundation, layout, and architecture needed to support that vision, with the feature system still in active development.
