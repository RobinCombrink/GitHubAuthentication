use anyhow::{Context, Result};
use secrecy::SecretString;
use std::{io, process::Command};

pub trait Authentication {
    fn get_token(&self) -> SecretString;
    fn get_username(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct GitHubCliAuthentication {
    pub token: SecretString,
    pub username: String,
}

impl GitHubCliAuthentication {
    pub fn new(username: String) -> Result<Self> {
        Self::is_github_cli_on_path()?;

        let token = Self::get_github_token(&username)?;
        Ok(Self { token, username })
    }
    fn switch_github_cli_user(user: &str) -> Result<()> {
        let args = vec![
            "auth".into(),
            "switch".into(),
            "--user".into(),
            format!("{user}"),
        ];
        let shell_program = Self::get_github_cli_command();

        Command::new(shell_program)
            .args(&args)
            .output()
            .with_context(|| {
                format!(
                    "Something went wrong switching GitHub CLI user in the program {}",
                    shell_program,
                )
            })?;
        Ok(())
    }
    fn get_github_token(username: &str) -> Result<SecretString> {
        Self::switch_github_cli_user(username)?;
        let args = vec!["auth".to_string(), "token".to_string()];

        let shell_program = Self::get_github_cli_command();

        let output = Command::new(shell_program)
            .args(&args)
            .output()
            .with_context(|| {
                format!(
                    "Something went wrong getting GitHub token in the program {}",
                    shell_program,
                )
            })?;

        Ok(String::from_utf8(output.stdout)
            .expect("Uf8 only for standard out")
            .trim()
            .to_owned()
            .into())
    }
    fn get_github_cli_command() -> &'static str {
        return "gh";
    }
    pub fn is_github_cli_on_path() -> Result<bool> {
        match Command::new("gh").output() {
            Ok(_) => Ok(true),
            Err(e) => {
                if let io::ErrorKind::NotFound = e.kind() {
                    Ok(false)
                } else {
                    Err(e).with_context(||format!("An unknown error has occured while checking if the `gh` command was available"))
                }
            }
        }
    }
}

impl Authentication for GitHubCliAuthentication {
    fn get_token(&self) -> SecretString {
        self.token.clone()
    }

    fn get_username(&self) -> String {
        self.username.clone()
    }
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;

    use super::*;

    #[test]
    fn gets_token_when_cli_installed() {
        let authentication = GitHubCliAuthentication::new("RobinCombrink".to_owned());
        assert!(
            authentication.is_ok_and(|client| { client.token.expose_secret().starts_with("gh") }),
        )
    }
}
