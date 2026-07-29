import { describe, expect, it } from "vitest";
import {
  emptyCloseBlockers,
  FrontendLifecycleCoordinator,
  hasCloseBlockers,
} from "./coordinator";

describe("FrontendLifecycleCoordinator", () => {
  it("requires explicit close review before shutdown", () => {
    const lifecycle = new FrontendLifecycleCoordinator();
    expect(() => lifecycle.beginShutdown()).toThrow();
    lifecycle.requestClose();
    expect(lifecycle.snapshot.shutdown).toBe("reviewing");
    lifecycle.beginShutdown();
    expect(lifecycle.snapshot.shutdown).toBe("quiescing");
  });

  it("invalidates pending confirmation and cancels review on OS lock", () => {
    const lifecycle = new FrontendLifecycleCoordinator();
    lifecycle.requestClose();
    lifecycle.onOsSessionLocked();
    expect(lifecycle.snapshot).toMatchObject({
      shutdown: "running",
      confirmationEpoch: 1,
    });
  });

  it("invalidates confirmation authority when the webview is lost", () => {
    const lifecycle = new FrontendLifecycleCoordinator();
    lifecycle.onWebviewLost();
    expect(lifecycle.snapshot.confirmationEpoch).toBe(1);
  });

  it("reports every documented close blocker", () => {
    expect(hasCloseBlockers(emptyCloseBlockers)).toBe(false);
    for (const blocker of [
      { dirtyRemoteBuffers: 1 },
      { transfers: 1 },
      { tunnels: 1 },
      { recordings: 1 },
      { protectedMigration: true },
    ]) {
      expect(hasCloseBlockers({ ...emptyCloseBlockers, ...blocker })).toBe(true);
    }
  });
});
