//! Diagrams Module
//!
//! This module handles diagram generation and rendering:
//! - Converting Mermaid code to actual images (PNG, SVG)
//! - NOT outputting text-based diagram code
//!
//! All diagrams are rendered as visual images, never as text.

pub mod renderer;

pub use renderer::{
    DiagramRenderer,
    DiagramRendererConfig,
    DiagramFormat,
    RenderedDiagram,
    render_architecture_diagram,
    render_er_diagram,
    render_state_diagram,
};
