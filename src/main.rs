mod server;
mod tests;

#[macro_use]
extern crate rocket;

use server::api_server::app;

#[launch]
fn rocket() -> _ {
    app()
}
