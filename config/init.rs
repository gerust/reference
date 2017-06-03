use ::routes;
use gerust_controller::*;
use app::App;
use controllers::echo::EchoController;
use contexts::HttpContext;
use gerust_router::Params;

pub fn init() {
    println!("Initializing project");

    let mut app: App<HttpContext> = App::new();
    let factory = factory::<HttpContext, Params, Params, EchoController<HttpContext>>();
    app.add_controller(Box::new(factory));

    routes::draw();
}