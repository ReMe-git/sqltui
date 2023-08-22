#[allow(unused)]
use crate::app::{App,AppMode,AppResult};
use crossterm::event::{KeyCode,KeyEvent,KeyModifiers};

pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App)-> AppResult<()>{ 
    match app.mode {
        AppMode::Normal=> normal_handler(app,key_event)?,
        AppMode::Editor=> editor_handler(app,key_event)?,
        AppMode::Message=> message_handler(app,key_event)?,
        AppMode::Table=> table_handler(app,key_event)?,
        AppMode::Login=> login_handler(app,key_event)?
    }
    Ok(())
}

fn login_handler(
    app: &mut App,
    key_event: KeyEvent
    )-> AppResult<()>{
    match key_event.code {
        KeyCode::Char(ch)=> {
            if ch== 'q'&& key_event.modifiers== KeyModifiers::CONTROL{
                app.state= false;
            }else{
                app.info.password.push(ch);
            }
        },
        KeyCode::Backspace=> {
            app.info.password.pop();
        },
        KeyCode::Esc=> {
            app.state= false;
        },
        KeyCode::Enter=> {
            app.mode= AppMode::Normal;
        },
        _=>{}
    }
    Ok(())
}

fn table_handler(
    app: &mut App,
    key_event: KeyEvent
    )-> AppResult<()>{
    match key_event.code {
        KeyCode::Esc=> {
            app.change_mode(AppMode::Normal);
        },
        KeyCode::Char('q')=> {
            app.change_mode(AppMode::Normal);
        },
        KeyCode::Up=> {
            app.table.prev_row();
        },
        KeyCode::Left=> {
            app.table.prev_col();
        },
        KeyCode::Down=> {
            app.table.next_row();
        },
        KeyCode::Right=> {
            app.table.next_col();
        }
        _=>{}
    }
    Ok(())
}

fn message_handler(
    app: &mut App,
    key_event: KeyEvent)-> AppResult<()>{
    match key_event.code {
        KeyCode::Esc=> {
            app.change_mode(AppMode::Normal);
        },
        KeyCode::Char('q')=> {
            app.change_mode(AppMode::Normal);
        },
        KeyCode::Up=> {
            app.message.scroll_up();
        },
        KeyCode::Down=> {
            app.message.scroll_down();
        },
        KeyCode::Left=> {
            app.message.scroll_left();
        },
        KeyCode::Right=> {
            app.message.scroll_right();
        },
        _=> {}
    }
    Ok(())
}

fn normal_handler(
    app: &mut App,
    key_event: KeyEvent)-> AppResult<()>{
    match key_event.code {
        KeyCode::Esc| KeyCode::Char('q')=> {
            app.quit();
        },
        KeyCode::Char('e')=> {
            app.change_mode(AppMode::Editor);
        },
        KeyCode::Char('m')=> {
            app.change_mode(AppMode::Message);
        },
        KeyCode::Char('t')=> {
            app.change_mode(AppMode::Table);
        },
        KeyCode::Char('c')=> {
            if key_event.modifiers== KeyModifiers::CONTROL {
                app.quit();
            }
        },
        KeyCode::Enter=> {
            app.send_query();
        }
        _=>{}
    }
    Ok(())
}

fn editor_handler(
    app: &mut App,
    key_event: KeyEvent)-> AppResult<()>{
    match key_event.code {
        KeyCode::Esc=> {
            app.change_mode(AppMode::Normal);
        },
        KeyCode::Backspace=>{
            match app.editor.delete(){
                false=> {
                    app.editor.cursor_left();
                    app.editor.scroll_check();
                },
                true=> {
                    app.editor.cursor_up();
                    app.editor.cursor_check(true);
                    app.editor.scroll_check();
                }
            }
        },
        KeyCode::Left=>{
            app.editor.cursor_left();
            app.editor.scroll_check();
        },
        KeyCode::Right=>{
           app.editor.cursor_right();
           app.editor.scroll_check();
        },
        KeyCode::Up=>{
            app.editor.cursor_up();
            app.editor.scroll_check();
            if key_event.modifiers== KeyModifiers::SHIFT{
                app.editor.cursor_check(true);
            }else {
                app.editor.cursor_check(false);
            }
        },
        KeyCode::Down=>{
            app.editor.cursor_down();
            app.editor.scroll_check();
            if key_event.modifiers== KeyModifiers::SHIFT{
                app.editor.cursor_check(true);
            }else {
                app.editor.cursor_check(false);
            }
        },
        KeyCode::Enter=>{
            app.editor.addline();
            app.editor.cursor_check(false);
            app.editor.scroll_check();
        },
        KeyCode::Char(ch)=>{
            if ch== 'q'&& key_event.modifiers== KeyModifiers::CONTROL{
                app.mode= AppMode::Normal;
            }else {
                app.editor.enter(ch);
                app.editor.cursor_right();
                app.editor.scroll_check();
            }
        },
        KeyCode::Tab=> {
            app.editor.enter(' ');
            app.editor.cursor_right();
            app.editor.scroll_check();
        }
        _=>{}
    }
    Ok(())
}


/*
fn host_handler(
    app: &mut App,
    key_event: KeyEvent)-> AppResult<()>{
    match key_event.code{
        KeyCode::Enter=> {
            app.change_mode(AppMode::Normal);
        },
        KeyCode::Esc=> {
            app.change_mode(AppMode::Normal);
        }
        KeyCode::Down=> {
            app.info.next_entry();
        },
        KeyCode::Up=> {
            app.info.prev_entry();
        },
        KeyCode::Char(ch)=> {
            app.info.enter(ch);
            app.info.scroll_check();
        },
        KeyCode::Backspace=> {
            app.info.delete();
            app.info.scroll_check();
        },
        _=> {}
    }
    Ok(())
}
*/
