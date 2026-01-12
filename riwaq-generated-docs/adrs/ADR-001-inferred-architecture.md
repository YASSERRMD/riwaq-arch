# ADR-001: Inferred Architecture


## Status

Proposed (Auto-generated)

## Context

This ADR documents the architecture inferred from static analysis of the codebase. The project consists of 35 modules with 35 total files.

## Decision

The codebase follows a **Modular monolith architecture** pattern.

### Key Modules

- client
- analyzer
- markdown
- vscode-extension/client
- mod

## Consequences

**Positive:**

- Clear separation of concerns through module boundaries
- Enables independent development of modules

