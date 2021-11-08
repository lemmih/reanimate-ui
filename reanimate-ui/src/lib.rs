use core::panic::Location;
use std::any::Any;
use std::any::TypeId;
use std::cell::Cell;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
// use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use ptree::{item::StringItem, print_tree, TreeBuilder};

// mod smooth;

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn zero() -> Size {
        Size {
            width: 0.0,
            height: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Constraint {
    pub min_width: f64,
    pub max_width: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl Constraint {
    pub fn new(width: f64, height: f64) -> Self {
        Constraint {
            min_width: 0.0,
            max_width: width,
            min_height: 0.0,
            max_height: height,
        }
    }

    pub fn sub_width(&mut self, width: f64) {
        self.max_width = (self.max_width - width).clamp(self.min_width, f64::INFINITY)
    }

    pub fn sub_height(&mut self, width: f64) {
        self.max_height = (self.max_width - width).clamp(self.min_height, f64::INFINITY)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    pub x: f64,
    pub y: f64,
}

impl Offset {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/*
Hydrate ViewTree.
  If view has changed:
    Generate new body.
    Align children with store.
    Hydrate children.
  Else:
    Walk store to find dirty views.
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Key(pub u32);

impl Key {
    #[track_caller]
    pub fn new() -> Key {
        Key(Location::caller().line())
    }
}

pub type Store = HashMap<Key, ViewTree>;

pub enum Event {
    MousePress(MouseButton, f64, f64),
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
    WheelUp,
    WheelDown,
}

pub trait View: AsAny + Debug + 'static {
    fn body(&self) -> AnyView {
        // AnyView(Box::new(EmptyView))
        AnyView::new(EmptyView)
    }
    #[track_caller]
    fn key(&self) -> Key {
        // Key::new()
        panic!("Missing key!");
    }
    #[track_caller]
    fn children<'a>(self: &'a Self) -> Vec<AnyView> {
        vec![self.body()]
    }

    fn event(&self, _size: Size, _children: &[ViewTree], _event: &Event) {}

    fn hydrate_single(&mut self, _other: AnyView) {}

    fn layout(&self, children: &[ViewTree], constraint: Constraint) -> Size {
        if let [child] = children {
            child.layout(constraint);
            child.view.size.get()
        } else {
            // eprintln!("Can't decide widget size: {:?}", self);
            Size::zero()
        }
    }

    fn set_offset(&self, children: &[ViewTree], offset: Offset) {
        for ViewTree { view, children } in children {
            view.borrow().set_offset(children, offset)
        }
    }

    fn is_dirty(&self) -> bool {
        true
    }

    fn clean(&self) {}

    fn is_same(&self, _other: &AnyView) -> bool {
        false
    }
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct EmptyView;
impl View for EmptyView {
    // fn children(&self) -> ChildIter {
    //     Box::new(std::iter::empty())
    // }
    fn children(&self) -> Vec<AnyView> {
        Vec::new()
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    text: String,
    offset: Cell<Offset>,
}

impl Text {
    pub fn new(text: impl ToString) -> Text {
        Text {
            text: text.to_string(),
            offset: Cell::new(Offset { x: 0.0, y: 0.0 }),
        }
    }
}

impl View for Text {
    // fn children(&self) -> ChildIter {
    //     Box::new(std::iter::empty())
    // }
    fn children(&self) -> Vec<AnyView> {
        Vec::new()
    }

    fn layout(&self, _children: &[ViewTree], _constraint: Constraint) -> Size {
        Size {
            width: self.text.len() as f64,
            height: 1.0,
        }
    }

    fn set_offset(&self, _children: &[ViewTree], offset: Offset) {
        self.offset.set(offset)
    }
}

#[derive(Debug)]
pub struct Stack {
    // key: Key,
    pub children: Vec<AnyView>,
}

impl Stack {
    #[track_caller]
    pub fn new() -> Stack {
        Stack {
            // key: Key::new(),
            children: vec![],
        }
    }

    #[track_caller]
    pub fn with_child(mut self, child: impl View) -> Self {
        self.children.push(child.any_view());
        self
    }

    #[track_caller]
    pub fn push(&mut self, child: impl View) {
        self.children.push(child.any_view());
    }
}

impl View for Stack {
    fn children(&self) -> Vec<AnyView> {
        self.children.clone()
    }

    fn hydrate_single(&mut self, other: AnyView) {
        if let Some(other) = other.downcast_ref::<Stack>() {
            // self.state.hydrate(&other.state);
            self.children = other.children.clone();
        }
    }

    fn layout(&self, children: &[ViewTree], mut constraint: Constraint) -> Size {
        let mut my_size = Size::zero();
        for child in children {
            child.layout(constraint);
            let size = child.view.size.get();
            constraint.sub_height(size.height);
            my_size.height += size.height;
            if my_size.width < size.width {
                my_size.width = size.width;
            }
        }
        my_size
    }

    fn set_offset(&self, children: &[ViewTree], mut offset: Offset) {
        for ViewTree { view, children } in children {
            let size = view.size.get();
            view.borrow().set_offset(children, offset);
            offset.y += size.height;
        }
    }
}

// pub struct Container<'a> {
//     pub children: Vec<Box<dyn View>>,
// }

pub trait Hydrate {
    fn hydrate(&mut self, other: Self);
}

#[derive(Clone)]
pub struct ViewTree {
    pub view: AnyView,
    pub children: Vec<ViewTree>,
}

/*
root: Stack<A, B>

ViewTree
  view: Stack<A, B>
  store: A => ViewTree A {} {}
         B => ViewTree B {} {}
  body: ()


App = Text
root: Padding<App>

ViewTree
  view: Padding<App>
  children: [Text]
*/
impl ViewTree {
    #[track_caller]
    pub fn new(root: AnyView) -> ViewTree {
        // let body = root.body();
        // let mut children = Vec::new();
        // for (key, child) in root.children() {
        //     let tree = ViewTree::new(child);
        //     children.push((key, tree));
        // }
        let children = root
            .borrow()
            .children()
            .into_iter()
            .map(ViewTree::new)
            .collect();
        ViewTree {
            view: root,
            children,
        }
    }

    /*
    Old: Stack [A,B,C]
    New: Stack [B,D]

    Hydrate B
    Drop A
    Drop C
    Generate new children for [B,D]
    */
    pub fn perform_hydrate_dirty(&mut self) {
        let ViewTree { view, children } = self;
        if view.borrow().is_dirty() {
            // eprintln!("Hydrating dirty: {:?}", view);
            let new_children = view.borrow().children();
            for (new_root, old_tree) in new_children.into_iter().zip(children.iter_mut()) {
                old_tree.perform_hydrate(new_root);
            }
        }
    }

    pub fn perform_hydrate(&mut self, root: AnyView) {
        let ViewTree { view, children } = self;
        // eprintln!("Hydrating: {:?}", view);
        if !view.borrow().is_dirty() && view.borrow().is_same(&root) {
            // eprintln!("Hydrating clean: {:?} {:?}", view, root);
            for child in children.iter_mut() {
                child.perform_hydrate_dirty();
            }
        } else {
            // eprintln!("Hydrating modified: {:?} {:?}", view, root);
            view.hydrate_any(root);
            let new_children = view.borrow().children();
            let prev_children = std::mem::replace(children, Vec::new());
            let (new, _del, upd) = ViewTree::diff(new_children, prev_children);
            for elt in new.into_iter() {
                children.push(ViewTree::new(elt));
            }
            for (new_root, mut old_tree) in upd.into_iter() {
                old_tree.perform_hydrate(new_root);
                children.push(old_tree);
            }
            children.sort_unstable_by_key(|tree| tree.view.key);
        }
        // Diff:
        //   New: [AnyView]
        //   Del: [ViewTree]
        //   Same: [(ViewTree, AnyView)]
        // Hydrate::hydrate(view, root);
        // for (key, child) in children {
        // }
        // let ViewHierarchy { view, children } = self;
        // if view.is_dirty() || view != &new {
        //     Hydrate::hydrate(view, new);
        //     Hydrateable::perform_hydrate(children, view.body());
        // } else {
        //     Hydrateable::perform_hydrate(children, children.clone_body());
        // }
    }

    fn diff(
        views: Vec<AnyView>,
        children: Vec<ViewTree>,
    ) -> (Vec<AnyView>, Vec<ViewTree>, Vec<(AnyView, ViewTree)>) {
        let mut views_iter = views.into_iter().peekable();
        let mut child_iter = children.into_iter().peekable();
        let mut new = Vec::new();
        let mut del = Vec::new();
        let mut upd = Vec::new();
        loop {
            match (views_iter.peek(), child_iter.peek()) {
                (None, None) => break,
                (Some(view), None) => {
                    new.push(view.clone());
                    views_iter.next();
                }
                (None, Some(_child)) => {
                    if let Some(child) = child_iter.next() {
                        del.push(child);
                    }
                    // child_iter.next();
                }
                (Some(view), Some(child)) => {
                    match view.key.cmp(&child.view.key) {
                        Ordering::Less => {
                            new.push(view.clone());
                            views_iter.next();
                        }
                        Ordering::Greater => {
                            if let Some(child) = child_iter.next() {
                                del.push(child);
                            }
                            // child_iter.next();
                        }
                        Ordering::Equal => {
                            if let Some(child) = child_iter.next() {
                                upd.push((view.clone(), child));
                                views_iter.next();
                            }
                        }
                    }
                }
            }
        }
        (new, del, upd)
    }

    pub fn layout(&self, constraint: Constraint) {
        let size = self.view.borrow().layout(&self.children, constraint);
        self.view.size.set(size);
    }

    pub fn set_offset(&self, offset: Offset) {
        self.view.borrow().set_offset(&self.children, offset);
    }

    pub fn pretty_print(&self) {
        print_tree(&self.tree()).unwrap();
    }

    pub fn tree(&self) -> StringItem {
        let ViewTree { view, children } = self;
        let mut builder = TreeBuilder::new(format!("{:?}", view.view.borrow()));
        for child in children {
            child.mk_tree(&mut builder);
        }
        builder.build()
    }

    fn mk_tree(&self, builder: &mut TreeBuilder) {
        if (*self.view.borrow()).type_id() != TypeId::of::<EmptyView>() {
            let ViewTree { view, children } = self;
            builder.begin_child(format!("{:?}: {:?}", view.key, view.view.borrow()));
            for child in children {
                child.mk_tree(builder);
            }
            builder.end_child();
        }
    }

    pub fn flatten(&self) -> Vec<AnyView> {
        let mut out = Vec::new();
        fn process(out: &mut Vec<AnyView>, tree: &ViewTree) {
            out.push(tree.view.clone());
            for child in tree.children.iter() {
                process(out, &child);
            }
        }
        process(&mut out, self);
        out
    }

    pub fn event(&self, event: &Event) {
        self.view
            .borrow()
            .event(self.view.size.get(), &self.children, event);
        for child in self.children.iter() {
            child.event(event);
        }
    }

    pub fn clean(&self) {
        self.view.borrow().clean();
        for child in self.children.iter() {
            child.clean();
        }
    }
}

#[derive(Clone)]
pub struct AnyView {
    key: Key,
    pub size: Rc<Cell<Size>>,
    view: Rc<RefCell<dyn View>>,
}

impl std::fmt::Debug for AnyView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AnyView")
            .field(&self.key)
            .field(&self.size.get())
            .field(&self.view.borrow())
            .finish()
    }
}

impl AnyView {
    #[track_caller]
    pub fn new(view: impl View) -> AnyView {
        AnyView {
            key: Key::new(),
            size: Rc::new(Cell::new(Size::zero())),
            view: Rc::new(RefCell::new(view)),
        }
    }

    #[track_caller]
    pub fn downcast_ref<'a, T: View>(&'a self) -> Option<std::cell::Ref<'a, T>> {
        let ref_view = self.view.try_borrow().ok()?;
        <dyn Any>::downcast_ref::<T>((*ref_view).as_any())?;
        let m = std::cell::Ref::map(ref_view, |val| {
            <dyn Any>::downcast_ref::<T>(val.as_any()).unwrap()
        });
        Some(m)
    }

    pub fn borrow(&self) -> std::cell::Ref<'_, dyn View> {
        self.view.borrow()
    }

    pub fn hydrate_any(&self, other: AnyView) {
        self.view.borrow_mut().hydrate_single(other);
    }
}

