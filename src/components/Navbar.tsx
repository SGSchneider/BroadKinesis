import { getCurrentWindow } from "@tauri-apps/api/window";
import { Copy, Minus, Square, X } from "lucide-react";
import { useEffect, useState } from "react";
import TransparentButton from "./TransparentButton";

export default function Navbar() {
	const appWindow = getCurrentWindow();
	const [isMaximized, setIsMaximized] = useState(false); // controle do ícone de maximizar/restaurar

	useEffect(() => {
		async function updateMaximizedState() {
			// Função para atualizar o estado de maximização
			const maximized = await appWindow.isMaximized();
			setIsMaximized(maximized);
		}

		updateMaximizedState(); // Atualiza o estado inicial ao montar o componente

		let unlisten: (() => void) | undefined;

		appWindow
			.onResized(() => {
				// Ouve o evento de redimensionamento da janela
				updateMaximizedState();
			})
			.then((dispose) => {
				unlisten = dispose;
			});

		return () => {
			// Limpeza do listener ao desmontar o componente
			if (unlisten) unlisten();
		};
	}, [appWindow]);

	return (
		<div
			data-tauri-drag-region
			className=" flex flex-row justify-between items-center w-full"
		>
			<div className="">
				<img src="/logo.svg" alt="Logo" className="w-8 h-8 mx-1" />
			</div>
			<div className="flex self-center ml-1">
				<TransparentButton
					onClick={async () => {
						await appWindow.minimize();
					}}
					className="hover:bg-neutral-800!"
				>
					<Minus size={16} />
				</TransparentButton>
				<TransparentButton
					onClick={async () => {
						await appWindow.toggleMaximize();
					}}
					className="hover:bg-neutral-800!"
				>
					{isMaximized ? <Copy size={16} rotate={180} /> : <Square size={16} />}
				</TransparentButton>
				<TransparentButton
					onClick={async () => {
						await appWindow.close();
					}}
					className="hover:bg-red-800!"
				>
					<X size={16} />
				</TransparentButton>
			</div>
		</div>
	);
}
