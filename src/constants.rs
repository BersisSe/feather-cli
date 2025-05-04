pub const MAIN: &str = r#"
use feather::{App, Request, Response, AppContext};
use feather::middleware::MiddlewareResult;

fn main() {
    let mut app = App::new();

    app.get("/", |_req: &mut Request, res: &mut Response, _ctx: &mut AppContext| {
        res.send_text("🎉 Feather app is live!");
        MiddlewareResult::Next
    });

    app.listen("127.0.0.1:8080");
}
"#;