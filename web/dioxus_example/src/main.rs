#![allow(non_snake_case)]
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::window;

fn main() {
    ambient_web::init_ambient(true, true);
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        "Hello",
        Ambient {},
        "World"
    })
}

fn Ambient(cx: Scope) -> Element {
    let uid = format!("Ambient_{}", cx.scope_id().0);
    use_effect(cx, (), {
        let uid = uid.clone();
        move |_| {
            let uid = uid.clone();
            async move {
                let document = window().unwrap().document();
                let element = document
                    .unwrap()
                    .get_element_by_id(&uid)
                    .unwrap()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap();
                ambient_web::start(Some(element)).await;
            }
        }
    });

    cx.render(rsx! {
        div {
            id: "{uid}",
        }
    })
}
