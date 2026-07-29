import "@testing-library/jest-dom/vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";
import type { ConfirmationChallenge } from "../generated/ipc/ConfirmationChallenge";
import {
  Button,
  StatusMessage,
  Tabs,
  TextField,
  TrustedConfirmation,
} from ".";

const challenge: ConfirmationChallenge = {
  nonce: "018f0000-0000-7000-8000-000000000001",
  operation_id: "018f0000-0000-7000-8000-000000000002",
  displayed_digest: "sha256:test",
  expires_at_unix_ms: "1",
};

describe("design-system accessibility contracts", () => {
  it("keeps field labels and validation relationships persistent", () => {
    render(
      <TextField
        description="Use a DNS name."
        error="A host is required."
        label="Host"
      />,
    );
    const input = screen.getByRole("textbox", { name: "Host" });
    expect(input).toHaveAttribute("aria-invalid", "true");
    expect(input).toHaveAccessibleDescription(
      "Use a DNS name. A host is required.",
    );
  });

  it("represents status with text and a non-color icon", () => {
    render(<StatusMessage tone="warning">Fingerprint review required</StatusMessage>);
    expect(screen.getByRole("status")).toHaveTextContent(
      "Fingerprint review required",
    );
    expect(screen.getByText("!", { selector: "[aria-hidden=true]" })).toBeInTheDocument();
  });

  it("prevents duplicate activation while a button is loading", () => {
    const onClick = vi.fn();
    render(
      <Button loading onClick={onClick}>
        Connect
      </Button>,
    );
    const button = screen.getByRole("button", { name: /Connect in progress/u });
    expect(button).toBeDisabled();
    fireEvent.click(button);
    expect(onClick).not.toHaveBeenCalled();
  });

  it("moves tab focus and selection with arrow keys", () => {
    function Fixture() {
      const [active, setActive] = useState("one");
      return (
        <Tabs
          activeId={active}
          items={[
            { id: "one", label: "One", panel: "First" },
            { id: "two", label: "Two", panel: "Second" },
          ]}
          label="Fixture"
          onChange={setActive}
        />
      );
    }
    render(<Fixture />);
    const first = screen.getByRole("tab", { name: "One" });
    first.focus();
    fireEvent.keyDown(first, { key: "ArrowRight" });
    expect(screen.getByRole("tab", { name: "Two" })).toHaveFocus();
    expect(screen.getByRole("tabpanel")).toHaveTextContent("Second");
  });

  it("renders hostile evidence as text inside reserved confirmation chrome", () => {
    const hostile = "‮prod\u202C <img src=x onerror=alert(1)> 非常に長い対象";
    const onConfirm = vi.fn();
    const { container } = render(
      <TrustedConfirmation
        actionLabel="Replace stored host key"
        challenge={challenge}
        consequence="Future connections will use this identity."
        evidence={[{ label: "Reported target", value: hostile }]}
        onCancel={() => undefined}
        onConfirm={onConfirm}
        target={hostile}
        title="Host identity changed"
      />,
    );
    expect(
      container.querySelector('[data-relio-trusted-surface="confirmation"]'),
    ).toBeInTheDocument();
    expect(container.querySelector("img")).not.toBeInTheDocument();
    expect(screen.getAllByText(hostile)).toHaveLength(2);
    fireEvent.click(
      screen.getByRole("button", { name: "Replace stored host key" }),
    );
    expect(onConfirm).toHaveBeenCalledWith(challenge);
  });
});
