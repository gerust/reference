extern crate gerust_app;
extern crate gerust_controller;
extern crate gerust_router;
extern crate gerust_context;

use std::sync::Arc;
use gerust_controller::{Controller, ControllerFactory};
use gerust_router::Params;
use gerust_context::Context;

struct App<C: Context> {
    controllers: Vec<Arc<ControllerFactory<C, Controller=Controller<C, Params=Params, Result=Params>>>>
}