mod bindings;
mod state;
use state::{BuildType, CustomInputs, Platform, PublishingFormat, State, SDK};
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
            <h1>{ "macigè" }<img src="public/controller.svg" class="logo" alt="logo" width="32" height="32" /></h1>
            <p class="p-1">{ "quickly generate template CI workflow files for mobile apps" }</p>
        </header>
    }
}

#[function_component]
fn Footer() -> Html {
    html! {
        <footer>
            <p>{ "Built in Rust " }<img src="/public/tram.svg" width="16" height="16" alt="tram-logo" />{ " by " }<a href="https://tramline.app">{ "Tramline" }</a></p>
            <p class="sm-t">{ "If you found the tool helpful, or would like to improve it, drop us a " }<a href="https://github.com/tramlinehq/macige/issues">{ "note!" }</a></p>
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
        <div class="copy-to-clipboard">
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
        <div ref={info_ref} class="mt-1"></div>
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
    UpdateSDK(String),
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
            sdk: SDK::Native,
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
            Msg::UpdateSDK(selected) => {
                self.state.clear_text();
                self.state.sdk = SDK::from_str(&selected).unwrap();
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
            Some(Msg::UpdateSDK(input.value()))
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
                <label for="sdk">{"SDK"}</label>
                <select name="sdk" oninput={_on_sdk_change} value={ self.state.sdk.to_string() }>{ for self.to_options(self.state.sdk) }</select>
                </div>

                <div class="picker-wide">
                <label for="build-type">{"Build Type "}<span class="sm-t">{"("}<a href="https://developer.android.com/studio/publish/app-signing">{"signing apps"}</a>{")"}</span></label>
                <select name="build-type" oninput={_on_build_type_change} value={ self.state.build_type.to_string() }>{ for self.to_options(self.state.build_type) }</select>
                </div>

                </div>

                <div class="pickers">
                if !matches!(self.state.sdk, SDK::Flutter) {
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
                  <div class="input-suffix">{ ". " }{self.state.custom_inputs.publishing_format.to_string().to_lowercase()}</div>
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
