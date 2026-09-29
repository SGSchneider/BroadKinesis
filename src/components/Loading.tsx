export default function Loading() {
	return (
		<div
			className="flex min-h-40 w-full items-center justify-center"
			role="status"
			aria-label="Carregando configurações da Twitch"
		>
			<div className="loading-dots" aria-hidden="true">
				<span />
				<span />
				<span />
			</div>
		</div>
	);
}
