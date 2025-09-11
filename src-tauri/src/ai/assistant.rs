/*!
 * Code Assistant Module
 * 
 * Provides AI-powered code assistance including completions, suggestions,
 * refactoring, and documentation generation.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAssistant {
    pub suggestions_cache: HashMap<String, Vec<CodeSuggestion>>,
    pub refactoring_history: Vec<RefactoringOperation>,
    pub assistant_settings: AssistantSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSuggestion {
    pub id: String,
    pub suggestion_type: SuggestionType,
    pub title: String,
    pub description: String,
    pub code: String,
    pub language: String,
    pub confidence: f32,
    pub position: CodePosition,
    pub metadata: SuggestionMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    Completion,
    Refactoring,
    Documentation,
    Test,
    BugFix,
    Optimization,
    Style,
    Import,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodePosition {
    pub file_path: PathBuf,
    pub line: u32,
    pub column: u32,
    pub end_line: Option<u32>,
    pub end_column: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionMetadata {
    pub complexity: ComplexityLevel,
    pub estimated_time: u32, // minutes
    pub dependencies: Vec<String>,
    pub tags: Vec<String>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringOperation {
    pub id: String,
    pub operation_type: RefactoringType,
    pub description: String,
    pub original_code: String,
    pub refactored_code: String,
    pub file_path: PathBuf,
    pub position: CodePosition,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefactoringType {
    ExtractMethod,
    ExtractVariable,
    InlineVariable,
    RenameSymbol,
    MoveMethod,
    ExtractInterface,
    SimplifyConditional,
    RemoveDeadCode,
    OptimizeImports,
    FormatCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantSettings {
    pub enable_completions: bool,
    pub enable_refactoring: bool,
    pub enable_documentation: bool,
    pub enable_test_generation: bool,
    pub max_suggestions: usize,
    pub confidence_threshold: f32,
    pub auto_apply_simple_suggestions: bool,
    pub include_examples: bool,
    pub language_specific_rules: HashMap<String, LanguageRules>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageRules {
    pub style_guide: String,
    pub naming_conventions: HashMap<String, String>,
    pub best_practices: Vec<String>,
    pub anti_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAnalysis {
    pub file_path: PathBuf,
    pub issues: Vec<CodeIssue>,
    pub metrics: CodeMetrics,
    pub suggestions: Vec<CodeSuggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeIssue {
    pub id: String,
    pub issue_type: IssueType,
    pub severity: IssueSeverity,
    pub message: String,
    pub position: CodePosition,
    pub fix_suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    Syntax,
    Style,
    Performance,
    Security,
    Maintainability,
    Documentation,
    Test,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    pub lines_of_code: u32,
    pub cyclomatic_complexity: u32,
    pub maintainability_index: f32,
    pub test_coverage: f32,
    pub documentation_coverage: f32,
}

impl Default for CodeAssistant {
    fn default() -> Self {
        Self {
            suggestions_cache: HashMap::new(),
            refactoring_history: Vec::new(),
            assistant_settings: AssistantSettings::default(),
        }
    }
}

impl Default for AssistantSettings {
    fn default() -> Self {
        Self {
            enable_completions: true,
            enable_refactoring: true,
            enable_documentation: true,
            enable_test_generation: true,
            max_suggestions: 10,
            confidence_threshold: 0.7,
            auto_apply_simple_suggestions: false,
            include_examples: true,
            language_specific_rules: Self::get_default_language_rules(),
        }
    }
}

impl AssistantSettings {
    fn get_default_language_rules() -> HashMap<String, LanguageRules> {
        let mut rules = HashMap::new();

        // Rust rules
        rules.insert("rust".to_string(), LanguageRules {
            style_guide: "rustfmt".to_string(),
            naming_conventions: HashMap::from([
                ("functions".to_string(), "snake_case".to_string()),
                ("variables".to_string(), "snake_case".to_string()),
                ("types".to_string(), "PascalCase".to_string()),
                ("constants".to_string(), "SCREAMING_SNAKE_CASE".to_string()),
            ]),
            best_practices: vec![
                "Use Result<T, E> for error handling".to_string(),
                "Prefer immutability".to_string(),
                "Use match instead of if-else chains".to_string(),
                "Leverage the borrow checker".to_string(),
            ],
            anti_patterns: vec![
                "Using unwrap() in production code".to_string(),
                "Unnecessary cloning".to_string(),
                "Ignoring compiler warnings".to_string(),
            ],
        });

        // JavaScript/TypeScript rules
        rules.insert("javascript".to_string(), LanguageRules {
            style_guide: "eslint".to_string(),
            naming_conventions: HashMap::from([
                ("functions".to_string(), "camelCase".to_string()),
                ("variables".to_string(), "camelCase".to_string()),
                ("classes".to_string(), "PascalCase".to_string()),
                ("constants".to_string(), "SCREAMING_SNAKE_CASE".to_string()),
            ]),
            best_practices: vec![
                "Use const and let instead of var".to_string(),
                "Prefer arrow functions for short functions".to_string(),
                "Use async/await instead of callbacks".to_string(),
                "Handle errors properly".to_string(),
            ],
            anti_patterns: vec![
                "Using var".to_string(),
                "Callback hell".to_string(),
                "Global variables".to_string(),
            ],
        });

        rules.insert("typescript".to_string(), LanguageRules {
            style_guide: "eslint + prettier".to_string(),
            naming_conventions: HashMap::from([
                ("functions".to_string(), "camelCase".to_string()),
                ("variables".to_string(), "camelCase".to_string()),
                ("classes".to_string(), "PascalCase".to_string()),
                ("interfaces".to_string(), "PascalCase".to_string()),
                ("types".to_string(), "PascalCase".to_string()),
            ]),
            best_practices: vec![
                "Use strict type checking".to_string(),
                "Prefer interfaces over types for object shapes".to_string(),
                "Use generic types for reusability".to_string(),
                "Avoid any type".to_string(),
            ],
            anti_patterns: vec![
                "Using any type".to_string(),
                "Ignoring type errors".to_string(),
                "Overusing type assertions".to_string(),
            ],
        });

        rules
    }
}

impl CodeAssistant {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn get_completions(&mut self, file_path: &PathBuf, position: CodePosition, context: &str) -> Result<Vec<CodeSuggestion>> {
        let cache_key = format!("{}:{}:{}", file_path.display(), position.line, position.column);
        
        if let Some(cached) = self.suggestions_cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        let language = self.detect_language(file_path);
        let suggestions = self.generate_completions(context, &language, &position).await?;
        
        self.suggestions_cache.insert(cache_key, suggestions.clone());
        Ok(suggestions)
    }

    async fn generate_completions(&self, context: &str, language: &str, position: &CodePosition) -> Result<Vec<CodeSuggestion>> {
        let mut suggestions = Vec::new();

        // Basic keyword completions based on language
        match language {
            "rust" => {
                suggestions.extend(self.get_rust_completions(context, position));
            }
            "javascript" | "typescript" => {
                suggestions.extend(self.get_js_completions(context, position));
            }
            "python" => {
                suggestions.extend(self.get_python_completions(context, position));
            }
            _ => {
                suggestions.extend(self.get_generic_completions(context, position));
            }
        }

        // Filter by confidence threshold
        suggestions.retain(|s| s.confidence >= self.assistant_settings.confidence_threshold);
        
        // Limit suggestions
        suggestions.truncate(self.assistant_settings.max_suggestions);

        Ok(suggestions)
    }

    fn get_rust_completions(&self, context: &str, position: &CodePosition) -> Vec<CodeSuggestion> {
        let mut suggestions = Vec::new();

        if context.contains("fn ") {
            suggestions.push(CodeSuggestion {
                id: Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::Completion,
                title: "Function template".to_string(),
                description: "Complete function definition".to_string(),
                code: "fn $0() {\n    \n}".to_string(),
                language: "rust".to_string(),
                confidence: 0.9,
                position: position.clone(),
                metadata: SuggestionMetadata {
                    complexity: ComplexityLevel::Low,
                    estimated_time: 1,
                    dependencies: vec![],
                    tags: vec!["function".to_string(), "template".to_string()],
                    examples: vec!["fn main() {\n    println!(\"Hello, world!\");\n}".to_string()],
                },
            });
        }

        if context.contains("let ") {
            suggestions.push(CodeSuggestion {
                id: Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::Completion,
                title: "Variable declaration".to_string(),
                description: "Complete variable declaration".to_string(),
                code: "let $0 = ".to_string(),
                language: "rust".to_string(),
                confidence: 0.8,
                position: position.clone(),
                metadata: SuggestionMetadata {
                    complexity: ComplexityLevel::Low,
                    estimated_time: 1,
                    dependencies: vec![],
                    tags: vec!["variable".to_string(), "declaration".to_string()],
                    examples: vec!["let x = 42;".to_string()],
                },
            });
        }

        suggestions
    }

    fn get_js_completions(&self, context: &str, position: &CodePosition) -> Vec<CodeSuggestion> {
        let mut suggestions = Vec::new();

        if context.contains("function ") {
            suggestions.push(CodeSuggestion {
                id: Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::Completion,
                title: "Function template".to_string(),
                description: "Complete function definition".to_string(),
                code: "function $0() {\n    \n}".to_string(),
                language: "javascript".to_string(),
                confidence: 0.9,
                position: position.clone(),
                metadata: SuggestionMetadata {
                    complexity: ComplexityLevel::Low,
                    estimated_time: 1,
                    dependencies: vec![],
                    tags: vec!["function".to_string(), "template".to_string()],
                    examples: vec!["function greet(name) {\n    return `Hello, ${name}!`;\n}".to_string()],
                },
            });
        }

        if context.contains("const ") || context.contains("let ") {
            suggestions.push(CodeSuggestion {
                id: Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::Completion,
                title: "Variable declaration".to_string(),
                description: "Complete variable declaration".to_string(),
                code: "const $0 = ".to_string(),
                language: "javascript".to_string(),
                confidence: 0.8,
                position: position.clone(),
                metadata: SuggestionMetadata {
                    complexity: ComplexityLevel::Low,
                    estimated_time: 1,
                    dependencies: vec![],
                    tags: vec!["variable".to_string(), "declaration".to_string()],
                    examples: vec!["const message = 'Hello, world!';".to_string()],
                },
            });
        }

        suggestions
    }

    fn get_python_completions(&self, context: &str, position: &CodePosition) -> Vec<CodeSuggestion> {
        let mut suggestions = Vec::new();

        if context.contains("def ") {
            suggestions.push(CodeSuggestion {
                id: Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::Completion,
                title: "Function template".to_string(),
                description: "Complete function definition".to_string(),
                code: "def $0():\n    pass".to_string(),
                language: "python".to_string(),
                confidence: 0.9,
                position: position.clone(),
                metadata: SuggestionMetadata {
                    complexity: ComplexityLevel::Low,
                    estimated_time: 1,
                    dependencies: vec![],
                    tags: vec!["function".to_string(), "template".to_string()],
                    examples: vec!["def greet(name):\n    return f'Hello, {name}!'".to_string()],
                },
            });
        }

        suggestions
    }

    fn get_generic_completions(&self, _context: &str, _position: &CodePosition) -> Vec<CodeSuggestion> {
        vec![]
    }

    pub async fn analyze_code(&self, file_path: &PathBuf, content: &str) -> Result<CodeAnalysis> {
        let language = self.detect_language(file_path);
        let issues = self.detect_issues(content, &language);
        let metrics = self.calculate_metrics(content);
        let suggestions = self.generate_suggestions(content, &language).await?;

        Ok(CodeAnalysis {
            file_path: file_path.clone(),
            issues,
            metrics,
            suggestions,
        })
    }

    fn detect_issues(&self, content: &str, language: &str) -> Vec<CodeIssue> {
        let mut issues = Vec::new();
        
        // Basic issue detection
        if content.contains("TODO") || content.contains("FIXME") {
            issues.push(CodeIssue {
                id: Uuid::new_v4().to_string(),
                issue_type: IssueType::Documentation,
                severity: IssueSeverity::Info,
                message: "TODO or FIXME comment found".to_string(),
                position: CodePosition {
                    file_path: PathBuf::new(),
                    line: 1,
                    column: 1,
                    end_line: None,
                    end_column: None,
                },
                fix_suggestion: Some("Complete the TODO or FIXME".to_string()),
            });
        }

        // Language-specific issue detection
        match language {
            "rust" => {
                if content.contains("unwrap()") {
                    issues.push(CodeIssue {
                        id: Uuid::new_v4().to_string(),
                        issue_type: IssueType::Style,
                        severity: IssueSeverity::Warning,
                        message: "Using unwrap() in production code".to_string(),
                        position: CodePosition {
                            file_path: PathBuf::new(),
                            line: 1,
                            column: 1,
                            end_line: None,
                            end_column: None,
                        },
                        fix_suggestion: Some("Use proper error handling with Result".to_string()),
                    });
                }
            }
            "javascript" | "typescript" => {
                if content.contains("var ") {
                    issues.push(CodeIssue {
                        id: Uuid::new_v4().to_string(),
                        issue_type: IssueType::Style,
                        severity: IssueSeverity::Warning,
                        message: "Using var instead of let/const".to_string(),
                        position: CodePosition {
                            file_path: PathBuf::new(),
                            line: 1,
                            column: 1,
                            end_line: None,
                            end_column: None,
                        },
                        fix_suggestion: Some("Use let or const instead of var".to_string()),
                    });
                }
            }
            _ => {}
        }

        issues
    }

    fn calculate_metrics(&self, content: &str) -> CodeMetrics {
        let lines_of_code = content.lines().count() as u32;
        let cyclomatic_complexity = self.calculate_cyclomatic_complexity(content);
        let maintainability_index = self.calculate_maintainability_index(content);
        
        CodeMetrics {
            lines_of_code,
            cyclomatic_complexity,
            maintainability_index,
            test_coverage: 0.0, // Would be calculated from test files
            documentation_coverage: 0.0, // Would be calculated from comments
        }
    }

    fn calculate_cyclomatic_complexity(&self, content: &str) -> u32 {
        let mut complexity = 1; // Base complexity
        
        for line in content.lines() {
            let line = line.trim();
            if line.contains("if ") || line.contains("while ") || line.contains("for ") || 
               line.contains("case ") || line.contains("catch ") || line.contains("&&") || 
               line.contains("||") || line.contains("?") {
                complexity += 1;
            }
        }
        
        complexity
    }

    fn calculate_maintainability_index(&self, content: &str) -> f32 {
        // Simplified maintainability index calculation
        let lines = content.lines().count() as f32;
        let comments = content.lines().filter(|line| line.trim().starts_with("//") || line.trim().starts_with("/*")).count() as f32;
        
        let comment_ratio = if lines > 0.0 { comments / lines } else { 0.0 };
        (comment_ratio * 100.0).min(100.0)
    }

    async fn generate_suggestions(&self, content: &str, language: &str) -> Result<Vec<CodeSuggestion>> {
        let mut suggestions = Vec::new();
        
        // Generate refactoring suggestions
        if content.len() > 1000 {
            suggestions.push(CodeSuggestion {
                id: Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::Refactoring,
                title: "Extract method".to_string(),
                description: "Consider extracting long functions into smaller methods".to_string(),
                code: "".to_string(),
                language: language.to_string(),
                confidence: 0.7,
                position: CodePosition {
                    file_path: PathBuf::new(),
                    line: 1,
                    column: 1,
                    end_line: None,
                    end_column: None,
                },
                metadata: SuggestionMetadata {
                    complexity: ComplexityLevel::Medium,
                    estimated_time: 15,
                    dependencies: vec![],
                    tags: vec!["refactoring".to_string(), "maintainability".to_string()],
                    examples: vec![],
                },
            });
        }

        Ok(suggestions)
    }

    pub async fn apply_refactoring(&mut self, _suggestion_id: &str, file_path: &PathBuf, content: &str) -> Result<RefactoringOperation> {
        let operation_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        // Placeholder refactoring operation
        let operation = RefactoringOperation {
            id: operation_id.clone(),
            operation_type: RefactoringType::ExtractMethod,
            description: "Extract method refactoring".to_string(),
            original_code: content.to_string(),
            refactored_code: content.to_string(), // Would be the actual refactored code
            file_path: file_path.clone(),
            position: CodePosition {
                file_path: file_path.clone(),
                line: 1,
                column: 1,
                end_line: None,
                end_column: None,
            },
            timestamp: now,
            success: true,
            error_message: None,
        };

        self.refactoring_history.push(operation.clone());
        Ok(operation)
    }

    pub fn get_refactoring_history(&self) -> &Vec<RefactoringOperation> {
        &self.refactoring_history
    }

    pub fn clear_suggestions_cache(&mut self) {
        self.suggestions_cache.clear();
    }

    fn detect_language(&self, file_path: &PathBuf) -> String {
        if let Some(extension) = file_path.extension().and_then(|ext| ext.to_str()) {
            match extension {
                "rs" => "rust".to_string(),
                "js" => "javascript".to_string(),
                "ts" => "typescript".to_string(),
                "tsx" => "typescript".to_string(),
                "jsx" => "javascript".to_string(),
                "py" => "python".to_string(),
                "html" => "html".to_string(),
                "css" => "css".to_string(),
                "json" => "json".to_string(),
                _ => "text".to_string(),
            }
        } else {
            "text".to_string()
        }
    }
}