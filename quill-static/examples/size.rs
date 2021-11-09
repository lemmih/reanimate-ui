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

/*
VStack
└─ Text

VStack sets environment object with callback.
Text registers itself in the initializer.
The finalizer

Parent:
  1. Gets constraints
  2. Iterates constraints to children.
  3. Store the sizes for rendering.
Child:


Goals:
  Give every node a size.
  Give each node a relative offset.

Children -> Vec<Layout-able>

(Constraint + Node) -> Size
Layout-able:
  Set size: Constraint -> Size
  Set offset: Offset -> ()
  Priority
  Optional<Flex>

Use-cases:
  .offset(x, y)
  .layout_priority(priority)
  .flex(flex)
*/

fn main() {
    let hierarchy = ViewHierarchy::new(App::new());
    hierarchy.pretty_print();
    println!("Size (bytes): {:?}", size_of::<ViewHierarchy<App>>());
}

/* Output:

ViewHierarchy
└─ App
Size (bytes): 0
*/
