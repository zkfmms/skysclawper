//! Web search tool — searches the web using Ecosia via nab.

use async_trait::async_trait;
use serde_json::Value;
use skyclaw_core::types::error::SkyclawError;
use skyclaw_core::{Tool, ToolContext, ToolDeclarations, ToolInput, ToolOutput};
use std::process::Command;

pub struct WebSearchTool;

impl Default for WebSearchTool {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSearchTool {
    pub fn new() -> Self {
        Self
    }

    fn nab_binary() -> String {
        if let Ok(p) = std::env::var("NAB_BIN") {
            return p;
        }
        if let Ok(path_env) = std::env::var("PATH") {
            for dir in path_env.split(':') {
                let cand = format!("{}/nab", dir);
                if std::fs::metadata(&cand).map(|m| m.is_file()).unwrap_or(false) {
                    return cand;
                }
            }
        }
        for cand in [
            "/opt/homebrew/bin/nab",
            "/usr/local/bin/nab",
            "/usr/bin/nab",
            "/tmp/nab-x86_64",
            "/tmp/nab",
        ] {
            if std::fs::metadata(cand).map(|m| m.is_file()).unwrap_or(false) {
                return cand.to_string();
            }
        }
        "nab".to_string()
    }

    fn parse_results(json_str: &str) -> Result<String, String> {
        let data: Value = serde_json::from_str(json_str)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let results = data.get("results")
            .ok_or("No 'results' field in response")?;

        let mainline = results.get("mainline")
            .and_then(|v| v.as_array())
            .ok_or("No 'mainline' results found")?;

        let mut formatted = String::new();
        let mut count = 0;

        for item in mainline {
            if item.get("type").and_then(|s| s.as_str()) == Some("web") 
               && item.get("provider").and_then(|s| s.as_str()) == Some("GOOGLE") {
                
                let title = item.get("title").and_then(|s| s.as_str()).unwrap_or("No title");
                let url = item.get("url").and_then(|s| s.as_str()).unwrap_or("");
                let description = item.get("description").and_then(|s| s.as_str()).unwrap_or("");
                
                count += 1;
                formatted.push_str(&format!("{}. {}\n", count, title));
                formatted.push_str(&format!("   URL: {}\n", url));
                formatted.push_str(&format!("   {}\n\n", description));

                if count >= 5 {
                    break;
                }
            }
        }

        if count == 0 {
            return Ok("No web results found.".to_string());
        }

        Ok(formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_results() {
        let json_input = r#"{
            "results": {
                "mainline": [
                    {
                        "type": "web",
                        "provider": "GOOGLE",
                        "title": "Rust Programming Language",
                        "url": "https://www.rust-lang.org/",
                        "description": "A language empowering everyone to build reliable and efficient software."
                    },
                    {
                        "type": "web",
                        "provider": "GOOGLE",
                        "title": "Rust - Wikipedia",
                        "url": "https://en.wikipedia.org/wiki/Rust_(programming_language)",
                        "description": "Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency."
                    },
                    {
                        "type": "ad",
                        "title": "Ignore me"
                    }
                ]
            }
        }"#;

        let result = WebSearchTool::parse_results(json_input).unwrap();
        
        assert!(result.contains("1. Rust Programming Language"));
        assert!(result.contains("URL: https://www.rust-lang.org/"));
        assert!(result.contains("2. Rust - Wikipedia"));
        assert!(!result.contains("Ignore me"));
    }

    #[test]
    fn test_parse_results_empty() {
        let json_input = r#"{
            "results": {
                "mainline": []
            }
        }"#;

        let result = WebSearchTool::parse_results(json_input).unwrap();
        assert_eq!(result, "No web results found.");
    }
}

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &str {
        "web_search"
    }

    fn description(&self) -> &str {
        "Search the web using Ecosia (via nab). Returns top 5 results with titles, URLs, and descriptions."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query"
                }
            },
            "required": ["query"]
        })
    }

    fn declarations(&self) -> ToolDeclarations {
        ToolDeclarations {
            file_access: Vec::new(),
            network_access: vec!["*.ecosia.org".to_string()],
            shell_access: false, // We use Command internally but it's a specific binary, not general shell
        }
    }

    async fn execute(
        &self,
        input: ToolInput,
        _ctx: &ToolContext,
    ) -> Result<ToolOutput, SkyclawError> {
        let query = input
            .arguments
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SkyclawError::Tool("Missing required parameter: query".into()))?;

        let url = format!("https://www.ecosia.org/search?q={}", query);
        
        let mut cmd = Command::new(Self::nab_binary());
        cmd.arg("fetch").arg(&url);

        // We expect JSON output from nab for this URL
        // Based on test_ecosia_nab.py, nab returns JSON directly for this URL.
        // However, we might need to ensure it's JSON if nab has flags. 
        // The python script didn't use flags, just `nab fetch url`.
        // Let's assume default behavior is correct as per python script.

        let output = cmd.output().map_err(|e| {
            SkyclawError::Tool(format!("Failed to invoke nab: {}", e))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Ok(ToolOutput {
                content: format!("nab failed: {}", stderr),
                is_error: true,
            });
        }

        let body = String::from_utf8_lossy(&output.stdout).to_string();
        
        match Self::parse_results(&body) {
            Ok(formatted) => Ok(ToolOutput {
                content: formatted,
                is_error: false,
            }),
            Err(e) => Ok(ToolOutput {
                content: format!("Failed to parse search results: {}\nRaw output: {}", e, body),
                is_error: true,
            }),
        }
    }
}
