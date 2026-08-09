# GitHubAuthentication

Obtains a credential to act as a named GitHub account, from whichever source holds one.

## What it does

Asks a credential source for a token valid to act as one named account, and answers with the
token or with why it could not. Obtaining a credential changes nothing about the machine: the
account is named on each request, so whichever account a source is otherwise pointed at is
neither read nor moved.

The GitHub command-line tool is the only source today. Further sources — a minted installation
token, a device flow — land beside it as their own functions rather than beneath a shared trait,
because they are asked in different terms and fail in different ways.

## Usage

```rust
use github_authentication::cli;

let token = cli::token_for("Alice")?;
```

Two refusals are named because a caller acts differently on each: `Refusal::ToolAbsent` says the
tool is not on the machine, and `Refusal::AccountUnheld` says it cannot answer for that account.
Every other cause arrives as `Refusal::Failed` carrying its reason. What a person should do about
either is the calling program's to say, not this crate's.

A token is a transparent wrapper over a `secrecy::SecretString` that cannot be constructed
empty, and carries no expiry.

## Design decisions

Recorded in [`docs/adr`](./docs/adr). The vocabulary is in [`CONTEXT.md`](./CONTEXT.md).
