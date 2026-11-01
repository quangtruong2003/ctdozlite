# Triage Labels

Map the triage skill's canonical state roles to the repository's established representations. Do not add parallel `needs-*` or `ready-*` state labels.

| Role | Repository representation |
| :--- | :--- |
| `needs-triage` | Open issue with no `type:*` or `area:*` label. |
| `needs-info` | `status:blocked` plus a comment naming the missing evidence. |
| `ready-for-agent` | `type:*` and `area:*` set, acceptance criteria present, and no open blocker. |
| `ready-for-human` | `priority:p0` / `priority:p1`, or protocol evidence / physical-device testing is required. |
| `wontfix` | `wontfix` label and a closing comment. |

The category roles remain `bug` and `enhancement`. See `docs/standards/issue-tracker.md`.
