use askama::Template;
use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub platform: Platform,
    pub sdk: SDK,
    pub build_type: BuildType,
    pub custom_inputs: CustomInputs,
    pub code_template: Option<String>,
    pub info_template: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomInputs {
    pub build_variant_name: Option<String>,
    pub build_variant_path: Option<String>,
    pub publishing_format: PublishingFormat,
}

impl State {
    pub fn clear_text(&mut self) {
        self.info_template = None;
        self.code_template = Some(String::new());
    }

    pub fn gen_templates(&mut self) {
        let (code_template, info_template) = match (self.platform, self.sdk, self.build_type) {
            (Platform::GitHub, SDK::Native, BuildType::Signed) => {
                let code_template = GithubNativeSigned {
                    title: "Android release build",
                    publishing_format: &self.custom_inputs.publishing_format.to_owned(),
                    build_variant_name: &self
                        .custom_inputs
                        .build_variant_name
                        .to_owned()
                        .unwrap_or_default(),
                    build_variant_path: &self
                        .custom_inputs
                        .build_variant_path
                        .to_owned()
                        .unwrap_or_default(),
                };

                let info_template = GithubNativeSignedInfo {};

                (
                    Some(code_template.render().unwrap()),
                    Some(info_template.render().unwrap()),
                )
            }

            (Platform::GitHub, SDK::Flutter, BuildType::Signed) => {
                let code_template = GithubFlutterSigned {
                    title: "Flutter Android release build",
                    publishing_format: &self.custom_inputs.publishing_format.to_owned(),
                    build_variant_path: &self
                        .custom_inputs
                        .build_variant_path
                        .to_owned()
                        .unwrap_or_default(),
                };

                let info_template = GithubFlutterSignedInfo {};

                (
                    Some(code_template.render().unwrap()),
                    Some(info_template.render().unwrap()),
                )
            }

            (Platform::GitHub, SDK::ReactNative, BuildType::Signed) => {
                let code_template = GithubReactNativeSigned {
                    title: "React Native Android release build",
                    publishing_format: &self.custom_inputs.publishing_format.to_owned(),
                    build_variant_name: &self
                        .custom_inputs
                        .build_variant_name
                        .to_owned()
                        .unwrap_or_default(),
                    build_variant_path: &self
                        .custom_inputs
                        .build_variant_path
                        .to_owned()
                        .unwrap_or_default(),
                };

                let info_template = GithubReactNativeSignedInfo {};

                (
                    Some(code_template.render().unwrap()),
                    Some(info_template.render().unwrap()),
                )
            }

            (Platform::GitHub, SDK::Native, BuildType::Unsigned) => {
                let code_template = GithubNativeUnsigned {
                    title: "Android debug build",
                    publishing_format: &self.custom_inputs.publishing_format.to_owned(),
                    build_variant_name: &self
                        .custom_inputs
                        .build_variant_name
                        .to_owned()
                        .unwrap_or_default(),
                    build_variant_path: &self
                        .custom_inputs
                        .build_variant_path
                        .to_owned()
                        .unwrap_or_default(),
                };

                (Some(code_template.render().unwrap()), None)
            }

            (Platform::GitHub, SDK::Flutter, BuildType::Unsigned) => {
                let code_template = GithubFlutterUnsigned {
                    title: "Flutter Android debug build",
                    publishing_format: &self.custom_inputs.publishing_format.to_owned(),
                    build_variant_path: &self
                        .custom_inputs
                        .build_variant_path
                        .to_owned()
                        .unwrap_or_default(),
                };

                (Some(code_template.render().unwrap()), None)
            }

            (Platform::GitHub, SDK::ReactNative, BuildType::Unsigned) => {
                let code_template = GithubReactNativeUnsigned {
                    title: "React Native Android debug build",
                    publishing_format: &self.custom_inputs.publishing_format.to_owned(),
                    build_variant_name: &self
                        .custom_inputs
                        .build_variant_name
                        .to_owned()
                        .unwrap_or_default(),
                    build_variant_path: &self
                        .custom_inputs
                        .build_variant_path
                        .to_owned()
                        .unwrap_or_default(),
                };

                (Some(code_template.render().unwrap()), None)
            }
        };

        self.code_template = code_template;
        self.info_template = info_template
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

#[derive(
    Clone, Copy, Debug, EnumIter, EnumString, Display, PartialEq, Serialize, Deserialize, Eq,
)]
pub enum PublishingFormat {
    #[strum(serialize = "APK")]
    Apk,
    #[strum(serialize = "AAB")]
    Aab,
}

#[derive(Template)]
#[template(path = "workflows/github-native-signed")]
struct GithubNativeSigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    build_variant_name: &'a str,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-flutter-signed")]
struct GithubFlutterSigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-react-native-signed")]
struct GithubReactNativeSigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    build_variant_name: &'a str,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-native-unsigned")]
struct GithubNativeUnsigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    build_variant_name: &'a str,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-flutter-unsigned")]
struct GithubFlutterUnsigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-react-native-unsigned")]
struct GithubReactNativeUnsigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    build_variant_name: &'a str,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "info/github-native-signed")]
struct GithubNativeSignedInfo {}

#[derive(Template)]
#[template(path = "info/github-flutter-signed")]
struct GithubFlutterSignedInfo {}

#[derive(Template)]
#[template(path = "info/github-react-native-signed")]
struct GithubReactNativeSignedInfo {}
