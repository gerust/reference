use ::routes;
use gerust_controller::*;
use app::App;
use controllers::echo::EchoController;
use contexts::HttpContext;

pub fn init() {
    println!("Initializing project");

    let app: App<HttpContext> = App::new();
    let factory = construct::<HttpContext>()
                        .for_controller::<EchoController<_>>();
    
    app.add_controller(Box::new(factory));

    routes::draw();
}