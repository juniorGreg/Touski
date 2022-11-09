use yewdux::prelude::*;

#[derive(Default, PartialEq, Store)]
pub struct LocalStore {
  pub authorized: bool,
}

