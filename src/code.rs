use crate::state::{BuildType, Platform, SDK};
use askama::Template;

#[derive(Template)]
#[template(path = "github-native-signed")]
struct GithubNativeSigned<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "github-flutter-signed")]
struct GithubFlutterSigned<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "github-react-native-signed")]
struct GithubReactNativeSigned<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "github-native-unsigned")]
struct GithubNativeUnsigned<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "github-flutter-unsigned")]
struct GithubFlutterUnsigned<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "github-react-native-unsigned")]
struct GithubReactNativeUnsigned<'a> {
    title: &'a str,
}

pub struct Code {
    pub platform: Platform,
    pub sdk: SDK,
    pub build_type: BuildType,
}

pub struct CodeResult {
    pub info_template: String,
    pub code_template: String,
}

impl Code {
    pub fn generate_templates(&self) -> CodeResult {
        match (self.platform, self.sdk, self.build_type) {
            (Platform::GitHub, SDK::Native, BuildType::Signed) => {
                let code_template = GithubNativeSigned {
                    title: "Android Play Store release build",
                };

                CodeResult {
                    code_template: code_template.render().unwrap(),
                    info_template: String::new(),
                }
            }

            (Platform::GitHub, SDK::Flutter, BuildType::Signed) => {
                let code_template = GithubFlutterSigned {
                    title: "Android Play Store release build",
                };

                CodeResult {
                    code_template: code_template.render().unwrap(),
                    info_template: String::new(),
                }
            }

            (Platform::GitHub, SDK::ReactNative, BuildType::Signed) => {
                let code_template = GithubReactNativeSigned {
                    title: "Android Play Store release build",
                };

                CodeResult {
                    code_template: code_template.render().unwrap(),
                    info_template: String::new(),
                }
            }

            (Platform::GitHub, SDK::Native, BuildType::Unsigned) => {
                let code_template = GithubNativeUnsigned {
                    title: "Android debug build",
                };

                CodeResult {
                    code_template: code_template.render().unwrap(),
                    info_template: String::new(),
                }
            }

            (Platform::GitHub, SDK::Flutter, BuildType::Unsigned) => {
                let code_template = GithubFlutterUnsigned {
                    title: "Android debug build",
                };

                CodeResult {
                    code_template: code_template.render().unwrap(),
                    info_template: String::new(),
                }
            }

            (Platform::GitHub, SDK::ReactNative, BuildType::Unsigned) => {
                let code_template = GithubReactNativeUnsigned {
                    title: "Android debug build",
                };

                CodeResult {
                    code_template: code_template.render().unwrap(),
                    info_template: String::new(),
                }
            }
        }
    }
}
