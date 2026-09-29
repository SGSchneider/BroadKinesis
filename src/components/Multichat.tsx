import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Crown, Star, Wrench } from "lucide-react";
import { useCallback, useEffect, useState } from "react";

type ChatMessage = {
	id: string;
	sender: string;
	text: string;
	color?: string;
	badges: ChatBadge[];
	emotes: ChatEmote[];
	bits?: number;
};

type ChatBadge = {
	name: string;
	version: string;
	imageUrl?: string;
};

type ChatEmote = {
	id: string;
	code: string;
};

export function TwitchChat() {
	const [messages, setMessages] = useState<ChatMessage[]>([]);
	const [error, setError] = useState<string | null>(null);

	const handleConnectChat = useCallback(async () => {
		const results = await Promise.allSettled([
			invoke("start_twitch_chat", { channel: "s_schneider" }),
			invoke("start_youtube_chat"),
		]);

		for (const result of results) {
			if (result.status === "rejected") {
				setError(String(result.reason));
			}
		}
	}, []);

	useEffect(() => {
		let unlisten: UnlistenFn[] = [];
		let isDisposed = false;

		async function setupChat() {
			const [stopMessages, stopErrors] = await Promise.all([
				listen<ChatMessage>("chat-message", (event) => {
					setMessages((prev) => [...prev, event.payload]);
				}),
				listen<string>("chat-error", (event) => {
					setError(event.payload);
				}),
			]);
			unlisten = [stopMessages, stopErrors];

			if (isDisposed) {
				unlisten.forEach((stop) => stop());
				return;
			}

			try {
				const history = await invoke<ChatMessage[]>("get_twitch_chat_history");
				if (!isDisposed) setMessages(history);
			} catch (reason) {
				setError(reason instanceof Error ? reason.message : String(reason));
			}

			await handleConnectChat();
		}

		void setupChat();
		return () => {
			isDisposed = true;
			unlisten.forEach((stop) => stop());
		};
	}, [handleConnectChat]);
	return (
		<div className="flex-1 overflow-y-auto bg-neutral-900 p-2 rounded">
			{error && <p className="mb-2 text-sm text-red-400">{error}</p>}
			{messages.map((msg) => (
				<div key={msg.id} className="mb-1 text-sm">
					{msg.badges.map((badge) =>
						badge.imageUrl ? (
							<img
								key={`${badge.name}-${badge.version}`}
								src={badge.imageUrl}
								alt={badge.name}
								title={`${badge.name} ${badge.version}`}
								className="mr-1 inline-block h-5 w-5 align-middle"
							/>
						) : (
							<BadgeIcon
								key={`${badge.name}-${badge.version}`}
								name={badge.name}
								version={badge.version}
							/>
						),
					)}
					<span
						className="font-bold mr-2"
						style={{ color: msg.color || "#ffffff" }}
					>
						{msg.sender}:
					</span>
					<MessageText message={msg} />
				</div>
			))}
		</div>
	);
}

function BadgeIcon({ name, version }: { name: string; version: string }) {
	const iconProps = {
		className: "mr-1 inline-block h-5 w-5 align-middle",
		strokeWidth: 2.5,
		"aria-label": `${name} ${version}`,
	};

	if (name === "owner") {
		return (
			<Crown
				{...iconProps}
				className="mr-1 inline-block h-5 w-5 align-middle text-yellow-400"
			/>
		);
	}

	if (name === "moderator") {
		return (
			<Wrench
				{...iconProps}
				className="mr-1 inline-block h-5 w-5 align-middle text-green-400"
			/>
		);
	}

	if (name === "member") {
		return (
			<Star
				{...iconProps}
				className="mr-1 inline-block h-5 w-5 align-middle text-blue-400"
			/>
		);
	}
	return <></>;
}

function MessageText({ message }: { message: ChatMessage }) {
	if (message.emotes.length === 0) {
		return <span className="text-gray-300">{message.text}</span>;
	}

	const parts: React.ReactNode[] = [];
	let cursor = 0;

	message.emotes.forEach((emote, index) => {
		const start = message.text.indexOf(emote.code, cursor);
		if (start === -1) return;

		if (start > cursor) {
			parts.push(
				<span key={`text-${index}`} className="text-gray-300">
					{message.text.slice(cursor, start)}
				</span>,
			);
		}

		parts.push(
			<img
				key={`emote-${index}`}
				src={`https://static-cdn.jtvnw.net/emoticons/v2/${emote.id}/default/dark/1.0`}
				alt={emote.code}
				title={emote.code}
				className="mx-0.5 inline-block h-7 w-7 align-middle"
			/>,
		);
		cursor = start + emote.code.length;
	});

	if (cursor < message.text.length) {
		parts.push(
			<span key="text-end" className="text-gray-300">
				{message.text.slice(cursor)}
			</span>,
		);
	}

	return <span>{parts}</span>;
}

export async function ToggleChatWindow() {
	// Find the window by the label defined below ("chat-window").
	const existingWindow = await WebviewWindow.getByLabel("chat-window");

	if (existingWindow) {
		// Close the window if it is already open.
		await existingWindow.close();
	} else {
		// Create a new window if it does not exist.
		const chatWindow = new WebviewWindow("chat-window", {
			title: "Chat - Broadkinesis",
			width: 350,
			height: 600,
			decorations: true, // Set to false when using the custom navbar.
		});

		chatWindow.once("tauri://error", (e) => {
			console.error("Error creating the chat window:", e);
		});
	}
}
