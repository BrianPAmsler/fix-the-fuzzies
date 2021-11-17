use crate::{Universe, scheduler::Scheduler};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, HtmlInputElement, HtmlButtonElement, HtmlDivElement}

/// Messages that represent the methods to be called on the View
pub enum ViewMessage {
    UpdateUniverse(&Universe),
}

#[wasm_bindgen]
pub struct View {
    sched: RefCell<Rc<Scheduler>>,
    speed_slider: HTMLInputElement,
    play_pause: HTMLButtonElement,
    reset: HTMLButtonElement,
    kill_all: HTMLButtonElement,
    fps: HTMLDivElement,
    canvas: HTMLCanvasElement,
    callbacks: Vec<(web_sys::EventTarget, String, Closure<dyn FnMut()>)>,
}

impl View {
    pub fn new(sched: Rc<Scheduler>) -> Option<View> {
        Some(View {
            sched: RefCell::new(sched),
            speed_slider,
            play_pause,
            reset,
            main,
            toggle_all,
            new_todo,
            callbacks: Vec::new(),
        })
    }
}

impl 
pub fn init(&mut self) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };
    let sched = self.sched.clone();
    let set_page = Closure::wrap(Box::new(move || {
        if let Some(location) = document.location() {
            if let Ok(hash) = location.hash() {
                if let Ok(sched) = &(sched.try_borrow_mut()) {
                    sched.add_message(Message::Controller(ControllerMessage::SetPage(hash)));
                }
            }
        }
    }) as Box<dyn FnMut()>);

    self.bind_add_item();
}
