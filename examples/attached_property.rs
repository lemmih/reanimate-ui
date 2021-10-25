#![feature(type_alias_impl_trait)]

use reanimate_ui::*;
use std::mem::size_of;

#[derive(Debug, PartialEq, Clone)]
pub struct App {}

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

trait Attachable: View {
    fn attach(self, str: impl Into<String>) -> Property<String, Self> {
        Property {
            property: str.into(),
            view: self,
        }
    }
}
impl<X: View> Attachable for X {}

fn main() {
    let hierarchy = ViewHierarchy::new(App {});
    hierarchy.pretty_print();
    println!("Size (bytes): {:?}", size_of::<ViewHierarchy<App>>());
}

/* Output:

ViewHierarchy
└─ App
   └─ Property { property: "string", view: Value(10) }
Size (bytes): 32
*/
