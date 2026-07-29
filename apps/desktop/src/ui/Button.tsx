import type { ButtonHTMLAttributes, ReactNode } from "react";

export type ButtonVariant = "primary" | "secondary" | "quiet" | "danger";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  children: ReactNode;
  loading?: boolean;
  variant?: ButtonVariant;
}

export function Button({
  children,
  disabled,
  loading = false,
  type = "button",
  variant = "secondary",
  ...props
}: ButtonProps) {
  return (
    <button
      {...props}
      className={`ui-button ui-button--${variant}`}
      disabled={disabled || loading}
      type={type}
    >
      {loading ? <span aria-hidden="true">◌</span> : null}
      <span>{children}</span>
      {loading ? <span className="sr-only"> in progress</span> : null}
    </button>
  );
}
