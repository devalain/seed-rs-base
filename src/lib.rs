use seed::{*, prelude::*};

pub enum Msg {
	NoMsg
}

struct Model {}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {}
}
fn update(msg: Msg, _model: &mut Model, orders: &mut impl Orders<Msg>) {
    orders.skip();
    match msg {
		Msg::NoMsg => {}
	}
}
fn view(_model: &Model) -> Vec<Node<Msg>> {
    nodes![div!["Hello world !"]]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
