extern crate cursive;

use cursive::event::{Event, Key};
use cursive::theme::{self, BaseColor, Color};
use cursive::views::{Dialog, TextView};
use cursive::Cursive;

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use std::time::{Duration, Instant};

trait AsSecsF64 {
    fn as_secs_f64(&self) -> f64;
}

impl AsSecsF64 for Duration {
    fn as_secs_f64(&self) -> f64 {
        self.as_secs() as f64 + self.subsec_nanos() as f64 * 1e-9
    }
}

#[derive(Default, Debug)]
struct State {
    last_tap: Option<Instant>,
    bpms: Vec<f64>,
}

impl State {
    fn new() -> Self {
        Default::default()
    }

    fn tap(&mut self) {
        let now = Instant::now();
        let prev_tap = mem::replace(&mut self.last_tap, Some(now));

        if let Some(prev) = prev_tap {
            let delta = (now - prev).as_secs_f64();
            self.bpms.push(1. / delta * 60.);
        }
    }

    fn reset(&mut self) {
        mem::replace(self, Default::default());
    }

    fn last_bpm(&self) -> Option<f64> {
        self.bpms.last().cloned()
    }

    fn avg_bpm(&self) -> Option<f64> {
        self.last_bpm()?;
        Some(self.bpms.iter().cloned().sum::<f64>() / self.bpms.len() as f64)
    }
}

fn render_layer(siv: &mut Cursive, state: &State) {
    let layer = match state.last_bpm() {
        Some(bpm) => {
            let avg = state.avg_bpm().unwrap();
            let mut content = format!(
                "Current BPM:   {:.2}\nAverage BPM:   {:.2}\nNearest whole: {}\n",
                bpm,
                avg,
                avg.round()
            );
            content.push_str("\nTap BACKSPACE to reset");
            TextView::new(content)
        }
        None => TextView::new(
            "Tap SPACE at least twice to measure bpm.
             Tap Q to quit.
            ".trim(),
        ),
    };
    siv.add_layer(Dialog::around(layer));
}

fn my_theme() -> theme::Theme {
    let mut theme = theme::load_default();
    theme.palette[theme::PaletteColor::Background] = Color::Dark(BaseColor::Black);
    theme
}

fn main() {
    let state = Rc::new(RefCell::new(State::new()));

    let mut siv = Cursive::default();
    siv.set_theme(my_theme());

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(Event::Key(Key::Backspace), {
        let state = state.clone();
        move |s| {
            state.borrow_mut().reset();
            s.pop_layer();
            render_layer(s, &*state.borrow());
        }
    });
    siv.add_global_callback(' ', {
        let state = state.clone();
        move |s| {
            state.borrow_mut().tap();
            s.pop_layer();
            render_layer(s, &*state.borrow());
        }
    });
    render_layer(&mut siv, &*state.borrow());

    siv.run();
}
