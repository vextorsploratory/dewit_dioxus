use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[derive(Clone, PartialEq, Props)]
pub struct AllsiesProps {
    pub lists: Signal<Vec<String>>,
}

#[component]
pub fn DewIt() -> Element {
    rsx! {
        div {
            id: "dewit",
            img { src: HEADER_SVG, id: "header" }
            div { id: "mainframe",
		  BreadCrumb{}
		  TopList{}
            }
        }
    }
}

#[component]
pub fn BreadCrumb() -> Element {
    rsx!{
	div { id: "breadcrumb",
	      "todo: add breadcrumb buttons"
	}
    }
}

#[component]
pub fn TopList() -> Element {

    let context = use_context::<AllsiesProps>();
    rsx!{
	div { id: "toplist",
	      for (i, item) in context.lists.read().iter().enumerate() {
		      ul {
			  li {
			      key: "{i}",
			      "{item}" 
			  }
		      }
	      }
	}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;
    use dioxus::core::VirtualDom;
    use dioxus::core::Element;
    use dioxus::core::Mutations;
    use dioxus_signals::*;

    #[component]
    fn TestApp() -> Element {
	let mut lists = use_signal(|| vec![]);

	for idx in 0..3 {
	    lists.write().push(format!("item {}", idx));
	    
	}

	let context = use_context_provider(|| AllsiesProps { lists });
	
	rsx! {
            TopList {}

	}
    }
	    
    #[test]
    fn test_one_empty_list() {
	let mut dom = VirtualDom::new(TestApp);
	let mut mutations = Mutations::default();
	dom.rebuild(&mut mutations);

	let html = dioxus_ssr::render(&dom);

	assert!(html.contains("item 0"));
	assert!(html.contains("item 1"));
	assert!(html.contains("item 2"));
    }
}
