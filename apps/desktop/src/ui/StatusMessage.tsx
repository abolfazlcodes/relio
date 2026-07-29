import type { ReactNode } from "react";

export type StatusTone = "success" | "warning" | "danger" | "info";

const statusIcon: Record<StatusTone, string> = {
  success: "✓",
  warning: "!",
  danger: "×",
  info: "i",
};

export interface StatusMessageProps {
  children: ReactNode;
  live?: "off" | "polite" | "assertive";
  tone: StatusTone;
}

export function StatusMessage({
  children,
  live = "polite",
  tone,
}: StatusMessageProps) {
  return (
    <div
      aria-live={live}
      className={`ui-message ui-message--${tone}`}
      role={tone === "danger" ? "alert" : "status"}
    >
      <span className="ui-message__icon" aria-hidden="true">
        {statusIcon[tone]}
      </span>
      <span>{children}</span>
    </div>
  );
}
