import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import TransparentButton from "../components/TransparentButton";
export default function Connections() {
	const [page, setPage] = useState("twitch");

	return (
		<div className="w-full">
			<div className="border w-full border-neutral-700">
				<TransparentButton
					onClick={() => {
						setPage("twitch");
					}}
				>
					Twitch
				</TransparentButton>
				|
				<TransparentButton
					onClick={() => {
						setPage("youtube");
					}}
				>
					YouTube
				</TransparentButton>
				|
				<TransparentButton
					onClick={() => {
						setPage("obs");
					}}
				>
					OBS
				</TransparentButton>
				|
			</div>

			<div>
				<Platform platform={page} />
			</div>
		</div>
	);
}

function Platform({ platform }: { platform: string }) {
	switch (platform) {
		case "twitch":
			return <Twitch />;
		case "youtube":
			return <Youtube />;
		case "obs":
			return <Obs />;
		default:
			return <Twitch />;
	}
}

function Twitch() {
	return "Twitch";
}

function Youtube() {
	return "Twitch";
}

function Obs() {
	const [address, setAddress] = useState<string>("127.0.0.1");
	const [port, setPort] = useState<string>("4455");
	const [password, setPassword] = useState<string>("");
	const [reconnect, setReconnect] = useState(true);
	const [reconnectTime, setReconnectTime] = useState("30");

	function Connect() {}

	return (
		<div className="w-full h-full flex flex-col">
			<div className="flex flex-row">
				<div>
					<form className="mt-1 ms-1">
						<p className="m-1">Address</p>
						<input
							type="text"
							placeholder="127.0.0.1"
							value={address}
							onChange={(event) => {
								const nextAddress = event.currentTarget.value.replace(
									/[^0-9.]/g,
									"",
								);
								setAddress(nextAddress);
								void SaveOnDB(
									nextAddress,
									port,
									password,
									reconnect,
									reconnectTime,
								);
							}}
							className="m-1"
						/>
					</form>
				</div>
				<div>
					<form className="mt-1 ms-1">
						<p className="m-1">Port</p>
						<input
							type="number"
							placeholder="4455"
							value={port}
							onChange={(event) => {
								const nextPort = event.currentTarget.value.replace(
									/[^0-9]/g,
									"",
								);
								setPort(nextPort);
								void SaveOnDB(
									address,
									nextPort,
									password,
									reconnect,
									reconnectTime,
								);
							}}
							className="m-1 hide-number-controls"
						/>
					</form>
				</div>
			</div>
			<div className="flex flex-row">
				<div>
					<form className="mt-1 ms-1">
						<p className="m-1">Password (if enabled)</p>
						<input
							type="password"
							placeholder="password"
							value={password}
							onChange={(event) => {
								const nextPassword = event.currentTarget.value;
								setPassword(nextPassword);
								void SaveOnDB(
									address,
									port,
									nextPassword,
									reconnect,
									reconnectTime,
								);
							}}
							className="m-1 hide-number-controls"
						/>
					</form>
				</div>
				<div className="flex flex-col mt-1">
					<p>
						<br />
					</p>
					<button type="submit" className="h-fit m-2" onClick={() => Connect()}>
						Conectar
					</button>
				</div>
			</div>
			<div className="flex flex-row">
				<div>
					<form className="mt-1 ms-1 flex">
						<p className="m-1">Auto Reconnect</p>
						<input
							type="checkbox"
							checked={reconnect}
							onChange={(event) => {
								const nextReconnect = event.currentTarget.checked;
								setReconnect(nextReconnect);
								void SaveOnDB(
									address,
									port,
									password,
									nextReconnect,
									reconnectTime,
								);
							}}
							className="m-2 hide-number-controls w-4 h-4"
						/>
					</form>
				</div>
				<div>
					{reconnect ? (
						<form className="mt-1 ms-1">
							<p className="m-1">Time Between Attempts (In Seconds)</p>
							<input
								type="number"
								placeholder="30"
								value={reconnectTime}
								onChange={(event) => {
									const nextReconnectTime = event.currentTarget.value;
									setReconnectTime(nextReconnectTime);
									void SaveOnDB(
										address,
										port,
										password,
										reconnect,
										nextReconnectTime,
									);
								}}
								className="hide-number-controls"
							/>
						</form>
					) : (
						<div></div>
					)}
				</div>
			</div>
		</div>
	);
}

type ObsWebsocket = {
	address: string;
	port: number;
	password: string;
	autoReconnect: boolean;
	autoReconnectTime: number;
};

async function SaveOnDB(
	address: string,
	port: string,
	password: string,
	reconnect: boolean,
	reconnectTime: string,
) {
	const values: ObsWebsocket = {
		address,
		port: Number(port),
		password,
		autoReconnect: reconnect,
		autoReconnectTime: Number(reconnectTime),
	};
	invoke("set_obs_websocket", { creds: values }).catch(console.error);
}
