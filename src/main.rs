mod bindings;
mod code;
mod state;
use state::{BuildType, Platform, State, SDK};
use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;
use wasm_bindgen::JsValue;
use web_sys::{HtmlElement, HtmlSelectElement};
use yew::prelude::*;
use yew::{events::InputEvent, function_component, html, Component, Context, Html};
use yew_hooks::prelude::*;

#[function_component]
fn Header() -> Html {
    html! {
        <header>
            <h1>{ "macigè" }<img src="public/controller.svg" class="logo" /></h1>
            <p class="p-1">{ "sample template CI workflow files for mobile apps" }</p>
        </header>
    }
}

#[function_component]
fn Footer() -> Html {
    html! {
        <footer>
            <p>{ "Built in Rust " }<img src="/public/tram.svg" width="16" />{ " by " }<a href="https://tramline.app">{ "Tramline" }</a></p>
        </footer>
    }
}

#[derive(Properties, PartialEq)]
pub struct CopyToClipboardProps {
    #[prop_or_default]
    pub code: Option<String>,
}

#[function_component]
fn CopyToClipboardButton(props: &CopyToClipboardProps) -> Html {
    let clipboard = use_clipboard();
    let should_say_copied = use_state(|| false);

    let onclick = {
        let clipboard = clipboard.clone();
        let should_say_copied_clone = should_say_copied.clone();

        if let Some(code) = props.code.clone() {
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();

                // some serious cloning going on in here?!
                let should_say_copied_cc = should_say_copied_clone.clone();
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
            <img src="/public/clipboard-text.svg" width="24" />
            </button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DisplayInfoProps {
    pub info: Option<String>,
}

#[function_component]
fn DisplayInfo(props: &DisplayInfoProps) -> Html {
    let info_ref = use_node_ref();

    if let Some(info_el) = info_ref.cast::<HtmlElement>() {
        info_el.set_inner_html(&props.info.clone().unwrap_or_else(|| "".to_string()));
    }

    html! {
        <div ref={info_ref}></div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DisplayCodeProps {
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
            bindings::Hljs::highlight(&code, &JsValue::from_serde(&options).unwrap());

        if let Some(code_el) = code_ref.cast::<HtmlElement>() {
            code_el.set_inner_html(&highlighted.value());
        }
    }

    html! {
        <pre class="code"><label>{ "YAML" }</label><code ref={code_ref}></code></pre>
    }
}

enum Msg {
    Generate,
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
            code: None,
            info: None,
        };

        Self { state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Generate => {
                let code = code::Code {
                    platform: self.state.platform,
                    sdk: self.state.sdk,
                    build_type: self.state.build_type,
                }
                .generate();

                self.state.code = Some(code.code_template);
                self.state.info = Some(code.info_template)
            }
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

                <select class="picker-wide" oninput={_on_platform_change} value={ self.state.platform.to_string() }>{ for self.to_options(self.state.platform) }</select>
                <select class="picker-wide" oninput={_on_sdk_change} value={ self.state.sdk.to_string() }>{ for self.to_options(self.state.sdk) }</select>
                <select class="picker-wide" oninput={_on_build_type_change} value={ self.state.build_type.to_string() }>{ for self.to_options(self.state.build_type) }</select>
                </div>

                <div><button class="cta" onclick={link.callback(|_| Msg::Generate)}>{ "Can I have it?" }</button></div>

                <div>
                <DisplayInfo info={ self.state.info.to_owned() } />
                <CopyToClipboardButton code={self.state.code.to_owned() } />
                <DisplayCode code={ self.state.code.to_owned() } />
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
