use yew::prelude::*;
use yew::{html, Component, Context, Html};


// Create the main app that will load all other Components
pub struct App {
}

// Message enum that is used for managing the life cycle of Components
pub enum Msg {
}

// Implement the Component interface
impl Component for App {
    type Message = Msg;
    type Properties = ();

    // Create a new App
    fn create(_ctx: &Context<Self>) -> Self {
        App {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Creates The HTML that will show up in the browser.
        html! {
              <p> {"Hello World"} </p>
        }
    }
}

// Entry point for starting the Yew application
pub fn main() {
    // Create the logger
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    // Start the Yew framework
    yew::Renderer::<App>::new().render();
}

