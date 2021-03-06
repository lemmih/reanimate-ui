#![feature(type_alias_impl_trait)]

use reanimate_ui::*;

#[derive(Debug, PartialEq, Clone)]
pub struct App {
    pub root_state: State<u32>,
    pub root_ro: u32,
}

impl App {
    pub fn new() -> Self {
        App {
            root_state: State::new(0),
            root_ro: 1,
        }
    }
}

impl View for App {
    type Body = impl View;
    fn body(&self) -> Self::Body {
        Level2::new(self.root_ro + *self.root_state.borrow())
    }
}

// This trait will be derived.
impl Hydrate for App {
    fn hydrate(&mut self, _: Self) {}
}

#[derive(Debug, PartialEq, Clone)]
pub struct Level2 {
    pub view_state: State<u32>,
    pub view_prop: u32,
}

impl Level2 {
    pub fn new(prop: u32) -> Self {
        Level2 {
            view_state: State::new(0),
            view_prop: prop,
        }
    }
}

impl View for Level2 {
    type Body = impl View;
    fn body(&self) -> Self::Body {
        Value::new(*self.view_state.borrow() + self.view_prop)
    }
}

// This trait will be derived.
impl Hydrate for Level2 {
    fn hydrate(&mut self, other: Self) {
        self.view_prop = other.view_prop;
    }
}

fn main() {
    eprintln!("Initial state:");
    let mut hierarchy = ViewHierarchy::new(App::new());
    // dbg!(&hierarchy);
    hierarchy.pretty_print();

    eprintln!("\nSetting App.root_state = 10");
    *hierarchy.view.root_state.borrow_mut() = 10;
    hierarchy.perform_hydrate(App::new());
    // dbg!(&hierarchy);
    hierarchy.pretty_print();

    eprintln!("\nSetting App.Level2.view_state = 100");
    *hierarchy
        .children
        .view
        .assume::<Level2>()
        .view_state
        .borrow_mut() = 100;
    hierarchy.perform_hydrate(App::new());
    // dbg!(&hierarchy);
    hierarchy.pretty_print();
}

/* Output:

Initial state:
App { root_state: State(0), root_ro: 1 }
└─ Level2 { view_state: State(0), view_prop: 1 }
   └─ Value(1)

Setting App.root_state = 10
App { root_state: StateDirty(10), root_ro: 1 }
└─ Level2 { view_state: State(0), view_prop: 11 }
   └─ Value(11)

Setting App.Level2.view_state = 100
App { root_state: StateDirty(10), root_ro: 1 }
└─ Level2 { view_state: StateDirty(100), view_prop: 11 }
   └─ Value(111)
*/
