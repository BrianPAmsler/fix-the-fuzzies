extern crate rand;
extern crate web_sys;
mod utils;
mod webgl;

use std::rc::Weak;
use utils::Timer;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use webgl::WebGLRenderer;

use std::rc::Rc;

/// Controller of the program
pub mod controller;
/// Scheduler
pub mod scheduler;
/// Presentation layer
pub mod webglview;
use crate::controller::{Controller, ControllerMessage};
use crate::scheduler::Scheduler;
use crate::store::Store;
use crate::view::{View, ViewMessage};

pub enum Message {
    Controller(ControllerMessage),
    View(ViewMessage),
}

/// Used for debugging to the console
pub fn exit(message: &str) {
    let v = wasm_bindgen::JsValue::from_str(&message.to_string());
    web_sys::console::exception_1(&v);
    std::process::abort();
}

fn app(name: &str) {
    let sched = Rc::new(Scheduler::new());
    let store = match Store::new(name) {
        Some(s) => s,
        None => return,
    };
    let controller = Controller::new(store, Rc::downgrade(&sched));
    if let Some(mut view) = View::new(Rc::clone(&sched)) {
        let sch: &Rc<Scheduler> = &sched;
        view.init();
        sch.set_view(view);
        sch.set_controller(controller);
        sched.add_message(Message::Controller(ControllerMessage::SetPage(
            "".to_string(),
        )));
    }
}

/// Entry point into the program from JavaScript
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    app("todos-wasmbindgen");

    Ok(())
}

#[allow(dead_code)]
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}

struct Event;

trait Observable {
    fn register(&mut self, observer: Weak<dyn Observer>);
}

trait Observer {
    fn notify(&self, event: &Event);
}

pub trait UniverseRenderer {
    fn render(&mut self, universe: &Universe) -> Result<(), JsValue>;
    fn get_cell_index(&self, x: u32, y: u32) -> (u32, u32);
}

#[wasm_bindgen]
pub struct UniverseController {
    universe: Universe,
    renderer: Box<dyn UniverseRenderer>,
}

#[wasm_bindgen]
impl UniverseController {
    pub fn new(canvas: HtmlCanvasElement, width: u32, height: u32) -> UniverseController {
        let renderer = WebGLRenderer::new(canvas, width, height).unwrap();
        UniverseController {
            universe: Universe::new(width, height),
            renderer: Box::new(renderer),
        }
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        self.renderer.render(&self.universe)
    }

    pub fn tick(&mut self) {
        self.universe.tick();
    }

    pub fn reset(&mut self) -> Result<(), JsValue> {
        self.universe.reset();
        self.renderer.render(&self.universe)
    }

    pub fn kill_all(&mut self) -> Result<(), JsValue> {
        self.universe.kill_all();
        self.renderer.render(&self.universe)
    }

    pub fn toggle_cell(&mut self, x: u32, y: u32) {
        let (x, y) = self.renderer.get_cell_index(x, y);
        self.universe.toggle_cell(x, y)
    }

    pub fn insert_pulsar(&mut self, x: u32, y: u32) -> Result<(), JsValue> {
        let (x, y) = self.renderer.get_cell_index(x, y);
        self.universe.insert_pulsar(x, y);
        self.renderer.render(&self.universe)
    }

    pub fn insert_glider(&mut self, x: u32, y: u32) -> Result<(), JsValue> {
        let (x, y) = self.renderer.get_cell_index(x, y);
        self.universe.insert_glider(x, y);
        self.renderer.render(&self.universe)
    }
}
