pub mod device;
pub mod login;
pub mod user;

pub mod utils {
    use lazy_static::lazy_static;
    use regex::Regex;
    use serde::de::{Error, Visitor};
    use serde::{Deserialize, Deserializer};
    use std::fmt::Formatter;
    use std::ops::Deref;

    #[derive(Clone, Debug)]
    pub struct ValidString(String);
    impl Deref for ValidString {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[allow(clippy::from_over_into)]
    impl Into<String> for ValidString {
        fn into(self) -> String {
            self.0
        }
    }

    impl<'de> Deserialize<'de> for ValidString {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer
                .deserialize_string(StringVisitor::new(vec![
                    Box::new(not_empty),
                    Box::new(malformed),
                    Box::new(len),
                ]))
                .map(ValidString)
        }
    }

    #[derive(Clone, Debug)]
    pub struct Username(String);
    impl Deref for Username {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[allow(clippy::from_over_into)]
    impl Into<String> for Username {
        fn into(self) -> String {
            self.0
        }
    }

    impl<'de> Deserialize<'de> for Username {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer
                .deserialize_string(StringVisitor::new(vec![
                    Box::new(not_empty),
                    Box::new(malformed),
                    Box::new(len),
                    Box::new(|str| {
                        if str.contains(" ") {
                            Err("username contains space")
                        } else {
                            Ok(())
                        }
                    }),
                ]))
                .map(Username)
        }
    }

    #[derive(Clone)]
    pub struct Email(String);
    impl Deref for Email {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[allow(clippy::from_over_into)]
    impl Into<String> for Email {
        fn into(self) -> String {
            self.0
        }
    }

    impl<'de> Deserialize<'de> for Email {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer
                .deserialize_string(StringVisitor::new(vec![Box::new(|str| {
                    if !EMAIL_REGEX.is_match(str) {
                        Err("invalid email")
                    } else {
                        Ok(())
                    }
                })]))
                .map(Email)
        }
    }

    #[derive(Clone)]
    pub struct Phone(String);
    impl Deref for Phone {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[allow(clippy::from_over_into)]
    impl Into<String> for Phone {
        fn into(self) -> String {
            self.0
        }
    }

    impl<'de> Deserialize<'de> for Phone {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer
                .deserialize_string(StringVisitor::new(vec![Box::new(|str| {
                    if !PHONE_REGEX.is_match(str) {
                        Err("invalid phone")
                    } else {
                        Ok(())
                    }
                })]))
                .map(Phone)
        }
    }

    type CheckFunc = Box<dyn Fn(&str) -> Result<(), &'static str>>;

    lazy_static! {
        static ref EMAIL_REGEX: Regex = Regex::new(
    r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
    )
    .unwrap();

        static ref PHONE_REGEX: Regex = Regex::new(
    r#"0[67][\s.]?\d{2}[\s.]?\d{2}[\s.]?\d{2}[\s.]?\d{2}|\+33[\s.]?[67][\s.]?\d{2}[\s.]?\d{2}[\s.]?\d{2}[\s.]?\d{2}"#,
    )
    .unwrap();
    }

    fn not_empty(str: &str) -> Result<(), &'static str> {
        if str.is_empty() {
            Err("empty string")
        } else {
            Ok(())
        }
    }

    fn malformed(str: &str) -> Result<(), &'static str> {
        if str.starts_with(" ") {
            Err("malformed string")
        } else {
            Ok(())
        }
    }

    fn len(str: &str) -> Result<(), &'static str> {
        if str.len() > 128 {
            Err("string too long")
        } else {
            Ok(())
        }
    }

    struct StringVisitor {
        rules: Vec<CheckFunc>,
    }

    impl StringVisitor {
        fn new(rules: Vec<CheckFunc>) -> Self {
            Self { rules }
        }
    }

    impl<'de> Visitor<'de> for StringVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a valid string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if let Some(error) = self.rules.into_iter().find_map(|f| f(v).err()) {
                Err(E::custom(error))
            } else {
                Ok(v.to_string())
            }
        }
    }
}
