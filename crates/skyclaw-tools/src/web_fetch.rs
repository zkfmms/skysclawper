//! Web fetch tool — retrieves content from URLs via HTTP GET.

use async_trait::async_trait;
use skyclaw_core::types::error::SkyclawError;
use skyclaw_core::{Tool, ToolContext, ToolDeclarations, ToolInput, ToolOutput};

/// Default request timeout in seconds.
const DEFAULT_TIMEOUT_SECS: u64 = 10;

/// Maximum response body size (32 KB — keeps tool output within token budget).
const MAX_RESPONSE_SIZE: usize = 32 * 1024;

pub struct WebFetchTool {
    client: reqwest::Client,
}

impl Default for WebFetchTool {
    fn default() -> Self {
        Self::new()
    }
}

impl WebFetchTool {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .user_agent("SkyClaw/0.1")
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self { client }
    }

    fn should_use_nab(input: &ToolInput) -> bool {
        if let Some(use_nab) = input
            .arguments
            .get("use_nab")
            .and_then(|v| v.as_bool())
        {
            return use_nab;
        }
        std::env::var("SKYSCLAWPER_USE_NAB")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
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
}

#[async_trait]
impl Tool for WebFetchTool {
    fn name(&self) -> &str {
        "web_fetch"
    }

    fn description(&self) -> &str {
        "Fetch the content of a web page or API endpoint via HTTP GET. \
         Returns the response body as text. Use this to look up documentation, \
         check APIs, fetch data, or research information on the web. \
         NOTE: This tool is synchronous and blocks until the fetch is complete. \
         You will receive the result immediately in the same turn. \
         Do NOT claim to be processing in the background."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to fetch (must start with http:// or https://)"
                },
                "headers": {
                    "type": "object",
                    "description": "Optional HTTP headers as key-value pairs",
                    "additionalProperties": { "type": "string" }
                },
                "use_nab": {
                    "type": "boolean",
                    "description": "Route the request through the nab binary for LLM-ready output (defaults from SKYSCLAWPER_USE_NAB)"
                },
                "cookies": {
                    "type": "string",
                    "description": "Cookie source for nab (auto, brave, chrome, firefox, safari, none)"
                },
                "format": {
                    "type": "string",
                    "enum": ["full", "compact", "json"],
                    "description": "nab output format (default: full)"
                },
                "raw_html": {
                    "type": "boolean",
                    "description": "Return raw HTML via nab instead of markdown"
                }
            },
            "required": ["url"]
        })
    }

    fn declarations(&self) -> ToolDeclarations {
        ToolDeclarations {
            file_access: Vec::new(),
            network_access: vec!["*".to_string()],
            shell_access: false,
        }
    }

    async fn execute(
        &self,
        input: ToolInput,
        _ctx: &ToolContext,
    ) -> Result<ToolOutput, SkyclawError> {
        let url = input
            .arguments
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SkyclawError::Tool("Missing required parameter: url".into()))?;

        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Ok(ToolOutput {
                content: "URL must start with http:// or https://".to_string(),
                is_error: true,
            });
        }

        if Self::should_use_nab(&input) {
            // Prefer in-process library if available; fallback to binary
            if let Ok(client) = nab::AcceleratedClient::new() {
                let mut body = match client.fetch_text(url).await {
                    Ok(text) => text,
                    Err(e) => {
                        return Ok(ToolOutput {
                            content: format!("nab fetch failed: {}", e),
                            is_error: true,
                        })
                    }
                };
                if body.len() > MAX_RESPONSE_SIZE {
                    body.truncate(MAX_RESPONSE_SIZE);
                    body.push_str("\n... [response truncated]");
                }
                return Ok(ToolOutput {
                    content: body,
                    is_error: false,
                });
            }

            let mut cmd = std::process::Command::new(Self::nab_binary());
            cmd.arg("fetch").arg(url);

            // raw_html flag: input parameter overrides env default
            let raw_html_input = input
                .arguments
                .get("raw_html")
                .and_then(|v| v.as_bool());
            let raw_html_env = std::env::var("SKYSCLAWPER_NAB_RAW_HTML")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
            if raw_html_input.unwrap_or(raw_html_env) {
                cmd.arg("--raw-html");
            }

            // format: input parameter overrides env default
            let format_input = input
                .arguments
                .get("format")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let format_env = std::env::var("SKYSCLAWPER_NAB_FORMAT").ok();
            if let Some(fmt) = format_input.or(format_env) {
                if !fmt.trim().is_empty() {
                    cmd.arg("-f").arg(fmt);
                }
            }

            if let Some(cookies) = input
                .arguments
                .get("cookies")
                .and_then(|v| v.as_str())
            {
                // Do not log cookie source; pass as flag only
                if !cookies.eq_ignore_ascii_case("none") {
                    cmd.arg("--cookies").arg(cookies);
                }
            } else if std::env::var("SKYSCLAWPER_NAB_COOKIES")
                .map(|v| !v.eq_ignore_ascii_case("none"))
                .unwrap_or(false)
            {
                // Fallback env-configured cookie source (e.g., 'auto'/'brave')
                if let Ok(src) = std::env::var("SKYSCLAWPER_NAB_COOKIES") {
                    cmd.arg("--cookies").arg(src);
                }
            }

            let output = cmd.output().map_err(|e| {
                SkyclawError::Tool(format!("Failed to invoke nab: {}", e))
            })?;

            if !output.status.success() {
                // Return stderr without echoing command or args
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let msg = if stderr.trim().is_empty() {
                    "nab returned a non-zero exit status".to_string()
                } else {
                    stderr
                };
                return Ok(ToolOutput {
                    content: msg,
                    is_error: true,
                });
            }

            let mut body = String::from_utf8_lossy(&output.stdout).to_string();
            if body.len() > MAX_RESPONSE_SIZE {
                body.truncate(MAX_RESPONSE_SIZE);
                body.push_str("\n... [response truncated]");
            }

            return Ok(ToolOutput {
                content: body,
                is_error: false,
            });
        }

        let mut request = self.client.get(url);

        if let Some(headers) = input.arguments.get("headers").and_then(|v| v.as_object()) {
            for (key, value) in headers {
                if let Some(val_str) = value.as_str() {
                    request = request.header(key.as_str(), val_str);
                }
            }
        }

        tracing::info!(url = %url, "Fetching URL");

        match request.send().await {
            Ok(response) => {
                let status = response.status();
                let status_code = status.as_u16();

                match response.text().await {
                    Ok(mut body) => {
                        if body.len() > MAX_RESPONSE_SIZE {
                            body.truncate(MAX_RESPONSE_SIZE);
                            body.push_str("\n... [response truncated]");
                        }

                        let content = format!(
                            "HTTP {} {}\n\n{}",
                            status_code,
                            status.canonical_reason().unwrap_or(""),
                            body,
                        );

                        Ok(ToolOutput {
                            content,
                            is_error: status.is_client_error() || status.is_server_error(),
                        })
                    }
                    Err(e) => Ok(ToolOutput {
                        content: format!("Failed to read response body: {}", e),
                        is_error: true,
                    }),
                }
            }
            Err(e) => Ok(ToolOutput {
                content: format!("Request failed: {}", e),
                is_error: true,
            }),
        }
    }
}
