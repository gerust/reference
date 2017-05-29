extern crate gerust_app;
extern crate gerust_controller;
extern crate gerust_router;
extern crate gerust_context;

use gerust_controller::{Controller, ControllerFactory};
use gerust_router::Params;
use gerust_context::Context;

pub struct App<C: Context> {
    controllers: Vec<Box<ControllerFactory<C, Controller=Controller<C, Params=Params, Result=Params>>>>,
}

impl<C: Context> gerust_app::App for App<C> {
    type ControllerFactory = ControllerFactory<C, Controller=Controller<C, Params=Params, Result=Params>>;

    fn controllers(&self) -> &[Box<Self::ControllerFactory>] {
        &self.controllers
    }
}