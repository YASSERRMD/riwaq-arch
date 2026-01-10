//! Multi-language code parser using tree-sitter.
//!
//! This module provides functionality to parse source code files
//! and extract structural information like functions, types, and imports.

use std::path::Path;
use tracing::{debug, warn};

use crate::errors::{Result, RiwaqError};
use crate::models::file::{
    ExportInfo, FieldInfo, FileSummary, ImportInfo, Language, TypeKind, TypeSummary, Visibility,
};
use crate::models::function::{ComplexityMetrics, FunctionSummary, ParameterInfo};

/// Parser for extracting code structure from source files.
pub struct CodeParser {
    /// Rust parser
    rust_parser: tree_sitter::Parser,
    /// Python parser
    python_parser: tree_sitter::Parser,
    /// JavaScript parser
    javascript_parser: tree_sitter::Parser,
    /// TypeScript parser
    typescript_parser: tree_sitter::Parser,
    /// Go parser
    go_parser: tree_sitter::Parser,
}

impl CodeParser {
    /// Create a new code parser with all language support.
    pub fn new() -> Result<Self> {
        Ok(Self {
            rust_parser: Self::create_parser(tree_sitter_rust::language())?,
            python_parser: Self::create_parser(tree_sitter_python::language())?,
            javascript_parser: Self::create_parser(tree_sitter_javascript::language())?,
            typescript_parser: Self::create_parser(tree_sitter_typescript::language_typescript())?,
            go_parser: Self::create_parser(tree_sitter_go::language())?,
        })
    }

