import {
	Bot,
	Database,
	Link,
	Menu,
	MessageCircleMore,
	Wrench,
} from "lucide-react";
import { useState } from "react";
import Connections from "../pages/Connections";
import TransparentButton from "./TransparentButton";

export default function Dashboard() {
	const [isOpen, setIsOpen] = useState(false);
	const width = isOpen ? "w-48" : "w-14"; // Ajusta a largura com base no estado
	const [page, setPage] = useState("actions");

	return (
		<div className="flex flex-row w-full h-full">
			<div>
				<div
					className={`flex flex-col ${width} h-screen outline-1 outline-neutral-800`}
				>
					<TransparentButton
						onClick={() => {
							setIsOpen(!isOpen);
						}}
						className="flex flex-row items-center mb-2"
					>
						<Menu size={24} />
						<h1
							className={`text-lg font-bold ml-2 ${isOpen ? "block" : "hidden"}`}
						>
							BroadKinesis
						</h1>
					</TransparentButton>
					<TransparentButton
						onClick={() => {
							setPage("actions");
						}}
						className="flex flex-row items-center"
					>
						<Bot size={24} />
						<h1
							className={`text-md font-bold ml-2 ${isOpen ? "block" : "hidden"}`}
						>
							Ações
						</h1>
					</TransparentButton>
					<TransparentButton
						onClick={() => {
							setPage("commands");
						}}
						className="flex flex-row items-center"
					>
						<MessageCircleMore size={24} />
						<h1
							className={`text-md font-bold ml-2 ${isOpen ? "block" : "hidden"}`}
						>
							Comandos
						</h1>
					</TransparentButton>
					<TransparentButton
						onClick={() => {
							setPage("variables");
						}}
						className="flex flex-row items-center"
					>
						<Database size={24} />
						<h1
							className={`text-md font-bold ml-2 ${isOpen ? "block" : "hidden"}`}
						>
							Variáveis
						</h1>
					</TransparentButton>
					<TransparentButton
						onClick={() => {
							setPage("connections");
						}}
						className="flex flex-row items-center"
					>
						<Link size={24} />
						<h1
							className={`text-md font-bold ml-2 ${isOpen ? "block" : "hidden"}`}
						>
							Conexões
						</h1>
					</TransparentButton>
					<TransparentButton
						onClick={() => {
							setPage("preferences");
						}}
						className="flex flex-row items-center"
					>
						<Wrench size={24} />
						<h1
							className={`text-md font-bold ml-2 ${isOpen ? "block" : "hidden"}`}
						>
							Preferências
						</h1>
					</TransparentButton>
					<div className="h-full"></div>
				</div>
			</div>
			<div className="w-full h-full">
				<Page page={page} />
			</div>
		</div>
	);
}

function Page({ page }: { page: string }) {
	switch (page) {
		case "actions":
			return <div>actions</div>;
		case "commands":
			return <div>commands</div>;
		case "variables":
			return <div>variables</div>;
		case "connections":
			return <Connections />;
		case "preferences":
			return <div>preferences</div>;
		default:
			return <div>erro</div>;
	}
}
