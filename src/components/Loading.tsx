export default function Loading() {
	return (
		<div
			className="flex min-h-40 w-full items-center justify-center"
			role="status"
			aria-label="Loading Twitch settings"
		>
			<div className="loading-dots" aria-hidden="true">
				<span />
				<span />
				<span />
			</div>
		</div>
	);
}
