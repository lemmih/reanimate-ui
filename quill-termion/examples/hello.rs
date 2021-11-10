use quill::*;
use quill_derive::*;
use quill_termion::*;

#[derive(Debug, Clone, PartialEq, Hydrate)]
struct App {
    pub padding: State<u32>,
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

#[derive(Debug, Clone, PartialEq, Hydrate)]
struct CountClicks {
    pub clicks: State<u32>,
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
    quill_termion::run(App::new())
}
