# BroadKinesis

BroadKinesis is an early-stage desktop application for organizing and automating live-streaming workflows. The project is being built as a Tauri desktop shell with a React and TypeScript user interface, with Rust reserved for native integrations and local configuration.

The current application is a working UI foundation. Navigation and window controls are available, while the feature pages are still placeholders for the upcoming streaming tools.

## Architecture

BroadKinesis is split into two layers:

```text
React / TypeScript UI
	|
	| Tauri APIs and commands
	v
Rust / Tauri desktop layer
	|
	+-- Local configuration
	+-- OBS integration (planned/under development)
	+-- Twitch and YouTube integrations (planned/under development)
```

### Frontend

- `src/main.tsx` mounts the React application.
- `src/App.tsx` composes the title bar and dashboard shell and synchronizes the UI with the operating system dark-mode preference.
- `src/components/Navbar.tsx` provides a custom, decoration-free window title bar using Tauri window APIs.
- `src/components/Dashboard.tsx` provides the collapsible navigation rail and the initial page routing for actions, commands, variables, connections, and preferences.
- `src/components/TransparentButton.tsx` contains the shared transparent button styling used by the shell controls.
- `src/App.css` defines the global theme variables, base controls, and transparent application surface.

### Desktop layer

- `src-tauri/src/main.rs` is the native executable entry point. It also applies the Linux WebKit rendering workaround used during startup.
- `src-tauri/src/lib.rs` initializes the Tauri application and defines the first configuration models and file helpers.
- Configuration is modeled for Twitch, OBS, and YouTube and is serialized as JSON near the application executable. The configuration helpers are present, but are not yet exposed to the frontend through Tauri commands.
- `src-tauri/tauri.conf.json` configures the transparent, undecorated desktop window and the frontend build lifecycle.

## Stack

- **Desktop runtime:** Tauri 2
- **Native layer:** Rust 2021
- **Frontend:** React 19 with TypeScript
- **Build and development:** Vite 8
- **Styling:** Tailwind CSS 4 and project CSS variables
- **Icons:** Lucide React
- **Streaming integration groundwork:** `obws` for OBS WebSocket communication
- **Data formats:** JSON for local configuration
- **Code quality:** Biome is included in the development dependencies

## Current functionality

The current 0.1.0 foundation includes:

- A transparent desktop window with custom minimize, maximize/restore, and close controls.
- A collapsible sidebar with the following sections:
  - Actions
  - Commands
  - Variables
  - Connections
  - Preferences
- Light and dark theme selection based on the operating system preference, including live updates when that preference changes.
- Initial Rust configuration types for Twitch, OBS, and YouTube.
- Tauri and Vite development/build wiring.

The dashboard sections currently render placeholders. Streaming actions, command execution, variable management, connection setup, and preferences are the next feature areas to implement.

## Project structure

```text
.
├── public/                 # Static frontend assets
├── src/
│   ├── assets/             # Frontend assets
│   ├── components/         # React UI components
│   ├── App.tsx             # Application shell
│   ├── App.css             # Global styles and theme variables
│   └── main.tsx            # React entry point
├── src-tauri/
│   ├── src/                # Rust application and native integrations
│   ├── capabilities/       # Tauri permission declarations
│   ├── icons/              # Desktop application icons
│   ├── Cargo.toml          # Rust dependencies and build settings
│   └── tauri.conf.json     # Tauri window and build configuration
├── index.html              # Vite HTML entry point
├── package.json            # Frontend scripts and dependencies
└── vite.config.ts          # Vite and Tauri development configuration
```

## Development

### Prerequisites

- Node.js and npm
- Rust and Cargo
- The Tauri 2 system dependencies for your operating system

Install frontend dependencies and start the Vite development server:

```bash
npm install
npm run dev
```

To run the application as a desktop window through Tauri:

```bash
npm run tauri dev
```

Create a production frontend bundle with:

```bash
npm run build
```

Build distributable desktop bundles with:

```bash
npm run tauri build
```

The frontend development server uses port `1420`, as required by the Tauri configuration.

## Planned direction

The architecture is intended to grow around a small set of native-backed streaming capabilities:

1. Expose configuration read/write operations as typed Tauri commands.
2. Add connection management for OBS, Twitch, and YouTube.
3. Implement reusable actions and commands that can call those integrations.
4. Add variables for sharing state between commands and actions.
5. Persist user preferences and provide connection diagnostics in the UI.

Keeping integrations in the Rust layer allows the React interface to remain focused on composition and state, while native services own credentials, network clients, and operating-system concerns.
