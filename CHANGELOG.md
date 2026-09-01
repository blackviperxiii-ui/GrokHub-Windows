# Changelog

## 2.8.0 — 2026-09-01

GrokHub cabin for Grok Build **1.0.17** alpha (covers 1.0.15–1.0.17).

- MCP tools that need a form or URL (`x.ai/mcp/elicit`) paint an Accept / Decline card instead of failing the turn.
- URL elicitation opens the link; form elicitation can take the first string field.
- Failed or waiting `input_required` tool calls stay visible on the Queue.
- 1.0.15–1.0.16 CLI speed and token-refresh fixes ride through `grok -p` / ACP.

## 2.7.1 — 2026-08-31

GrokHub cabin for Grok Build **1.0.14** alpha (parity with Linux `8ffd565`, plus Cursor #8).

- `/usage` also runs `grok usage <session>` for persisted per-turn tokens and cost.
- Retry status in the composer shows a short reason.
- Failed task/todo tool calls stay on the Queue as **failed**.
- `/inspect` notes Grok Build version and that Claude bypass locks are advisory.
- Subagent coordinator “unreachable” retries instead of killing the turn.
- `/models` lists per-effort model ids when the CLI prints them.
- A chat opens on its newest message, with a way back down (Cursor #8).

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
