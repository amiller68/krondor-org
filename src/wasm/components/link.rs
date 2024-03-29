use cid::Cid;
use leptos::*;
use leptos_use::use_event_listener;
use serde::{Deserialize, Serialize};

use crate::wasm::utils::origin_url;

// TODO: Force history update here, since this naviagtes completelty internally
#[component]
pub fn InternalLink(query: String, msg: String) -> impl IntoView {
    let url = origin_url();
    let url = format!("{}/{}", url, query);
    let a_href_ref = create_node_ref::<leptos::html::A>();
    let _ = use_event_listener(
        a_href_ref,
        leptos::ev::click,
        move |_event: web_sys::MouseEvent| {
            let a_ref = a_href_ref.get().expect("a_ref");
            let url = a_ref.href();
            let window = web_sys::window().expect("window");
            window.location().set_href(&url).expect("href");
        },
    );

    view! {
        <a
            href=url
            ref=a_href_ref
            class="link"
        >
            {msg}
        </a>
    }
}

impl IntoView for ObjectLink {
    fn into_view(self) -> View {
        view! {
            <InternalLink query=format!("?route=object&query={}", self.cid.to_string()) msg=self.title/>
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ObjectLink {
    pub cid: Cid,
    pub title: String,
}
