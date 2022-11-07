use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub platform: Platform,
    pub sdk: SDK,
    pub build_type: BuildType,
    pub code: Option<String>,
    pub info: Option<String>,
}

impl State {
    pub fn clear_text(&mut self) {
        self.info = Some(String::new());
        self.code = Some(String::new());
    }
}

#[derive(
    Clone, Copy, Debug, EnumIter, EnumString, Display, PartialEq, Serialize, Deserialize, Eq,
)]
pub enum Platform {
    #[strum(serialize = "GitHub Actions")]
    GitHub,
}

#[derive(
    Clone, Copy, Debug, EnumIter, EnumString, Display, PartialEq, Serialize, Deserialize, Eq,
)]
pub enum SDK {
    #[strum(serialize = "Native App")]
    Native,
    #[strum(serialize = "Flutter")]
    Flutter,
    #[strum(serialize = "React Native")]
    ReactNative,
}

#[derive(
    Clone, Copy, Debug, EnumIter, EnumString, Display, PartialEq, Serialize, Deserialize, Eq,
)]
pub enum BuildType {
    #[strum(serialize = "Debug (unsigned)")]
    Unsigned,
    #[strum(serialize = "Release (signed)")]
    Signed,
}