// impl View for AnyView {
//     fn body(&self) -> AnyView {
//         self.borrow().body()
//     }
//     #[track_caller]
//     fn key(&self) -> Key {
//         self.borrow().
//     }
//     // #[track_caller]
//     // fn children<'a>(self: &'a Self) -> ChildIter<'a> {
//     //     // Box::new([self.body()].into_iter().map(|b| b))
//     //     Box::new(std::iter::once((self.key(), self.body())))
//     //     // Box::new(std::iter::empty())
//     // }
//     #[track_caller]
//     fn children<'a>(self: &'a Self) -> Vec<AnyView> {
//         vec![self.body()]
//     }

//     fn hydrate_single(&mut self, _other: AnyView) {}

//     fn layout(&self, children: &[ViewTree]) -> Size {
//         if let [ViewTree { view, children }] = children {
//             view.layout(children)
//         } else {
//             Size {
//                 width: 0.0,
//                 height: 0.0,
//             }
//         }
//     }

//     fn set_offset(&self, offset: Offset) {
//         todo!()
//     }
// }

// impl Deref for AnyView {
//     type Target = dyn View;
//     fn deref(&self) -> &Self::Target {
//         self.view.deref()
//     }
// }

pub trait ToAnyView: View + Sized {
    #[track_caller]
    fn any_view(self) -> AnyView {
        AnyView::new(self)
    }
}
impl<X: View + Sized> ToAnyView for X {}

