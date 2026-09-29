import { getCurrentWindow } from "@tauri-apps/api/window"; // Import the window API.
import { useEffect } from "react";
import Navbar from "./components/Navbar";
import "./App.css";
import Dashboard from "./components/Dashboard";
import { TwitchChat } from "./components/Multichat"; // Chat window component.

export default function App() {
	const appWindow = getCurrentWindow();

	useEffect(() => {
		// Check whether the operating system prefers dark mode.
		const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

		function applyTheme(isDark: boolean) {
			if (isDark) {
				document.documentElement.classList.add("dark");
			} else {
				document.documentElement.classList.remove("dark");
			}
		}

		// Apply the initial theme.
		applyTheme(mediaQuery.matches);

		// Listen for system theme changes while the app is open.
		const listener = (e: MediaQueryListEvent) => applyTheme(e.matches);
		mediaQuery.addEventListener("change", listener);
		return () => mediaQuery.removeEventListener("change", listener);
	}, []);

	if (appWindow.label === "chat-window") {
		return <Multichat />;
	}

	return (
		<main className="w-screen h-screen bg-transparent! overflow-y-clip">
			<div className="flex flex-col w-full h-full">
				<div className="flex w-full h-fit">
					<Navbar />
				</div>
				<Dashboard />
			</div>
		</main>
	);
}

function Multichat() {
	return (
		<main className="w-screen h-screen bg-transparent overflow-hidden">
			<div className="flex flex-col w-full h-full">
				<TwitchChat />
			</div>
		</main>
	);
}
