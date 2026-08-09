use {secrecy::SecretString, thiserror::Error};

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct GitHubToken(SecretString);

#[derive(Debug, Error, PartialEq, Eq)]
#[error("the source produced no token")]
pub struct EmptyToken;

impl TryFrom<String> for GitHubToken {
    type Error = EmptyToken;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(EmptyToken);
        }
        Ok(Self(SecretString::from(value)))
    }
}

impl GitHubToken {
    pub fn secret(&self) -> &SecretString {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use {super::*, secrecy::ExposeSecret};

    #[test]
    fn a_token_keeps_the_value_it_was_constructed_from() {
        let token =
            GitHubToken::try_from("a-credential".to_owned()).expect("a non-empty value is a token");

        assert_eq!(token.secret().expose_secret(), "a-credential");
    }

    #[test]
    fn an_empty_value_is_not_a_token() {
        assert_eq!(
            GitHubToken::try_from(String::new()).unwrap_err(),
            EmptyToken
        );
    }
}
