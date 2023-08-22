use sqltui::app::{App, AppMode, AppResult};
use sqltui::event::{Event, EventHandler};
use sqltui::handler::handle_key_events;
use sqltui::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

fn main()-> AppResult<()>{
    let mut app= App::new(); 
    if app.info.password.len()== 0{
        app.mode= AppMode::Login;
    }
    let backend= CrosstermBackend::new(io::stderr());
    let terminal= Terminal::new(backend)?;
    let events= EventHandler::new(200);
    let mut tui= Tui::new(terminal,events);

    tui.init()?;
    while app.state {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Key(key_event)=> handle_key_events(key_event,&mut app)?,
            _=>{}
        }
    }

    tui.exit()?;
    Ok(())
}
