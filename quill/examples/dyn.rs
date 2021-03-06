use quill::*;

#[derive(Debug)]
struct App;

impl Hydrate for App {
    fn hydrate(&mut self, _other: &Self) {}
}

impl View for App {
    fn body(&self) -> AnyView {
        // let mut stack = Stack::new();
        // let mut stack1 = Stack::new();
        // let mut stack2 = Stack::new();
        // stack1.push(Level2::new());
        // stack2.push(Level2::new());
        // stack.push(stack1);
        // stack.push(stack2);
        // stack.push(Level3::new());
        // stack.any_view()
        Level2::new().any_view()
    }
}

#[derive(Debug)]
struct Level2 {
    state: State<bool>,
    property: u32,
}

impl Level2 {
    fn new() -> Self {
        Level2 {
            state: State::new(false),
            property: 0,
        }
    }
}

impl Hydrate for Level2 {
    fn hydrate(&mut self, other: &Self) {
        self.property = other.property;
    }
}

impl View for Level2 {
    fn body(&self) -> AnyView {
        let mut stack = Stack::new();
        stack.push(Level3::new(1 + if self.state.get() { 10 } else { 0 }));
        stack.push(Level3::new(2));
        if self.state.get() {
            stack.push(Level3::new(3));
        }
        stack.any_view()
    }
}

#[derive(Debug)]
struct Level3 {
    state: State<u32>,
    prop: u32,
}

impl Level3 {
    fn new(prop: u32) -> Self {
        Level3 {
            state: State::new(0),
            prop,
        }
    }
}

impl Hydrate for Level3 {
    fn hydrate(&mut self, other: &Self) {
        self.prop = other.prop;
    }
}

impl View for Level3 {
    fn body(&self) -> AnyView {
        Text::new("level 3").any_view()
    }
}

fn main() {
    // _layout_test()
    _hydrate_test()
}

fn _layout_test() {
    let tree = ViewTree::new(App.any_view());
    tree.layout(Constraint {
        min_width: 0.0,
        max_width: 100.0,
        min_height: 0.0,
        max_height: 100.0,
    });
    tree.set_offset(Offset::zero());
    tree.pretty_print();
}

fn _hydrate_test() {
    let mut tree = ViewTree::new(App.any_view());
    tree.pretty_print();

    {
        let level3 = tree.children[0].children[0].children[0]
            .view
            .downcast_ref::<Level3>()
            .expect("Type error");
        level3.state.set(10);
    }

    tree.perform_hydrate(App.any_view());
    tree.pretty_print();

    {
        let level2 = tree.children[0]
            .view
            .downcast_ref::<Level2>()
            .expect("Type error");
        level2.state.set(true);
    }

    tree.perform_hydrate(App.any_view());
    tree.pretty_print();
}
