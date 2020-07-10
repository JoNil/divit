use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{EventTarget, MouseEvent};

pub trait EventTargetEx {
    fn add_listner(&self, event_type: &str, callback: impl FnMut(MouseEvent) + 'static);
}

impl<AsRefEventTarget> EventTargetEx for AsRefEventTarget
where
    AsRefEventTarget: AsRef<EventTarget>,
{
    fn add_listner(&self, event_type: &str, callback: impl FnMut(MouseEvent) + 'static) {
        let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
        self.as_ref()
            .add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}
