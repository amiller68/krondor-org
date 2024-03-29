use cid::Cid;
use leptos::*;
use leptos_router::Router;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::types::Manifest;
use crate::wasm::components::{ErrorMessageBox, InternalLink};
use crate::wasm::device::{WasmDevice, WasmDeviceError};
use crate::wasm::env::APP_NAME;

// This router is an attempt to make SPAs easy
// Register and use pages here

mod about;
mod audio;
mod index;
mod object;
mod status;
mod visual;
mod writing;

use about::AboutPage;
use audio::AudioPage;
use index::IndexPage;
use object::ObjectPage;
use status::StatusPage;
use visual::VisualPage;
use writing::WritingPage;

/// A Shared page context to pass to all pages within our internal router
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PageContext {
    root_cid: Cid,
    chain_id: u32,
    manifest: Manifest,
    route: Option<String>,
    query: Option<String>,
}

impl PageContext {
    pub fn root_cid(&self) -> &Cid {
        &self.root_cid
    }
    pub fn chain_id(&self) -> u32 {
        self.chain_id
    }
    pub fn manifest(&self) -> &Manifest {
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
                "about" => AboutPage::from_ctx(self),
                "object" => ObjectPage::from_ctx(self),
                "writing" => WritingPage::from_ctx(self),
                "audio" => AudioPage::from_ctx(self),
                "visual" => VisualPage::from_ctx(self),
                "status" => StatusPage::from_ctx(self),
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
                <header><InternalLink query="".to_string()  msg={APP_NAME.to_string()}/></header>
                <main>
                    <PageRoute/>
                </main>
                <nav id="drawer">
                    <ul>
                        <li><InternalLink query="".to_string()  msg="Home".to_string()/></li>
                        <li><InternalLink query="?route=about".to_string()  msg="About".to_string()/></li>
                        <li><InternalLink query="?route=writing".to_string()  msg="Writing".to_string()/></li>
                        <li><InternalLink query="?route=audio".to_string()  msg="Audio".to_string()/></li>
                        <li><InternalLink query="?route=visual".to_string()  msg="Visual".to_string()/></li>
                        <li><InternalLink query="?route=status".to_string()  msg="Status".to_string()/></li>
                    </ul>
                </nav>
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
            // let device = WasmDevice::new().map_err(PageError::WasmDevice)?;
            let device = match WasmDevice::new().map_err(PageError::WasmDevice) {
                Ok(device) => device,
                Err(e) => {
                    return PageContextResource {
                        ctx: None,
                        error_message: Some(e.to_string()),
                    }
                }
            };
            let chain_id = device.chain_id();
            let route = route.get();
            let query = query.get();
            let root_cid = match device.read_root_cid().await.map_err(PageError::RootCidRead) {
                Ok(cid) => cid,
                Err(e) => {
                    return PageContextResource {
                        ctx: None,
                        error_message: Some(e.to_string()),
                    }
                }
            };
            if root_cid == Cid::default() {
                return PageContextResource {
                    ctx: None,
                    error_message: Some(PageError::NoRootCid.to_string()),
                };
            }
            let manifest = match device
                .read_manifest(&root_cid)
                .await
                .map_err(PageError::ManifestRead)
            {
                Ok(manifest) => manifest,
                Err(e) => {
                    return PageContextResource {
                        ctx: None,
                        error_message: Some(e.to_string()),
                    }
                }
            };

            let ctx = PageContext {
                root_cid,
                chain_id,
                manifest,
                route,
                query,
            };

            PageContextResource {
                ctx: Some(ctx),
                error_message: None,
            }
        },
    );

    view! {
        <div>
            {move || match ctx.get() {
                None => view! { Loading... }.into_view(),
                Some(c) => {
                    match c.error_message() {
                        Some(msg) => view! { <ErrorMessageBox msg=msg.clone()/> }.into_view(),
                        None => c.ctx().into_view(),
                    }
                }
            }}
        </div>
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct PageContextResource {
    ctx: Option<PageContext>,
    error_message: Option<String>,
}

impl PageContextResource {
    fn error_message(&self) -> Option<String> {
        self.error_message.clone()
    }
    fn ctx(&self) -> PageContext {
        self.ctx.clone().expect("ctx")
    }
}

#[derive(Debug, thiserror::Error)]
enum PageError {
    #[error("Failed to initialize web device: {0}")]
    WasmDevice(WasmDeviceError),
    #[error("No valid root cid found, please check that you've pushed a valid manifest")]
    NoRootCid,
    #[error("Failed to read root cid: {0}")]
    RootCidRead(WasmDeviceError),
    #[error("Failed to read manifest: {0}")]
    ManifestRead(WasmDeviceError),
}
