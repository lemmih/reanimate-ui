#![feature(type_alias_impl_trait)]

use reanimate_ui::*;
use std::mem::size_of;

#[derive(Debug, PartialEq, Clone)]
pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }
}

impl View for App {
    type Body = impl View;
    fn body(&self) -> Self::Body {
        ()
    }
}

// This trait will be derived.
impl Hydrate for App {
    fn hydrate(&mut self, _: Self) {}
}

fn main() {
    let hierarchy = ViewHierarchy::new(App::new());
    hierarchy.pretty_print();
    println!("Size (bytes): {:?}", size_of::<ViewHierarchy<App>>());
}

/* Output:

App
Size (bytes): 0
*/
