import type { EventEnvelope } from "../generated/ipc/EventEnvelope";

export class EventGapError extends Error {
  public constructor(
    public readonly expected: number,
    public readonly received: number,
  ) {
    super("ipc.event_gap");
    this.name = "EventGapError";
  }
}

export class EventCursor {
  private sequence = 0;

  public accept<T>(event: EventEnvelope<T>): void {
    const expected = this.sequence + 1;
    if (event.sequence !== expected) {
      throw new EventGapError(expected, event.sequence);
    }
    this.sequence = event.sequence;
  }

  public reset(sequence: number): void {
    this.sequence = sequence;
  }

  public current(): number {
    return this.sequence;
  }
}
