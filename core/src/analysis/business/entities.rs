//! Entity/Data Model Analyzer
//!
//! Detects database models and domain entities:
//! - ORM models (SQLAlchemy, Diesel, TypeORM, GORM, etc.)
//! - Relationships (1:1, 1:N, N:N)
//! - Constraints (unique, foreign keys, indexes)
//! - Validation rules

use super::*;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use walkdir::WalkDir;

pub struct EntityAnalyzer;

impl EntityAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub async fn analyze(&self, root_path: &Path) -> anyhow::Result<(Vec<Entity>, Vec<EntityRelationship>)> {
        let mut entities = Vec::new();
        let mut relationships = Vec::new();

        // Walk through source files
        for entry in WalkDir::new(root_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if !Self::is_source_file(path) || Self::should_skip_dir(path) {
                continue;
            }

            if let Ok(content) = fs::read_to_string(path).await {
                let relative_path = path.strip_prefix(root_path)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();
                
                let (found_entities, found_relationships) = 
                    self.extract_entities(&content, &relative_path);
                
                entities.extend(found_entities);
                relationships.extend(found_relationships);
            }
        }

        Ok((entities, relationships))
    }

    fn extract_entities(&self, content: &str, file: &str) -> (Vec<Entity>, Vec<EntityRelationship>) {
        let mut entities = Vec::new();
        let mut relationships = Vec::new();

        // Detect Rust struct with derive attributes (Diesel, SQLx, SeaORM)
        self.extract_rust_entities(content, file, &mut entities, &mut relationships);
        
        // Detect Python models (SQLAlchemy, Django)
        self.extract_python_entities(content, file, &mut entities, &mut relationships);
        
        // Detect TypeScript/JavaScript entities (TypeORM, Prisma)
        self.extract_typescript_entities(content, file, &mut entities, &mut relationships);
        
        // Detect Go entities (GORM)
        self.extract_go_entities(content, file, &mut entities, &mut relationships);

        (entities, relationships)
    }

    fn extract_rust_entities(
        &self,
        content: &str,
        file: &str,
        entities: &mut Vec<Entity>,
        relationships: &mut Vec<EntityRelationship>,
    ) {
        // Match Rust structs with ORM-related derives
        let struct_regex = Regex::new(
            r#"(?s)#\[derive\([^)]*(?:Queryable|Insertable|Selectable|Entity|Model)[^)]*\)\]\s*(?:#\[[^\]]*\]\s*)*pub\s+struct\s+(\w+)\s*\{([^}]+)\}"#
        ).unwrap();

        for cap in struct_regex.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let body = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            
            let line = content[..cap.get(0).unwrap().start()]
                .lines().count() + 1;
            
            let fields = self.parse_rust_fields(body, &name, relationships);
            let validations = self.extract_rust_validations(body);
            let primary_key = self.find_primary_key(&fields);

            entities.push(Entity {
                name,
                entity_type: EntityType::Model,
                fields,
                validations,
                primary_key,
                indexes: Vec::new(),
                file: file.to_string(),
                line,
                description: None,
            });
        }

        // Also match enum types that could be status enums
        let enum_regex = Regex::new(
            r#"(?s)#\[derive\([^)]*(?:Serialize|DbEnum|sqlx::Type)[^)]*\)\]\s*pub\s+enum\s+(\w+)\s*\{([^}]+)\}"#
        ).unwrap();

        for cap in enum_regex.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let body = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            
            let line = content[..cap.get(0).unwrap().start()]
                .lines().count() + 1;
            
            // Parse enum variants as fields
            let fields: Vec<EntityField> = body.lines()
                .filter_map(|l| {
                    let trimmed = l.trim().trim_end_matches(',');
                    if !trimmed.is_empty() && !trimmed.starts_with("//") {
                        Some(EntityField {
                            name: trimmed.to_string(),
                            field_type: "variant".to_string(),
                            nullable: false,
                            unique: false,
                            indexed: false,
                            default_value: None,
                            description: None,
                        })
                    } else {
                        None
                    }
                })
                .collect();

            entities.push(Entity {
                name,
                entity_type: EntityType::Enum,
                fields,
                validations: Vec::new(),
                primary_key: None,
                indexes: Vec::new(),
                file: file.to_string(),
                line,
                description: None,
            });
        }
    }

    fn parse_rust_fields(
        &self,
        body: &str,
        entity_name: &str,
        relationships: &mut Vec<EntityRelationship>,
    ) -> Vec<EntityField> {
        let mut fields = Vec::new();
        
        let field_regex = Regex::new(
            r#"(?:pub\s+)?(\w+)\s*:\s*([^,\n]+)"#
        ).unwrap();

        for cap in field_regex.captures_iter(body) {
            let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let field_type = cap.get(2).map(|m| m.as_str().trim().to_string()).unwrap_or_default();

            let nullable = field_type.starts_with("Option<");
            let unique = body.contains(&format!("#[unique({})]", name)) || 
                        body.contains("#[column(unique)]");
            let indexed = body.contains("#[indexed]") || body.contains("index");

            // Detect relationships
            if field_type.contains("Vec<") {
                // One-to-Many
                if let Some(related) = self.extract_type_from_vec(&field_type) {
                    relationships.push(EntityRelationship {
                        from_entity: entity_name.to_string(),
                        to_entity: related,
                        relationship_type: RelationshipType::OneToMany,
                        field_name: name.clone(),
                        cascade: field_type.contains("cascade"),
                    });
                }
            } else if !Self::is_primitive_type(&field_type) && !field_type.starts_with("Option<") {
                // Could be Many-to-One
                let related = field_type.trim_end_matches('>').split('<').last()
                    .unwrap_or(&field_type).to_string();
                if !Self::is_primitive_type(&related) && related.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    relationships.push(EntityRelationship {
                        from_entity: entity_name.to_string(),
                        to_entity: related,
                        relationship_type: RelationshipType::ManyToOne,
                        field_name: name.clone(),
                        cascade: false,
                    });
                }
            }

            fields.push(EntityField {
                name,
                field_type,
                nullable,
                unique,
                indexed,
                default_value: None,
                description: None,
            });
        }

        fields
    }

    fn extract_rust_validations(&self, body: &str) -> Vec<Validation> {
        let mut validations = Vec::new();
        
        // Look for validation attributes
        let validation_regex = Regex::new(
            r#"#\[validate\((\w+)(?:\s*=\s*([^)]+))?\)\]\s*\w+\s*:\s*\w+"#
        ).unwrap();

        for cap in validation_regex.captures_iter(body) {
            let rule_type = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let parameters = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();

            validations.push(Validation {
                field: "".to_string(), // Would need more context
                rule_type,
                parameters,
            });
        }

        validations
    }

    fn extract_python_entities(
        &self,
        content: &str,
        file: &str,
        entities: &mut Vec<Entity>,
        relationships: &mut Vec<EntityRelationship>,
    ) {
        // SQLAlchemy models
        let class_regex = Regex::new(
            r#"(?s)class\s+(\w+)\s*\([^)]*(?:Base|Model|db\.Model)[^)]*\)\s*:\s*((?:[^\n]|\n(?!\s*class\s))*)"#
        ).unwrap();

        for cap in class_regex.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let body = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            
            let line = content[..cap.get(0).unwrap().start()]
                .lines().count() + 1;
            
            let fields = self.parse_sqlalchemy_fields(body, &name, relationships);

            entities.push(Entity {
                name,
                entity_type: EntityType::Model,
                fields,
                validations: Vec::new(),
                primary_key: Some("id".to_string()),
                indexes: Vec::new(),
                file: file.to_string(),
                line,
                description: None,
            });
        }
    }

    fn parse_sqlalchemy_fields(
        &self,
        body: &str,
        entity_name: &str,
        relationships: &mut Vec<EntityRelationship>,
    ) -> Vec<EntityField> {
        let mut fields = Vec::new();
        
        // Column definition
        let column_regex = Regex::new(
            r#"(\w+)\s*=\s*(?:db\.)?Column\s*\(\s*(?:db\.)?(\w+)"#
        ).unwrap();

        for cap in column_regex.captures_iter(body) {
            let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let field_type = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();

            let nullable = !body.contains(&format!("{name}.*nullable=False"));
            let unique = body.contains(&format!("{name}.*unique=True"));

            fields.push(EntityField {
                name,
                field_type,
                nullable,
                unique,
                indexed: false,
                default_value: None,
                description: None,
            });
        }

        // Relationship definition
        let rel_regex = Regex::new(
            r#"(\w+)\s*=\s*(?:db\.)?relationship\s*\(\s*["'](\w+)["']"#
        ).unwrap();

        for cap in rel_regex.captures_iter(body) {
            let field_name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let related = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();

            let is_many = body.contains(&format!("{field_name}.*uselist=True")) ||
                         !body.contains(&format!("{field_name}.*uselist=False"));

            relationships.push(EntityRelationship {
                from_entity: entity_name.to_string(),
                to_entity: related,
                relationship_type: if is_many { RelationshipType::OneToMany } else { RelationshipType::OneToOne },
                field_name,
                cascade: body.contains("cascade"),
            });
        }

        fields
    }

    fn extract_typescript_entities(
        &self,
        content: &str,
        file: &str,
        entities: &mut Vec<Entity>,
        relationships: &mut Vec<EntityRelationship>,
    ) {
        // TypeORM entities
        let entity_regex = Regex::new(
            r#"(?s)@Entity\s*(?:\([^)]*\))?\s*(?:export\s+)?class\s+(\w+)(?:\s+extends\s+\w+)?\s*\{([^}]+)\}"#
        ).unwrap();

        for cap in entity_regex.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let body = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            
            let line = content[..cap.get(0).unwrap().start()]
                .lines().count() + 1;
            
            let fields = self.parse_typeorm_fields(body, &name, relationships);

            entities.push(Entity {
                name,
                entity_type: EntityType::Model,
                fields,
                validations: Vec::new(),
                primary_key: Some("id".to_string()),
                indexes: Vec::new(),
                file: file.to_string(),
                line,
                description: None,
            });
        }
    }

    fn parse_typeorm_fields(
        &self,
        body: &str,
        entity_name: &str,
        relationships: &mut Vec<EntityRelationship>,
    ) -> Vec<EntityField> {
        let mut fields = Vec::new();
        
        // Column definition
        let column_regex = Regex::new(
            r#"@(?:Primary)?(?:Generated)?Column\s*(?:\([^)]*\))?\s*(\w+)(?:\s*:\s*(\w+))?"#
        ).unwrap();

        for cap in column_regex.captures_iter(body) {
            let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let field_type = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or("string".to_string());

            fields.push(EntityField {
                name,
                field_type,
                nullable: true,
                unique: false,
                indexed: false,
                default_value: None,
                description: None,
            });
        }

        // Relationships
        let rel_patterns = vec![
            ("OneToOne", RelationshipType::OneToOne),
            ("OneToMany", RelationshipType::OneToMany),
            ("ManyToOne", RelationshipType::ManyToOne),
            ("ManyToMany", RelationshipType::ManyToMany),
        ];

        for (decorator, rel_type) in rel_patterns {
            let pattern = format!(r#"@{}\s*\([^)]*\)\s*(\w+)\s*:\s*(\w+)"#, decorator);
            if let Ok(regex) = Regex::new(&pattern) {
                for cap in regex.captures_iter(body) {
                    let field_name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
                    let related = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();

                    relationships.push(EntityRelationship {
                        from_entity: entity_name.to_string(),
                        to_entity: related,
                        relationship_type: rel_type.clone(),
                        field_name,
                        cascade: body.contains("cascade"),
                    });
                }
            }
        }

        fields
    }

    fn extract_go_entities(
        &self,
        content: &str,
        file: &str,
        entities: &mut Vec<Entity>,
        _relationships: &mut Vec<EntityRelationship>,
    ) {
        // GORM models
        let struct_regex = Regex::new(
            r#"(?s)type\s+(\w+)\s+struct\s*\{([^}]+)\}"#
        ).unwrap();

        for cap in struct_regex.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let body = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            
            // Check if it's a GORM model (has gorm tags)
            if !body.contains("gorm:") && !body.contains("gorm.Model") {
                continue;
            }
            
            let line = content[..cap.get(0).unwrap().start()]
                .lines().count() + 1;
            
            let fields = self.parse_gorm_fields(body);

            entities.push(Entity {
                name,
                entity_type: EntityType::Model,
                fields,
                validations: Vec::new(),
                primary_key: Some("ID".to_string()),
                indexes: Vec::new(),
                file: file.to_string(),
                line,
                description: None,
            });
        }
    }

    fn parse_gorm_fields(&self, body: &str) -> Vec<EntityField> {
        let mut fields = Vec::new();
        
        let field_regex = Regex::new(
            r#"(\w+)\s+(\w+)(?:\s+`[^`]*`)?"#
        ).unwrap();

        for cap in field_regex.captures_iter(body) {
            let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let field_type = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();

            if name == "gorm" || field_type == "Model" {
                continue;
            }

            fields.push(EntityField {
                name,
                field_type,
                nullable: true,
                unique: false,
                indexed: false,
                default_value: None,
                description: None,
            });
        }

        fields
    }

    fn extract_type_from_vec(&self, field_type: &str) -> Option<String> {
        let regex = Regex::new(r#"Vec<([^>]+)>"#).ok()?;
        regex.captures(field_type)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn find_primary_key(&self, fields: &[EntityField]) -> Option<String> {
        // Common primary key names
        for name in &["id", "ID", "Id", "uuid", "pk"] {
            if fields.iter().any(|f| f.name == *name) {
                return Some(name.to_string());
            }
        }
        None
    }

    fn is_primitive_type(type_name: &str) -> bool {
        matches!(type_name, 
            "i8" | "i16" | "i32" | "i64" | "i128" |
            "u8" | "u16" | "u32" | "u64" | "u128" |
            "f32" | "f64" | "bool" | "String" | "str" |
            "int" | "float" | "string" | "bool" | "number" |
            "Int" | "Float" | "String" | "Boolean" | "Number"
        )
    }

    fn is_source_file(path: &Path) -> bool {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        matches!(ext, "rs" | "py" | "js" | "ts" | "go" | "java")
    }

    fn should_skip_dir(path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        path_str.contains("node_modules") ||
        path_str.contains("target") ||
        path_str.contains(".git") ||
        path_str.contains("vendor")
    }
}

impl Default for EntityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
