use async_trait::async_trait
struct LargeLanguageModel {
    name: String,
    text: String,
    lgg: String,

}

struct VersionControl {
    name: String
    num: i32
}

#[async_trait]
impl VersionControl for LargeLanguageModel {
    async fn generate (&self, prompt: &str) -> Result<String, Srting> {
        Ok(format!)
    }
}

trait GPT {}
trait DeepSeek {}

impl GPT for LargeLanguageModel {}
impl DeepSeek for LargeLanguageModel {}