import {
  useRef,
  type KeyboardEvent,
  type ReactNode,
} from "react";

export interface TabItem {
  id: string;
  label: string;
  panel: ReactNode;
}

export interface TabsProps {
  activeId: string;
  items: readonly TabItem[];
  label: string;
  onChange: (id: string) => void;
}

export function Tabs({ activeId, items, label, onChange }: TabsProps) {
  const tabRefs = useRef(new Map<string, HTMLButtonElement>());
  const active = items.find((item) => item.id === activeId) ?? items[0];

  function moveFocus(event: KeyboardEvent, currentIndex: number): void {
    if (!["ArrowLeft", "ArrowRight", "Home", "End"].includes(event.key)) return;
    event.preventDefault();
    let nextIndex = currentIndex;
    if (event.key === "Home") nextIndex = 0;
    if (event.key === "End") nextIndex = items.length - 1;
    if (event.key === "ArrowLeft") nextIndex = (currentIndex - 1 + items.length) % items.length;
    if (event.key === "ArrowRight") nextIndex = (currentIndex + 1) % items.length;
    const next = items[nextIndex];
    if (next) {
      onChange(next.id);
      tabRefs.current.get(next.id)?.focus();
    }
  }

  return (
    <div>
      <div aria-label={label} className="ui-tabs" role="tablist">
        {items.map((item, index) => (
          <button
            aria-controls={`${item.id}-panel`}
            aria-selected={item.id === active?.id}
            className="ui-tab"
            id={`${item.id}-tab`}
            key={item.id}
            onClick={() => onChange(item.id)}
            onKeyDown={(event) => moveFocus(event, index)}
            ref={(element) => {
              if (element) tabRefs.current.set(item.id, element);
              else tabRefs.current.delete(item.id);
            }}
            role="tab"
            tabIndex={item.id === active?.id ? 0 : -1}
            type="button"
          >
            {item.label}
          </button>
        ))}
      </div>
      {active ? (
        <section
          aria-labelledby={`${active.id}-tab`}
          id={`${active.id}-panel`}
          role="tabpanel"
          tabIndex={0}
        >
          {active.panel}
        </section>
      ) : null}
    </div>
  );
}
