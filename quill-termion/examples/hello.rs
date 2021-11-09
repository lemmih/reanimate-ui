use quill::*;
use quill_termion::*;

#[derive(Debug, Clone, PartialEq)]
struct App {
    pub padding: State<u32>,
}

impl Hydrate for App {
    fn hydrate(&mut self, _other: &Self) {}
    fn is_same(&self, other: &Self) -> bool {
        self.eq(&other)
    }
    fn is_dirty(&self) -> bool {
        self.padding.is_dirty()
    }

    fn clean(&self) {
        self.padding.clean()
    }
}

impl App {
    fn new() -> Self {
        App {
            padding: State::new(0),
        }
    }
}

impl View for App {
    fn body(&self) -> AnyView {
        let mut stack = Stack::new();
        stack.push(TermText::new("Inc padding").on_click({
            let this = self.clone();
            move || this.padding.set(this.padding.get() + 1)
        }));
        stack.push(TermText::new("Dec padding").on_click({
            let this = self.clone();
            move || this.padding.set(this.padding.get().saturating_sub(1))
        }));
        stack.push(CountClicks::new().padding(self.padding.get() as f64));
        stack.push(CountClicks::new());
        stack.push(Stats::new());
        stack.any_view()
    }
}

#[derive(Debug, Clone)]
struct CountClicks {
    pub clicks: State<u32>,
}

impl Hydrate for CountClicks {
    fn hydrate(&mut self, _other: &Self) {}
}

impl CountClicks {
    fn new() -> Self {
        CountClicks {
            clicks: State::new(0),
        }
    }
}

impl View for CountClicks {
    fn body(&self) -> AnyView {
        TermText::new(format!("Clicks: {}", self.clicks.get()))
            .on_click({
                let this = self.clone();
                move || this.clicks.set(this.clicks.get() + 1)
            })
            .any_view()
    }
}

fn main() -> std::io::Result<()> {
    quill_termion::run(App::new())?;
    Ok(())
    // let stdin = stdin();
    // // let mut screen = MouseTerminal::from(AlternateScreen::from(stdout().into_raw_mode()?));
    // let mut screen = MouseTerminal::from(stdout().into_raw_mode()?);
    // write!(
    //     screen,
    //     "{}{}{}",
    //     clear::All,
    //     cursor::Goto(1, 1),
    //     cursor::Hide,
    // )?;
    // // let size = terminal_size()?;
    // // write!(screen, "ReanimateUI/Termion test. {:?}", size)?;
    // screen.flush()?;

    // let (event_sender, event_receiver) = sync_channel(10);

    // thread::spawn(move || {
    //     for c in stdin.events() {
    //         let evt = c.unwrap();
    //         event_sender.send(evt).unwrap();
    //     }
    // });

    // let mut tree = ViewTree::new(App::new().any_view());

    // 'outer: loop {
    //     // write!(screen, "{}", clear::All)?;
    //     while let Ok(evt) = event_receiver.try_recv() {
    //         let evt_str = format!("{:?}", evt);
    //         let size = terminal_size()?;
    //         write!(
    //             screen,
    //             "{}{}",
    //             // clear::All,
    //             cursor::Goto(1 + size.0 - evt_str.len() as u16, 1),
    //             evt_str
    //         )?;
    //         match evt {
    //             Event::Key(Key::Char('q')) => break 'outer,
    //             // Event::Key(Key::Char('1')) => write!(screen, "{}", ToMainScreen)?,
    //             // Event::Key(Key::Char('2')) => write!(screen, "{}", ToAlternateScreen)?,
    //             Event::Mouse(mouse_event) => {
    //                 match mouse_event {
    //                     MouseEvent::Press(_btn, x, y) => tree.event(
    //                         &reanimate_ui::Event::MousePress(MouseButton::Left, x as f64, y as f64),
    //                     ),
    //                     _ => {}
    //                 }
    //             }
    //             _ => (),
    //         }
    //         screen.flush()?;
    //     }

    //     tree.perform_hydrate(App::new().any_view());
    //     let size = terminal_size()?;
    //     tree.layout(Constraint {
    //         min_width: 0.0,
    //         max_width: size.0 as f64,
    //         min_height: 0.0,
    //         max_height: size.1 as f64,
    //     });
    //     tree.set_offset(Offset { x: 1., y: 1. });

    //     for view in tree.flatten() {
    //         if let Some(text) = view.downcast_ref::<TermText>() {
    //             text.render(&mut screen);
    //         } else if let Some(handler) = view.downcast_ref::<OnClick<TermText>>() {
    //             let text = &handler.child;
    //             write!(
    //                 screen,
    //                 "{}{}",
    //                 cursor::Goto(text.offset.get().x as u16, text.offset.get().y as u16),
    //                 text.text
    //             )?;
    //             screen.flush()?;
    //         }
    //     }
    //     // tree.pretty_print();

    //     thread::sleep(time::Duration::from_secs_f32(1.0 / 60.0));
    // }

    // write!(screen, "{}", cursor::Show)?;

    // // thread::sleep(time::Duration::from_secs(10));
    // // 1. Create app
    // // 2. Hydrate
    // // 3. Layout
    // // 4. Render
    // // 5. Handle events
    // // 6. Goto 2.
    // Ok(())
}
