use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use gloo_net::{ http::Request, Error };

mod index;
mod admin;
mod nav;
pub mod store;

use index::Index;
use admin::Admin;
use nav::{Nav, Route};


fn switch(routes: &Route) -> Html {
  match routes {
    Route::Index => html!{
      <Index />
    },
    Route::Admin => html! {
      <Admin />
    }
  }
}


#[function_component(App)]
fn app() -> Html {
  html! {
    <BrowserRouter>
       <Nav />
       <Switch<Route> render={Switch::render(switch)} />
       <footer>
          { "© 2022" }
       </footer>
    </BrowserRouter>
  }
}

fn main() {
  yew::start_app::<App>();
}
