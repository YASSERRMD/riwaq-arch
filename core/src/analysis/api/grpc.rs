//! gRPC/Protocol Buffers Analyzer
//!
//! Parses .proto files to extract:
//! - Service definitions
//! - RPC methods (unary, streaming)
//! - Message types

use super::*;
use regex::Regex;
use std::path::Path;
use tokio::fs;
use walkdir::WalkDir;

pub struct GrpcAnalyzer;

impl GrpcAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub async fn analyze(&self, root_path: &Path) -> anyhow::Result<GrpcSummary> {
        let mut proto_files = Vec::new();
        let mut services = Vec::new();
        let mut messages = Vec::new();

        // Find all .proto files
        for entry in WalkDir::new(root_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.extension().map(|e| e == "proto").unwrap_or(false) {
                let relative_path = path.strip_prefix(root_path)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();
                
                proto_files.push(relative_path.clone());

                if let Ok(content) = fs::read_to_string(path).await {
                    let package = Self::extract_package(&content);
                    let parsed_services = Self::parse_services(&content, &relative_path, &package);
                    let parsed_messages = Self::parse_messages(&content);
                    
                    services.extend(parsed_services);
                    messages.extend(parsed_messages);
                }
            }
        }

        Ok(GrpcSummary {
            proto_files,
            services,
            messages,
        })
    }

    fn extract_package(content: &str) -> String {
        let package_regex = Regex::new(r#"package\s+([a-zA-Z0-9_.]+)\s*;"#).unwrap();
        
        package_regex.captures(content)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default()
    }

    fn parse_services(content: &str, file: &str, package: &str) -> Vec<GrpcService> {
        let mut services = Vec::new();
        
        // Match service definitions
        let service_regex = Regex::new(
            r#"service\s+(\w+)\s*\{([^}]*)\}"#
        ).unwrap();

        for service_cap in service_regex.captures_iter(content) {
            let service_name = service_cap.get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let service_body = service_cap.get(2)
                .map(|m| m.as_str())
                .unwrap_or("");

            let methods = Self::parse_methods(service_body);

            services.push(GrpcService {
                name: service_name,
                package: package.to_string(),
                methods,
                file: file.to_string(),
            });
        }

        services
    }

    fn parse_methods(service_body: &str) -> Vec<GrpcMethod> {
        let mut methods = Vec::new();

        // Match RPC definitions
        let rpc_regex = Regex::new(
            r#"rpc\s+(\w+)\s*\(\s*(stream\s+)?(\w+)\s*\)\s*returns\s*\(\s*(stream\s+)?(\w+)\s*\)"#
        ).unwrap();

        for rpc_cap in rpc_regex.captures_iter(service_body) {
            let name = rpc_cap.get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let client_streaming = rpc_cap.get(2).is_some();
            let input_type = rpc_cap.get(3)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let server_streaming = rpc_cap.get(4).is_some();
            let output_type = rpc_cap.get(5)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();

            // Look for comment above the RPC
            let description = Self::extract_rpc_comment(service_body, &name);

            methods.push(GrpcMethod {
                name,
                input_type,
                output_type,
                client_streaming,
                server_streaming,
                description,
            });
        }

        methods
    }

    fn parse_messages(content: &str) -> Vec<GrpcMessage> {
        let mut messages = Vec::new();

        // Match message definitions
        let message_regex = Regex::new(
            r#"message\s+(\w+)\s*\{([^}]*)\}"#
        ).unwrap();

        for msg_cap in message_regex.captures_iter(content) {
            let name = msg_cap.get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let body = msg_cap.get(2)
                .map(|m| m.as_str())
                .unwrap_or("");

            let fields = Self::parse_fields(body);

            messages.push(GrpcMessage {
                name,
                fields,
            });
        }

        messages
    }

    fn parse_fields(message_body: &str) -> Vec<GrpcField> {
        let mut fields = Vec::new();

        // Match field definitions
        let field_regex = Regex::new(
            r#"(repeated\s+|optional\s+)?(\w+)\s+(\w+)\s*=\s*(\d+)"#
        ).unwrap();

        for field_cap in field_regex.captures_iter(message_body) {
            let modifier = field_cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            let repeated = modifier.contains("repeated");
            let optional = modifier.contains("optional");
            
            let field_type = field_cap.get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let name = field_cap.get(3)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let number: u32 = field_cap.get(4)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            fields.push(GrpcField {
                name,
                field_type,
                number,
                repeated,
                optional,
            });
        }

        fields
    }

    fn extract_rpc_comment(body: &str, method_name: &str) -> Option<String> {
        // Simple extraction: look for // comment above the rpc line
        let lines: Vec<&str> = body.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            if line.contains("rpc") && line.contains(method_name) {
                // Check previous line for comment
                if i > 0 {
                    let prev_line = lines[i - 1].trim();
                    if prev_line.starts_with("//") {
                        return Some(prev_line.trim_start_matches('/').trim().to_string());
                    }
                }
            }
        }
        None
    }
}

impl Default for GrpcAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_service() {
        let proto = r#"
            package payment.v1;
            
            service PaymentService {
                // Process a payment
                rpc ProcessPayment(PaymentRequest) returns (PaymentResponse);
                rpc StreamTransactions(TransactionRequest) returns (stream TransactionEvent);
            }
        "#;
        
        let package = GrpcAnalyzer::extract_package(proto);
        assert_eq!(package, "payment.v1");
        
        let services = GrpcAnalyzer::parse_services(proto, "payment.proto", &package);
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].name, "PaymentService");
        assert_eq!(services[0].methods.len(), 2);
        assert!(!services[0].methods[0].server_streaming);
        assert!(services[0].methods[1].server_streaming);
    }

    #[test]
    fn test_parse_message() {
        let proto = r#"
            message PaymentRequest {
                string order_id = 1;
                int64 amount = 2;
                repeated string tags = 3;
            }
        "#;
        
        let messages = GrpcAnalyzer::parse_messages(proto);
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].name, "PaymentRequest");
        assert_eq!(messages[0].fields.len(), 3);
        assert!(messages[0].fields[2].repeated);
    }
}
