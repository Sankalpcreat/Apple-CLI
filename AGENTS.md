# AGENTS.md (apple-cli)

Read this first if you are an LLM/agent working in this repo.

## What this is

`apple-cli` is a local-first macOS CLI that drives **Notes**, **Reminders**, **Calendar**, and **Messages** via AppleScript. It runs entirely on device and requires macOS Automation permissions for each target app.

Binary name: `apple`

## Quick start

```bash
git clone https://github.com/Sankalpcreat/Apple-CLI.git
cd Apple-CLI
cargo build --release
sudo cp target/release/apple /usr/local/bin/
```

Local install (no sudo):

```bash
cargo install --path .
```

First run will trigger Automation permission prompts for Notes/Reminders/Calendar/Messages.
If commands fail with `-10827` or `AppleEvent handler failed`, enable permissions:
System Settings → Privacy & Security → Automation.

## Repo map

```
apple-cli/
├── README.md
├── AGENTS.md
├── Cargo.toml
├── assets/
│   └── apple-cli-banner.png
└── src/
    ├── main.rs
    ├── common.rs
    ├── notes.rs
    ├── reminders.rs
    ├── calendar.rs
    └── messages.rs
```

`src/common.rs` contains AppleScript execution helpers and record parsing.

## Command surface (high level)

Notes:
- `apple notes accounts list`
- `apple notes folders list|create|delete`
- `apple notes list|get|create|update|delete|move|search|show`
- `apple notes attachments list|save|delete`

Reminders:
- `apple reminders lists`
- `apple reminders lists-create|lists-update|lists-delete`
- `apple reminders list|get|create|update|complete|delete`

Calendar:
- `apple calendar calendars`
- `apple calendar calendars-create|calendars-delete`
- `apple calendar events|get|create|update|delete|show`
- `apple calendar alarms list|add|delete`
- `apple calendar attendees list|add`

Messages:
- `apple messages services`
- `apple messages chats [--type imessage|sms|rcs]`
- `apple messages chat-participants --id <chat_id>`
- `apple messages buddies --type imessage`
- `apple messages send --to <handle> --text "Hello"`
- `apple messages send-chat --id <chat_id> --text "Hello"`

## Known limitations

- Notes: attachment delete can fail on some macOS builds. Deleting the note removes attachments reliably.
- Calendar: alarm delete can fail on some macOS builds; status updates are best-effort.
- Messages: no transcript/history, read receipts, typing indicators, stickers, or voice notes (not exposed in AppleScript dictionary).

## Testing

There is no automated test suite; verification is done by running the CLI against a live macOS profile with Automation permissions. See README for the latest manual test status and dates.

## Safe automation guidance

These commands can delete user data (notes, reminders, events). When adding or running destructive actions:
- Confirm the target IDs and names.
- Prefer creating test folders/lists/calendars first.
- Avoid bulk delete without explicit confirmation.

## When making changes

- Keep AppleScript code in the relevant module.
- Update README and AGENTS.md when command surface changes.
- If you add a new command group, add it to `src/main.rs` and to the README command list.
