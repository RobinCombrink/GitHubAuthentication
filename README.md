# GitHubAuthentication

Reusable Rust crate for GitHub authentication via the GitHub CLI.

## What it does

Provides a trait-based authentication abstraction that retrieves GitHub tokens by delegating to the locally installed GitHub CLI (`gh`). Supports multi-account switching and wraps tokens in `secrecy::SecretString` to prevent accidental logging.

## Usage

```rust
use github_authentication::authentication::{Authentication, GitHubCliAuthentication};

let auth = GitHubCliAuthentication::new("username".to_string())?;
let token = auth.get_token();
```

Requires `gh` CLI installed and authenticated.

## Design Decisions

- **Delegates to `gh` CLI rather than implementing OAuth flows**: Avoids managing client secrets and refresh tokens. Users already have `gh` authenticated locally.
- **`secrecy::SecretString` for tokens**: Prevents tokens from appearing in debug output or logs.
- **Trait-based**: `Authentication` trait allows swapping implementations in tests or for different auth providers.
