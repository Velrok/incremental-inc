use ratatui::{
    crossterm::event::{self, Event},
    layout::{Alignment, Constraint, Flex, Layout},
    widgets::Paragraph,
};
use std::{io::Result, time::Duration};

struct App {
    running: bool,
}

enum Message {
    Quit,
}

static TITLE_BANNER: &str = include_str!("../assets/title-medium.txt");

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App { running: true };
    while app.running {
        terminal.draw(|frame| view(&app, frame));

        if let Some(msg) = read_inputs()? {
            update(&mut app, msg);
        }
    }
    ratatui::restore();
    Ok(())
}

fn read_inputs() -> Result<Option<Message>> {
    if !event::poll(Duration::from_millis(250))? {
        return Ok(None);
    }

    match event::read()? {
        // Event::FocusGained => todo!(),
        // Event::FocusLost => todo!(),
        // Event::Mouse(mouse_event) => todo!(),
        // Event::Paste(_) => todo!(),
        // Event::Resize(_, _) => todo!(),
        Event::Key(key_event) => {
            if !key_event.is_press() {
                return Ok(None);
            }
            match key_event.code {
                event::KeyCode::Char('q') => Ok(Some(Message::Quit)),
                _ => Ok(None),
            }
        }
        _ => Ok(None),
    }
}

fn update(app: &mut App, msg: Message) {
    match msg {
        Message::Quit => app.running = false,
    }
}

fn view(app: &App, frame: &mut ratatui::Frame<'_>) {
    let banner_height = TITLE_BANNER.lines().count();
    let [area] = Layout::vertical([Constraint::Length(banner_height as u16)])
        .flex(Flex::Center)
        .areas(frame.area());
    let banner = Paragraph::new(TITLE_BANNER).alignment(Alignment::Center);
    frame.render_widget(banner, area);
}
