use yew::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use gloo_net::{ http::Request, Error };

pub struct Admin {
  ingredient_input: NodeRef,
}

impl Admin {
  async fn add_ingredient_on_database(new_ingredient: &str) -> Result<(), Error> {
    
    let mut query = "/api/ingredients/".to_owned();
    query.push_str(new_ingredient);

    Request::post(&query).send().await.unwrap();
    Ok(())
  }
}

pub enum TouskiAdminEvent {
  AddIngredientOnDatabase,
  ShowError(Error),
  Success(())
}

impl Component for Admin {
  type Message = TouskiAdminEvent;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      ingredient_input: NodeRef::default()
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      TouskiAdminEvent::AddIngredientOnDatabase => {
        if let Some(input) = self.ingredient_input.cast::<InputElement>() {
           let ingredient = input.value();
           
           if ingredient.is_empty() {
             return false;
           }

           ctx.link().send_future(async move {
             match Admin::add_ingredient_on_database(&ingredient).await {
               Ok(()) => TouskiAdminEvent::Success(()),
               Err(err) => TouskiAdminEvent::ShowError(err),
             }
           });

           input.set_value("");
         }
      },
      TouskiAdminEvent::Success(()) => println!("Ingredient added to database."),
      TouskiAdminEvent::ShowError(err) => println!("{}", err)
    }
    false
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let ingredient_txt = "ingredient";
    let on_add_ingredient = ctx.link().callback(|_| TouskiAdminEvent::AddIngredientOnDatabase);
    html! {
        <>
          <header>
            <h1>{ "Admin" }</h1>
          </header>
          <main>
             <section class="form">
                <label for={ ingredient_txt }>{ "Ajouter un ingrédient dans la base de donnée : " }</label>

                <input ref={self.ingredient_input.clone()} name={ ingredient_txt } />
                <button onclick={ on_add_ingredient }>{ "Ajouter" }</button>
             </section>
          </main>
        </>
    }
  }
}


