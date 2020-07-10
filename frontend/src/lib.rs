use std::rc::Rc;
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::HtmlSpanElement;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = Rc::new(web_sys::window().unwrap().document().unwrap());

    let div = Rc::new(document
        .create_element("span")?
        .dyn_into::<HtmlSpanElement>()?);

    div.set_inner_text("Hej");
    div.set_class_name("badge badge-primary");

    document.body().unwrap().append_child(&div)?;

    let div_style = Rc::new(div.style());

    //div_style.set_property("background-color", "black")?;
    div_style.set_property("position", "absolute")?;
    //div_style.set_property("width", "64px")?;
    //div_style.set_property("height", "64px")?;
    div_style.set_property("top", "40px")?;
    div_style.set_property("left", "40px")?;

    {
        let div_style = div_style.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            div_style
                .set_property("left", &format!("{}px", event.client_x()))
                .unwrap();
            div_style
                .set_property("top", &format!("{}px", event.client_y()))
                .unwrap();
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let div = div.clone();
        let inner_document = document.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            div.set_class_name("badge badge-secondary");

            inner_document.body().unwrap().append_child(&div.clone()).unwrap();

        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let div = div.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            div.set_class_name("badge badge-primary");
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
