use event_target_ex::EventTargetEx;
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::HtmlSpanElement;
use valerie::{Node, App, h1, div, p};

mod event_target_ex;

fn ui() -> Node {
    div!(
        h1!("Hello, World!"),
        p!("Hej"),
        p!("Hej"),
        p!("Hej")
    ).into()
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    let div = document
        .create_element("span")?
        .dyn_into::<HtmlSpanElement>()?;

    div.set_inner_text(" ");
    div.set_class_name("badge badge-primary");

    document.body().unwrap().append_child(&div)?;

    let div_style = div.style();

    div_style.set_property("position", "absolute")?;
    div_style.set_property("width", "64px")?;
    div_style.set_property("height", "64px")?;
    div_style.set_property("top", "40px")?;
    div_style.set_property("left", "40px")?;

    let down = Rc::new(Cell::new(false));

    document.add_listner("mousemove", {
        let div = div.clone();
        let div_style = div_style.clone();
        let inner_document = document.clone();
        let down = down.clone();

        move |event| {
            div_style
                .set_property("left", &format!("{}px", event.client_x()))
                .unwrap();
            div_style
                .set_property("top", &format!("{}px", event.client_y()))
                .unwrap();

            if down.get() {
                let new_div = div.clone_node().unwrap();
                new_div
                    .dyn_ref::<HtmlSpanElement>()
                    .unwrap()
                    .set_inner_text(" ");

                inner_document
                    .body()
                    .unwrap()
                    .append_child(&new_div)
                    .unwrap();
            }
        }
    });

    document.add_listner("mousedown", {
        let down = down.clone();
        move |_| down.set(true)
    });

    document.add_listner("mouseup", {
        let down = down.clone();
        move |_| down.set(false)
    });

    App::render_single(ui());

    Ok(())
}
