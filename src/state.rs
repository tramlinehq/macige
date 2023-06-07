use askama::Template;
use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub app_platform: AppPlatform,
    pub platform: Platform,
    pub sdk: Sdk,
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
    pub show_versions: bool,
}

impl State {
    pub fn clear_text(&mut self) {
        self.info_template = None;
        self.code_template = Some(String::new());
    }

    pub fn gen_templates(&mut self) {
        let (code_template, info_template) = match (self.platform, self.sdk, self.build_type) {
            (Platform::GitHub, Sdk::Native, BuildType::Signed) => {
                let code_template = self.github_native_signed();
                let info_template = self.github_native_signed_info();
                (Some(code_template), Some(info_template))
            }
            (Platform::GitHub, Sdk::Flutter, BuildType::Signed) => {
                let code_template = self.github_flutter_signed();
                let info_template = self.github_flutter_signed_info();
                (Some(code_template), Some(info_template))
            }
            (Platform::GitHub, Sdk::ReactNative, BuildType::Signed) => {
                let code_template = self.github_react_native_signed();
                let info_template = self.github_react_native_signed_info();
                (Some(code_template), Some(info_template))
            }
            (Platform::GitHub, Sdk::Native, BuildType::Unsigned) => {
                let code_template = self.github_native_unsigned();
                (Some(code_template), None)
            }
            (Platform::GitHub, Sdk::Flutter, BuildType::Unsigned) => {
                let code_template = self.github_flutter_unsigned();
                (Some(code_template), None)
            }
            (Platform::GitHub, Sdk::ReactNative, BuildType::Unsigned) => {
                let code_template = self.github_react_native_unsigned();
                (Some(code_template), None)
            }
        };

        self.code_template = code_template;
        self.info_template = info_template;
    }

    fn github_native_signed_info(&self) -> String {
        GithubNativeSignedInfo {
            show_versions: &self.custom_inputs.show_versions.to_owned(),
        }
        .render()
        .unwrap()
    }

    fn github_flutter_signed_info(&self) -> String {
        GithubFlutterSignedInfo {
            show_versions: &self.custom_inputs.show_versions.to_owned(),
        }
        .render()
        .unwrap()
    }

    fn github_react_native_signed_info(&self) -> String {
        GithubReactNativeSignedInfo {
            show_versions: &self.custom_inputs.show_versions.to_owned(),
        }
        .render()
        .unwrap()
    }

    fn github_native_signed(&self) -> String {
        GithubNativeSigned {
            title: "Android release build",
            publishing_format: &self.custom_inputs.publishing_format,
            show_versions: &self.custom_inputs.show_versions,
            build_variant_name: &self
                .custom_inputs
                .build_variant_name
                .as_ref()
                .unwrap_or(&String::new()),
            build_variant_path: &self
                .custom_inputs
                .build_variant_path
                .as_ref()
                .unwrap_or(&String::new()),
        }
        .render()
        .unwrap()
    }

    fn github_flutter_signed(&self) -> String {
        GithubFlutterSigned {
            title: "Flutter Android release build",
            publishing_format: &self.custom_inputs.publishing_format,
            show_versions: &self.custom_inputs.show_versions,
            build_variant_path: &self
                .custom_inputs
                .build_variant_path
                .as_ref()
                .unwrap_or(&String::new()),
        }
        .render()
        .unwrap()
    }

    fn github_react_native_signed(&self) -> String {
        GithubReactNativeSigned {
            title: "React Native Android release build",
            publishing_format: &self.custom_inputs.publishing_format,
            show_versions: &self.custom_inputs.show_versions,
            build_variant_name: &self
                .custom_inputs
                .build_variant_name
                .as_ref()
                .unwrap_or(&String::new()),
            build_variant_path: &self
                .custom_inputs
                .build_variant_path
                .as_ref()
                .unwrap_or(&String::new()),
        }
        .render()
        .unwrap()
    }

    fn github_native_unsigned(&self) -> String {
        GithubNativeUnsigned {
            title: "Android debug build",
            publishing_format: &self.custom_inputs.publishing_format,
            show_versions: &self.custom_inputs.show_versions,
            build_variant_name: &self
                .custom_inputs
                .build_variant_name
                .as_ref()
                .unwrap_or(&String::new()),
            build_variant_path: &self
                .custom_inputs
                .build_variant_path
                .as_ref()
                .unwrap_or(&String::new()),
        }
        .render()
        .unwrap()
    }

    fn github_flutter_unsigned(&self) -> String {
        GithubFlutterUnsigned {
            title: "Flutter Android debug build",
            publishing_format: &self.custom_inputs.publishing_format,
            show_versions: &self.custom_inputs.show_versions,
            build_variant_path: &self
                .custom_inputs
                .build_variant_path
                .as_ref()
                .unwrap_or(&String::new()),
        }
        .render()
        .unwrap()
    }

    fn github_react_native_unsigned(&self) -> String {
        GithubReactNativeUnsigned {
            title: "React Native Android debug build",
            publishing_format: &self.custom_inputs.publishing_format,
            show_versions: &self.custom_inputs.show_versions,
            build_variant_name: &self
                .custom_inputs
                .build_variant_name
                .as_ref()
                .unwrap_or(&String::new()),
            build_variant_path: &self
                .custom_inputs
                .build_variant_path
                .as_ref()
                .unwrap_or(&String::new()),
        }
        .render()
        .unwrap()
    }
}

#[derive(
    Clone, Copy, Debug, EnumIter, EnumString, Display, PartialEq, Serialize, Deserialize, Eq,
)]
pub enum AppPlatform {
    #[strum(serialize = "Android")]
    Android,
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
pub enum Sdk {
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
    show_versions: &'a bool,
    build_variant_name: &'a str,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-flutter-signed")]
struct GithubFlutterSigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    show_versions: &'a bool,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-react-native-signed")]
struct GithubReactNativeSigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    show_versions: &'a bool,
    build_variant_name: &'a str,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-native-unsigned")]
struct GithubNativeUnsigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    show_versions: &'a bool,
    build_variant_name: &'a str,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-flutter-unsigned")]
struct GithubFlutterUnsigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    show_versions: &'a bool,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-react-native-unsigned")]
struct GithubReactNativeUnsigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    show_versions: &'a bool,
    build_variant_name: &'a str,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "info/github-native-signed")]
struct GithubNativeSignedInfo<'a> {
    show_versions: &'a bool,
}

#[derive(Template)]
#[template(path = "info/github-native-unsigned")]
struct GithubNativeUnsignedInfo<'a> {
    show_versions: &'a bool,
}

#[derive(Template)]
#[template(path = "info/github-flutter-signed")]
struct GithubFlutterSignedInfo<'a> {
    show_versions: &'a bool,
}

#[derive(Template)]
#[template(path = "info/github-flutter-unsigned")]
struct GithubFlutterUnsignedInfo<'a> {
    show_versions: &'a bool,
}

#[derive(Template)]
#[template(path = "info/github-react-native-signed")]
struct GithubReactNativeSignedInfo<'a> {
    show_versions: &'a bool,
}

#[derive(Template)]
#[template(path = "info/github-react-native-unsigned")]
struct GithubReactNativeUnsignedInfo<'a> {
    show_versions: &'a bool,
}