    fn create_parser(language: tree_sitter::Language) -> Result<tree_sitter::Parser> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(language)
            .map_err(|e| RiwaqError::Internal(format!("Failed to set language: {}", e)))?;
        Ok(parser)
    }

    /// Parse a file and extract its structure.
    pub fn parse_file(&mut self, path: &Path, content: &str, language: Language) -> Result<FileSummary> {
        let mut summary = FileSummary::new(path.to_path_buf(), language);
        summary.line_count = content.lines().count();
        summary.content_hash = Self::compute_hash(content);

        if !language.is_supported() {
            debug!(?path, ?language, "Skipping unsupported language");
            return Ok(summary);
        }

        let parser = match language {
            Language::Rust => &mut self.rust_parser,
            Language::Python => &mut self.python_parser,
            Language::JavaScript => &mut self.javascript_parser,
            Language::TypeScript => &mut self.typescript_parser,
            Language::Go => &mut self.go_parser,
            _ => return Ok(summary),
        };

        let tree = parser
            .parse(content, None)
            .ok_or_else(|| RiwaqError::parse_error(path, "Failed to parse file"))?;

        let root = tree.root_node();

        // Extract based on language
        match language {
            Language::Rust => self.extract_rust(&root, content, &mut summary),
            Language::Python => self.extract_python(&root, content, &mut summary),
            Language::JavaScript | Language::TypeScript => {
                self.extract_javascript(&root, content, &mut summary)
            }
            Language::Go => self.extract_go(&root, content, &mut summary),
            _ => {}
        }

        Ok(summary)
    }

    /// Extract structure from Rust code.
    fn extract_rust(
        &self,
        root: &tree_sitter::Node,
        source: &str,
        summary: &mut FileSummary,
    ) {
        let mut cursor = root.walk();

        for child in root.children(&mut cursor) {
            match child.kind() {
                "function_item" => {
                    if let Some(func) = self.parse_rust_function(&child, source) {
                        summary.functions.push(func);
                    }
                }
                "struct_item" => {
                    if let Some(type_info) = self.parse_rust_struct(&child, source) {
                        summary.types.push(type_info);
                    }
                }
                "enum_item" => {
                    if let Some(type_info) = self.parse_rust_enum(&child, source) {
                        summary.types.push(type_info);
                    }
                }
                "trait_item" => {
                    if let Some(type_info) = self.parse_rust_trait(&child, source) {
                        summary.types.push(type_info);
                    }
                }
                "impl_item" => {
                    self.parse_rust_impl(&child, source, summary);
                }
                "use_declaration" => {
                    if let Some(import) = self.parse_rust_use(&child, source) {
                        summary.imports.push(import);
                    }
                }
                "mod_item" => {
                    // Module declaration
                    if let Some(name_node) = child.child_by_field_name("name") {
                        let name = Self::node_text(&name_node, source);
                        summary.exports.push(ExportInfo {
                            name,
                            is_default: false,
                            is_reexport: false,
                            line: child.start_position().row + 1,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    fn parse_rust_function(&self, node: &tree_sitter::Node, source: &str) -> Option<FunctionSummary> {
        let name_node = node.child_by_field_name("name")?;
        let name = Self::node_text(&name_node, source);

        let signature = Self::node_text(node, source)
            .lines()
            .next()
            .unwrap_or("")
            .to_string();

        let mut func = FunctionSummary::new(
            name,
            signature,
            node.start_position().row + 1,
            node.end_position().row + 1,
        );

        // Check for async
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "async" {
                func.is_async = true;
                break;
            }
        }

        // Parse parameters
        if let Some(params_node) = node.child_by_field_name("parameters") {
            func.parameters = self.parse_rust_parameters(&params_node, source);
        }

        // Parse return type
        if let Some(ret_node) = node.child_by_field_name("return_type") {
            func.return_type = Some(Self::node_text(&ret_node, source));
        }

        // Check visibility
        func.visibility = self.parse_rust_visibility(node);

        // Compute complexity
        if let Some(body) = node.child_by_field_name("body") {
            func.complexity = Some(self.compute_complexity(&body, source));
        }

        Some(func)
    }

    fn parse_rust_parameters(
        &self,
        params_node: &tree_sitter::Node,
        source: &str,
    ) -> Vec<ParameterInfo> {
        let mut params = Vec::new();
        let mut cursor = params_node.walk();

        for child in params_node.children(&mut cursor) {
            if child.kind() == "parameter" {
                if let Some(pattern) = child.child_by_field_name("pattern") {
                    let name = Self::node_text(&pattern, source);
                    let type_annotation = child
                        .child_by_field_name("type")
                        .map(|t| Self::node_text(&t, source));

                    params.push(ParameterInfo {
                        name,
                        type_annotation,
                        default_value: None,
                        is_variadic: false,
                    });
                }
            }
        }

        params
    }

    fn parse_rust_struct(&self, node: &tree_sitter::Node, source: &str) -> Option<TypeSummary> {
        let name_node = node.child_by_field_name("name")?;
        let name = Self::node_text(&name_node, source);

        let mut type_summary = TypeSummary {
            name,
            kind: TypeKind::Struct,
            doc_comment: self.extract_doc_comment(node, source),
            line_start: node.start_position().row + 1,
            line_end: node.end_position().row + 1,
            methods: Vec::new(),
            fields: Vec::new(),
            visibility: self.parse_rust_visibility(node),
        };

        // Parse fields
        if let Some(body) = node.child_by_field_name("body") {
            let mut cursor = body.walk();
            for child in body.children(&mut cursor) {
                if child.kind() == "field_declaration" {
                    if let Some(field_name) = child.child_by_field_name("name") {
                        let type_annotation = child
                            .child_by_field_name("type")
                            .map(|t| Self::node_text(&t, source));

                        type_summary.fields.push(FieldInfo {
                            name: Self::node_text(&field_name, source),
                            type_annotation,
                            doc_comment: None,
                            visibility: self.parse_rust_visibility(&child),
                        });
                    }
                }
            }
        }

        Some(type_summary)
    }

    fn parse_rust_enum(&self, node: &tree_sitter::Node, source: &str) -> Option<TypeSummary> {
        let name_node = node.child_by_field_name("name")?;
        let name = Self::node_text(&name_node, source);

        Some(TypeSummary {
            name,
            kind: TypeKind::Enum,
            doc_comment: self.extract_doc_comment(node, source),
            line_start: node.start_position().row + 1,
            line_end: node.end_position().row + 1,
            methods: Vec::new(),
            fields: Vec::new(),
            visibility: self.parse_rust_visibility(node),
        })
    }

    fn parse_rust_trait(&self, node: &tree_sitter::Node, source: &str) -> Option<TypeSummary> {
        let name_node = node.child_by_field_name("name")?;
        let name = Self::node_text(&name_node, source);

        Some(TypeSummary {
            name,
            kind: TypeKind::Trait,
            doc_comment: self.extract_doc_comment(node, source),
            line_start: node.start_position().row + 1,
            line_end: node.end_position().row + 1,
            methods: Vec::new(),
            fields: Vec::new(),
            visibility: self.parse_rust_visibility(node),
        })
    }

    fn parse_rust_impl(&self, node: &tree_sitter::Node, source: &str, summary: &mut FileSummary) {
        // Parse impl block methods and add them to the appropriate type
        if let Some(body) = node.child_by_field_name("body") {
            let mut cursor = body.walk();
            for child in body.children(&mut cursor) {
                if child.kind() == "function_item" {
                    if let Some(func) = self.parse_rust_function(&child, source) {
                        summary.functions.push(func);
                    }
                }
            }
        }
    }

    fn parse_rust_use(&self, node: &tree_sitter::Node, source: &str) -> Option<ImportInfo> {
        let argument = node.child_by_field_name("argument")?;
        let path = Self::node_text(&argument, source);

        Some(ImportInfo {
            path: path.clone(),
            items: vec![path],
            is_wildcard: source[node.byte_range()].contains("::*"),
            alias: None,
            line: node.start_position().row + 1,
        })
    }

    fn parse_rust_visibility(&self, node: &tree_sitter::Node) -> Visibility {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "visibility_modifier" {
                return Visibility::Public;
            }
        }
        Visibility::Private
    }

    /// Extract structure from Python code.
    fn extract_python(
        &self,
        root: &tree_sitter::Node,
        source: &str,
        summary: &mut FileSummary,
    ) {
        let mut cursor = root.walk();

        for child in root.children(&mut cursor) {
            match child.kind() {
                "function_definition" => {
                    if let Some(func) = self.parse_python_function(&child, source) {
                        summary.functions.push(func);
                    }
                }
                "class_definition" => {
                    if let Some(type_info) = self.parse_python_class(&child, source) {
                        summary.types.push(type_info);
                    }
                }
                "import_statement" | "import_from_statement" => {
                    if let Some(import) = self.parse_python_import(&child, source) {
                        summary.imports.push(import);
                    }
                }
                _ => {}
            }
        }
    }

    fn parse_python_function(
        &self,
        node: &tree_sitter::Node,
        source: &str,
    ) -> Option<FunctionSummary> {
        let name_node = node.child_by_field_name("name")?;
        let name = Self::node_text(&name_node, source);

        let signature = Self::node_text(node, source)
            .lines()
            .next()
            .unwrap_or("")
            .to_string();

        let mut func = FunctionSummary::new(
            name.clone(),
            signature,
            node.start_position().row + 1,
            node.end_position().row + 1,
        );

        // Check for async
        let func_text = Self::node_text(node, source);
        func.is_async = func_text.starts_with("async");

        // Parse parameters
        if let Some(params_node) = node.child_by_field_name("parameters") {
            func.parameters = self.parse_python_parameters(&params_node, source);
        }

        // Check visibility (underscore prefix = private)
        func.visibility = if name.starts_with('_') {
            Visibility::Private
        } else {
            Visibility::Public
        };

        Some(func)
    }

    fn parse_python_parameters(
        &self,
        params_node: &tree_sitter::Node,
        source: &str,
    ) -> Vec<ParameterInfo> {
        let mut params = Vec::new();
        let mut cursor = params_node.walk();

        for child in params_node.children(&mut cursor) {
            match child.kind() {
                "identifier" => {
                    let name = Self::node_text(&child, source);
                    if name != "self" && name != "cls" {
                        params.push(ParameterInfo::new(name));
                    }
                }
                "typed_parameter" | "default_parameter" | "typed_default_parameter" => {
                    if let Some(name_node) = child.child_by_field_name("name") {
                        let name = Self::node_text(&name_node, source);
                        let type_annotation = child
                            .child_by_field_name("type")
                            .map(|t| Self::node_text(&t, source));
                        let default_value = child
                            .child_by_field_name("value")
                            .map(|v| Self::node_text(&v, source));

                        params.push(ParameterInfo {
                            name,
                            type_annotation,
                            default_value,
                            is_variadic: false,
                        });
                    }
                }
                _ => {}
            }
        }

        params
    }

    fn parse_python_class(&self, node: &tree_sitter::Node, source: &str) -> Option<TypeSummary> {
        let name_node = node.child_by_field_name("name")?;
        let name = Self::node_text(&name_node, source);

        let mut type_summary = TypeSummary {
            name: name.clone(),
            kind: TypeKind::Class,
            doc_comment: self.extract_doc_comment(node, source),
            line_start: node.start_position().row + 1,
            line_end: node.end_position().row + 1,
            methods: Vec::new(),
            fields: Vec::new(),
            visibility: if name.starts_with('_') {
                Visibility::Private
            } else {
                Visibility::Public
            },
        };

        // Parse methods
        if let Some(body) = node.child_by_field_name("body") {
            let mut cursor = body.walk();
            for child in body.children(&mut cursor) {
                if child.kind() == "function_definition" {
                    if let Some(method) = self.parse_python_function(&child, source) {
                        type_summary.methods.push(method);
                    }
                }
            }
        }

        Some(type_summary)
    }

    fn parse_python_import(&self, node: &tree_sitter::Node, source: &str) -> Option<ImportInfo> {
        let text = Self::node_text(node, source);

        Some(ImportInfo {
            path: text.clone(),
            items: vec![text],
            is_wildcard: source[node.byte_range()].contains("import *"),
            alias: None,
            line: node.start_position().row + 1,
        })
    }

    /// Extract structure from JavaScript/TypeScript code.
    fn extract_javascript(
        &self,
        root: &tree_sitter::Node,
        source: &str,
        summary: &mut FileSummary,
    ) {
        let mut cursor = root.walk();

        for child in root.children(&mut cursor) {
            match child.kind() {
                "function_declaration" | "arrow_function" | "function" => {
                    if let Some(func) = self.parse_js_function(&child, source) {
                        summary.functions.push(func);
                    }
                }
                "class_declaration" => {
                    if let Some(type_info) = self.parse_js_class(&child, source) {
                        summary.types.push(type_info);
                    }
                }
                "import_statement" => {
                    if let Some(import) = self.parse_js_import(&child, source) {
                        summary.imports.push(import);
                    }
                }
                "export_statement" => {
                    if let Some(export) = self.parse_js_export(&child, source) {
                        summary.exports.push(export);
                    }
                }
                "lexical_declaration" | "variable_declaration" => {
                    // Handle const/let/var declarations that might be functions
                    self.extract_js_variable_functions(&child, source, summary);
                }
                _ => {}
            }
        }
    }

    fn parse_js_function(&self, node: &tree_sitter::Node, source: &str) -> Option<FunctionSummary> {
        let name = node
            .child_by_field_name("name")
            .map(|n| Self::node_text(&n, source))
            .unwrap_or_else(|| "<anonymous>".to_string());

        let signature = Self::node_text(node, source)
            .lines()
            .next()
            .unwrap_or("")
            .to_string();

        let mut func = FunctionSummary::new(
            name,
            signature,
            node.start_position().row + 1,
            node.end_position().row + 1,
        );

        // Check for async
        let func_text = Self::node_text(node, source);
        func.is_async = func_text.contains("async");

        // Parse parameters
        if let Some(params_node) = node.child_by_field_name("parameters") {
            func.parameters = self.parse_js_parameters(&params_node, source);
        }

        func.visibility = Visibility::Public;

        Some(func)
    }

    fn parse_js_parameters(
        &self,
        params_node: &tree_sitter::Node,
        source: &str,
    ) -> Vec<ParameterInfo> {
        let mut params = Vec::new();
        let mut cursor = params_node.walk();

        for child in params_node.children(&mut cursor) {
            match child.kind() {
                "identifier" => {
                    params.push(ParameterInfo::new(Self::node_text(&child, source)));
                }
                "required_parameter" | "optional_parameter" => {
                    if let Some(pattern) = child.child_by_field_name("pattern") {
                        let name = Self::node_text(&pattern, source);
                        let type_annotation = child
                            .child_by_field_name("type")
                            .map(|t| Self::node_text(&t, source));

                        params.push(ParameterInfo {
                            name,
                            type_annotation,
                            default_value: None,
                            is_variadic: false,
                        });
                    }
                }
                _ => {}
            }
        }

        params
    }

    fn parse_js_class(&self, node: &tree_sitter::Node, source: &str) -> Option<TypeSummary> {
        let name_node = node.child_by_field_name("name")?;
        let name = Self::node_text(&name_node, source);

        let mut type_summary = TypeSummary {
            name,
            kind: TypeKind::Class,
            doc_comment: self.extract_doc_comment(node, source),
            line_start: node.start_position().row + 1,
            line_end: node.end_position().row + 1,
            methods: Vec::new(),
            fields: Vec::new(),
            visibility: Visibility::Public,
        };

        // Parse methods
        if let Some(body) = node.child_by_field_name("body") {
            let mut cursor = body.walk();
            for child in body.children(&mut cursor) {
                if child.kind() == "method_definition" {
                    if let Some(method) = self.parse_js_function(&child, source) {
                        type_summary.methods.push(method);
                    }
                }
            }
        }

        Some(type_summary)
    }

    fn parse_js_import(&self, node: &tree_sitter::Node, source: &str) -> Option<ImportInfo> {
        let text = Self::node_text(node, source);

        Some(ImportInfo {
            path: text.clone(),
            items: vec![text],
            is_wildcard: source[node.byte_range()].contains("* as"),
            alias: None,
            line: node.start_position().row + 1,
        })
    }

    fn parse_js_export(&self, node: &tree_sitter::Node, source: &str) -> Option<ExportInfo> {
        let text = Self::node_text(node, source);
        let is_default = text.contains("export default");

        Some(ExportInfo {
            name: text,
            is_default,
            is_reexport: text.contains("from "),
            line: node.start_position().row + 1,
        })
    }

    fn extract_js_variable_functions(
        &self,
        node: &tree_sitter::Node,
        source: &str,
        summary: &mut FileSummary,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "variable_declarator" {
                if let Some(value) = child.child_by_field_name("value") {
                    if value.kind() == "arrow_function" || value.kind() == "function" {
                        if let Some(name_node) = child.child_by_field_name("name") {
                            let name = Self::node_text(&name_node, source);
                            let signature = Self::node_text(node, source)
                                .lines()
                                .next()
                                .unwrap_or("")
                                .to_string();

                            let mut func = FunctionSummary::new(
                                name,
                                signature,
                                node.start_position().row + 1,
                                node.end_position().row + 1,
                            );
                            func.visibility = Visibility::Public;
                            summary.functions.push(func);
                        }
                    }
                }
            }
        }
    }

    /// Extract structure from Go code.
    fn extract_go(&self, root: &tree_sitter::Node, source: &str, summary: &mut FileSummary) {
        let mut cursor = root.walk();

        for child in root.children(&mut cursor) {
            match child.kind() {
                "function_declaration" | "method_declaration" => {
                    if let Some(func) = self.parse_go_function(&child, source) {
                        summary.functions.push(func);
                    }
                }
                "type_declaration" => {
                    if let Some(type_info) = self.parse_go_type(&child, source) {
                        summary.types.push(type_info);
                    }
                }
                "import_declaration" => {
                    if let Some(import) = self.parse_go_import(&child, source) {
                        summary.imports.push(import);
                    }
                }
                _ => {}
            }
        }
    }

    fn parse_go_function(&self, node: &tree_sitter::Node, source: &str) -> Option<FunctionSummary> {
        let name_node = node.child_by_field_name("name")?;
        let name = Self::node_text(&name_node, source);

        let signature = Self::node_text(node, source)
            .lines()
            .next()
            .unwrap_or("")
            .to_string();

        let mut func = FunctionSummary::new(
            name.clone(),
            signature,
            node.start_position().row + 1,
            node.end_position().row + 1,
        );

        // In Go, public = uppercase first letter
        func.visibility = if name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            Visibility::Public
        } else {
            Visibility::Private
        };

        Some(func)
    }

    fn parse_go_type(&self, node: &tree_sitter::Node, source: &str) -> Option<TypeSummary> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "type_spec" {
                if let Some(name_node) = child.child_by_field_name("name") {
                    let name = Self::node_text(&name_node, source);

                    let kind = if let Some(type_node) = child.child_by_field_name("type") {
                        match type_node.kind() {
                            "struct_type" => TypeKind::Struct,
                            "interface_type" => TypeKind::Interface,
                            _ => TypeKind::TypeAlias,
                        }
                    } else {
                        TypeKind::TypeAlias
                    };

                    return Some(TypeSummary {
                        name: name.clone(),
                        kind,
                        doc_comment: None,
                        line_start: node.start_position().row + 1,
                        line_end: node.end_position().row + 1,
                        methods: Vec::new(),
                        fields: Vec::new(),
                        visibility: if name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
                        {
                            Visibility::Public
                        } else {
                            Visibility::Private
                        },
                    });
                }
            }
        }
        None
    }

    fn parse_go_import(&self, node: &tree_sitter::Node, source: &str) -> Option<ImportInfo> {
        let text = Self::node_text(node, source);

        Some(ImportInfo {
            path: text.clone(),
            items: vec![text],
            is_wildcard: false,
            alias: None,
            line: node.start_position().row + 1,
        })
    }

    /// Compute complexity metrics for a code block.
    fn compute_complexity(&self, node: &tree_sitter::Node, _source: &str) -> ComplexityMetrics {
        let mut metrics = ComplexityMetrics::new();
        self.count_complexity_recursive(node, 0, &mut metrics);
        metrics
    }

    fn count_complexity_recursive(
        &self,
        node: &tree_sitter::Node,
        depth: u32,
        metrics: &mut ComplexityMetrics,
    ) {
        metrics.max_nesting = metrics.max_nesting.max(depth);

        match node.kind() {
            "if_expression" | "if_statement" | "match_expression" | "switch_statement"
            | "ternary_expression" | "conditional_expression" => {
                metrics.cyclomatic += 1;
                metrics.branches += 1;
            }
            "for_expression" | "for_statement" | "while_expression" | "while_statement"
            | "loop_expression" | "for_in_statement" | "for_range_clause" => {
                metrics.cyclomatic += 1;
                metrics.loops += 1;
            }
            "else_clause" | "else_if_clause" | "case_clause" => {
                metrics.branches += 1;
            }
            "binary_expression" => {
                // Count logical operators
                if let Some(op) = node.child_by_field_name("operator") {
                    let op_text = op.kind();
                    if op_text == "&&" || op_text == "||" {
                        metrics.cyclomatic += 1;
                    }
                }
            }
            _ => {}
        }

        let new_depth = if matches!(
            node.kind(),
            "block"
                | "if_expression"
                | "if_statement"
                | "for_expression"
                | "for_statement"
                | "while_expression"
                | "while_statement"
                | "match_expression"
                | "switch_statement"
        ) {
            depth + 1
        } else {
            depth
        };

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.count_complexity_recursive(&child, new_depth, metrics);
        }
    }

    /// Extract doc comments before a node.
    fn extract_doc_comment(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
        // Look for comment nodes before this node
        if let Some(prev) = node.prev_sibling() {
            if prev.kind().contains("comment") {
                return Some(Self::node_text(&prev, source));
            }
        }
        None
    }

    /// Get the text content of a node.
    fn node_text(node: &tree_sitter::Node, source: &str) -> String {
        source[node.byte_range()].to_string()
    }

    /// Compute a simple hash of the content.
    fn compute_hash(content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }
}

