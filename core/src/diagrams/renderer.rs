//! Diagram Renderer Module
//!
//! Renders Mermaid diagrams to actual images (SVG) using mermaid-rs.
//! This module outputs IMAGES, not text-based Mermaid code.

use mermaid_rs::Mermaid;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::info;

/// Diagram output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagramFormat {
    Svg,
}

impl DiagramFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Svg => "svg",
        }
    }
}

/// Rendered diagram result
#[derive(Debug, Clone)]
pub struct RenderedDiagram {
    /// Path to the generated image file
    pub path: PathBuf,
    /// SVG content as string
    pub svg: String,
    /// Format of the output
    pub format: DiagramFormat,
}

/// Diagram renderer configuration
#[derive(Debug, Clone)]
pub struct DiagramRendererConfig {
    /// Output directory for diagrams
    pub output_dir: PathBuf,
}

impl Default for DiagramRendererConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./diagrams"),
        }
    }
}

/// Diagram renderer using mermaid-rs
pub struct DiagramRenderer {
    config: DiagramRendererConfig,
    mermaid: Mermaid,
}

impl DiagramRenderer {
    pub fn new(config: DiagramRendererConfig) -> Result<Self, String> {
        let mermaid = Mermaid::new().map_err(|e| e.to_string())?;
        Ok(Self { config, mermaid })
    }

    /// Render a Mermaid diagram to an SVG file
    pub async fn render(&self, mermaid_code: &str, name: &str) -> Result<RenderedDiagram, String> {
        // Ensure output directory exists
        fs::create_dir_all(&self.config.output_dir).await
            .map_err(|e| format!("Failed to create output dir: {}", e))?;

        // Clean the mermaid code (remove markdown fences if present)
        let clean_code = mermaid_code
            .trim()
            .trim_start_matches("```mermaid")
            .trim_end_matches("```")
            .trim();

        // Render to SVG
        let svg = self.mermaid.render(clean_code)
            .map_err(|e| format!("Failed to render diagram: {}", e))?;

        // Write to file
        let output_path = self.config.output_dir
            .join(format!("{}.svg", name));

        fs::write(&output_path, &svg).await
            .map_err(|e| format!("Failed to write SVG: {}", e))?;

        info!(path = %output_path.display(), "Diagram rendered to SVG image");

        Ok(RenderedDiagram {
            path: output_path,
            svg,
            format: DiagramFormat::Svg,
        })
    }

    /// Render directly to SVG string (no file output)
    pub fn render_to_svg(&self, mermaid_code: &str) -> Result<String, String> {
        let clean_code = mermaid_code
            .trim()
            .trim_start_matches("```mermaid")
            .trim_end_matches("```")
            .trim();

        self.mermaid.render(clean_code)
            .map_err(|e| format!("Failed to render: {}", e))
    }
}

/// Render architecture diagram to SVG image
pub async fn render_architecture_diagram(
    mermaid_code: &str,
    output_dir: &Path,
    name: &str,
) -> Result<RenderedDiagram, String> {
    let config = DiagramRendererConfig {
        output_dir: output_dir.to_path_buf(),
    };

    let renderer = DiagramRenderer::new(config)?;
    renderer.render(mermaid_code, name).await
}

/// Render ER diagram to SVG image
pub async fn render_er_diagram(
    mermaid_code: &str,
    output_dir: &Path,
    name: &str,
) -> Result<RenderedDiagram, String> {
    let config = DiagramRendererConfig {
        output_dir: output_dir.to_path_buf(),
    };

    let renderer = DiagramRenderer::new(config)?;
    renderer.render(mermaid_code, name).await
}

/// Render state diagram to SVG image
pub async fn render_state_diagram(
    mermaid_code: &str,
    output_dir: &Path,
    name: &str,
) -> Result<RenderedDiagram, String> {
    let config = DiagramRendererConfig {
        output_dir: output_dir.to_path_buf(),
    };

    let renderer = DiagramRenderer::new(config)?;
    renderer.render(mermaid_code, name).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_extension() {
        assert_eq!(DiagramFormat::Svg.extension(), "svg");
    }

    #[test]
    fn test_default_config() {
        let config = DiagramRendererConfig::default();
        assert_eq!(config.output_dir, PathBuf::from("./diagrams"));
    }
}
