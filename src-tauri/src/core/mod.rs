/*!
 * Core Module
 * 
 * Contains the core IDE functionality including editor, project management,
 * terminal, debugger, Git integration, and Language Server Protocol support.
 */

pub mod editor;
pub mod project;
pub mod terminal;
pub mod debugger;
pub mod git;
pub mod lsp;

// Re-export main types for convenience
// Note: Most re-exports removed as they were unused
// These can be re-added when implementing actual functionality