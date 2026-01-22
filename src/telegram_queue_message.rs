use serde::{Deserialize, Serialize};
use html_escape::encode_text;

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
