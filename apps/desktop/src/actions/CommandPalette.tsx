import {
  useEffect,
  useMemo,
  useRef,
  useState,
  type KeyboardEvent,
} from "react";
import type { ActionRegistry } from "./registry";

export interface CommandPaletteProps<TContext> {
  context: TContext;
  onClose: () => void;
  open: boolean;
  registry: ActionRegistry<TContext>;
}

export function CommandPalette<TContext>({
  context,
  onClose,
  open,
  registry,
}: CommandPaletteProps<TContext>) {
  const [query, setQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const dialogRef = useRef<HTMLDialogElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const priorFocus = useRef<HTMLElement | null>(null);
  const results = useMemo(
    () => registry.search(query, context),
    [context, query, registry],
  );

  useEffect(() => {
    const dialog = dialogRef.current;
    if (!dialog) return;
    if (open) {
      priorFocus.current = document.activeElement as HTMLElement | null;
      if (typeof dialog.showModal === "function" && !dialog.open) dialog.showModal();
      else dialog.setAttribute("open", "");
      setQuery("");
      setSelectedIndex(0);
      inputRef.current?.focus();
    } else if (dialog.open) {
      if (typeof dialog.close === "function") dialog.close();
      else dialog.removeAttribute("open");
    }
  }, [open]);

  useEffect(() => {
    if (!open) priorFocus.current?.focus();
  }, [open]);

  useEffect(() => {
    setSelectedIndex(0);
  }, [query]);

  async function invokeSelected(): Promise<void> {
    const selected = results[selectedIndex];
    if (!selected?.available) return;
    if (await registry.dispatch(selected.actionId, context)) onClose();
  }

  function handleKeyDown(event: KeyboardEvent): void {
    if (event.key === "Escape") {
      event.preventDefault();
      onClose();
    } else if (event.key === "ArrowDown") {
      event.preventDefault();
      setSelectedIndex((current) => Math.min(current + 1, results.length - 1));
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      setSelectedIndex((current) => Math.max(current - 1, 0));
    } else if (event.key === "Enter") {
      event.preventDefault();
      void invokeSelected();
    }
  }

  return (
    <dialog
      aria-labelledby="command-palette-title"
      className="command-palette"
      onCancel={(event) => {
        event.preventDefault();
        onClose();
      }}
      onKeyDown={handleKeyDown}
      ref={dialogRef}
    >
      <h1 className="sr-only" id="command-palette-title">
        Command palette
      </h1>
      <label className="command-palette__search">
        <span aria-hidden="true">&gt;</span>
        <span className="sr-only">Search actions and destinations</span>
        <input
          aria-controls="command-palette-results"
          aria-label="Search actions and destinations"
          onChange={(event) => setQuery(event.currentTarget.value)}
          placeholder="Search actions and destinations"
          ref={inputRef}
          value={query}
        />
      </label>
      <p aria-live="polite" className="sr-only">
        {results.length} results
      </p>
      <ul
        aria-label="Command results"
        className="command-palette__results"
        id="command-palette-results"
        role="listbox"
      >
        {results.map((result, index) => (
          <li
            aria-disabled={!result.available}
            aria-selected={index === selectedIndex}
            className="command-palette__result"
            id={`palette-result-${index}`}
            key={result.actionId}
            onMouseDown={(event) => event.preventDefault()}
            onMouseEnter={() => setSelectedIndex(index)}
            onClick={() => {
              if (result.available) void registry.dispatch(result.actionId, context).then(onClose);
            }}
            role="option"
          >
            <span>
              <strong>{result.label}</strong>
              <small>{result.category}</small>
              {result.reason ? <small>{result.reason}</small> : null}
            </span>
            {result.shortcut ? <kbd aria-label={result.shortcut}>{result.shortcut}</kbd> : null}
          </li>
        ))}
      </ul>
    </dialog>
  );
}
