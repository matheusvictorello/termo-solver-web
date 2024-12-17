use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::EventTarget;

#[derive(Debug)]
pub struct WindowEventListener {
    event:   &'static str,
    closure: Closure<dyn Fn(KeyboardEvent)>,
}

impl WindowEventListener {
    #[inline]
    pub fn onkeyup(closure: Closure<dyn Fn(KeyboardEvent)>) -> Self {
        Self::new("keyup", closure)
    }

    pub fn new(event: &'static str, closure: Closure<dyn Fn(KeyboardEvent)>) -> Self {
        let closure_ref = closure
            .as_ref()
            .unchecked_ref();

        let window = web_sys::window().expect("Expecting window");

        let _ = window
            .add_event_listener_with_callback(
                event,
                closure_ref,
            );

        Self {
            event,
            closure,
        }
    }
}

impl Drop for WindowEventListener {
    fn drop(&mut self) {
        let closure_ref = self.closure
            .as_ref()
            .unchecked_ref();

        let window = web_sys::window().expect("Expecting window");

        let _ = window
            .remove_event_listener_with_callback(
                self.event,
                closure_ref,
            );
    }
}