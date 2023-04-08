use serde_json::Value;
use tower_lsp::{
    jsonrpc::{Error, Result},
    lsp_types::Url,
};

use crate::workspace::Workspace;

use super::TypstServer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LspCommand {
    ExportPdf,
}

impl From<LspCommand> for String {
    fn from(command: LspCommand) -> Self {
        match command {
            LspCommand::ExportPdf => "typst-lsp.doPdfExport".to_string(),
        }
    }
}

impl LspCommand {
    pub fn parse(command: &str) -> Option<Self> {
        match command {
            "typst-lsp.doPdfExport" => Some(Self::ExportPdf),
            _ => None,
        }
    }

    pub fn all_as_string() -> Vec<String> {
        vec![Self::ExportPdf.into()]
    }
}

/// Here are implemented the handlers for each command.
impl TypstServer {
    /// Export the current document as a PDF file. The client is responsible for passing the correct file URI.
    pub async fn command_export_pdf(
        &self,
        workspace: &Workspace,
        arguments: Vec<Value>,
    ) -> Result<()> {
        if arguments.is_empty() {
            return Err(Error::invalid_params("Missing file URI argument"));
        }
        let Some(file_uri) = arguments.first().and_then(|v| v.as_str()) else {
            return Err(Error::invalid_params(
                "Missing file URI as first argument",
            ));
        };
        let file_uri = Url::parse(file_uri)
            .map_err(|_| Error::invalid_params("Parameter is not a valid URI"))?;

        let source = workspace
            .sources
            .get_source_by_uri(&file_uri)
            .ok_or_else(|| {
                Error::invalid_params(format!(
                    "URI {file_uri} does not refer to a known source file"
                ))
            })?;

        self.run_export(workspace, source).await;

        Ok(())
    }
}