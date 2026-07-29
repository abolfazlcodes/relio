import { Button } from "../ui";

export type SurfaceStateKind = "empty" | "loading" | "error" | "unavailable";

export interface SurfaceStateProps {
  actionLabel?: string;
  description: string;
  kind: SurfaceStateKind;
  onAction?: () => void;
  title: string;
}

const stateSymbol: Record<SurfaceStateKind, string> = {
  empty: "◇",
  loading: "◌",
  error: "×",
  unavailable: "—",
};

export function SurfaceState({
  actionLabel,
  description,
  kind,
  onAction,
  title,
}: SurfaceStateProps) {
  return (
    <section
      aria-busy={kind === "loading" || undefined}
      className={`surface-state surface-state--${kind}`}
      role={kind === "error" ? "alert" : "status"}
    >
      <span aria-hidden="true" className="surface-state__symbol">
        {stateSymbol[kind]}
      </span>
      <h2>{title}</h2>
      <p>{description}</p>
      {actionLabel && onAction ? (
        <Button onClick={onAction} variant="primary">
          {actionLabel}
        </Button>
      ) : null}
    </section>
  );
}
