pub mod event;

use crate::app::editor::event::Request;
use eg_graph_editor_lib::{graph::Graph, proof::action::Action};
use leptos::*;
use leptos_use::{core::StorageType, storage::use_storage, utils::JsonCodec};
use std::collections::VecDeque;
use tracing::info;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/dist-node-editor/node-editor.js")]
extern "C" {
    fn init(element_name: String);
    fn currently_selected_nodes() -> Vec<String>;
    fn new_sybgrpah(new_id: String, parent: String);
}

#[component]
pub fn Editor() -> impl IntoView {
    let graph = create_rw_signal(Graph::new());
    let stack: RwSignal<VecDeque<Vec<Action>>> = create_rw_signal(VecDeque::new());

    let (recv, send, _) = use_storage::<Option<Request>, JsonCodec>(StorageType::Session, "msgs");

    create_effect(move |_| {
        if let Some(ev) = recv() {
            match ev {
                x => info!("{:?}", x),
            }
        }
    });

    spawn_local(async move {
        init("editor_container".to_string());
    });

    view! {
        <div class="flex flex-col items-center flex-grow p-2 ">

        <div class="flex">
            <button >+</button>
        </div>


            <div
                id="editor_container"
                class="
                flex-grow 
                w-full
                [--background-color:rgba(0,0,0,0.1)]
                
                [--minimap-background-color:#eee]
                dark:[--minimap-background-color:#333]
                
                [--minimap-node-color:#555]
                dark:[--minimap-node-color:#eee]
                
                [--minimap-mask-color:rgba(0,0,0,0.4)]
                dark:[--minimap-mask-color:rgba(255,255,255,0.4)]
                
                [--background-pattern-color:rgba(0,0,0,0.9)]
                dark:[--background-pattern-color:rgba(255,255,255,0.9)]
                
                [--node-boxshadow-selected:0_0_0_0.25em_purple]
                
                [--attribution-background-color:var(--minimap-background-color)]
                "
            ></div>
        </div>
    }
}
