use async_trait::async_trait;
use rust_mcp_sdk::{
    *, 
    error::SdkResult, 
    macros, 
    mcp_server::{server_runtime, ServerHandler, McpServerOptions}, 
    schema::*,
};
use std::sync::Arc;
use crate::commands;

#[macros::mcp_tool(name = "mka_list_workflows", description = "List all available MKA workflows for this project.")]
#[derive(Debug, serde::Deserialize, serde::Serialize, macros::JsonSchema)]
pub struct ListWorkflowsTool {}

#[macros::mcp_tool(name = "mka_get_workflow", description = "Get the technical map (TOON) for a specific workflow. Use this to discover WHERE logic lives. This tool returns a list of files and methods involved in a feature.")]
#[derive(Debug, serde::Deserialize, serde::Serialize, macros::JsonSchema)]
pub struct GetWorkflowTool {
    pub id: String,
}

#[macros::mcp_tool(name = "mka_sync", description = "Sync the MKA index and heal broken workflow paths automatically.")]
#[derive(Debug, serde::Deserialize, serde::Serialize, macros::JsonSchema)]
pub struct SyncTool {}

#[derive(Default)]
struct MkaHandler;

#[async_trait]
impl ServerHandler for MkaHandler {
    async fn handle_list_tools_request(
        &self,
        _request: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            tools: vec![
                ListWorkflowsTool::tool(),
                GetWorkflowTool::tool(),
                SyncTool::tool(),
            ],
            meta: None,
            next_cursor: None,
        })
    }

    async fn handle_call_tool_request(
        &self,
        params: CallToolRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        match params.name.as_str() {
            "mka_list_workflows" => {
                match commands::workflow_list::get_workflows_toon().await {
                    Ok(output) => Ok(CallToolResult::text_content(vec![output.into()])),
                    Err(e) => Ok(CallToolResult::text_content(vec![format!("Error listing workflows: {}", e).into()])),
                }
            }
            "mka_get_workflow" => {
                let args: GetWorkflowTool = serde_json::from_value(serde_json::Value::Object(params.arguments.unwrap_or_default()))
                    .map_err(|e| CallToolError::invalid_arguments(params.name.clone(), Some(e.to_string())))?;
                // snippets are forced to false for MCP stability
                match commands::workflow_get::get_workflow_content(&args.id, false).await {
                    Ok(output) => Ok(CallToolResult::text_content(vec![output.into()])),
                    Err(e) => Ok(CallToolResult::text_content(vec![format!("Error getting workflow {}: {}", args.id, e).into()])),
                }
            }
            "mka_sync" => {
                match commands::sync::handle().await {
                    Ok(_) => Ok(CallToolResult::text_content(vec!["MKA synchronization complete.".into()])),
                    Err(e) => Ok(CallToolResult::text_content(vec![format!("Error syncing MKA: {}", e).into()])),
                }
            }
            _ => Err(CallToolError::unknown_tool(params.name)),
        }
    }
}

pub async fn handle() -> SdkResult<()> {
    let server_info = InitializeResult {
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            logging: None,
            prompts: None,
            resources: None,
            completions: None,
            experimental: None,
            tasks: None,
        },
        instructions: Some("MKA acts as a high-density Navigation Map. Use these tools to find WHERE logic lives before performing standard file reads. \
                           1. MAP CHECK: Always run `mka_list_workflows` to see the architectural intent. \
                           2. DISCOVERY: Use `mka_get_workflow` to see which files and methods are involved in a feature. Use this as your primary guide for where to perform standard `read_file` operations.".into()),
        meta: None,
        protocol_version: ProtocolVersion::V2025_11_25.into(),
        server_info: Implementation {
            name: "mka-mcp-server".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            description: Some("MKA MCP Server for architectural navigation".into()),
            icons: vec![],
            website_url: None,
            title: Some("MKA MCP Server".into()),
        },
    };

    let transport = StdioTransport::new(TransportOptions::default())?;
    let handler = MkaHandler::default().to_mcp_server_handler();
    
    let options = McpServerOptions {
        server_details: server_info,
        transport,
        handler,
        task_store: None,
        client_task_store: None,
        message_observer: None,
    };

    let server = server_runtime::create_server(options);
    
    eprintln!("MKA MCP Server starting on stdio...");
    server.start().await
}

#[cfg(test)]
mod mcp_tests {
    use super::*;
    use tokio::sync::RwLockReadGuard;
    use rust_mcp_sdk::auth::AuthInfo;
    use rust_mcp_sdk::task_store::{ServerTaskStore, ClientTaskStore};
    use rust_mcp_sdk::error::McpSdkError;

    struct MockRuntime {
        info: InitializeResult,
    }

    impl MockRuntime {
        fn new() -> Self {
            Self {
                info: InitializeResult {
                    capabilities: ServerCapabilities {
                        tools: None,
                        logging: None,
                        prompts: None,
                        resources: None,
                        completions: None,
                        experimental: None,
                        tasks: None,
                    },
                    instructions: None,
                    meta: None,
                    protocol_version: "2025-11-25".to_string(),
                    server_info: Implementation {
                        name: "mock".to_string(),
                        version: "1.0".to_string(),
                        description: None,
                        icons: vec![],
                        website_url: None,
                        title: None,
                    },
                }
            }
        }
    }

    #[async_trait]
    impl McpServer for MockRuntime {
        async fn start(self: Arc<Self>) -> SdkResult<()> { Ok(()) }
        async fn set_client_details(&self, _client_details: InitializeRequestParams) -> SdkResult<()> { Ok(()) }
        fn server_info(&self) -> &InitializeResult { &self.info }
        fn client_info(&self) -> Option<InitializeRequestParams> { None }
        async fn auth_info(&self) -> RwLockReadGuard<'_, Option<AuthInfo>> { panic!("Not implemented") }
        async fn auth_info_cloned(&self) -> Option<AuthInfo> { None }
        async fn update_auth_info(&self, _auth_info: Option<AuthInfo>) {}
        async fn wait_for_initialization(&self) {}
        fn task_store(&self) -> Option<Arc<ServerTaskStore>> { None }
        fn client_task_store(&self) -> Option<Arc<ClientTaskStore>> { None }
        async fn stderr_message(&self, _message: String) -> std::result::Result<(), McpSdkError> { Ok(()) }
        fn session_id(&self) -> Option<String> { None }
        async fn send(&self, _message: MessageFromServer, _request_id: Option<RequestId>, _timeout: Option<std::time::Duration>) -> std::result::Result<Option<ClientMessage>, McpSdkError> { Ok(None) }
        async fn send_batch(&self, _messages: Vec<ServerMessage>, _timeout: Option<std::time::Duration>) -> std::result::Result<Option<Vec<ClientMessage>>, McpSdkError> { Ok(None) }
    }

    #[tokio::test]
    async fn test_mka_list_workflows_tool_registration() {
        let handler = MkaHandler::default();
        let runtime = Arc::new(MockRuntime::new());
        let result = handler.handle_list_tools_request(None, runtime).await.unwrap();
        
        let tool_names: Vec<String> = result.tools.iter().map(|t| t.name.clone()).collect();
        assert!(tool_names.contains(&"mka_list_workflows".to_string()));
        assert!(tool_names.contains(&"mka_get_workflow".to_string()));
        assert!(tool_names.contains(&"mka_sync".to_string()));
        assert!(!tool_names.contains(&"mka_get_method".to_string()));
    }
}
