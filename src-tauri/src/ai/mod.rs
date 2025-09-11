/*!
 * AI Module
 * 
 * Contains AI-related functionality including model management, chat engine,
 * context management, and code assistance.
 */

pub mod model_manager;
pub mod chat;
pub mod context;
pub mod assistant;

// Re-export main types for convenience
// Note: Most re-exports removed as they were unused
// These can be re-added when implementing actual functionality
pub use model_manager::GenerationParams;