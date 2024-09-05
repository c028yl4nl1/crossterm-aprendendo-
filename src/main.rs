use crossterm::style::PrintStyledContent;
use crossterm::style::{StyledContent, Stylize};
use crossterm::{
    cursor::{self, MoveTo, MoveUp},
    event::{poll, KeyCode},
    style::{style, Color},
    terminal::{self, Clear},
    QueueableCommand,
};
use crossterm::{
    event::{
        read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event,
    },
    execute,
};
use std::io;
use std::{
    io::{stdout, Cursor, Write},
    time::Duration,
};

fn main() {
    let mut stdout = stdout();
    let (mut w, mut h) = terminal::size().unwrap();

    // stdout.queue()
    let mut bar = "🭶".repeat(w as usize);
    let mut cmd = String::new();
    let mut chat = Vec::new();
    let styled_content = style("Bem Vindo ao chat , Click enter \n\n")
        .with(Color::Green)
        .on(Color::Black);
    execute!(stdout, PrintStyledContent(styled_content));

    loop {
        while poll(Duration::ZERO).unwrap() {
            match read() {
                Ok(evento) => {
                    // evento

                    match evento {
                        Event::Resize(wn, hn) => {
                            w = wn;
                            h = hn;
                            bar = "🭶".repeat(w as usize);
                        }
                        Event::Key(event) => match event.code {
                            KeyCode::Char(caracter) => {
                                cmd.push(caracter);
                            }

                            KeyCode::Enter => {
                                chat.push(cmd.clone());
                                cmd.clear();
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }

                _ => {}
            }

            stdout.queue(Clear(terminal::ClearType::All));
            for (row, cmd) in chat.iter().enumerate() {
                stdout.queue(MoveTo(0, row as u16));
                stdout.write(cmd.as_bytes());
            }
            stdout.queue(MoveTo(0, h - 3));
            stdout.write(bar.as_bytes());
            stdout.queue(MoveTo(0, h - 2));
            sleep(50);
            stdout.flush();
        }
    }
}

fn sleep(number: u64) {
    let time = std::thread::sleep(std::time::Duration::from_millis(number));
}
