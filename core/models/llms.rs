struct LargeLanguageModel {
    name: String,
    text: String,
    lgg: String,

}

trait GPT {}
trait DeepSeek {}

impl GPT for LargeLanguageModel {}
impl DeepSeek for LargeLanguageModel {}