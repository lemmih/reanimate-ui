// ReanimateUI design:
//
// A user interface is a tree of views. This tree is completely static with
// the exception of list views which can have N children. Note: the type
// of those children is static.
//
// Views can contain:
//   * read-only properties. These can contain data from the parent view.
//     Example: the text of a Text view.
//   * local state. The state is persistent even across changes to the read-only
//     parameters.
//   * environmental data. The data is set at an arbitrary point higher up the
//     view hierarchy.
//   * interpolated values. Similar to the read-only properties expect they
//     smooth interpolate when changed.
//
// UI loop:
// 1. Create new ViewHierarchy
// 2. Layout ViewHierarchy
// 3. Render ViewHierarchy
// 4. If no state changes, go to step 3. Layout cannot change if there are no
//    state changes.
// 5. Update ViewHierarchy, go to step 2.

#![feature(type_alias_impl_trait)]
#![feature(associated_type_defaults)]
// #![allow(dead_code)]

use std::any::Any;
use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////////
// Hydrate

// Copy variables from 'new' into self. Don't override state variables.
pub trait Hydrate {
    fn hydrate(&mut self, new: Self);
}

////////////////////////////////////////////////////////////////////////////////
// Render

pub trait Render {
    fn render(&self);
}

impl Render for () {
    fn render(&self) {}
}

impl<A: Render, B: Render> Render for (A, B) {
    fn render(&self) {
        self.0.render();
        self.1.render();
    }
}

impl<X: View> Render for ViewHierarchy<X> {
    fn render(&self) {
        let ViewHierarchy { view, children } = self;
        View::render(view, children)
    }
}

////////////////////////////////////////////////////////////////////////////////
// View helpers (HasBody, Hydrateable, Buildable)

pub trait HasBody {
    type Body;
    fn clone_body(&self) -> Self::Body;
}

impl HasBody for () {
    type Body = ();
    fn clone_body(&self) -> Self::Body {
        ()
    }
}

impl<A: HasBody, B: HasBody> HasBody for (A, B) {
    type Body = (A::Body, B::Body);
    fn clone_body(&self) -> Self::Body {
        (self.0.clone_body(), self.1.clone_body())
    }
}

impl<X: View> HasBody for ViewHierarchy<X> {
    type Body = X;
    fn clone_body(&self) -> Self::Body {
        self.view.clone()
    }
}

pub trait Hydrateable: HasBody {
    fn perform_hydrate(&mut self, new: Self::Body);
}

impl Hydrateable for () {
    fn perform_hydrate(&mut self, _new: Self::Body) {}
}

impl<A: Hydrateable, B: Hydrateable> Hydrateable for (A, B) {
    fn perform_hydrate(&mut self, new: Self::Body) {
        let (a, b) = self;
        a.perform_hydrate(new.0);
        b.perform_hydrate(new.1);
    }
}

impl<X: View> Hydrateable for ViewHierarchy<X> {
    fn perform_hydrate(&mut self, new: Self::Body) {
        let ViewHierarchy { view, children } = self;
        if view == &new {
            // eprintln!("View didn't change. Cloning previous body.");
            Hydrateable::perform_hydrate(children, children.clone_body());
        } else {
            Hydrate::hydrate(view, new);
            Hydrateable::perform_hydrate(children, view.body());
        }
    }
}

pub trait Buildable: HasBody {
    fn build_children(from: Self::Body) -> Self;
}

impl Buildable for () {
    fn build_children(_from: Self::Body) -> Self {}
}

impl<A: Buildable, B: Buildable> Buildable for (A, B) {
    fn build_children(from: Self::Body) -> Self {
        (
            Buildable::build_children(from.0),
            Buildable::build_children(from.1),
        )
    }
}

impl<X: View> Buildable for ViewHierarchy<X> {
    fn build_children(from: Self::Body) -> Self {
        ViewHierarchy::new(from)
    }
}

////////////////////////////////////////////////////////////////////////////////
// View

