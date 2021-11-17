use crate::view::ViewMessage;
use crate::{Message, Scheduler, Universe};
use euclid::Point2D;
use js_sys::Date;

use std::cell::RefCell;
use std::rc::Weak;

/// The controller of the application turns page state into functionality
pub struct Controller {
    universe,
    sched: RefCell<Option<Weak<Scheduler>>>,
}

pub enum GliderDirection {
    NW,
    NE,
    SE,
    SW,
}

/// Messages that represent the methods to be called on the Controller
pub enum ControllerMessage {
    Tick,
    KillAll,
    Reset,
    ToggleCell(Point2D),
    InsertPulsar(Point2D),
    InsertGlider(Point2D, GliderDirection),
}

impl Controller {
    pub fn new(universe: Universe, sched: Weak<Scheduler>) -> Controller {
        Controller {
            universe,
            sched: RefCell::new(Some(sched)),
        }
    }

    /// Used by `Scheduler` to convert a `ControllerMessage` into a function call on a `Controller`
    pub fn call(&mut self, method_name: ControllerMessage) {
        use self::ControllerMessage::*;
        match method_name {
            Tick => self.universe.tick(),
            KillAll => self.universe.kill_all(),
            Reset => self.universe.reset()
            ToggleCell(point) => self.universe.toggle_cell(point),
            InsertPulsar(point) => self.universe.insert_pulsar(point),
            InsertGlider(point, glider_direction) => self.universe.insert_glider(point, glider_direction),
        }
    }
}
