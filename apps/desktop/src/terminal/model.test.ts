import { describe, expect, it, vi } from "vitest";
import type { TerminalChannelEvent } from "../generated/ipc/TerminalChannelEvent";
import type { TerminalTransport } from "./transport";
import { INITIAL_OUTPUT_CREDIT_BYTES, TerminalModel } from "./model";

function fixture() {
  let onEvent: ((event: TerminalChannelEvent) => void) | undefined;
  const transport: TerminalTransport = {
    grantCredit: vi.fn(async () => undefined),
    listProfiles: vi.fn(async () => []),
    resize: vi.fn(async () => undefined),
    sendInput: vi.fn(async () => undefined),
    start: vi.fn(async (_request, listener) => {
      onEvent = listener;
      return {
        initial_input_sequence: "0",
        maximum_output_credit_bytes: 4 * INITIAL_OUTPUT_CREDIT_BYTES,
        session_id: "session-1",
      };
    }),
    stop: vi.fn(async () => undefined),
  };
  const writes: Array<Uint8Array> = [];
  const oscHandlers = new Map<number, (data: string) => boolean>();
  const terminal = {
    cols: 80,
    element: undefined,
    loadAddon: vi.fn(),
    onBinary: vi.fn(() => ({ dispose: vi.fn() })),
    onData: vi.fn(() => ({ dispose: vi.fn() })),
    onTitleChange: vi.fn(() => ({ dispose: vi.fn() })),
    open: vi.fn(),
    parser: {
      registerCsiHandler: vi.fn(),
      registerDcsHandler: vi.fn(),
      registerEscHandler: vi.fn(),
      registerOscHandler: vi.fn((identifier: number, handler: (data: string) => boolean) => {
        oscHandlers.set(identifier, handler);
        return { dispose: vi.fn() };
      }),
    },
    rows: 24,
    write: vi.fn((data: Uint8Array, callback?: () => void) => {
      writes.push(data);
      callback?.();
    }),
  };
  const events = {
    onExternalUri: vi.fn(),
    onOutputGap: vi.fn(),
    onState: vi.fn(),
    onTitle: vi.fn(),
  };
  const fitAddon = { activate: vi.fn(), dispose: vi.fn(), fit: vi.fn(), proposeDimensions: vi.fn() };
  const model = new TerminalModel(
    transport,
    events,
    {},
    terminal as never,
    fitAddon as never,
  );
  return { events, model, onEvent: () => onEvent, oscHandlers, transport, writes };
}

describe("terminal model byte path", () => {
  it("credits only output committed to xterm and reports sequence gaps", async () => {
    const { events, model, onEvent, transport, writes } = fixture();
    await model.start("default");
    expect(transport.grantCredit).toHaveBeenCalledWith({
      bytes: INITIAL_OUTPUT_CREDIT_BYTES,
      session_id: "session-1",
    });

    onEvent()?.({ bytes: [65, 66], event: "output", sequence: "0" });
    onEvent()?.({ bytes: [67], event: "output", sequence: "2" });

    expect(writes.map((bytes) => [...bytes])).toEqual([[65, 66], [67]]);
    expect(events.onOutputGap).toHaveBeenCalledWith("1", "2");
    expect(transport.grantCredit).toHaveBeenLastCalledWith({
      bytes: 1,
      session_id: "session-1",
    });
  });

  it("blocks terminal-driven clipboard writes through OSC 52", () => {
    const { oscHandlers } = fixture();
    expect(oscHandlers.get(52)?.("clipboard payload")).toBe(true);
  });

  it("keeps 32 MiB of sustained adapter output within the initial budget", async () => {
    const { model, onEvent, writes } = fixture();
    await model.start("default");
    const payload = Array.from({ length: 8 * 1_024 }, () => 65);
    const startedAt = performance.now();

    for (let sequence = 0; sequence < 4_096; sequence += 1) {
      onEvent()?.({ bytes: payload, event: "output", sequence: String(sequence) });
    }

    expect(writes).toHaveLength(4_096);
    expect(performance.now() - startedAt).toBeLessThan(2_000);
  });
});
