export default function TransparentButton({
	onClick,
	children,
	className,
}: {
	onClick: () => void;
	children: React.ReactNode;
	className?: string;
}) {
	return (
		<button
			type="button"
			className={`bg-transparent! border-none! outline-none! ${className}`}
			onClick={onClick}
		>
			{children}
		</button>
	);
}
