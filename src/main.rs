use dioxus::prelude::*;
use dew::dewit::{AllsiesProps,DewIt};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut lists = use_signal(|| vec![]);

    for idx in 0..3 {
	lists.write().push(format!("item {}", idx));
	
    }

    let context = use_context_provider(|| AllsiesProps { lists });
    
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        DewIt {}

    }
}
