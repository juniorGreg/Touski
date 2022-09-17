use yew::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use gloo_net::{ http::Request, Error };
use wasm_bindgen::prelude::*;
use futures::executor::block_on;

pub enum TouskiEvent {
  AddIngredient,
  SearchRandomRecipe, 
  GetIngredientsHint,
  SetIngredientsHint(Vec<String>),
  ShowError(Error)
}

pub struct App {
  ingredients_vec: Vec<String>,
  ingredient_input: NodeRef,
  ingredients_hint: Vec<String>
}

impl Component for App {
  type Message = TouskiEvent;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    let ingredients_hint = vec!["patate".to_string(), "banane".to_string(), "papaye".to_string(), "anana".to_string(), "pitahaya".to_string()];
    Self {
      ingredients_vec: Vec::new(),
      ingredients_hint,
      ingredient_input: NodeRef::default()
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      TouskiEvent::AddIngredient => {
          if let Some(input) = self.ingredient_input.cast::<InputElement>() {
            let ingredient = input.value();
            if !ingredient.is_empty() {
               self.ingredients_vec.push(ingredient.trim().to_string());
            }
            input.set_value("");
          }
      }
      TouskiEvent::SearchRandomRecipe => {
        println!("search");
      },
      TouskiEvent::GetIngredientsHint => {
        if let Some(input) = self.ingredient_input.cast::<InputElement>() {
         let hint = input.value();

         if hint.is_empty() {
          return false;
         }

         ctx.link().send_future(async move {
            match App::get_ingredients_hint(&hint).await {
              Ok(new_ingredient_hint) => TouskiEvent::SetIngredientsHint(new_ingredient_hint),
              Err(err) => TouskiEvent::ShowError(err),
            }
         });
        }

        return false;
      },
      TouskiEvent::SetIngredientsHint(new_ingredient_hint) => {
        self.ingredients_hint = new_ingredient_hint;
      },
      TouskiEvent::ShowError(err) => println!("{}", err)

    }
    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let ingredient_txt = "ingredient";
    let ingredients_txt = "ingredients";
    let on_add_ingredient = ctx.link().callback(|_| TouskiEvent::AddIngredient);
    let on_search_random_recipe = ctx.link().callback(|_| TouskiEvent::SearchRandomRecipe);
    let on_get_ingredient_hint = ctx.link().callback(|_| TouskiEvent::GetIngredientsHint);

    html!{
      <>
        <header>
          <h1>{ "Touski" }</h1>
        </header>
        <main>
          <h2>{ "Recettes" }</h2>
          <section class="form">
            <label for={ ingredient_txt }>{ "Ajouter un ingrédient : " }</label>
            <input onkeypress={on_get_ingredient_hint} ref={self.ingredient_input.clone()} list={ ingredients_txt } name={ ingredient_txt } />
            <button onclick={on_add_ingredient}>{ "Ajouter" }</button>
            <datalist id={ ingredients_txt }>
              {  
                  self.ingredients_hint.iter().map(|ingredient_hint| {
                    html!{<option value={ ingredient_hint.to_string() } />}
                  }).collect::<Html>()
              }
            </datalist>
          </section>
          <ul>
            {
              self.ingredients_vec.iter().map(|ingredient| {
                html! {<li> {ingredient} </li>}
              }).collect::<Html>()
            }
          </ul>
          <button onclick={on_search_random_recipe}>{ "Chercher une recette aléatoire" }</button>
        </main>
        <footer>
          { "© 2022" }
        </footer>
      </>
    }
  }  
}

impl App {
  async fn get_ingredients_hint(hint: &str) -> Result<Vec<String>, Error> {
    
    let mut query = "/api/ingredients/".to_owned();
    query.push_str(hint);

    let resp = Request::get(&query).send().await.unwrap();
    
    let ingredients_hint: Vec<String> = resp.json().await.unwrap();

    Ok(ingredients_hint)    
  }
}
fn main() {
  yew::start_app::<App>();
}
