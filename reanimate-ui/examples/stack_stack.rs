use reanimate_ui::*;

#[derive(Debug)]
struct App;

impl View for App {
    fn body(&self) -> AnyView {
        let mut stack = Stack::new();
        stack.push(
            Stack::new()
                .with_child(Text::new("1"))
                .with_child(Text::new("2")),
        );
        stack.push(
            Stack::new()
                .with_child(Text::new("3"))
                .with_child(Text::new("4")),
        );
        stack.any_view()
    }
}

fn main() {
    layout_test()
}

fn layout_test() {
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
