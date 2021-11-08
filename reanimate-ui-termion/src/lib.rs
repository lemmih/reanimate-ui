use reanimate_ui::*;
use std::cell::Cell;
use std::io::{stdin, stdout, Write};
use std::rc::Rc;
use std::sync::mpsc::sync_channel;
use std::{thread, time};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::*;
use termion::terminal_size;
use termion::{clear, cursor};

#[derive(Debug, Clone)]
pub struct TermText {
    pub text: String,
    pub offset: Cell<Offset>,
}

impl TermText {
    pub fn new(text: impl ToString) -> TermText {
        TermText {
            text: text.to_string(),
            offset: Cell::new(Offset { x: 1.0, y: 1.0 }),
        }
    }
    pub fn render(&self, screen: &mut impl std::io::Write) {
        write!(
            screen,
            "{}{}",
            cursor::Goto(self.offset.get().x as u16, self.offset.get().y as u16),
            self.text
        )
        .unwrap();
    }
}

impl Hydrate for TermText {
    fn hydrate(&mut self, other: &Self) {
        self.text = other.text.clone();
    }
}

impl View for TermText {
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

#[derive(Debug, Clone)]
pub struct Stats {
    body: Rc<Cell<u32>>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            body: Rc::new(Cell::new(0)),
        }
    }
}

impl Hydrate for Stats {
    fn hydrate(&mut self, _other: &Self) {}
    fn is_same(&self, _other: &Self) -> bool {
        true
    }
    fn is_dirty(&self) -> bool {
        false
    }
}

impl View for Stats {
    fn body(&self) -> AnyView {
        self.body.set(self.body.get() + 1);
        TermText::new(format!("Body {}", self.body.get())).any_view()
    }
}

#[derive(Clone)]
pub struct OnClick<T: View> {
    pub cb: std::rc::Rc<dyn Fn()>,
    pub offset: Cell<Offset>,
    pub child: T,
}
impl<T: View> std::fmt::Debug for OnClick<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("OnClick").field(&self.child).finish()
    }
}

pub trait Clickable: View + Sized {
    fn on_click(self, cb: impl Fn() + 'static) -> OnClick<Self> {
        OnClick::new(std::rc::Rc::new(cb), self)
    }
}
impl<X: View> Clickable for X {}

impl<T: View> OnClick<T> {
    pub fn new(cb: std::rc::Rc<dyn Fn()>, child: T) -> Self {
        OnClick {
            cb,
            child,
            offset: Cell::new(Offset::zero()),
        }
    }
}

impl<T: View + Clone> Hydrate for OnClick<T> {
    fn hydrate(&mut self, other: &Self) {
        self.cb = other.cb.clone();
        self.child = other.child.clone();
    }
}

impl<T: View + Clone> View for OnClick<T> {
    fn body(&self) -> AnyView {
        self.child.clone().any_view()
    }
    // fn children(&self) -> Vec<AnyView> {
    //     self.child.children()
    // }

