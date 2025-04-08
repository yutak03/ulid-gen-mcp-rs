use std::sync::{Arc, Mutex};

use rmcp::{
    model::{CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool, ServerHandler,
};

#[derive(Debug, Clone)]
pub struct Ulid {
    ulid: Arc<Mutex<ulid::Ulid>>,
}

#[tool(tool_box)]
impl Ulid {
    pub fn new() -> Self {
        Self {
            ulid: Arc::new(Mutex::new(ulid::Ulid::new())),
        }
    }

    #[tool(description = "generate a ULID")]
    pub fn generate(&self) -> anyhow::Result<CallToolResult, rmcp::Error> {
        let mut ulid = self.ulid.lock().unwrap();
        *ulid = ulid::Ulid::new();
        tracing::info!("Generated new ULID: {}", ulid);
        Ok(CallToolResult::success(vec![Content::text(
            ulid.to_string(),
        )]))
    }
}

#[tool(tool_box)]
impl ServerHandler for Ulid {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This is a server for generating ULID".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ulid_new() {
        let ulid = Ulid::new();
        assert!(!ulid.ulid.lock().unwrap().to_string().is_empty());
    }
    
    #[test]
    fn test_ulid_generate() {
        let ulid = Ulid::new();
        
        // 最初のULID値を取得
        let first_ulid = ulid.ulid.lock().unwrap().to_string();
        
        // generateメソッドを呼び出し
        let result = ulid.generate();
        assert!(result.is_ok());
        
        // 新しいULID値を取得
        let new_ulid = ulid.ulid.lock().unwrap().to_string();
        
        // ULIDが変更されていることを確認
        assert_ne!(first_ulid, new_ulid);
        
        // 結果の検証
        if let Ok(call_result) = result {
            // contentフィールド（旧バージョンではcontents）を使用
            assert!(!call_result.content.is_empty());
            
            // コンテンツの種類に基づいて処理
            let content = &call_result.content[0];
            
            // rawフィールドから内容を取得 (テキスト形式である前提)
            if let Some(text_value) = content.raw.as_text() {
                // RawTextContentからtext文字列を取得
                assert_eq!(text_value.text, new_ulid);
                assert_eq!(text_value.text.len(), 26); // ULIDは常に26文字
            } else {
                panic!("Expected text content");
            }
        }
    }
    
    #[test]
    fn test_server_info() {
        let ulid = Ulid::new();
        let info = ulid.get_info();
        
        assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
        
        // capabilitiesがOptionで包まれているため、is_some()で確認
        assert!(info.capabilities.prompts.is_some());
        assert!(info.capabilities.resources.is_some());
        assert!(info.capabilities.tools.is_some());
        
        assert_eq!(info.instructions, Some("This is a server for generating ULID".into()));
    }
    
    #[test]
    fn test_ulid_format() {
        let ulid = Ulid::new();
        let ulid_str = ulid.ulid.lock().unwrap().to_string();
        
        // ULIDの形式をチェック (26文字、Base32エンコード文字のみ)
        assert_eq!(ulid_str.len(), 26);
        assert!(ulid_str.chars().all(|c| c.is_ascii_alphanumeric() && !c.is_lowercase()));
    }
}