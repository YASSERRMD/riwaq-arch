//! Markdown utilities for documentation generation.

use std::fmt::Write;

/// A builder for creating Markdown documents.
#[derive(Debug, Default)]
pub struct MarkdownBuilder {
    content: String,
}

impl MarkdownBuilder {
    /// Create a new Markdown builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a level 1 heading.
    pub fn h1(&mut self, text: &str) -> &mut Self {
        writeln!(self.content, "# {}\n", text).unwrap();
        self
    }

    /// Add a level 2 heading.
    pub fn h2(&mut self, text: &str) -> &mut Self {
        writeln!(self.content, "## {}\n", text).unwrap();
        self
    }

    /// Add a level 3 heading.
    pub fn h3(&mut self, text: &str) -> &mut Self {
        writeln!(self.content, "### {}\n", text).unwrap();
        self
    }

    /// Add a level 4 heading.
    pub fn h4(&mut self, text: &str) -> &mut Self {
        writeln!(self.content, "#### {}\n", text).unwrap();
        self
    }

    /// Add a paragraph.
    pub fn paragraph(&mut self, text: &str) -> &mut Self {
        writeln!(self.content, "{}\n", text).unwrap();
        self
    }

    /// Add bold text inline (does not add newline).
    pub fn bold(&mut self, text: &str) -> &mut Self {
        write!(self.content, "**{}**", text).unwrap();
        self
    }

    /// Add italic text inline.
    pub fn italic(&mut self, text: &str) -> &mut Self {
        write!(self.content, "*{}*", text).unwrap();
        self
    }

    /// Add inline code.
    pub fn code_inline(&mut self, text: &str) -> &mut Self {
        write!(self.content, "`{}`", text).unwrap();
        self
    }

    /// Add a code block.
    pub fn code_block(&mut self, language: &str, code: &str) -> &mut Self {
        writeln!(self.content, "```{}", language).unwrap();
        writeln!(self.content, "{}", code).unwrap();
        writeln!(self.content, "```\n").unwrap();
        self
    }

    /// Add a blockquote.
    pub fn blockquote(&mut self, text: &str) -> &mut Self {
        for line in text.lines() {
            writeln!(self.content, "> {}", line).unwrap();
        }
        writeln!(self.content).unwrap();
        self
    }

    /// Add a bullet list.
    pub fn bullet_list(&mut self, items: &[&str]) -> &mut Self {
        for item in items {
            writeln!(self.content, "- {}", item).unwrap();
        }
        writeln!(self.content).unwrap();
        self
    }

    /// Add a numbered list.
    pub fn numbered_list(&mut self, items: &[&str]) -> &mut Self {
        for (i, item) in items.iter().enumerate() {
            writeln!(self.content, "{}. {}", i + 1, item).unwrap();
        }
        writeln!(self.content).unwrap();
        self
    }

    /// Add a table.
    pub fn table(&mut self, headers: &[&str], rows: &[Vec<&str>]) -> &mut Self {
        // Header row
        writeln!(self.content, "| {} |", headers.join(" | ")).unwrap();
        
        // Separator row
        let separator: Vec<&str> = headers.iter().map(|_| "---").collect();
        writeln!(self.content, "| {} |", separator.join(" | ")).unwrap();
        
        // Data rows
        for row in rows {
            writeln!(self.content, "| {} |", row.join(" | ")).unwrap();
        }
        writeln!(self.content).unwrap();
        self
    }

    /// Add a horizontal rule.
    pub fn hr(&mut self) -> &mut Self {
        writeln!(self.content, "---\n").unwrap();
        self
    }

    /// Add a link.
    pub fn link(&mut self, text: &str, url: &str) -> &mut Self {
        write!(self.content, "[{}]({})", text, url).unwrap();
        self
    }

    /// Add an image.
    pub fn image(&mut self, alt: &str, url: &str) -> &mut Self {
        writeln!(self.content, "![{}]({})\n", alt, url).unwrap();
        self
    }

