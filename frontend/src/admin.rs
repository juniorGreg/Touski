use yew::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use gloo_net::{ http::Request, Error };

pub struct Admin;

impl Component for Admin {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
        <>
          <header>
            <h1>{ "Admin" }</h1>
          </header>
          <main>
          </main>
        </>
    }
  }
}


