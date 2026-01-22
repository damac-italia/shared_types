use serde::{Deserialize, Serialize};
use html_escape::encode_text;

/// Represents the status level of a message for visual formatting.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum MessageStatus {
    None,
    Info,
    Warn,
    Error,
    Ok,
}

impl MessageStatus {
    /// Returns the emoji associated with the status.
    pub fn emoji(&self) -> &'static str {
        match self {
            MessageStatus::None => "",
            MessageStatus::Info => "ℹ️",
            MessageStatus::Warn => "⚠️",
            MessageStatus::Error => "🚨",
            MessageStatus::Ok => "✅",
        }
    }
}

/// Represents a message received from the queue to be sent to Telegram.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelegramQueueMessage {
    #[serde(rename = "chatId")]
    pub chat_id: i64,
    pub message: String,
    #[serde(rename = "forceSend")]
    pub force_send: bool,
}

impl TelegramQueueMessage {

    /// Creates a new TelegramQueueMessage.
    pub fn new(chat_id: i64, message: String, force_send: bool) -> Self {
        Self {
            chat_id,
            message,
            force_send
        }
    }

    /// Returns a builder for creating a formatted TelegramQueueMessage.
    pub fn builder(chat_id: i64) -> TelegramMessageBuilder {
        TelegramMessageBuilder::new(chat_id)
    }

    /// Sanitizes the message content for safe Telegram display.
    ///
    /// This function performs the following steps:
    /// 1. Trims the message to the configured maximum length.
    ///    - If the message is longer than allowed, appends "..." to indicate truncation.
    /// 2. Escapes all HTML special characters to prevent injection of arbitrary HTML or scripts.
    /// 3. Re-enables a limited set of allowed Telegram HTML tags for basic formatting:
    ///    `b, strong, i, em, u, ins, s, strike, del, code, pre, blockquote, tg-spoiler`.
    ///
    /// Notes:
    /// - Attributes on tags are not allowed, and unsupported tags remain escaped.
    /// ```
    pub fn sanitize_message(&mut self, max_message_length: usize) {
        let overflow_length = self.message.len() > max_message_length;
        let trimmed: String = self
            .message
            .chars()
            .take(max_message_length)
            .collect();

        let mut escaped = encode_text(&trimmed).to_string();
        escaped = if overflow_length {
            format!("{}...", escaped)
        } else {
            escaped
        };

        let allowed_simple = [
            "b", "strong", "i", "em", "u", "ins",
            "s", "strike", "del", "code", "pre",
            "blockquote", "tg-spoiler",
        ];

        for tag in &allowed_simple {
            let open = format!("&lt;{}&gt;", tag);
            let close = format!("&lt;/{}&gt;", tag);

            escaped = escaped
                .replace(&open, &format!("<{}>", tag))
                .replace(&close, &format!("</{}>", tag));
        }

        self.message = escaped;
    }
}

/// A builder for creating formatted TelegramQueueMessage objects.
pub struct TelegramMessageBuilder {
    chat_id: i64,
    status: MessageStatus,
    job_name: String,
    content: String,
    force_send: bool,
}

impl TelegramMessageBuilder {
    /// Initializes a new builder with the required chat_id.
    pub fn new(chat_id: i64) -> Self {
        Self {
            chat_id,
            status: MessageStatus::None,
            job_name: String::new(),
            content: String::new(),
            force_send: false,
        }
    }

    /// Sets the status level, which adds an emoji prefix.
    pub fn status(mut self, status: MessageStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the job name, which will be formatted in italics.
    pub fn job_name(mut self, job_name: impl Into<String>) -> Self {
        self.job_name = job_name.into();
        self
    }

    /// Sets the message content.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Sets whether to force send the message.
    pub fn force_send(mut self, force_send: bool) -> Self {
        self.force_send = force_send;
        self
    }

    /// Builds the TelegramQueueMessage with the specified formatting.
    ///
    /// The resulting message format is:
    /// `{emoji} - <i>{job_name}</i>\n{content}`
    pub fn build(self) -> TelegramQueueMessage {
        let status_prefix = if matches!(self.status, MessageStatus::None) {
            "".to_string()
        } else {
            format!("{} - ", self.status.emoji())
        };

        let message = format!("{}<i>{}</i>\n{}", status_prefix, self.job_name, self.content);

        TelegramQueueMessage {
            chat_id: self.chat_id,
            message,
            force_send: self.force_send,
        }
    }
}

/// Macro to easily create a formatted TelegramQueueMessage.
///
/// Usage:
/// ```rust
/// telegram_msg!(chat_id: 123, status: Error, job: "ftp", content: "failed: {}", e)
/// ```
#[macro_export]
macro_rules! telegram_msg {
    (chat_id: $chat_id:expr, status: $status:ident, job: $job:expr, content: $($arg:tt)*) => {
        $crate::telegram_queue_message::TelegramMessageBuilder::new($chat_id)
            .status($crate::telegram_queue_message::MessageStatus::$status)
            .job_name($job)
            .content(format!($($arg)*))
            .build()
    };
    (chat_id: $chat_id:expr, status: $status:ident, job: $job:expr, force_send: $force_send:expr, content: $($arg:tt)*) => {
        $crate::telegram_queue_message::TelegramMessageBuilder::new($chat_id)
            .status($crate::telegram_queue_message::MessageStatus::$status)
            .job_name($job)
            .content(format!($($arg)*))
            .force_send($force_send)
            .build()
    };
}