#[derive(Clone, PartialEq)]
pub struct State<X: Copy> {
    dirty: Rc<Cell<bool>>,
    value: Rc<Cell<X>>,
}

impl<X: Debug + Copy> std::fmt::Debug for State<X> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if self.dirty.get() {
            "StateDirty"
        } else {
            "State"
        };
        f.debug_tuple(name).field(&self.value.get()).finish()
    }
}

impl<X: Copy> State<X> {
    pub fn new(value: X) -> State<X> {
        State {
            dirty: Rc::new(Cell::new(true)),
            value: Rc::new(Cell::new(value)),
        }
    }

    pub fn get(&self) -> X {
        self.value.get()
    }

    pub fn set(&self, value: X) {
        self.value.set(value);
        self.dirty.set(true);
    }

    pub fn clean(&self) {
        self.dirty.set(false)
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty.get()
    }
}

/*
perform_hydrate store tree new =
  if tree.view == new
    No changes. Hydrate children.
    ...
  else
    'body' may have changed.
    tree.view.hydrate(store, new)
    tree.view.hydrate(tree.store, new.body)
*/

/*
AnyView + Vec<ViewTree>: Constraint -> Size
AnyView.set_offset: Fn(Offset)

Cache: (AnyView, Vec<ViewTree>, Constraint) -> Size

fn query<Property>(AnyView) -> Option<Property>
*/
