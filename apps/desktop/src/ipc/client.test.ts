import { describe, expect, it } from "vitest";
import { EventCursor, EventGapError } from "./event-cursor";
import { IpcClient, type IpcTransport } from "./client";

class EchoTransport implements IpcTransport {
  public async invoke(_command: string, payload: unknown): Promise<unknown> {
    const request = payload as { request_id: string };
    return {
      contract_version: 1,
      request_id: request.request_id,
      outcome: "ok",
      body: { accepted: true },
    };
  }
}

describe("typed IPC client", () => {
  it("correlates a typed response to its request", async () => {
    const client = new IpcClient(new EchoTransport());
    await expect(
      client.query<{}, { accepted: boolean }>("test.status", {}, { windowId: "main" }),
    ).resolves.toEqual({ accepted: true });
  });

  it("normalizes malformed responses to a safe error", async () => {
    const client = new IpcClient({
      invoke: async () => ({ raw_secret: "must-not-cross" }),
    });
    await expect(
      client.query("test.status", {}, { windowId: "main" }),
    ).rejects.toMatchObject({
      detail: {
        code: "ipc.response_malformed",
        safe_message_key: "ipc.response_malformed",
      },
    });
  });

  it("detects an ordered-event gap", () => {
    const cursor = new EventCursor();
    cursor.accept({
      contract_version: 1,
      subscription_id: crypto.randomUUID(),
      aggregate_type: "test",
      aggregate_id: crypto.randomUUID(),
      sequence: 1,
      occurred_at_utc: new Date().toISOString(),
      operation_id: null,
      body: {},
    });
    expect(() =>
      cursor.accept({
        contract_version: 1,
        subscription_id: crypto.randomUUID(),
        aggregate_type: "test",
        aggregate_id: crypto.randomUUID(),
        sequence: 3,
        occurred_at_utc: new Date().toISOString(),
        operation_id: null,
        body: {},
      }),
    ).toThrow(EventGapError);
  });
});
