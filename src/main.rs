mod bindings;
mod state;
use state::{BuildType, CustomInputs, Platform, PublishingFormat, Sdk, State};
use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;
use web_sys::HtmlInputElement as InputElement;
use web_sys::{HtmlElement, HtmlSelectElement};
use yew::prelude::*;
use yew::{events::InputEvent, function_component, html, Component, Context, Html};
use yew_hooks::prelude::*;

#[function_component]
fn Header() -> Html {
    html! {
        <header>
            <h1 class="mt-header">{ "macige" }<img src="public/controller.svg" class="logo" alt="logo" width="32" height="32" /></h1>
            <p class="p-1 subheading">
                <u>{ "M" }</u>
                { "OBILE " }
                <u>{ "A" }</u>
                { "PP " }
                <u>{ "CI" }</u>
                { " WORKFLOW " }
                <u>{ "GE" }</u>
                { "NERATOR" }
            </p>
            <a href="https://github.com/tramlinehq/macige" class="github-corner" aria-label="View source on GitHub">
                <svg width="100" height="100" viewBox="0 0 250 250" style="fill:#fff; color:#304050; position: absolute; top: 0; border: 0; right: 0;" aria-hidden="true">
                    <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"></path>
                    <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor" style="transform-origin: 130px 106px;" class="octo-arm"></path>
                    <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z" fill="currentColor" class="octo-body"></path>
                </svg>
            </a>
        </header>
    }
}

