// SPDX-License-Identifier: Apache-2.0
//! SigmaOS--Dante — SigmaOS -Dante component inspired by Linux Mint / Omarchy / Arch Linux.
//!
//! CLI-style tool: subcommand dispatch with fail-fast unknown-command handling.
//! Principles: single responsibility, fail-fast, defensive programming,
//! component isolation, stable API, deterministic tests.

#![forbid(unsafe_code)]

use std::fmt;
/// Errors for `ToolError` operations. Fail-fast: invalid input is rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolError {
    /// Tool name must not be empty
    EmptyName,
    /// Unrecognized subcommand
    UnknownCommand(String),
    /// Subcommand requires arguments
    MissingArgs(String),
}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "tool name must not be empty"),
            Self::UnknownCommand(x) => write!(f, "unrecognized subcommand: {x}"),
            Self::MissingArgs(x) => write!(f, "subcommand requires arguments: {x}"),
        }
    }
}

impl std::error::Error for ToolError {}

/// A command-line tool with a small fixed subcommand set.
#[derive(Debug)]
pub struct Tool {
    name: String,
}

impl Tool {
    /// Create the tool; fails fast on empty name.
    pub fn new(name: impl Into<String>) -> Result<Self, ToolError> {
        let name = name.into();
        if name.trim().is_empty() { return Err(ToolError::EmptyName); }
        Ok(Self { name })
    }

    /// Tool name.
    pub fn name(&self) -> &str { &self.name }

    /// Dispatch a subcommand. Supported: `status`, `version`, `help`.
    /// Commands needing args that get none fail fast.
    pub fn run(&self, args: &[&str]) -> Result<String, ToolError> {
        match args.first().copied().unwrap_or("help") {
            "status" => Ok(format!("{}: ready", self.name)),
            "version" => Ok(format!("{}: 0.1.0", self.name)),
            "help" => Ok(format!("{}: commands: status, version, help", self.name)),
            other => Err(ToolError::UnknownCommand(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test: status_and_version
    /// Arrange: Create a Tool instance with name "demo"
    /// Act: Run "status" and "version" subcommands
    /// Assert: Verify expected output format
    #[test]
    fn status_and_version() {
        // Arrange
        let t = Tool::new("demo").unwrap();

        // Act
        let status_result = t.run(&["status"]);
        let version_result = t.run(&["version"]);

        // Assert
        assert_eq!(status_result.unwrap(), "demo: ready");
        assert!(version_result.unwrap().contains("0.1.0"));
    }

    /// Test: no_args_shows_help
    /// Arrange: Create a Tool instance with name "demo"
    /// Act: Run with no arguments (should show help)
    /// Assert: Help message is displayed
    #[test]
    fn no_args_shows_help() {
        // Arrange
        let t = Tool::new("demo").unwrap();

        // Act
        let help_output = t.run(&[]);

        // Assert
        assert!(help_output.unwrap().contains("commands"));
    }

    /// Test: unknown_command_fails
    /// Arrange: Create a Tool instance with name "demo"
    /// Act: Run with unrecognized subcommand "bogus"
    /// Assert: Returns ToolError::UnknownCommand
    #[test]
    fn unknown_command_fails() {
        // Arrange
        let t = Tool::new("demo").unwrap();

        // Act
        let result = t.run(&["bogus"]);

        // Assert
        assert!(matches!(result, Err(ToolError::UnknownCommand(_))));
    }

    /// Test: empty_name_fails
    /// Arrange: Test Tool::new with empty/whitespace name
    /// Act: Call Tool::new with " " (space)
    /// Assert: Returns error (EmptyName)
    #[test]
    fn empty_name_fails() {
        // Arrange & Act & Assert
        assert!(Tool::new(" ").is_err());
    }
}

/// Backwards-compatible component handle (kept for existing callers/tests).
/// Follows Single Responsibility Principle: only holds component name state.
#[derive(Debug, Default)]
pub struct Component {
    name: &'static str,
}

impl Component {
    /// Create the component handle.
    /// Fail-fast: always succeeds as name has default value.
    pub fn new() -> Self {
        Self { name: "SigmaOS Component" }
    }

    /// Get the component name for inspection/verification.
    pub fn name(&self) -> &str {
        &self.name
    }
}