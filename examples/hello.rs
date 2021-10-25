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
    dbg!(hierarchy);
    println!("Size: {:?}", size_of::<ViewHierarchy<App>>());
}

/* Output:

[examples/hello.rs:29] hierarchy = ViewHierarchy {
    view: App,
    children: ViewHierarchy {
        view: (),
        children: (),
    },
}
Size: 0
*/
