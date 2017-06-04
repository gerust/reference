extern crate gerust_app;
extern crate gerust_controller;
extern crate gerust_router;
extern crate gerust_context;

use gerust_controller::{Controller, ControllerFactory, ControllerSet};
use gerust_router::Params;
use gerust_context::Context;

pub struct App<'factories, 'controllers, C: Context> {
    controllers: ControllerSet<'factories, 'controllers, C, Params, Params>,
}

impl<'factories, 'controllers, C: Context + 'controllers> App<'factories, 'controllers, C> {
    pub fn new() -> Self {
        App { controllers: ControllerSet::new() }
    }

    pub fn controllers_mut(&mut self) -> &mut ControllerSet<'factories, 'controllers, C, Params, Params> {
        &mut self.controllers
    }
}
