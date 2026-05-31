#[cfg(feature = "mlx")]
pub mod mlx;
pub mod openai_compatible;

use crate::errors::Result;

/// Strip a leading reasoning block emitted by reasoning models (Qwen3, R1,
/// DeepSeek, etc.) when the server ignores the no-reasoning request flags.
///
/// Handles two cases:
/// - Closed `<think>...</think>`: drops everything up to and including `</think>`
///   and trims leading whitespace from the answer that follows.
/// - Unterminated `<think>` (truncated by max_tokens): the model never produced
///   an answer, so returns `""` so the caller's empty-content guard fires and
///   falls back to raw ASR text.
pub fn strip_reasoning(s: &str) -> &str {
    // Use a lowercase copy only for searching; slicing is done on the original
    // `s` so byte offsets remain valid.  <think>/</think> are pure ASCII, so
    // to_ascii_lowercase() is length-preserving and offsets are stable.
    let lower = s.to_ascii_lowercase();
    if let Some(open) = lower.find("<think>") {
        if let Some(close_rel) = lower[open..].find("</think>") {
            let end = open + close_rel + "</think>".len();
            return s[end..].trim_start();
        }
        // Unterminated reasoning block (response was cut off by max_tokens).
        // There is no answer — return "" so the empty-content guard triggers
        // ASR fallback rather than pasting a partial monologue.
        return "";
    }
    s
}

/// LLM correction providers supported by this build.
pub fn supported_providers() -> &'static [&'static str] {
    &[
        "openai",
        #[cfg(feature = "mlx")]
        "mlx",
    ]
}

/// Request for LLM text correction.
pub struct CorrectionRequest {
    pub asr_text: String,
    pub dictionary_entries: Vec<String>,
    pub system_prompt: String,
    pub user_prompt: String,
}

/// Trait for LLM correction providers.
#[async_trait::async_trait]
pub trait LlmProvider: Send {
    async fn correct(&self, request: &CorrectionRequest) -> Result<String>;
}

#[cfg(test)]
mod tests {
    use super::strip_reasoning;

    #[test]
    fn strip_reasoning_no_think_block_unchanged() {
        assert_eq!(strip_reasoning("hello world"), "hello world");
    }

    #[test]
    fn strip_reasoning_removes_closed_think_block() {
        let s = "<think>some reasoning here</think>final answer";
        assert_eq!(strip_reasoning(s), "final answer");
    }

    #[test]
    fn strip_reasoning_trims_whitespace_after_think() {
        let s = "<think>reasoning</think>\n\nfinal answer";
        assert_eq!(strip_reasoning(s), "final answer");
    }

    #[test]
    fn strip_reasoning_unterminated_returns_empty() {
        let s = "<think>reasoning that was cut off by max_tokens";
        assert_eq!(strip_reasoning(s), "");
    }

    #[test]
    fn strip_reasoning_case_insensitive() {
        let s = "<THINK>reasoning</THINK>answer";
        assert_eq!(strip_reasoning(s), "answer");
    }

    #[test]
    fn strip_reasoning_cjk_answer_preserved() {
        let s = "<think>思考过程</think>会议纪要：今天讨论了项目进展。";
        assert_eq!(strip_reasoning(s), "会议纪要：今天讨论了项目进展。");
    }
}
