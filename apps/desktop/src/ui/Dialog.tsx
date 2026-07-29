import {
  useEffect,
  useId,
  useRef,
  type ReactNode,
} from "react";

export interface DialogProps {
  actions: ReactNode;
  children: ReactNode;
  onCancel: () => void;
  open: boolean;
  title: string;
}

export function Dialog({
  actions,
  children,
  onCancel,
  open,
  title,
}: DialogProps) {
  const dialogRef = useRef<HTMLDialogElement>(null);
  const titleRef = useRef<HTMLHeadingElement>(null);
  const titleId = useId();

  useEffect(() => {
    const dialog = dialogRef.current;
    if (!dialog) return;
    if (open && !dialog.open) {
      if (typeof dialog.showModal === "function") dialog.showModal();
      else dialog.setAttribute("open", "");
      titleRef.current?.focus();
    } else if (!open && dialog.open) {
      if (typeof dialog.close === "function") dialog.close();
      else dialog.removeAttribute("open");
    }
  }, [open]);

  return (
    <dialog
      aria-labelledby={titleId}
      className="ui-dialog"
      onCancel={(event) => {
        event.preventDefault();
        onCancel();
      }}
      ref={dialogRef}
    >
      <h1 className="ui-dialog__title" id={titleId} ref={titleRef} tabIndex={-1}>
        {title}
      </h1>
      <div>{children}</div>
      <footer className="ui-dialog__actions">{actions}</footer>
    </dialog>
  );
}
