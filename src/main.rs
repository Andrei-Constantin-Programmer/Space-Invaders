use std::error::Error;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crossterm::ExecutableCommand;
use crossterm::cursor::Hide;
use crossterm::cursor::Show;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::terminal;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use space_invaders::frame;
use space_invaders::frame::new_frame;
use space_invaders::render;
use space_invaders::audio::Sound;
use space_invaders::audio::AudioHandler;

fn main() -> Result <(), Box<dyn Error>> {

    let mut audio = AudioHandler::new();

    audio.play(&Sound::Startup);

    // Setup terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop
    let (render_sender, render_receiver) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let current_frame = match render_receiver.recv() {
                Ok(frame) => frame,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &current_frame, false);
            last_frame = current_frame;
        }
    });

    // Game loop
    'gameloop: loop {

        let current_frame = new_frame();
        

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play(&Sound::Lose);
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        let _ = render_sender.send(current_frame);
        thread::sleep(Duration::from_millis(1));
    }

    drop(render_sender);
    render_handle.join().unwrap();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
