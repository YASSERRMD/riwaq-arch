# ADR-001: Inferred Architecture


## Status

Proposed (Auto-generated)

## Context

This ADR documents the architecture inferred from static analysis of the codebase. The project consists of 84 modules with 84 total files.

## Decision

The codebase follows a **Modular monolith architecture** pattern.

### Key Modules

- jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ArchitectureToolWindowFactory
- visual-studio-extension/ServerManager
- jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/AskAboutSelectionAction
- file
- vscode-extension/views/architectureTree

## Consequences

**Positive:**

- Clear separation of concerns through module boundaries
- Enables independent development of modules

