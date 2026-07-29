import { useEffect, useRef } from "react";
import type { ConfirmationChallenge } from "../generated/ipc/ConfirmationChallenge";
import { Button } from "./Button";

export interface ConfirmationEvidence {
  label: string;
  value: string;
}

export interface TrustedConfirmationProps {
  actionLabel: string;
  challenge: ConfirmationChallenge;
  consequence: string;
  evidence: readonly ConfirmationEvidence[];
  onCancel: () => void;
  onConfirm: (challenge: ConfirmationChallenge) => void;
  target: string;
  title: string;
}

export function TrustedConfirmation({
  actionLabel,
  challenge,
  consequence,
  evidence,
  onCancel,
  onConfirm,
  target,
  title,
}: TrustedConfirmationProps) {
  const dialogRef = useRef<HTMLDialogElement>(null);
  const titleRef = useRef<HTMLHeadingElement>(null);
  useEffect(() => {
    const dialog = dialogRef.current;
    if (!dialog) return;
    if (typeof dialog.showModal === "function" && !dialog.open) dialog.showModal();
    else dialog.setAttribute("open", "");
    titleRef.current?.focus();
  }, []);
  return (
    <dialog
      aria-describedby="security-consequence"
      aria-labelledby="security-title"
      aria-modal="true"
      className="ui-dialog trusted-confirmation"
      onCancel={(event) => { event.preventDefault(); onCancel(); }}
      ref={dialogRef}
      data-relio-trusted-surface="confirmation"
      role="alertdialog"
    >
      <div className="trusted-confirmation__label">
        <span aria-hidden="true">◆</span>
        <span>Relio security check</span>
      </div>
      <h1 className="ui-dialog__title" id="security-title" ref={titleRef} tabIndex={-1}>
        {title}
      </h1>
      <p>
        Target: <strong>{target}</strong>
      </p>
      <dl className="trusted-confirmation__evidence">
        {evidence.map((item) => (
          <div key={item.label}>
            <dt>{item.label}</dt>
            <dd dir="auto">{item.value}</dd>
          </div>
        ))}
      </dl>
      <p id="security-consequence">{consequence}</p>
      <div className="ui-dialog__actions">
        <Button onClick={onCancel}>Cancel</Button>
        <Button
          onClick={() => onConfirm(challenge)}
          variant="danger"
        >
          {actionLabel}
        </Button>
      </div>
    </dialog>
  );
}