    // fn layout(&self, children: &[ViewTree], constraint: Constraint) -> Size {
    //     self.child.layout(children, constraint)
    // }
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
        self.offset.set(offset);
        for ViewTree { view, children } in children {
            view.borrow().set_offset(children, offset)
        }
    }
    fn event(&self, size: Size, _children: &[ViewTree], event: &Event) {
        match event {
            Event::MousePress(_btn, x, y) => {
                let offset = self.offset.get();
                // dbg!(x, y, size, offset);
                if *x >= offset.x
                    && *x <= offset.x + size.width - 1.0
                    && *y >= offset.y
                    && *y <= offset.y + size.height - 1.0
                {
                    (*self.cb)();
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Box<T: View> {
    pub child: T,
}

pub trait Boxable: View + Sized {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
impl<X: View> Boxable for X {}

impl<T: View> Box<T> {
    pub fn new(child: T) -> Self {
        Box { child }
    }

    pub fn render(_screen: impl std::io::Write) {}
}

impl<T: View + Clone> Hydrate for Box<T> {
    fn hydrate(&mut self, other: &Self) {
        self.child = other.child.clone();
    }
}

impl<T: View + Clone> View for Box<T> {
    fn body(&self) -> AnyView {
        self.child.clone().any_view()
    }
    fn layout(&self, children: &[ViewTree], mut constraint: Constraint) -> Size {
        if let [child] = children {
            constraint.max_height -= 2.;
            constraint.max_width -= 2.;
            child.layout(constraint);
            let mut size = child.view.size.get();
            size.width += 2.0;
            size.height += 2.0;
            size
        } else {
            // eprintln!("Can't decide widget size: {:?}", self);
            Size::zero()
        }
    }

    fn set_offset(&self, children: &[ViewTree], mut offset: Offset) {
        offset.x += 1.0;
        offset.y += 1.0;
        for ViewTree { view, children } in children {
            view.borrow().set_offset(children, offset)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Padding {
    pub padding: f64,
    pub child: AnyView,
}

pub trait Paddable: View + Sized {
    fn padding(self, padding: f64) -> Padding {
        Padding::new(self, padding)
    }
}
impl<X: View> Paddable for X {}

impl Padding {
    pub fn new(child: impl View, padding: f64) -> Self {
        Padding {
            padding,
            child: child.any_view(),
        }
    }

    pub fn render(_screen: impl std::io::Write) {}
}

impl Hydrate for Padding {
    fn hydrate(&mut self, other: &Self) {
        self.padding = other.padding;
        self.child = other.child.clone();
    }
}

impl View for Padding {
    fn body(&self) -> AnyView {
        self.child.clone()
    }
    fn layout(&self, children: &[ViewTree], mut constraint: Constraint) -> Size {
        if let [child] = children {
            constraint.max_height -= self.padding * 2.0;
            constraint.max_width -= self.padding * 2.0;
            child.layout(constraint);
            let mut size = child.view.size.get();
            size.width += self.padding * 2.0;
            size.height += self.padding * 2.0;
            size
        } else {
            // eprintln!("Can't decide widget size: {:?}", self);
            Size::zero()
        }
    }

    fn set_offset(&self, children: &[ViewTree], mut offset: Offset) {
        offset.x += self.padding;
        offset.y += self.padding;
        for ViewTree { view, children } in children {
            view.borrow().set_offset(children, offset)
        }
    }
}

enum Mode {
    UI,
    Tree,
}

impl Mode {
    fn toggle(&mut self) {
        match self {
            Mode::UI => *self = Mode::Tree,
            Mode::Tree => *self = Mode::UI,
        }
    }
}

pub fn run(app: impl View + Clone) -> std::io::Result<()> {
    let stdin = stdin();
    let mut screen = MouseTerminal::from(AlternateScreen::from(stdout().into_raw_mode()?));
    // let mut screen = MouseTerminal::from(stdout().into_raw_mode()?);
    let mut mode = Mode::UI;
    write!(
        screen,
        "{}{}{}",
        clear::All,
        cursor::Goto(1, 1),
        cursor::Hide,
    )?;
    // let size = terminal_size()?;
    // write!(screen, "ReanimateUI/Termion test. {:?}", size)?;
    screen.flush()?;

    let (event_sender, event_receiver) = sync_channel(10);

    thread::spawn(move || {
        for c in stdin.events() {
            let evt = c.unwrap();
            if event_sender.send(evt).is_err() {
                break;
            }
        }
    });

    let mut tree = ViewTree::new(app.clone().any_view());

    'outer: loop {
        // write!(screen, "{}", clear::All)?;
        while let Ok(evt) = event_receiver.try_recv() {
            let evt_str = format!("{:?}", evt);
            let size = terminal_size()?;
            write!(
                screen,
                "{}{}{}",
                clear::All,
                cursor::Goto(1 + size.0 - evt_str.len() as u16, 1),
                evt_str
            )?;
            match evt {
                termion::event::Event::Key(termion::event::Key::Char('q')) => break 'outer,
                termion::event::Event::Key(termion::event::Key::Char('\t')) => {
                    write!(screen, "{}", clear::All)?;
                    mode.toggle()
                }
                // Event::Key(Key::Char('1')) => write!(screen, "{}", ToMainScreen)?,
                // Event::Key(Key::Char('2')) => write!(screen, "{}", ToAlternateScreen)?,
                termion::event::Event::Mouse(mouse_event) => match mouse_event {
                    termion::event::MouseEvent::Press(_btn, x, y) => tree.event(
                        &reanimate_ui::Event::MousePress(MouseButton::Left, x as f64, y as f64),
                    ),
                    _ => {}
                },
                _ => (),
            }
            screen.flush()?;
        }

        tree.perform_hydrate(app.clone().any_view());
        let size = terminal_size()?;
        tree.layout(Constraint {
            min_width: 0.0,
            max_width: size.0 as f64,
            min_height: 0.0,
            max_height: size.1 as f64,
        });
        tree.set_offset(Offset { x: 1., y: 1. });
        tree.clean();

        match mode {
            Mode::UI => {
                for view in tree.flatten() {
                    if let Some(text) = view.downcast_ref::<TermText>() {
                        text.render(&mut screen);
                    }
                }
                screen.flush()?;
            }
            Mode::Tree => {
                write!(screen, "{}{}UI Tree:", clear::All, cursor::Goto(1, 1))?;
                let mut vec = Vec::new();
                ptree::write_tree(&tree.tree(), &mut vec)?;
                let output = std::str::from_utf8(&vec).unwrap();
                for (nth, line) in output.lines().enumerate() {
                    write!(screen, "{}{}", cursor::Goto(1, 1 + nth as u16), line)?;
                }
                screen.flush()?;
            }
        }

        // tree.pretty_print();

        thread::sleep(time::Duration::from_secs_f32(1.0 / 60.0));
    }

    write!(screen, "{}", ToMainScreen).unwrap();
    write!(screen, "{}", cursor::Show)?;
    screen.flush()?;

    // thread::sleep(time::Duration::from_secs(10));
    // 1. Create app
    // 2. Hydrate
    // 3. Layout
    // 4. Render
    // 5. Handle events
    // 6. Goto 2.
    Ok(())
}
