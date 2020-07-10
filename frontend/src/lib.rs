use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue, closure::Closure, prelude::wasm_bindgen};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let mut divs = Vec::new();
    let mut styles = Vec::new();

    for _ in 0..16 {

        let div = document
            .create_element("span")?
            .dyn_into::<web_sys::HtmlSpanElement>()?;

        div.set_inner_text("Hej");
        div.set_class_name("badge badge-primary");

        document.body().unwrap().append_child(&div)?;

        let div_style = div.style();

        //div_style.set_property("background-color", "black")?;
        div_style.set_property("position", "absolute")?;
        //div_style.set_property("width", "64px")?;
        //div_style.set_property("height", "64px")?;
        div_style.set_property("top", "40px")?;
        div_style.set_property("left", "40px")?;

        divs.push(div);
        styles.push(div_style);
    }

    let divs = Rc::new(divs);
    let styles = Rc::new(styles);

    {
        let styles = styles.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            
            for (i, div_style) in styles.iter().enumerate() {
                div_style
                    .set_property("left", &format!("{}px", event.client_x() + (i % 4) as i32 * 32))
                    .unwrap();
                div_style
                    .set_property("top", &format!("{}px", event.client_y()+ (i / 4) as i32 * 24))
                    .unwrap();
            }

        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let divs = divs.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            
            for div in &*divs {
                div.set_class_name("badge badge-secondary");
            }

        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let divs = divs.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            
            for div in &*divs {
                div.set_class_name("badge badge-primary");
            }

        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
