use leptos::*;
use leptos_router::*;
use leptos_router::Router;
use serde::{Deserialize, Serialize};
use cid::Cid;

use crate::types::Manifest;
use crate::wasm::env::{APP_NAME, APP_VERSION};
use crate::wasm::components::InternalLink;
use crate::wasm::compat::{WasmDevice, WasmDeviceError};

// This router is an attempt to make SPAs easy
// Register and use pages here

mod index;
// mod items;

use index::IndexPage;
// use items::ItemsPage;

/// A Shared page context to pass to all pages within our internal router
#[derive(Clone, Serialize, Deserialize)]
pub struct PageContext {
    manifest: Option<Manifest>,
    route: Option<String>,
    query: Option<String>,
}

impl PageContext {
    pub fn manifest(&self) -> &Option<Manifest> {
        &self.manifest
    }
    pub fn route(&self) -> &Option<String> {
        &self.route
    }
    pub fn query(&self) -> &Option<String> {
        &self.query
    }
}

impl IntoView for PageContext {
    fn into_view(self) -> View {
        let page: Box<dyn Page> = match self.route() {
            Some(route) => match route.as_str() {
                // "items" => ItemsPage::from_ctx(self),
                _ => IndexPage::from_ctx(self),
            },
            _ => IndexPage::from_ctx(self),
        };
        page.into_view_ref()
    }
}

/// Trait object for passing page views to the router
pub trait Page: Send + Sync {
    fn ctx(&self) -> &PageContext;
    fn from_ctx(ctx: PageContext) -> Box<dyn Page>
    where
        Self: Sized;
    fn into_view_ref(&self) -> View;
}

#[component]
pub fn InternalRouter() -> impl IntoView {
    view! {
        <Router>
            <input type="checkbox" id="drawer-toggle" name="drawer-toggle"/>
            <label for="drawer-toggle" id="drawer-toggle-label"></label>
            <header>{APP_NAME}</header>
            <nav id="drawer">
                <ul>
                    <li><InternalLink query="".to_string()  msg="Home".to_string()/></li>
                </ul>
            </nav>
            <main id="page-content">
                <PageRoute/>
            </main>
        </Router>
    }
}

/// An internal router should use the context to render a page
#[component]
fn PageRoute() -> impl IntoView {
    let (route, _) = create_query_signal::<String>("route");
    let (query, _) = create_query_signal::<String>("query");

    let ctx = create_resource(
        || (),
        move |_| async move {
            // TODO: move device init out of here, but works for now
            let device = WasmDevice::new().expect("failed to init device");
            let route = route.get();
            let query = query.get();
            let root_cid = device.read_root_cid().await.expect("failed to read root cid");
            if root_cid == Cid::default() {
                return PageContext {
                    manifest: None,
                    route,
                    query,
                };
            }
            let manifest = device
                .read_manifest(&root_cid)
                .await
                .expect("failed to read dor store");

            let ctx = PageContext {
                manifest: Some(manifest),
                route,
                query,
            };

            ctx
        },
    );

    view! {
        <div>
            {move || match ctx.get() {
                None => view! { Loading... }.into_view(), 
                Some(c) => c.into_view()
            }}
        </div>
    }
}