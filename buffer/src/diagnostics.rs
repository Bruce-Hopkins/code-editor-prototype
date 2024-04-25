// use rustc_hash::FxHashMap;
// use lsp_types::DiagnosticSeverity;

// use crate::selection::Range;

// struct Diagnostics(FxHashMap<String, Diagnostic>);

// struct Diagnostic{
//     document_url: String,
//     issue: DiagnosticIssue
// }

// struct DiagnosticIssue {
//     range: Range,
//     code_description: String,
//     message: String,
//     severity: Severity
// }

// #[derive(Debug, Clone)]
// pub enum Severity {
//     Error,
//     Warning,
//     Info,
//     Hint
// }

// impl From<DiagnosticSeverity> for Severity {
//     fn from(value: DiagnosticSeverity) -> Self {
//         match value {
//             DiagnosticSeverity::ERROR => Self::Error,
//             DiagnosticSeverity::WARNING => Self::Warning,
//             DiagnosticSeverity::INFORMATION => Self::Info,
//             DiagnosticSeverity::HINT => Self::Hint,
//             _ => panic!("Unknown Severity")
//         }
//     }
// }