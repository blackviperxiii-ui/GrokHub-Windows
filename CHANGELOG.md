# Changelog

## 2.7.0 — 2026-08-31

Parity with Linux after Cursor PRs #5–#7:

- JSON stores no longer wipe chat history at 1 MiB; corrupt files are quarantined.
- Hub pairing holes closed; hub state is durable.
- Clock automations fire again; composer pills persist; `/usage` counts Imagine and tokens.
- Searchable History, cabin skills on the Skills page, honest settings knobs.

## 2.7.0 — 2026-08-29

GrokHub cabin for Grok Build **1.0.13** stable.

### Grok Build 1.0.13

- Truncated replies and transient 5xx / stalls keep the turn instead of dumping an error.
- Compaction failures show the real CLI message.
- Credit-limit errors offer **Try Again** (`/retry`).
- Hook `ask` reasons paint on the permission card.
- Loops page reminds you to stop a loop when the work is done.
- Overlay / docs use `grok update` on the stable channel (`--alpha` is optional).

### History = `grok sessions`

- The sidebar and History page are `grok sessions list` only (no subagent disk walk).
- Delete runs `grok sessions delete` against `~/.grok`, then relists after that command finishes so rows cannot flicker back.
- New chats use the user Grok home so they appear in the same list as the TUI.

### This desktop

- Chat is `grok -p` with `--sandbox off` and a desktop rule: Grok has filesystem and shell here.
- Grok.com-style “I don’t have access to your computer” thoughts are stripped from the pane.
- Halt / Stop still SIGTERMs the `grok -p` child.

## 2.6.42 — 2026-08-26

ACP Ask, compact/rewind, context bar, Grok Build 1.0.11–1.0.12 wiring, thought clustering, History See all / Delete all.
