use gerust_controller::Controller;
use gerust_http::HttpContext;
// this needs to be replaced
use gerust_router::Params;

pub struct EchoController<'a, C: HttpContext + 'a> {
    context: &'a C   
}

impl<'a, C: HttpContext + 'a> Controller<'a, C> for EchoController<'a, C> {
    type Params = Params;
    type Result = Params;

    fn new(context: &'a C) -> Self { EchoController { context } }

    fn execute(&self, input: Params) -> Params {
        let mut params = Params::new();
        params.insert("echo".into(), "echo".into());
        params
    }
}