pub trait View: Any + Debug + Hydrate + Sized + PartialEq + Clone
where
    Self::Body: View,
    Self::Children: Render + Hydrateable + HasBody<Body = Self::Body>,
    Self::Children: Buildable + Debug + Sized,
{
    type Body;
    type Children = ViewHierarchy<Self::Body>;
    fn body(&self) -> Self::Body;
    fn render(&self, children: &Self::Children) {
        println!("Default render for: {}", std::any::type_name::<Self>());
        children.render()
    }
    fn build_children(&self) -> Self::Children {
        Buildable::build_children(self.body())
    }
    fn initialize(&self) {}
    fn finalize(&self) {}

    // Unpack 'impl View' into a specific type. Doesn't fail at compile-time but
    // no checking happens at run-time.
    // This function is only used during testing.
    fn assume<X: 'static>(self: &mut Self) -> &mut X {
        let self_dyn = self as &mut dyn Any;
        self_dyn.downcast_mut::<X>().expect("static type error")
    }
}

impl<A: View, B: View> View for (A, B) {
    type Body = (A::Body, B::Body);
    type Children = (A::Children, B::Children);
    fn body(&self) -> Self::Body {
        (self.0.body(), self.1.body())
    }
    fn render(&self, children: &Self::Children) {
        View::render(&self.0, &children.0);
        View::render(&self.1, &children.1);
    }
}

impl<A: View, B: View> Hydrate for (A, B) {
    fn hydrate(&mut self, previous: Self) {
        self.0.hydrate(previous.0);
        self.1.hydrate(previous.1);
    }
}

impl View for () {
    type Body = ();
    type Children = ();
    fn body(&self) -> Self::Body {
        ()
    }
    fn build_children(&self) -> Self::Children {
        ()
    }
}

impl Hydrate for () {
    fn hydrate(&mut self, _: Self) {}
}

////////////////////////////////////////////////////////////////////////////////
// Text view

#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Text {
            text: text.to_string(),
        }
    }
}

impl View for Text {
    type Body = ();
    type Children = ();
    fn body(&self) -> Self::Body {
        ()
    }
}

impl Hydrate for Text {
    fn hydrate(&mut self, _: Self) {}
}

////////////////////////////////////////////////////////////////////////////////
// Layout

// // LayoutFlex
// pub struct LayoutFlex<X> {
//     pub flex: u32,
//     pub view: X,
// }

// pub struct GetLayoutFlex {
//     pub set_flex: fn(u32),
// }

// pub struct EnvVar {
//     pub some_value: u32,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Layout {
//     pub offset_x: f32,
//     pub offset_y: f32,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Placer<X: View> {
//     pub layout: Layout,
//     pub view: X,
// }

// impl<X: View> View for Placer<X> {
//     type Body = X::Body;
//     type Children = X::Children;
//     fn body(&self) -> Self::Body {
//         self.view.body()
//     }
//     fn build_children(&self) -> Self::Children {
//         self.view.build_children()
//     }
//     fn render(&self, children: &Self::Children) {
//         println!("Container: {}", std::any::type_name::<Self>());
//         View::render(&self.view, children)
//     }
// }

// impl<X: View> Hydrate for Placer<X> {
//     fn hydrate(&mut self, _: Self) {}
// }

////////////////////////////////////////////////////////////////////////////////
// ViewHierarchy

#[derive(Debug)]
pub struct ViewHierarchy<X: View> {
    pub view: X,
    pub children: X::Children,
}

impl<X: View> ViewHierarchy<X> {
    pub fn new(view: X) -> ViewHierarchy<X> {
        let children = view.build_children();
        ViewHierarchy { view, children }
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct InternalTestView {}

// impl Hydrate for InternalTestView {
//     fn hydrate(&mut self, _: Self) {}
// }

// impl View for InternalTestView {
//     type Body = impl View;
//     type Children = ViewHierarchy<Self::Body>;
//     fn body(&self) -> Self::Body {
//         ()
//     }
// }

// pub fn print_view() {
//     let hierarchy = ViewHierarchy::new(InternalTestView {});
//     hierarchy.render();
//     // dbg!(hierarchy);
// }

////////////////////////////////////////////////////////////////////////////////
// State

#[derive(Debug, Clone, PartialEq)]
pub struct State<X>(pub X);

impl<X> State<X> {
    pub fn new(value: X) -> Self {
        State(value)
    }
}

impl<X> Hydrate for State<X> {
    fn hydrate(&mut self, _other: Self) {}
}

// // Check if 'assume' is zero-cost.
// // Run: cargo asm --rust reanimate_ui::downcast_test
// pub fn downcast_test() -> bool {
//     let mut x = InternalTestView {}.body();
//     let view: &mut () = x.assume();
//     true
// }
