use {
    crate::GitHubToken,
    std::{io, process::Command},
    thiserror::Error,
};

pub fn token_for(account: &str) -> Result<GitHubToken, Refusal> {
    token_for_from(&InstalledGitHubCli, account)
}

#[derive(Debug, Error)]
pub enum Refusal {
    #[error("the GitHub CLI (gh) is not on the path")]
    ToolAbsent,

    #[error("the GitHub CLI holds no account named {account}")]
    AccountUnheld { account: String },

    #[error("the GitHub CLI could not produce a token for {account}: {reason}")]
    Failed { account: String, reason: String },
}

struct ToolOutput {
    succeeded: bool,
    standard_output: Vec<u8>,
}

trait GitHubCli {
    fn run(&self, arguments: &[&str]) -> io::Result<ToolOutput>;
}

struct InstalledGitHubCli;

impl GitHubCli for InstalledGitHubCli {
    fn run(&self, arguments: &[&str]) -> io::Result<ToolOutput> {
        let output = Command::new("gh").args(arguments).output()?;
        Ok(ToolOutput {
            succeeded: output.status.success(),
            standard_output: output.stdout,
        })
    }
}

fn token_for_from(tool: &impl GitHubCli, account: &str) -> Result<GitHubToken, Refusal> {
    let failed = |reason: String| Refusal::Failed {
        account: account.to_owned(),
        reason,
    };

    let output = match tool.run(&["auth", "token", "--user", account]) {
        Ok(output) => output,
        Err(absent) if absent.kind() == io::ErrorKind::NotFound => return Err(Refusal::ToolAbsent),
        Err(unreachable) => return Err(failed(unreachable.to_string())),
    };

    if !output.succeeded {
        return Err(Refusal::AccountUnheld {
            account: account.to_owned(),
        });
    }

    let text = String::from_utf8(output.standard_output)
        .map_err(|not_text| failed(format!("the token it wrote is not valid UTF-8: {not_text}")))?;

    GitHubToken::try_from(text.trim().to_owned()).map_err(|empty| failed(empty.to_string()))
}

#[cfg(test)]
mod tests {
    use {super::*, secrecy::ExposeSecret, std::cell::RefCell};

    struct StubbedGitHubCli {
        answer: RefCell<Option<io::Result<ToolOutput>>>,
        arguments: RefCell<Vec<String>>,
    }

    impl StubbedGitHubCli {
        fn answering(answer: io::Result<ToolOutput>) -> Self {
            Self {
                answer: RefCell::new(Some(answer)),
                arguments: RefCell::new(Vec::new()),
            }
        }

        fn writing(standard_output: &[u8]) -> Self {
            Self::answering(Ok(ToolOutput {
                succeeded: true,
                standard_output: standard_output.to_vec(),
            }))
        }

        fn refusing() -> Self {
            Self::answering(Ok(ToolOutput {
                succeeded: false,
                standard_output: Vec::new(),
            }))
        }

        fn arguments(&self) -> Vec<String> {
            self.arguments.borrow().clone()
        }
    }

    impl GitHubCli for StubbedGitHubCli {
        fn run(&self, arguments: &[&str]) -> io::Result<ToolOutput> {
            *self.arguments.borrow_mut() = arguments
                .iter()
                .map(|argument| (*argument).to_owned())
                .collect();
            self.answer
                .borrow_mut()
                .take()
                .expect("the tool is run exactly once per request")
        }
    }

    #[test]
    fn a_token_is_obtained_for_the_account_the_request_names() {
        let tool = StubbedGitHubCli::writing(b"a-credential\n");

        let token = token_for_from(&tool, "Alice").expect("a token is produced");

        assert_eq!(token.secret().expose_secret(), "a-credential");
    }

    #[test]
    fn the_account_is_named_in_the_request_rather_than_switched_to() {
        let tool = StubbedGitHubCli::writing(b"a-credential");

        token_for_from(&tool, "Alice").expect("a token is produced");

        assert_eq!(tool.arguments(), ["auth", "token", "--user", "Alice"]);
    }

    #[test]
    fn a_tool_missing_from_the_machine_is_refused_as_absent() {
        let tool = StubbedGitHubCli::answering(Err(io::Error::from(io::ErrorKind::NotFound)));

        let refusal = token_for_from(&tool, "Alice").expect_err("no token is produced");

        assert_eq!(
            refusal.to_string(),
            "the GitHub CLI (gh) is not on the path"
        );
    }

    #[test]
    fn an_account_the_tool_cannot_answer_for_is_refused_by_name() {
        let tool = StubbedGitHubCli::refusing();

        let refusal = token_for_from(&tool, "Alice").expect_err("no token is produced");

        assert_eq!(
            refusal.to_string(),
            "the GitHub CLI holds no account named Alice"
        );
    }

    #[test]
    fn a_token_that_is_not_text_is_refused_rather_than_panicking() {
        let tool = StubbedGitHubCli::writing(&[0xff, 0xfe]);

        let refusal = token_for_from(&tool, "Alice").expect_err("no token is produced");

        assert!(
            refusal.to_string().contains("not valid UTF-8"),
            "expected the refusal to name the cause, got {refusal}"
        );
    }

    #[test]
    fn a_tool_that_succeeds_while_writing_nothing_produces_no_token() {
        let tool = StubbedGitHubCli::writing(b"\n");

        let refusal = token_for_from(&tool, "Alice").expect_err("no token is produced");

        assert!(
            refusal.to_string().contains("produced no token"),
            "expected the refusal to name the cause, got {refusal}"
        );
    }
}
