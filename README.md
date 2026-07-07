# shared_types

![License](https://img.shields.io/badge/license-MIT-blue)
![Language](https://img.shields.io/badge/rust-2024-orange)

Shared types between microservices.

## Use case

A small Rust library crate holding data types shared across microservices. It
currently provides the Telegram message contract used to enqueue messages for
delivery to Telegram, plus helpers to build and sanitize those messages safely.

## Features

- **`TelegramQueueMessage`** — serde-serializable queue payload with `chatId`,
  `message`, and `forceSend` fields (JSON camelCase via `#[serde(rename)]`).
  — `src/telegram.rs:29`
- **`MessageStatus`** — status level (`None`, `Info`, `Warn`, `Error`, `Ok`) with
  an associated emoji via `.emoji()`. — `src/telegram.rs:6`
- **`TelegramMessageBuilder`** — fluent builder that formats a message as
  `{emoji} - <i>{job_name}</i>\n{content}`. — `src/telegram.rs:100`
- **`sanitize_message`** — truncates to a max length (appending `...` on
  overflow), HTML-escapes content, then re-enables a whitelist of Telegram HTML
  tags (`b, strong, i, em, u, ins, s, strike, del, code, pre, blockquote,
  tg-spoiler`). — `src/telegram.rs:65`
- **`telegram_msg!`** macro — one-line construction of a formatted message.
  — `src/telegram.rs:172`

## Requirements

- Rust with edition 2024 support. — `Cargo.toml`

## Installation

Add as a git dependency in your `Cargo.toml`:

```toml
[dependencies]
shared_types = { git = "https://github.com/damac-italia/shared_types" }
```

## Usage

Builder:

```rust
use shared_types::{TelegramQueueMessage, MessageStatus};

let msg = TelegramQueueMessage::builder(123)
    .status(MessageStatus::Error)
    .job_name("ftp")
    .content("connection failed")
    .force_send(true)
    .build();
```

Macro:

```rust
use shared_types::telegram_msg;

let e = "timeout";
let msg = telegram_msg!(chat_id: 123, status: Error, job: "ftp", content: "failed: {}", e);

// with force_send:
let msg = telegram_msg!(chat_id: 123, status: Ok, job: "ftp", force_send: true, content: "done");
```

Sanitize before sending:

```rust
let mut msg = TelegramQueueMessage::new(123, "<b>hi</b> <script>x</script>".into(), false);
msg.sanitize_message(4096);
// keeps <b>…</b>, escapes <script>
```

## Dependencies

- `serde` (with `derive`) — serialization. — `Cargo.toml`
- `html-escape` — HTML escaping in `sanitize_message`. — `Cargo.toml`

## Project structure

```
src/lib.rs        crate root; re-exports public types
src/telegram.rs   Telegram message types, builder, sanitizer, macro
```

## License

MIT. — `Cargo.toml`
<!-- TODO: no LICENSE file present in repo; add one to match the MIT declaration in Cargo.toml -->
