use iced::Color;
use lsp_types::DiagnosticSeverity;
use serde_json::Value;

use crate::core::{position::Position, selection::Range};

#[derive(Debug, Clone)]
pub enum LspResponse {
    Diagnostics(ClientDiagnostics),
    Progress,
    NoMessage,
    ErrorMessage(String),
    UnknownMessage,
    Initialized,
}

#[derive(Debug, Clone)]
pub struct ClientDiagnostics {
    pub issues: Vec<Issue>,
    pub uri: String
}

impl ClientDiagnostics {
    pub fn diagnostic_in_position(&self, position: Position) -> Option<Issue> {
        
        self.issues.clone().into_iter().find(|value| value.range.pos_in_range(position))
    }
}

#[derive(Debug, Clone)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint
}

impl Severity {
    pub fn color(&self) -> Color {
        match self {
            Severity::Error => Color::from_rgba8(239, 48, 84, 0.9),
            Severity::Warning => Color::from_rgba8(245, 230, 99, 0.9),
            Severity::Info => Color::from_rgba8(71, 168, 189, 0.9),
            Severity::Hint => Color::from_rgba8(71, 168, 189, 0.9),
        }
    }
}

impl From<DiagnosticSeverity> for Severity {
    fn from(value: DiagnosticSeverity) -> Self {
        match value {
            DiagnosticSeverity::ERROR => Self::Error,
            DiagnosticSeverity::WARNING => Self::Warning,
            DiagnosticSeverity::INFORMATION => Self::Info,
            DiagnosticSeverity::HINT => Self::Hint,
            _ => panic!("Unknown Severity")
        }
    }
}

impl LspResponse {
    pub fn from_response(method: &str, json: &Value) -> Self {
        match method {
            "textDocument/publishDiagnostics" => {
                let params = json.get("params").unwrap();
                let params: lsp_types::PublishDiagnosticsParams =
                    serde_json::from_value(params.clone()).unwrap();

                let issues: Vec<Issue> = params
                    .diagnostics
                    .into_iter()
                    .map(Issue::from)
                    .collect();
                LspResponse::Diagnostics(ClientDiagnostics {
                    issues,
                    uri: params.uri.to_string()
                })
            }
            "$/progress" => LspResponse::Progress,
            _ => LspResponse::UnknownMessage,
        }
    }

}

/**
 * A warning or error from the LSP server.
 */
#[derive(Debug, Clone)]
pub struct Issue {
    pub range: Range,
    pub code_description: Option<lsp_types::CodeDescription>,
    pub message: String,
    pub severity: Severity
}


impl From<lsp_types::Diagnostic> for Issue {
    fn from(value: lsp_types::Diagnostic) -> Self {
        let serverity = value.severity.unwrap_or(DiagnosticSeverity::ERROR);
        let severity = Severity::from(serverity);

        Self {
            range: Range::from(value.range),
            code_description: value.code_description,
            message: value.message,
            severity
        }
    }
}
