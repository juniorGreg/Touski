use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use gloo_net::{ http::Request, Error };
mod index;

use index::Index;

#[derive(Clone, Routable, PartialEq)]
enum Route {
  #[at("/")]
  Index,
  #[at("/admin")]
  Admin,
}

pub enum TouskiEvent {
  AddIngredient,
  SearchRandomRecipe, 
  GetIngredientsHint,
  SetIngredientsHint(Vec<String>),
  ShowError(Error)
}

#[function_component(Admin)]
fn admin() -> Html {
  html! {
      <div>
        <h1>{ "Admin" }</h1>
      </div>
  }
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

#[function_component(App)]
fn app() -> Html {
  html! {
    <BrowserRouter>
       <Switch<Route> render={Switch::render(switch)} /> 
    </BrowserRouter>
  }
}

fn main() {
  yew::start_app::<App>();
}
