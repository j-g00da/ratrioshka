use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratrioshka::Ratrioshka;

fn main() {
    let terminal = ratatui::init();
    run(terminal);
    ratatui::restore();
}

fn run(mut terminal: DefaultTerminal) {
    loop {
        terminal.draw(render).unwrap();
        if matches!(event::read().unwrap(), Event::Key(_)) {
            break;
        }
    }
}
fn render(frame: &mut Frame) {
    let inner_tui = Ratrioshka::new(|inner_frame| {
        inner_frame.render_widget(
            Block::bordered().title("Inner TUI").white().on_black(),
            inner_frame.area(),
        )
    });

    let outer_block = Block::bordered().title("Outer TUI").white().on_black();

    frame.render_widget(&outer_block, frame.area());
    frame.render_widget(inner_tui, outer_block.inner(frame.area()))
}
