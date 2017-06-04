use ::routes;
use gerust_controller::*;
use app::App;
use controllers::echo::EchoController;
use contexts::HttpContext;
use gerust_router::Params;

pub fn init() {
    println!("Initializing project");

    let mut app: App<HttpContext> = App::new();
    let factory = factory::<HttpContext, EchoController<HttpContext>>();
    app.controllers_mut().register_controller(Box::new(factory));

    routes::draw();
}