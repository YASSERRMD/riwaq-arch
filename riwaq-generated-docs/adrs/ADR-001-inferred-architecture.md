# ADR-001: Inferred Architecture


## Status

Proposed (Auto-generated)

## Context

This ADR documents the architecture inferred from static analysis of the codebase. The project consists of 86 modules with 86 total files.

## Decision

The codebase follows a **Modular monolith architecture** pattern.

### Key Modules

- jetbrains-plugin/main/kotlin/com/riwaq/arch/server/ServerManager
- visual-studio-extension/GuidList
- vscode-extension/panels/architecturePanel
- jetbrains-plugin/main/kotlin/com/riwaq/arch/client/models/ApiModels
- vscode-extension/panels/askPanel

## Consequences

**Positive:**

- Clear separation of concerns through module boundaries
- Enables independent development of modules

