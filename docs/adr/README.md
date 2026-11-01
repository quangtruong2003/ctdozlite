# Architecture Decision Records

One file per decision: `NNNN-<kebab-name>.md`, numbered from `0001`, never renumbered.

## Template

```markdown
# ADR-0001: <Decision title>

- **Status:** proposed | accepted | superseded by ADR-NNNN | deprecated
- **Date:** YYYY-MM-DD
- **Deciders:** <names/roles>

## Context

Forces at play, constraints, evidence (capture note, upstream behaviour).

## Decision

What we will do, in one paragraph, present tense.

## Consequences

- Positive:
- Negative:
- Follow-ups:

## Alternatives Considered

| Option | Why rejected |
| :--- | :--- |
```

## Index

| ADR | Title | Status |
| :--- | :--- | :--- |
| [ADR-0001](0001-history-sync.md) | Kiến trúc đồng bộ lịch sử tin nhắn (History Sync & Backup Import) | Accepted |
| [ADR-0002](0002-self-updater.md) | Giao thức tự cập nhật và cấu trúc `latest.json` (Self-Updater Contract) | Accepted |
| [ADR-0003](0003-changelog-policy.md) | Chính sách Changelog & Release Notes | Accepted |
| [ADR-0004](0004-history-sync-cursor.md) | Con trỏ History Sync lấy từ Store | Accepted |

Related domain language lives in [`/GLOSSARY.md`](../../GLOSSARY.md).
