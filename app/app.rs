extern crate gerust_app;
extern crate gerust_controller;
extern crate gerust_router;
extern crate gerust_context;

use gerust_controller::{Controller, ControllerFactory};
use gerust_router::Params;
use gerust_context::Context;

pub struct App<'factories, 'controllers, C: Context> {
    controllers: Vec<Box<ControllerFactory<'factories, 'controllers, C, Params, Params, Box<Controller<'controllers, C, Params=Params, Result=Params> + 'controllers>> + 'factories>>,
}

impl<'factories, 'controllers, C: Context + 'controllers> App<'factories, 'controllers, C> {
    pub fn new() -> Self {
        App { controllers: Vec::new() }
    }

    pub fn add_controller(&mut self, controller: Box<ControllerFactory<'factories, 'controllers, C, Params, Params, Box<Controller<'controllers, C, Params=Params, Result=Params> + 'controllers>> + 'factories>) {
        self.controllers.push(controller)
    }

    pub fn controllers(&self) -> &[Box<ControllerFactory<'factories, 'controllers, C, Params, Params, Box<Controller<'controllers, C, Params=Params, Result=Params> + 'controllers>> + 'factories>] {
        self.controllers.as_ref()
    }
}


impl<'factories, 'controllers, C: Context> gerust_app::App for App<'factories, 'controllers, C> {

}