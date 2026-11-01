# 09 — One-to-one History Sync evidence status

Date: 2026-11-02

## Finding

- The implemented HTTP history flow covers groups and My Cloud through the group cloud-message service.
- The existing chat-data note records that general message-history retrieval lives in Zalo's separate sync-v2 workers, which use binary framing.
- The exact sync-v2 frame envelope, authentication handshake, one-to-one history request, response shape, and cursor semantics are not decoded in the repository.
- No sanitized one-to-one history request/response capture or fixture is present in the repository.

## Decision

Do not send guessed one-to-one history requests. Keep one-to-one History Sync unsupported until a supported request/response path is observed and recorded. The GUI should describe this as an application limitation, not claim that Zalo has no such capability.

## Evidence needed to reopen

Capture or decode one authenticated one-to-one history request and response from a supported Zalo PC client. Record the command/frame IDs, request envelope, response shape, cursor behavior, and observed error codes. Redact cookies, keys, IMEI, phone numbers, and message content before storing any fixture.
