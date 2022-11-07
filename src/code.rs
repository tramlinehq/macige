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

pub fn generate(platform: Platform, sdk: SDK, build_type: BuildType) -> String {
    match (platform, sdk, build_type) {
        (Platform::GitHub, SDK::Native, BuildType::Signed) => {
            let template = GithubNativeSigned {
                title: "Android Play Store release build",
            };

            template.render().unwrap()
        }

        (Platform::GitHub, SDK::Flutter, BuildType::Signed) => {
            let template = GithubFlutterSigned {
                title: "Android Play Store release build",
            };

            template.render().unwrap()
        }

        (Platform::GitHub, SDK::ReactNative, BuildType::Signed) => {
            let template = GithubReactNativeSigned {
                title: "Android Play Store release build",
            };

            template.render().unwrap()
        }

        (Platform::GitHub, SDK::Native, BuildType::Unsigned) => {
            let template = GithubNativeUnsigned {
                title: "Android debug build",
            };

            template.render().unwrap()
        }

        (Platform::GitHub, SDK::Flutter, BuildType::Unsigned) => {
            let template = GithubFlutterUnsigned {
                title: "Android debug build",
            };

            template.render().unwrap()
        }

        (Platform::GitHub, SDK::ReactNative, BuildType::Unsigned) => {
            let template = GithubReactNativeUnsigned {
                title: "Android debug build",
            };

            template.render().unwrap()
        }
    }
}
