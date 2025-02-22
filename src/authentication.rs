use anyhow::{anyhow, Context, Result};
use secrecy::SecretString;
use std::{process::Command, sync::LazyLock};

pub trait Authentication {
    fn get_token(&self) -> Result<SecretString>;
    fn get_username(&self) -> String;
}

pub struct GitHubCliAuthentication {
    pub token: LazyLock<Result<SecretString>>,
    pub username: String,
}

impl GitHubCliAuthentication {
    pub fn new(username: String) -> Self {
        Self {
            token: LazyLock::new(|| Self::get_github_token()),
            username,
        }
    }
    pub fn switch_github_cli_user(user: &str) -> Result<()> {
        let args = vec![
            "gh".into(),
            "auth".into(),
            "switch".into(),
            "--user".into(),
            format!("{user}"),
        ];
        let shell_program = Self::get_shell_program();

        Command::new(shell_program)
            .args(&args)
            .output()
            .with_context(|| {
                format!(
                    "Something went wrong executing the command: {:#?} in the program {}",
                    args, shell_program
                )
            })?;
        Ok(())
    }

    fn get_github_token() -> Result<SecretString> {
        let args = vec![
            "/C".into(),
            "gh".to_string(),
            "auth".to_string(),
            "token".to_string(),
        ];

        let shell_program = Self::get_shell_program();

        let output = Command::new(shell_program)
            .args(&args)
            .output()
            .with_context(|| {
                format!(
                    "Something went wrong executing the command: {:#?} in the program {}",
                    args, shell_program
                )
            })?;

        Ok(String::from_utf8(output.stdout)
            .expect("Uf8 only for standard out")
            .trim()
            .to_owned()
            .into())
    }
    fn get_shell_program() -> &'static str {
        return "cmd";
    }
}

impl Authentication for GitHubCliAuthentication {
    fn get_token(&self) -> Result<SecretString> {
        let token_ref = self.token.as_ref();

        match token_ref {
            Ok(token) => Ok(token.clone()), // Assuming SecretString implements Clone
            Err(err) => Err(anyhow!("Could not get token")).with_context(|| format!("{:#?}", err)), // Clone the context, not the error
        }
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
            authentication
                .token
                .as_ref()
                .is_ok_and(|token| { token.expose_secret().starts_with("gh") }),
            "Failed token: {}",
            &authentication.token.as_ref().unwrap().expose_secret()[0..6]
        )
    }
}
