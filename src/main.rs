mod code;
mod state;
use state::{BuildType, Platform, State, SDK};
use std::str::FromStr;
use strum::IntoEnumIterator;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew::{events::InputEvent, html, Component, Context, Html};

enum Msg {
    Generate,
    CopyToClipboard,
    UpdatePlatform(String),
    UpdateSDK(String),
    UpdateBuildType(String),
}

struct App {
    state: State,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let state = State {
            platform: Platform::GitHub,
            sdk: SDK::Native,
            build_type: BuildType::Unsigned,
            code: String::new(),
        };

        Self { state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Generate => {
                self.state.code =
                    code::generate(self.state.platform, self.state.sdk, self.state.build_type);
                true
            }
            Msg::CopyToClipboard => {
                let window = web_sys::window().expect("Missing Window!");
                window
                    .navigator()
                    .clipboard()
                    .expect("Missing Clipboard!")
                    .write_text(&self.state.code);
                true
            }
            Msg::UpdatePlatform(selected) => {
                self.state.platform = Platform::from_str(&selected).unwrap();
                true
            }
            Msg::UpdateSDK(selected) => {
                self.state.sdk = SDK::from_str(&selected).unwrap();
                true
            }
            Msg::UpdateBuildType(selected) => {
                self.state.build_type = BuildType::from_str(&selected).unwrap();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let _platform_options = Platform::iter().map(|platform| {
            let is_selected = platform == self.state.platform;

            html! {
                <option selected={is_selected} value={ platform.to_string() }>
                  { platform.to_string() }
                </option>
            }
        });

        let _sdk_options = SDK::iter().map(|sdk| {
            let is_selected = sdk == self.state.sdk;

            html! {
                <option selected={is_selected} value={ sdk.to_string() }>
                    { sdk.to_string() }
                </option>
            }
        });

        let _build_type_options = BuildType::iter().map(|build_type| {
            let is_selected = build_type == self.state.build_type;

            html! {
                <option selected={is_selected} value={ build_type.to_string() }>
                    { build_type.to_string() }
                </option>
            }
        });

        let _on_platform_change = link.batch_callback(|e: InputEvent| {
            e.prevent_default();
            let input: HtmlSelectElement = e.target_unchecked_into();
            Some(Msg::UpdatePlatform(input.value()))
        });

        let _on_sdk_change = link.batch_callback(|e: InputEvent| {
            e.prevent_default();
            let input: HtmlSelectElement = e.target_unchecked_into();
            Some(Msg::UpdateSDK(input.value()))
        });

        let _on_build_type_change = link.batch_callback(|e: InputEvent| {
            e.prevent_default();
            let input: HtmlSelectElement = e.target_unchecked_into();
            Some(Msg::UpdateBuildType(input.value()))
        });

        html! {
            <>
                <header>
                    <h1>{ "Cong" }<img src="public/controller.svg" class="logo" /></h1>
                    <p class="p-1">{ "quickly generate template CI workflow files for mobile apps" }</p>
                </header>

                <main>
                    <div class="pickers">
                        <select oninput={_on_platform_change} value={ self.state.platform.to_string() }>{ for _platform_options }</select>
                        <select class="picker-wide" oninput={_on_sdk_change} value={ self.state.sdk.to_string() }>{ for _sdk_options }</select>
                        <select class="picker-wide" oninput={_on_build_type_change} value={ self.state.build_type.to_string() }>{ for _build_type_options }</select>
                    </div>

                <div><button class="cta" onclick={link.callback(|_| Msg::Generate)}>{ "Can I have it?" }</button></div>

                <div>
                <button class="copy hint--top" data-hint="Copied!" onclick={link.callback(|_| Msg::CopyToClipboard)}>
                    <img src="/public/clipboard-text.svg" width="24" />
                </button>
                <pre class="code code-html"><label>{ "YAML" }</label><code>{ self.state.code.to_string() }</code></pre>
                </div>
                </main>

                <footer><p>{ "Built with Rust " }<img src="/public/tram.svg" width="16" />{ " by " }<a href="https://tramline.app">{ "Tramline" }</a></p></footer>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
