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
        Value::new(10).attach("string")
    }
}

// This trait will be derived.
impl Hydrate for App {
    fn hydrate(&mut self, _: Self) {}
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attached<X: View> {
    pub new_property: String,
    pub view: X,
}

impl<X: View> Attached<X> {
    pub fn new(str: impl Into<String>, view: X) -> Self {
        Attached {
            new_property: str.into(),
            view,
        }
    }
}

impl<X: View> View for Attached<X> {
    type Children = X::Children;
    type Body = X::Body;
    fn body(&self) -> Self::Body {
        self.view.body()
    }

    fn build_children(&self) -> Self::Children {
        self.view.build_children()
    }
}

trait Attachable: View {
    fn attach(self, str: impl Into<String>) -> Attached<Self> {
        Attached::new(str, self)
    }
}
impl<X: View> Attachable for X {}

// This trait will be derived.
impl<X: View> Hydrate for Attached<X> {
    fn hydrate(&mut self, _: Self) {}
}

fn main() {
    let hierarchy = ViewHierarchy::new(App::new());
    hierarchy.pretty_print();
    println!("Size (bytes): {:?}", size_of::<ViewHierarchy<App>>());
}

/* Output:

ViewHierarchy
└─ App
   └─ Attached { new_property: "string", view: Value(10) }
Size (bytes): 32
*/
