use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::HtmlSpanElement;
use std::{cell::Cell, rc::Rc};

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

    {
        let div = div.clone();
        let div_style = div_style.clone();
        let inner_document = document.clone();
        let down = down.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            div_style
                .set_property("left", &format!("{}px", event.client_x()))
                .unwrap();
            div_style
                .set_property("top", &format!("{}px", event.client_y()))
                .unwrap();

            if down.get() {
                let new_div = div.clone_node().unwrap();
                new_div.dyn_ref::<HtmlSpanElement>().unwrap().set_inner_text(" ");

                inner_document.body().unwrap().append_child(&new_div).unwrap();
            }

        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let down = down.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            down.set(true);
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let down = down.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            down.set(false);
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
