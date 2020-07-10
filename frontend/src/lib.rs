use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue, closure::Closure, prelude::wasm_bindgen};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let div = document
        .create_element("div")?
        .dyn_into::<web_sys::HtmlDivElement>()?;

    document.body().unwrap().append_child(&div)?;

    div.style().set_property("background-color", "black")?;
    div.style().set_property("position", "absolute")?;
    div.style().set_property("width", "64px")?;
    div.style().set_property("height", "64px")?;
    div.style().set_property("top", "40px")?;
    div.style().set_property("left", "40px")?;

    let div = Rc::new(div);

    {
        let div = div.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            
            let target = event.target().unwrap();

            if target.dyn_ref::<JsValue>() != div.dyn_ref::<JsValue>() {

                div.style()
                    .set_property("left", &format!("{}px", event.offset_x()))
                    .unwrap();
                div.style()
                    .set_property("top", &format!("{}px", event.offset_y()))
                    .unwrap();

                web_sys::console::log_1(&event);
            }

        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
