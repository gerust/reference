extern crate gerust_app;
extern crate gerust_controller;
extern crate gerust_router;
extern crate gerust_context;

use gerust_controller::{Controller, ControllerFactory, ControllerSet};
use gerust_router::Params;
use gerust_context::Context;

pub struct App<C: Context> {
    controllers: ControllerSet<C, Params, Params>,
}

impl<C: Context> App<C> {
    pub fn new() -> Self {
        App { controllers: ControllerSet::new() }
    }

    pub fn controllers_mut(&mut self) -> &mut ControllerSet<C, Params, Params> {
        &mut self.controllers
    }
}
