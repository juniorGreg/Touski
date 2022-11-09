use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::store::LocalStore;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Index,
  #[at("/admin")]
  Admin,
}

#[function_component(Nav)]
pub fn nav() -> Html {
  let (local_store, dispatch) = use_store::<LocalStore>();

  html!{
    <nav>
      <Link<Route> to={Route::Index}>{"Accueil"}</Link<Route>>
      if local_store.authorized {
        <Link<Route> to={Route::Admin}>{"Admin"}</Link<Route>>
        <button class={"logout"}>{"logout"}</button>
      }
      else {
        <button class={"login"}>{"login"}</button>
      }

    </nav>
  }
}