    /// Add raw Markdown content.
    pub fn raw(&mut self, content: &str) -> &mut Self {
        writeln!(self.content, "{}", content).unwrap();
        self
    }

    /// Add a newline.
    pub fn newline(&mut self) -> &mut Self {
        writeln!(self.content).unwrap();
        self
    }

    /// Add a table of contents placeholder.
    pub fn toc(&mut self) -> &mut Self {
        writeln!(self.content, "<!-- toc -->\n").unwrap();
        self
    }

    /// Add a collapsible section.
    pub fn collapsible(&mut self, summary: &str, content: &str) -> &mut Self {
        writeln!(self.content, "<details>").unwrap();
        writeln!(self.content, "<summary>{}</summary>\n", summary).unwrap();
        writeln!(self.content, "{}", content).unwrap();
        writeln!(self.content, "</details>\n").unwrap();
        self
    }

    /// Add an admonition/callout (GitHub-style).
    pub fn admonition(&mut self, kind: AdmonitionKind, text: &str) -> &mut Self {
        let prefix = match kind {
            AdmonitionKind::Note => "> **Note**",
            AdmonitionKind::Warning => "> **Warning**",
            AdmonitionKind::Tip => "> **Tip**",
            AdmonitionKind::Important => "> **Important**",
        };
        writeln!(self.content, "{}", prefix).unwrap();
        for line in text.lines() {
            writeln!(self.content, "> {}", line).unwrap();
        }
        writeln!(self.content).unwrap();
        self
    }

    /// Build the final Markdown string.
    pub fn build(self) -> String {
        self.content
    }

    /// Get a reference to the current content.
    pub fn as_str(&self) -> &str {
        &self.content
    }
}

/// Types of admonitions/callouts.
#[derive(Debug, Clone, Copy)]
pub enum AdmonitionKind {
    Note,
    Warning,
    Tip,
    Important,
}

/// Format a file path as a relative link.
pub fn file_link(path: &str) -> String {
    format!("[`{}`]({})", path, path)
}

/// Format a list of items as a comma-separated string.
pub fn comma_list(items: &[String]) -> String {
    if items.is_empty() {
        "None".to_string()
    } else if items.len() == 1 {
        items[0].clone()
    } else {
        let last = items.len() - 1;
        format!("{} and {}", items[..last].join(", "), items[last])
    }
}

/// Escape special Markdown characters.
pub fn escape(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '*' | '_' | '`' | '[' | ']' | '(' | ')' | '#' | '+' | '-' | '.' | '!' | '|' => {
                format!("\\{}", c)
            }
            _ => c.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_builder() {
        let md = MarkdownBuilder::new()
            .h1("Title")
            .paragraph("This is a test.")
            .bullet_list(&["Item 1", "Item 2"])
            .build();

        assert!(md.contains("# Title"));
        assert!(md.contains("This is a test."));
        assert!(md.contains("- Item 1"));
        assert!(md.contains("- Item 2"));
    }

    #[test]
    fn test_table() {
        let md = MarkdownBuilder::new()
            .table(
                &["Name", "Type"],
                &[
                    vec!["foo", "String"],
                    vec!["bar", "i32"],
                ],
            )
            .build();

        assert!(md.contains("| Name | Type |"));
        assert!(md.contains("| foo | String |"));
    }

    #[test]
    fn test_escape() {
        assert_eq!(escape("hello*world"), "hello\\*world");
        assert_eq!(escape("[link]"), "\\[link\\]");
    }

    #[test]
    fn test_comma_list() {
        assert_eq!(comma_list(&[]), "None");
        assert_eq!(comma_list(&["a".to_string()]), "a");
        assert_eq!(comma_list(&["a".to_string(), "b".to_string()]), "a and b");
        assert_eq!(
            comma_list(&["a".to_string(), "b".to_string(), "c".to_string()]),
            "a, b and c"
        );
    }
}
