import { getCurrentWindow } from "@tauri-apps/api/window";
import {
	Copy,
	MessagesSquare,
	Minus,
	Square,
	SquareText,
	X,
} from "lucide-react";
import { useEffect, useState } from "react";
import { ToggleChatWindow } from "./Multichat";
import TransparentButton from "./TransparentButton";

export default function Navbar() {
	const appWindow = getCurrentWindow();
	const [isMaximized, setIsMaximized] = useState(false); // Maximize/restore icon state.

	useEffect(() => {
		async function updateMaximizedState() {
			// Update the maximized state.
			const maximized = await appWindow.isMaximized();
			setIsMaximized(maximized);
		}

		updateMaximizedState(); // Set the initial state when the component mounts.

		let unlisten: (() => void) | undefined;

		appWindow
			.onResized(() => {
				// Listen for window resize events.
				updateMaximizedState();
			})
			.then((dispose) => {
				unlisten = dispose;
			});

		return () => {
			// Clean up the listener when the component unmounts.
			if (unlisten) unlisten();
		};
	}, [appWindow]);

	return (
		<div
			data-tauri-drag-region
			className=" flex flex-row justify-between items-center w-full"
		>
			<div className="flex-row flex">
				<TransparentButton
					className="p-0! m-0! w-8 h-8"
					onClick={() => console.log("click")}
				>
					<img src="/assets/logo.svg" alt="Logo" className="w-8 h-8 m-1" />
				</TransparentButton>
				<div>
					<TransparentButton onClick={() => ToggleChatWindow()}>
						<MessagesSquare />
					</TransparentButton>
					<TransparentButton onClick={() => console.log("click")}>
						<SquareText />
					</TransparentButton>
				</div>
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
