#![feature(type_alias_impl_trait)]
use std::any::Any;
use std::mem::size_of;

use reanimate_ui::*;

// Build: Nothing -> ViewHierarchy
// Update:
//   Layout: upd ViewHierarchy
//   State: upd ViewHierarchy
// Render: ViewHierarchy -> UI

// Hydrate ViewHierarchy:
//   &mut ViewHierarchy<X> + X

// App
// |- X
// |- Y
//
// Create new App', and hydrate App + App'.
// Create new Body (X', Y'), and hydrate X + X', Y + Y'.
// Run finalizer from leafs to root.
// Run layout from root to leafs.

// ViewHierarchy<X> -> Vec<Layout>
// Ask: LayoutPriority
// Ask: LayoutFlex
// Constraints -> Size

#[derive(Debug, PartialEq, Clone)]
pub struct App {
    pub state: State<u32>, // smooth: Smooth<f32>,
                           // environment: Env<u32>,
                           // derived: bool
}

impl App {
    pub fn new() -> Self {
        eprintln!("New App");
        App {
            state: State::new(1),
        }
    }
    // pub fn view(&self) -> () {}
    pub fn view(&self) -> impl View {
        eprintln!("Build body: App");
        (
            StateLocalTest::new(10 * self.state.0),
            StateLocalTest::new(20),
        )
    }
}

// This trait will be derived.
impl Hydrate for App {
    fn hydrate(&mut self, _: Self) {
        eprintln!("hydrate App");
    }
}

impl View for App {
    type Body = impl View;
    fn body(&self) -> Self::Body {
        self.view()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TestView {}

impl TestView {
    pub fn new() -> Self {
        eprintln!("New TestView");
        TestView {}
    }
    pub fn view(&self) -> impl View {
        eprintln!("Building body: TestView");
        Text::new("text view")
    }
}

impl Hydrate for TestView {
    fn hydrate(&mut self, _: Self) {
        eprintln!("hydrate TestView");
    }
}
impl View for TestView {
    type Body = impl View;
    type Children = ViewHierarchy<Self::Body>;
    fn body(&self) -> Self::Body {
        self.view()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateLocalTest {
    state: State<u32>, // This value should persist across updates.
    derived: u32,      // This value should be derived from parents.
}

impl StateLocalTest {
    fn new(property: u32) -> Self {
        eprintln!("New StateLocalTest");
        StateLocalTest {
            state: State::new(0),
            derived: property * 2,
        }
    }
}

impl Hydrate for StateLocalTest {
    fn hydrate(&mut self, other: Self) {
        eprintln!("hydrate StateLocalTest");
        self.derived = other.derived;
    }
}
impl View for StateLocalTest {
    type Body = impl View;
    type Children = ViewHierarchy<Self::Body>;
    fn body(&self) -> Self::Body {
        eprintln!("Build body: StateLocalTest");
        ()
    }
}

fn main() {
    // println!("TypeId: {:?}", TypeId::of::<()>());
    // println!("Size: {:?}", size_of::<ViewHierarchy<App>>());
    // println!(
    //     "Size: {:?}",
    //     size_of::<ViewHierarchy<Placer<(Text, Text)>>>()
    // );
    // let hierarchy = ViewHierarchy::new(App {});
    // hierarchy.render();
    // dbg!(hierarchy);
    hydrate_test();
}

fn hydrate_test() {
    eprintln!("Building initial hierarchy:");
    let mut hierarchy = ViewHierarchy::new(App::new());
    eprintln!("\nHierarchy:");
    dbg!(&hierarchy);

    // hierarchy.view.state.0 = 1;
    {
        hierarchy.view.state.0 = 2;
        let view = hierarchy
            .children
            .view
            .assume::<(StateLocalTest, StateLocalTest)>();
        // view.0.state.0 = 1;
        // view.1.state.0 = 2;
        // let view: &mut (StateLocalTest, StateLocalTest) = (&mut hierarchy.children.view
        //     as &mut dyn Any)
        //     .downcast_mut()
        //     .unwrap();
        // view.0.state.0 = 1;
    }

    eprintln!("\nHydrating:");
    hierarchy.perform_hydrate(App::new());
    eprintln!("\nHierarchy:");
    dbg!(&hierarchy);
}

// fn get_child_view(hierarchy: ViewHierarchy<X>) -> X::Children
