/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

//! Utilities for reporting errors to an LSP client
use crate::lsp::{
    publish_diagnostic, set_ready_status, set_running_status, url_from_location, Diagnostic,
    DiagnosticSeverity, Position, PublishDiagnosticsParams, Range, Url,
};
use common::Diagnostic as CompilerDiagnostic;
use crossbeam::crossbeam_channel::Sender;
use lsp_server::Message;
use relay_compiler::{
    errors::{BuildProjectError, Error, Result},
    source_for_location,
    status_reporter::StatusReporter,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct LSPStatusReporter {
    active_diagnostics: Arc<RwLock<HashMap<Url, Vec<Diagnostic>>>>,
    sender: Sender<Message>,
    root_dir: PathBuf,
}

impl LSPStatusReporter {
    pub fn new(root_dir: PathBuf, sender: Sender<Message>) -> Self {
        Self {
            active_diagnostics: Default::default(),
            sender,
            root_dir,
        }
    }

    fn add_diagnostic(&self, url: Url, diagnostic: Diagnostic) {
        self.active_diagnostics
            .write()
            .unwrap()
            .entry(url)
            .or_default()
            .push(diagnostic);
    }

    fn commit_diagnostics(&self) {
        for (url, diagnostics) in self.active_diagnostics.read().unwrap().iter() {
            let params = PublishDiagnosticsParams {
                diagnostics: diagnostics.clone(),
                uri: url.clone(),
                version: None,
            };
            publish_diagnostic(params, &self.sender).ok();
        }
    }

    fn report_diagnostic(&self, diagnostic: &CompilerDiagnostic) {
        let message = diagnostic.message().to_string();

        let location = diagnostic.location();

        let url = match url_from_location(location, &self.root_dir) {
            Some(url) => url,
            None => {
                // TODO(brandondail) we should always be able to parse as a Url, so log here when we don't
                if let Ok(root) = Url::from_directory_path(&self.root_dir) {
                    root
                } else {
                    return;
                }
            }
        };

        let source =
            if let Some(source) = source_for_location(&self.root_dir, location.source_location()) {
                source
            } else {
                return;
            };
        let range = location
            .span()
            .to_range(&source.text, source.line_index, source.column_index);

        let diagnostic = Diagnostic {
            code: None,
            message,
            range,
            related_information: None,
            severity: Some(DiagnosticSeverity::Error),
            source: None,
            tags: None,
        };
        self.add_diagnostic(url, diagnostic);
    }

    fn report_error(&self, error: &Error) {
        match error {
            Error::DiagnosticsError { errors } => {
                for diagnostic in errors {
                    self.report_diagnostic(diagnostic);
                }
            }
            Error::BuildProjectsErrors { errors } => {
                for error in errors {
                    self.print_project_error(error);
                }
            }
            Error::Cancelled => {
                // Ignore the cancellation
            }
            error => {
                self.print_generic_error(format!("{}", error));
            }
        }
    }

    fn print_project_error(&self, error: &BuildProjectError) {
        match error {
            BuildProjectError::ValidationErrors { errors } => {
                for diagnostic in errors {
                    self.report_diagnostic(diagnostic);
                }
            }
            BuildProjectError::PersistErrors { errors } => {
                for error in errors {
                    self.print_generic_error(format!("{}", error));
                }
            }
            _ => {
                self.print_generic_error(format!("{}", error));
            }
        }
    }

    fn print_generic_error(&self, message: String) {
        let diagnostic = Diagnostic {
            code: None,
            message,
            range: Range::new(Position::new(0, 0), Position::new(0, 0)),
            related_information: None,
            severity: Some(DiagnosticSeverity::Error),
            source: None,
            tags: None,
        };
        let url = Url::from_directory_path(&self.root_dir).unwrap();
        self.add_diagnostic(url, diagnostic);
    }
}

impl StatusReporter for LSPStatusReporter {
    fn build_starts(&self) {
        set_running_status(&self.sender);
    }

    fn build_finishes(&self, result: &Result<()>) {
        set_ready_status(&self.sender);
        {
            let mut active_diagnostics = self.active_diagnostics.write().unwrap();
            for diagnostics in active_diagnostics.values_mut() {
                diagnostics.clear();
            }
        }
        if let Err(error) = result {
            self.report_error(error);
        }
        self.commit_diagnostics();
    }
}
