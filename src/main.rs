mod bindings;
mod code;
mod state;
use gloo_utils::format::JsValueSerdeExt;
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
            <h1>{ "Cong" }<img src="public/controller.svg" class="logo" /></h1>
            <p class="p-1">{ "quickly generate template CI workflow files for mobile apps" }</p>
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
    pub code: String,
}

#[function_component]
fn CopyToClipboardButton(props: &CopyToClipboardProps) -> Html {
    let clipboard = use_clipboard();

    let on_click = {
        let code = props.code.clone();
        let clipboard = clipboard.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            Some(clipboard.write_text(code.to_owned()));
        })
    };

    html! {
        <>
            <button class="copy" onclick={on_click}>
              <img src="/public/clipboard-text.svg" width="24" />
            </button>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct DisplaySnippetProps {
    pub code: String,
}

#[function_component]
fn DisplaySnippet(props: &DisplaySnippetProps) -> Html {
    let code_ref = use_node_ref();

    let options = bindings::HighlightOptions {
        language: "yaml".to_string(),
        ignore_illegals: true,
    };

    let highlighted: bindings::HighlightResult =
        bindings::Hljs::highlight(&props.code.clone(), &JsValue::from_serde(&options).unwrap());

    if let Some(code_el) = code_ref.cast::<HtmlElement>() {
        code_el.set_inner_html(&highlighted.value());
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
            code: String::new(),
        };

        Self { state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Generate => {
                self.state.code =
                    code::generate(self.state.platform, self.state.sdk, self.state.build_type);
            }
            Msg::UpdatePlatform(selected) => {
                self.state.platform = Platform::from_str(&selected).unwrap();
            }
            Msg::UpdateSDK(selected) => {
                self.state.sdk = SDK::from_str(&selected).unwrap();
            }
            Msg::UpdateBuildType(selected) => {
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
                <div class="pickers">
                <select class="picker-wide" oninput={_on_platform_change} value={ self.state.platform.to_string() }>{ for self.to_options(self.state.platform) }</select>
                <select class="picker-wide" oninput={_on_sdk_change} value={ self.state.sdk.to_string() }>{ for self.to_options(self.state.sdk) }</select>
                <select class="picker-wide" oninput={_on_build_type_change} value={ self.state.build_type.to_string() }>{ for self.to_options(self.state.build_type) }</select>
                </div>
                <div><button class="cta" onclick={link.callback(|_| Msg::Generate)}>{ "Can I have it?" }</button></div>

                <div><CopyToClipboardButton code={self.state.code.to_string()} /><DisplaySnippet code={ self.state.code.to_string() } /></div>
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
