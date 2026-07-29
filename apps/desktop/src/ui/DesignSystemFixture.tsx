import { useState } from "react";
import type { ConfirmationChallenge } from "../generated/ipc/ConfirmationChallenge";
import {
  Button,
  StatusMessage,
  Tabs,
  TextField,
  TrustedConfirmation,
} from ".";

const hostileText =
  "‮production.example\u202C — 非常に長いホスト名.example.invalid — <script>alert(1)</script>";

const fixtureChallenge: ConfirmationChallenge = {
  nonce: "018f0000-0000-7000-8000-000000000001",
  operation_id: "018f0000-0000-7000-8000-000000000002",
  displayed_digest: "sha256:fixture",
  expires_at_unix_ms: "0",
};

export function DesignSystemFixture() {
  const [tab, setTab] = useState("controls");
  return (
    <main className="design-fixture">
      <h1>Relio component fixture</h1>
      <Tabs
        activeId={tab}
        items={[
          {
            id: "controls",
            label: "Controls",
            panel: (
              <div className="fixture-stack">
                <TextField
                  description="Persistent descriptions do not rely on placeholders."
                  label="Host name"
                  value={hostileText}
                  readOnly
                />
                <div className="fixture-row">
                  <Button variant="primary">Create workspace</Button>
                  <Button>Cancel</Button>
                  <Button disabled>Unavailable</Button>
                  <Button loading>Connect</Button>
                </div>
                <StatusMessage tone="success">Connected successfully</StatusMessage>
                <StatusMessage tone="warning">Review required</StatusMessage>
                <StatusMessage tone="danger">Connection failed</StatusMessage>
              </div>
            ),
          },
          {
            id: "security",
            label: "Security",
            panel: (
              <TrustedConfirmation
                actionLabel="Replace stored host key"
                challenge={fixtureChallenge}
                consequence="The stored identity will be replaced for future connections."
                evidence={[
                  { label: "Fingerprint", value: "SHA256:abcdefghijklmnop" },
                  { label: "Reported target", value: hostileText },
                ]}
                onCancel={() => undefined}
                onConfirm={() => undefined}
                target={hostileText}
                title="Host identity changed"
              />
            ),
          },
        ]}
        label="Component groups"
        onChange={setTab}
      />
    </main>
  );
}
