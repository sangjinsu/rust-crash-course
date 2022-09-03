use std::error::Error;
use std::io::stdout;
use std::sync::mpsc::channel;
use std::{io, thread};
use std::time::Duration;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::{event, ExecutableCommand};
use crossterm::event::KeyCode::Char;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;
use invaders::{frame, render};
use invaders::frame::{new_frame};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");


    let mut stdout = stdout();
    enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let (render_tx, render_rx) = channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    'gameloop: loop {
        let curr_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
