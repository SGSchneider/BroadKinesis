import { useEffect } from "react";
import Navbar from "./components/Navbar";
import "./App.css";
import Dashboard from "./components/Dashboard";

export default function App() {
	useEffect(() => {
		// 1. Consulta se o SO prefere modo escuro
		const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

		function applyTheme(isDark: boolean) {
			if (isDark) {
				document.documentElement.classList.add("dark");
			} else {
				document.documentElement.classList.remove("dark");
			}
		}

		// Aplica no carregamento inicial
		applyTheme(mediaQuery.matches);

		// Ouve se o usuário alternar o tema nas configurações do sistema com o app aberto
		const listener = (e: MediaQueryListEvent) => applyTheme(e.matches);
		mediaQuery.addEventListener("change", listener);
		return () => mediaQuery.removeEventListener("change", listener);
	}, []);

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
