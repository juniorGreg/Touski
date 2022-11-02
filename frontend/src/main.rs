use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use gloo_net::{ http::Request, Error };

mod index;
mod admin;

use index::Index;
use admin::Admin;

#[derive(Clone, Routable, PartialEq)]
enum Route {
  #[at("/")]
  Index,
  #[at("/admin")]
  Admin,
}

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

#[function_component(Nav)]
fn nav() -> Html {
  html!{
    <nav>
      <Link<Route> to={Route::Index}>{"Accueil"}</Link<Route>>
      <Link<Route> to={Route::Admin}>{"Admin"}</Link<Route>>
    </nav>
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
