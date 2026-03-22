# TUI Screen Contract

**Feature Branch**: `001-tui-voting-client`
**Date**: 2026-03-21

This document defines the screens, their purpose, and navigation
between them.

## Screen Inventory

| Screen          | Purpose                                | Entry condition              |
|-----------------|----------------------------------------|------------------------------|
| Welcome/Setup   | First-run identity creation            | No identity file exists      |
| Password Prompt | Unlock encrypted identity              | Encrypted identity exists    |
| Election List   | Browse available elections             | Identity loaded              |
| Election Detail | View candidates, register, get token   | Election selected from list  |
| Vote            | Select candidates, confirm ballot      | Valid token exists            |
| Results         | View election results                  | Election status = Finished   |
| Settings        | Configure relays, manage identity      | Accessible from any screen   |
| Help Overlay    | Keyboard shortcuts reference           | Press `?` from any screen    |

## Navigation Flow

```text
[Launch]
   │
   ├─ No identity ──→ [Welcome/Setup] ──→ [Election List]
   │
   └─ Has identity
        │
        ├─ Encrypted ──→ [Password Prompt] ──→ [Election List]
        │
        └─ Plaintext ──→ [Election List]

[Election List]
   │
   ├─ Select election ──→ [Election Detail]
   │
   ├─ Press 's' ──→ [Settings]
   │
   └─ Press 'q' ──→ [Quit]

[Election Detail]
   │
   ├─ Register (if not registered) ──→ token input ──→ confirm
   │
   ├─ Request token (if registered, no token) ──→ progress ──→ confirm
   │
   ├─ Vote (if has token) ──→ [Vote]
   │
   ├─ View results (if finished) ──→ [Results]
   │
   └─ Press 'Esc' ──→ [Election List]

[Vote]
   │
   ├─ Select candidates ──→ [Confirm] ──→ submit ──→ [Election Detail]
   │
   └─ Press 'Esc' ──→ [Election Detail]
```

## Global Keyboard Shortcuts

| Key       | Action                              |
|-----------|-------------------------------------|
| `q`       | Quit (with confirmation if mid-flow)|
| `?`       | Toggle help overlay                 |
| `s`       | Open settings (from Election List)  |
| `Esc`     | Go back / cancel                    |
| `Enter`   | Confirm / select                    |
| `j` / `↓` | Move down                          |
| `k` / `↑` | Move up                            |
| `h` / `←` | Move left / collapse               |
| `l` / `→` | Move right / expand                |
| `Tab`     | Next field / section                |

## Screen-Specific Interactions

### Vote Screen — Plurality
- Candidate list displayed as radio buttons
- `j`/`k` to navigate, `Enter` or `Space` to select
- Only one candidate selectable at a time
- `Enter` on "Submit" to proceed to confirmation

### Vote Screen — STV
- Candidate list with rank numbers
- `Enter` to add candidate to ranking (appends to end)
- Number keys `1`-`9` to set specific rank
- `d` to remove from ranking
- `Enter` on "Submit" to proceed to confirmation

### Confirmation Dialog
- Shows selected candidates and election name
- Two buttons: "Confirm" and "Go Back"
- `Enter` to confirm, `Esc` to go back