#[function_component]
fn Footer() -> Html {
    html! {
        <footer>
            <p>
                { "Built using " }
                <a href="https://yew.rs/">{ "yew" }</a>
                { ", "}
                <a href="https://simplecss.org/">{ "simple.css" }</a>
                { ", and "}
                <a href="https://highlightjs.org/">{ "highlight.js" }</a>
                { " by " }
                <a href="https://tramline.app/">{ "Tramline" }</a>
                { "." }
            </p>
            <p>
                { "Want to improve something? Send us a PR or create an issue " }
                <a href="https://github.com/tramlinehq/macige/">{ "on GitHub" }</a>
                { "!" }
            </p>
        </footer>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct CopyToClipboardProps {
    #[prop_or_default]
    pub code: Option<String>,
}

#[function_component]
fn CopyToClipboardButton(props: &CopyToClipboardProps) -> Html {
    let clipboard = use_clipboard();
    let should_say_copied = use_state(|| false);

    let onclick = {
        let should_say_copied_clone = should_say_copied.clone();

        if let Some(code) = props.code.clone() {
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                let should_say_copied_cc = should_say_copied_clone.clone(); // some serious cloning going on in here?!
                should_say_copied_cc.set(true);
                gloo::timers::callback::Timeout::new(1300, move || {
                    should_say_copied_cc.set(false);
                })
                .forget();

                clipboard.write_text(code.clone())
            })
        } else {
            Callback::noop()
        }
    };

    html! {
        <div class="copy-to-clipboard mt-1">
            <p>{"Copy to clipboard"}</p>
            if *should_say_copied {
                <p class="sm-t"><mark>{"Copied!"}</mark></p>
            }
            <button class="copy" onclick={onclick}>
            <img src="/public/clipboard-text.svg" width="24" height="24" alt="copy to clipboard" />
            </button>
        </div>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct DisplayInfoProps {
    pub info: Option<String>,
}

#[function_component]
fn DisplayInfo(props: &DisplayInfoProps) -> Html {
    let info_ref = use_node_ref();

    if let Some(info_el) = info_ref.cast::<HtmlElement>() {
        if let Some(info) = props.info.clone() {
            info_el.set_inner_html(&info);
        } else {
            info_el.set_inner_html("");
        }
    }

    html! {
        <div ref={info_ref}></div>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct DisplayCodeProps {
    pub info: Option<String>,
    pub code: Option<String>,
}

#[function_component]
fn DisplayCode(props: &DisplayCodeProps) -> Html {
    let code_ref = use_node_ref();

    let options = bindings::HighlightOptions {
        language: "yaml".to_owned(),
        ignore_illegals: true,
    };

    if let Some(code) = props.code.clone() {
        let highlighted: bindings::HighlightResult =
            bindings::Hljs::highlight(&code, &serde_wasm_bindgen::to_value(&options).unwrap());

        if let Some(code_el) = code_ref.cast::<HtmlElement>() {
            code_el.set_inner_html(&highlighted.value());
        }
    }

    html! {
        <>
            <DisplayInfo info={ props.info.clone() } />
            <CopyToClipboardButton code={ props.code.clone() } />
            <pre class="code"><label>{ "YAML" }</label><code ref={code_ref}></code></pre>
        </>
    }
}

enum Msg {
    Generate,
    UpdatePlatform(String),
    UpdateSdk(String),
    UpdateBuildType(String),
    UpdateBuildVariantName(String),
    UpdateBuildVariantPath(String),
    UpdatePublishingFormat(String),
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
            sdk: Sdk::Native,
            build_type: BuildType::Unsigned,
            code_template: None,
            info_template: None,
            custom_inputs: CustomInputs {
                build_variant_name: Some("Debug".to_string()),
                build_variant_path: Some("debug/app-debug".to_string()),
                publishing_format: PublishingFormat::Apk,
            },
        };

        Self { state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Generate => self.state.gen_templates(),
            Msg::UpdatePlatform(selected) => {
                self.state.clear_text();
                self.state.platform = Platform::from_str(&selected).unwrap();
            }
            Msg::UpdateSdk(selected) => {
                self.state.clear_text();
                self.state.sdk = Sdk::from_str(&selected).unwrap();
            }
            Msg::UpdateBuildType(selected) => {
                self.state.clear_text();
                self.state.build_type = BuildType::from_str(&selected).unwrap();
                if matches!(self.state.build_type, BuildType::Signed) {
                    self.state.custom_inputs.build_variant_name = Some("Release".to_string());
                    self.state.custom_inputs.build_variant_path =
                        Some("release/app-prod-release".to_string())
                } else {
                    self.state.custom_inputs.build_variant_name = Some("Debug".to_string());
                    self.state.custom_inputs.build_variant_path =
                        Some("debug/app-debug".to_string())
                }
            }
            Msg::UpdateBuildVariantName(value) => {
                self.state.clear_text();
                self.state.custom_inputs.build_variant_name = Some(value);
            }
            Msg::UpdateBuildVariantPath(value) => {
                self.state.clear_text();
                self.state.custom_inputs.build_variant_path = Some(value);
            }
            Msg::UpdatePublishingFormat(selected) => {
                self.state.clear_text();
                self.state.custom_inputs.publishing_format =
                    PublishingFormat::from_str(&selected).unwrap();
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let _on_platform_change = link.batch_callback(|e: InputEvent| {
            e.prevent_default();
            let input: HtmlSelectElement = e.target_unchecked_into();
            Some(Msg::UpdatePlatform(input.value()))
        });

        let _on_sdk_change = link.batch_callback(|e: InputEvent| {
            e.prevent_default();
            let input: HtmlSelectElement = e.target_unchecked_into();
            Some(Msg::UpdateSdk(input.value()))
        });

        let _on_build_type_change = link.batch_callback(|e: InputEvent| {
            e.prevent_default();
            let input: HtmlSelectElement = e.target_unchecked_into();
            Some(Msg::UpdateBuildType(input.value()))
        });

        let _on_build_variant_name_change = link.batch_callback(|e: InputEvent| {
            let input: InputElement = e.target_unchecked_into();
            Some(Msg::UpdateBuildVariantName(input.value()))
        });

        let _on_build_variant_path_change = link.batch_callback(|e: InputEvent| {
            let input: InputElement = e.target_unchecked_into();
            Some(Msg::UpdateBuildVariantPath(input.value()))
        });

        let _on_publishing_format_change = link.batch_callback(|e: InputEvent| {
            e.prevent_default();
            let input: HtmlSelectElement = e.target_unchecked_into();
            Some(Msg::UpdatePublishingFormat(input.value()))
        });

        html! {
            <>
                <Header />

                <main>

                <p class="notice">
                { "Stop building your mobile apps by hand and passing builds around! 🚨" }
                <br/><br/>
                { "Use a " } <a href="https://en.wikipedia.org/wiki/CI/CD">{"CI server"}</a> { " instead!" }
                <br/><br/>
                <ol>
                <li>{ "Pick a CI provider" }</li>
                <li>{ "Select build attributes" }</li>
                <li>{ "Read the instructions" }</li>
                <li>{ "Tweak, as necessary" }</li>
                <li>{ "Get up and running! 🚀" }</li>
                </ol>
                </p>

                <div class="pickers">

                <div class="picker-wide">
                <label for="ci-provider">{"CI Provider"}</label>
                <select name="ci-provider" oninput={_on_platform_change} value={ self.state.platform.to_string() }>{ for self.to_options(self.state.platform) }</select>
                </div>

                <div class="picker-wide">
                <label for="sdk">{"Sdk"}</label>
                <select name="sdk" oninput={_on_sdk_change} value={ self.state.sdk.to_string() }>{ for self.to_options(self.state.sdk) }</select>
                </div>

                <div class="picker-wide">
                <label for="build-type">{"Build Type "}<span class="sm-t">{"("}<a href="https://developer.android.com/studio/publish/app-signing">{"signing apps"}</a>{")"}</span></label>
                <select name="build-type" oninput={_on_build_type_change} value={ self.state.build_type.to_string() }>{ for self.to_options(self.state.build_type) }</select>
                </div>

                </div>

                <div class="pickers">
                if !matches!(self.state.sdk, Sdk::Flutter) {
                    <div class="picker-wide">
                    <label for="build-variant">{"Build Variant "}<span class="sm-t">{"("}<a href="https://developer.android.com/studio/build/build-variants">{"build variants"}</a>{")"}</span></label>
                    <input id="build-variant" oninput={_on_build_variant_name_change} type="text" value={ self.state.custom_inputs.build_variant_name.to_owned() } />
                    </div>
                }
                <div class="picker-wider">
                <label for="pub-format">{"Publishing Format"}</label>
                <select aria-labelledby="pub-format" name="pub-format" oninput={_on_publishing_format_change} value={ self.state.custom_inputs.publishing_format.to_string() }>{ for self.to_options(self.state.custom_inputs.publishing_format) }</select>
                </div>

                </div>

                <div class="pickers">
                  <div class="picker-wider">
                  <label for="output-path">{"Build Output Path "}<span class="sm-t">{"(relative path to the base output directory)"}</span></label>
                  <div class="input-wrapper suffix">
                  <input aria-labelledby="output-path" id="output-path" oninput={_on_build_variant_path_change} class="build-variant" type="text" value={ self.state.custom_inputs.build_variant_path.to_owned() } />
                  <div class="input-suffix">{ "." }{self.state.custom_inputs.publishing_format.to_string().to_lowercase()}</div>
                  </div>
                </div>
                </div>

                <div><button class="cta" onclick={link.callback(|_| Msg::Generate)}>{ "Can I have it?" }</button></div>

                <div>
                <DisplayCode code={ self.state.code_template.to_owned() } info={ self.state.info_template.to_owned() } />
                </div>

                </main>

                <Footer />
            </>
        }
    }
}

impl App {
    fn to_options<E: fmt::Display + Eq + IntoEnumIterator>(
        &self,
        cur: E,
    ) -> impl Iterator<Item = Html> {
        E::iter().map(move |val| {
            let is_selected = val == cur;

            html! {
                <option selected={is_selected} value={ val.to_string() }>
                { val.to_string() }
            </option>
            }
        })
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
