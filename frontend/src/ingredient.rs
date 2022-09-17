use yew::prelude::*;


pub struct Ingredient;

impl Component for Ingredient {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <li>{"Tomate"}</li>
    }
  }
}
