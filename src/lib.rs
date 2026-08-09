//! Obtains a credential to act as a named GitHub account, without changing anything about the
//! machine it asks.
//!
//! ```no_run
//! let _token = github_authentication::cli::token_for("Alice")?;
//! # Ok::<(), github_authentication::cli::Refusal>(())
//! ```

pub mod cli;

mod token;

pub use token::{EmptyToken, GitHubToken};
