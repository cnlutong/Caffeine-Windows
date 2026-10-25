import type { CSSProperties } from "react";

type IconName = "coffee" | "off" | "system" | "display" | "info";
const paths: Record<IconName, React.ReactNode> = {
  coffee: (
    <>
      <path d="M4 8h13v6a6 6 0 0 1-12 0L4 8Z" />
      <path d="M17 9h1a3 3 0 1 1 0 6h-1M7 4c-1-1 1-2 0-3m5 3c-1-1 1-2 0-3M5 22h12" />
    </>
  ),
  off: <path d="M20.5 14A8.5 8.5 0 0 1 10 3.5 8.5 8.5 0 1 0 20.5 14Z" />,
  system: (
    <>
      <path d="M20.5 14A8.5 8.5 0 0 1 10 3.5M5 6a8.5 8.5 0 0 0 12.5 12.5M3 3l18 18" />
    </>
  ),
  display: (
    <>
      <circle cx="12" cy="12" r="4.5" />
      <path d="M12 1v2m0 18v2M1 12h2m18 0h2M4.2 4.2l1.4 1.4m12.8 12.8 1.4 1.4m0-15.6-1.4 1.4M5.6 18.4l-1.4 1.4" />
    </>
  ),
  info: (
    <>
      <circle cx="12" cy="12" r="9" />
      <path d="M12 11v6m0-10v.1" />
    </>
  ),
};

export function Icon({
  name,
  size = 22,
  style,
}: {
  name: IconName;
  size?: number;
  style?: CSSProperties;
}) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.7"
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
      style={style}
    >
      {paths[name]}
    </svg>
  );
}