impl Default for CodeParser {
    fn default() -> Self {
        Self::new().expect("Failed to create default CodeParser")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rust_function() {
        let mut parser = CodeParser::new().unwrap();
        let source = r#"
/// A test function
pub fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}
"#;
        let summary = parser
            .parse_file(Path::new("test.rs"), source, Language::Rust)
            .unwrap();

        assert_eq!(summary.functions.len(), 1);
        assert_eq!(summary.functions[0].name, "hello");
        assert_eq!(summary.functions[0].visibility, Visibility::Public);
    }

    #[test]
    fn test_parse_rust_struct() {
        let mut parser = CodeParser::new().unwrap();
        let source = r#"
pub struct User {
    pub name: String,
    age: u32,
}
"#;
        let summary = parser
            .parse_file(Path::new("test.rs"), source, Language::Rust)
            .unwrap();

        assert_eq!(summary.types.len(), 1);
        assert_eq!(summary.types[0].name, "User");
        assert_eq!(summary.types[0].kind, TypeKind::Struct);
    }

    #[test]
    fn test_parse_python_function() {
        let mut parser = CodeParser::new().unwrap();
        let source = r#"
def hello(name: str) -> str:
    return f"Hello, {name}!"

async def async_hello(name):
    return await greet(name)
"#;
        let summary = parser
            .parse_file(Path::new("test.py"), source, Language::Python)
            .unwrap();

        assert_eq!(summary.functions.len(), 2);
        assert_eq!(summary.functions[0].name, "hello");
        assert!(summary.functions[1].is_async);
    }

    #[test]
    fn test_compute_hash() {
        let hash1 = CodeParser::compute_hash("hello world");
        let hash2 = CodeParser::compute_hash("hello world");
        let hash3 = CodeParser::compute_hash("different");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}
