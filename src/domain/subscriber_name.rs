use crate::exception::biz_exception::BizError::ParamInvalid;
use crate::exception::biz_exception::BizResult;

const MAX_NAME_LEN: usize = 64;
const FORBIDDEN_CHARACTERS: [char; 10] = ['/', '(', ')', ',', '"', '<', '>', '\\', '{', '}'];

#[derive(Debug)]
pub struct SubscriberUserName(String);

impl SubscriberUserName {
    pub fn parse(s: String) -> BizResult<Self> {
        //校验订阅者的名字
        let is_empty_or_whitespace = s.trim().is_empty();
        if is_empty_or_whitespace {
            Err(ParamInvalid("姓名为空".to_string()))?
        }

        let is_too_long = s.chars().count() > MAX_NAME_LEN;

        if is_too_long {
            Err(ParamInvalid(format!("姓名超过{}个字符", MAX_NAME_LEN)))?
        }

        let contains_forbidden_characters = s.chars().any(|c| FORBIDDEN_CHARACTERS.contains(&c));
        if contains_forbidden_characters {
            Err(ParamInvalid("包含非法字符".to_string()))?
        }
        Ok(Self(s))
    }
}

impl AsRef<str> for SubscriberUserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::subscriber_name::{SubscriberUserName, FORBIDDEN_CHARACTERS, MAX_NAME_LEN};
    use claim::{assert_err, assert_ok};

    #[test]
    fn too_long_name_is_invalid() {
        let name = "a".repeat(MAX_NAME_LEN + 1);
        assert_err!(SubscriberUserName::parse(name));
    }

    #[test]
    fn lg_max_name_len_is_valid() {
        let name = "a".repeat(MAX_NAME_LEN);
        assert_ok!(SubscriberUserName::parse(name));
    }

    #[test]
    fn whitespace_only_name_is_invalid() {
        let name = " ".to_string();
        assert_err!(SubscriberUserName::parse(name));
    }

    #[test]
    fn empty_only_name_is_invalid() {
        let name = "".to_string();
        assert_err!(SubscriberUserName::parse(name));
    }
    #[test]
    fn contains_forbidden_characters_are_invalid() {
        for name in FORBIDDEN_CHARACTERS {
            assert_err!(SubscriberUserName::parse(name.to_string()));
        }
    }

    #[test]
    fn valid_name_is_ok() {
        let name = "Jason Gao".to_string();
        assert_ok!(SubscriberUserName::parse(name));
    }
}
