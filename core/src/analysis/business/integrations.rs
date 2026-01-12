//! Integration Analyzer
//!
//! Detects external service integrations:
//! - Payment processors (Stripe, PayPal)
//! - Email services (SendGrid, Mailgun)
//! - Authentication (Auth0, Firebase)
//! - Storage (S3, GCS)
//! - And more

use super::*;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use walkdir::WalkDir;

pub struct IntegrationAnalyzer;

impl IntegrationAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub async fn analyze(&self, root_path: &Path) -> anyhow::Result<Vec<Integration>> {
        let mut integrations: HashMap<String, Integration> = HashMap::new();

        for entry in WalkDir::new(root_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if Self::should_skip_dir(path) {
                continue;
            }

            if let Ok(content) = fs::read_to_string(path).await {
                let relative_path = path.strip_prefix(root_path)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();
                
                let found = self.detect_integrations(&content, &relative_path);
                
                for integration in found {
                    integrations
                        .entry(integration.name.clone())
                        .or_insert(integration);
                }
            }
        }

        Ok(integrations.into_values().collect())
    }

    fn detect_integrations(&self, content: &str, file: &str) -> Vec<Integration> {
        let mut integrations = Vec::new();

        // Define known integrations
        let known_integrations: Vec<(&str, IntegrationType, &str, Vec<&str>)> = vec![
            // Payment
            ("Stripe", IntegrationType::Payment, "Payment processing", 
             vec!["stripe", "STRIPE_", "sk_live_", "pk_live_"]),
            ("PayPal", IntegrationType::Payment, "Payment processing",
             vec!["paypal", "PAYPAL_"]),
            ("Square", IntegrationType::Payment, "Payment processing",
             vec!["square", "SQUARE_"]),
            ("Braintree", IntegrationType::Payment, "Payment processing",
             vec!["braintree", "BRAINTREE_"]),
            
            // Email
            ("SendGrid", IntegrationType::Email, "Email delivery",
             vec!["sendgrid", "SENDGRID_"]),
            ("Mailgun", IntegrationType::Email, "Email delivery",
             vec!["mailgun", "MAILGUN_"]),
            ("AWS SES", IntegrationType::Email, "Email delivery",
             vec!["ses", "SES_", "aws.ses"]),
            ("Postmark", IntegrationType::Email, "Email delivery",
             vec!["postmark", "POSTMARK_"]),
            
            // Authentication
            ("Auth0", IntegrationType::Authentication, "Identity management",
             vec!["auth0", "AUTH0_"]),
            ("Firebase Auth", IntegrationType::Authentication, "Authentication",
             vec!["firebase.auth", "FIREBASE_"]),
            ("Cognito", IntegrationType::Authentication, "AWS authentication",
             vec!["cognito", "COGNITO_"]),
            ("Okta", IntegrationType::Authentication, "Identity management",
             vec!["okta", "OKTA_"]),
            
            // Storage
            ("AWS S3", IntegrationType::Storage, "Object storage",
             vec!["s3", "S3_", "aws.s3", "boto3.client('s3')"]),
            ("Google Cloud Storage", IntegrationType::Storage, "Object storage",
             vec!["gcs", "storage.googleapis", "GOOGLE_CLOUD_"]),
            ("Azure Blob", IntegrationType::Storage, "Blob storage",
             vec!["azure.storage.blob", "AZURE_STORAGE_"]),
            ("Cloudinary", IntegrationType::Storage, "Media management",
             vec!["cloudinary", "CLOUDINARY_"]),
            
            // Database
            ("PostgreSQL", IntegrationType::Database, "Relational database",
             vec!["postgresql://", "postgres://", "POSTGRES_", "pg_"]),
            ("MySQL", IntegrationType::Database, "Relational database",
             vec!["mysql://", "MYSQL_"]),
            ("MongoDB", IntegrationType::Database, "Document database",
             vec!["mongodb://", "MONGO_", "mongoose"]),
            ("Redis", IntegrationType::Cache, "In-memory cache",
             vec!["redis://", "REDIS_", "redis.Redis"]),
            
            // Analytics
            ("Segment", IntegrationType::Analytics, "Customer data platform",
             vec!["segment", "SEGMENT_"]),
            ("Mixpanel", IntegrationType::Analytics, "Product analytics",
             vec!["mixpanel", "MIXPANEL_"]),
            ("Amplitude", IntegrationType::Analytics, "Product analytics",
             vec!["amplitude", "AMPLITUDE_"]),
            ("Google Analytics", IntegrationType::Analytics, "Web analytics",
             vec!["gtag", "GA_TRACKING", "analytics.google"]),
            
            // Messaging
            ("Twilio", IntegrationType::Messaging, "SMS/Voice",
             vec!["twilio", "TWILIO_"]),
            ("AWS SNS", IntegrationType::Messaging, "Pub/Sub messaging",
             vec!["sns", "SNS_"]),
            ("RabbitMQ", IntegrationType::Messaging, "Message broker",
             vec!["rabbitmq", "RABBITMQ_", "amqp://"]),
            ("Kafka", IntegrationType::Messaging, "Event streaming",
             vec!["kafka", "KAFKA_"]),
            
            // Search
            ("Elasticsearch", IntegrationType::Search, "Search engine",
             vec!["elasticsearch", "ELASTICSEARCH_", "elastic"]),
            ("Algolia", IntegrationType::Search, "Search as a service",
             vec!["algolia", "ALGOLIA_"]),
            ("Typesense", IntegrationType::Search, "Search engine",
             vec!["typesense", "TYPESENSE_"]),
        ];

        for (name, integration_type, purpose, patterns) in known_integrations {
            if patterns.iter().any(|p| content.to_lowercase().contains(&p.to_lowercase())) {
                // Extract endpoints if possible
                let endpoints = self.extract_endpoints(content, name);
                let auth_method = self.detect_auth_method(content, name);

                integrations.push(Integration {
                    name: name.to_string(),
                    integration_type,
                    purpose: purpose.to_string(),
                    endpoints,
                    auth_method,
                    file: file.to_string(),
                });
            }
        }

        integrations
    }

    fn extract_endpoints(&self, content: &str, service_name: &str) -> Vec<String> {
        let mut endpoints = Vec::new();

        // Look for API endpoint URLs
        let url_regex = Regex::new(
            r#"["'](https?://[^"'\s]+)"#
        ).ok();

        if let Some(regex) = url_regex {
            for cap in regex.captures_iter(content) {
                if let Some(url) = cap.get(1) {
                    let url_str = url.as_str();
                    if url_str.to_lowercase().contains(&service_name.to_lowercase()) ||
                       url_str.contains("api.") {
                        endpoints.push(url_str.to_string());
                    }
                }
            }
        }

        // Deduplicate
        endpoints.sort();
        endpoints.dedup();
        endpoints.truncate(5); // Limit to 5 endpoints

        endpoints
    }

    fn detect_auth_method(&self, content: &str, service_name: &str) -> Option<String> {
        let lower = content.to_lowercase();
        let service_lower = service_name.to_lowercase();

        // Look for API key patterns
        if lower.contains(&format!("{}_api_key", service_lower)) ||
           lower.contains(&format!("{}_secret", service_lower)) {
            return Some("API Key".to_string());
        }

        // Look for OAuth patterns
        if lower.contains("oauth") && lower.contains(&service_lower) {
            return Some("OAuth 2.0".to_string());
        }

        // Look for bearer token
        if lower.contains("bearer") && lower.contains(&service_lower) {
            return Some("Bearer Token".to_string());
        }

        None
    }

    fn should_skip_dir(path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        path_str.contains("node_modules") ||
        path_str.contains("target") ||
        path_str.contains(".git") ||
        path_str.contains("vendor") ||
        path_str.contains("__pycache__")
    }
}

impl Default for IntegrationAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_stripe() {
        let content = r#"
            use stripe::Client;
            
            let client = Client::new(std::env::var("STRIPE_SECRET_KEY")?);
        "#;
        
        let analyzer = IntegrationAnalyzer::new();
        let integrations = analyzer.detect_integrations(content, "payment.rs");
        
        assert!(integrations.iter().any(|i| i.name == "Stripe"));
    }

    #[test]
    fn test_detect_postgres() {
        let content = r#"
            DATABASE_URL=postgresql://user:pass@localhost:5432/mydb
        "#;
        
        let analyzer = IntegrationAnalyzer::new();
        let integrations = analyzer.detect_integrations(content, ".env");
        
        assert!(integrations.iter().any(|i| i.name == "PostgreSQL"));
    }
}
