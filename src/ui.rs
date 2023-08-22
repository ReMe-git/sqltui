use std::vec;

use ratatui::{
    backend::Backend,
    layout::{*,Constraint::*},
    style::*,
    widgets::*,
    Frame
};

use crate::app::{App,AppMode};

pub fn render<B: Backend>(app: &mut App,frame: &mut Frame<'_,B>) {
    let size= frame.size();
    render_background(frame, size);
    
    match app.mode{
        AppMode::Login=> {
            render_login(app, frame, size);
            return;
        },
        _=>{}
    }

    let main_chunks= Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            vec![
            Length(size.width- size.width/ 2),
            Length(size.width/ 2),
            Min(0)
        ]).split(frame.size());

    let output_chunks= Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            vec![
            Length(size.height- size.height/ 2),
            Length(size.height/ 2),
            Min(0)
        ]).split(main_chunks[1]);
    
    render_editor(app, frame, main_chunks[0]);
    render_table(app, frame, output_chunks[0]);
    render_message(app, frame, output_chunks[1]);
}

fn render_background<B: Backend>(frame:&mut Frame<'_,B>,size: Rect){
    frame.render_widget(Block::default()
                        .bg(Color::Rgb(25, 25, 25))
                        .borders(Borders::NONE)
                        , size);
}

fn render_login<B: Backend>(app: &mut App,frame:&mut Frame<'_,B>,size: Rect){
    let context: String= std::iter::repeat('*')
        .take(app.info.password.len())
        .collect();
    frame.render_widget(Paragraph::new(format!(
            "Password is empty,enter password or skip\r\n\
            Press \'Enter\' to skip/ensure password\r\n\
            Press \'Esc/<C-q>\' to quit\r\nPassword: {}_",context)
            ), size)
}

fn render_editor<B: Backend>(app: &mut App,frame :&mut Frame<'_,B>,size: Rect){
    app.editor.getsize((size.height,size.width));

    let editor= Paragraph::new(app.editor.context.concat())
        .style(
            Style::default()
            .fg(match app.mode{
                AppMode::Editor=> Color::LightGreen,
                _=> Color::Gray
            })
            )
        .alignment(Alignment::Left)
        .scroll(app.editor.scroll)
        .block(
            Block::default()
            .title("Editor")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
           );
    frame.render_widget(editor,size);
            
    frame.set_cursor(size.x+ app.editor.cursor_index as u16+ 1- app.editor.scroll.1,
                     size.y+ app.editor.line_index as u16+ 1- app.editor.scroll.0);
}

fn render_message<B: Backend>(app:&mut App,frame:&mut Frame<'_,B>,size: Rect){
    app.message.get_size((size.width,size.height));

    let message= Paragraph::new({
        format!("{}",app.message.context.concat())
        })
    .block(
        Block::default()
        .title("Message")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
       )
    .style(
        Style::default()
        .fg(match app.mode{
            AppMode::Message=> Color::LightGreen,
            _=> Color::Gray
        })
        )
    .alignment(Alignment::Left)
    .scroll(app.message.scroll);
    frame.render_widget(message,size);
}

fn render_table<B: Backend>(app:&mut App,frame: &mut Frame<'_,B>,size: Rect){
    app.table.get_size((size.width,size.height));
    
    let col_bg: usize= app.table.scroll.0 as usize;
    let row_bg: usize= app.table.scroll.1 as usize;
    let header_cells= app.table.headers[col_bg..]
        .iter()
        .map(|h| Cell::from(h.to_string())
             .style(Style::default()
                    .fg(Color::Rgb(25, 25, 25))
                    )
             );
    let header= Row::new(header_cells)
        .height(1)
        .style(Style::default()
               .bg(Color::Gray)
               );

    let items= app.table.items[row_bg..].iter().map(|item| {
        let cells= item[col_bg..].iter().map(|c| Cell::from(c.to_string()));
        Row::new(cells)
            .height(1)
            .style(Style::default()
                   .fg(Color::Gray)
                   )
    });
    let widths: Vec<Constraint>= vec![
        Length(size.width/ 3),
        Length(size.width/ 3),
        Length(size.width/ 3),
        Min(0),
    ];
    let table= Table::new(items)
        .header(header)
        .block(Block::default()
               .style(Style::default()
                      .fg(match app.mode{
                          AppMode::Table=> Color::LightGreen,
                          _=> Color::Gray
                      })
                      )
               .borders(Borders::ALL)
               .border_type(BorderType::Thick)
               .title("Table"))
        .widths(&widths);
    frame.render_widget(table,size);
}

/*
fn render_info<B: Backend>(app: &mut App,frame:&mut Frame<'_,B>,size: Rect){
    let chunks= Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
                     Length(size.width/ 3- 2),
                     Length(size.width- size.width/3 * 2+ 4),
                     Min(0)
        ])
        .split(size);
    let block= Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
                     Length(size.height/ 8- 1),
                     Length(17),
                     Min(0)
        ]).split(chunks[1]);
    frame.render_widget(Block::default()
                        .fg(Color::Gray)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick), block[1]);
    let chunks= Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
                     Length(size.width/ 3),
                     Length(size.width- size.width/ 3* 2),
                     Min(0)
        ]).split(size);

    let lines= Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
                     Length(size.height/ 8),
                     Length(3),
                     Length(3),
                     Length(3),
                     Length(3),
                     Length(3),
                     Min(0)
        ]).split(chunks[1]);
    app.info.get_length(lines[0].width);

    let mut render_line= |context: String, name: &str,select: bool, size: Rect|{
        let new= Paragraph::new(
            match select{
                true=> format!("{}_",context),
                false=> context
            })
        .style(
            Style::default()
            .fg(match select{
                true=> Color::Green,
                false=> Color::White
            })
            )
        .alignment(Alignment::Left)
        .scroll(
            match select{
                true=> (0,app.info.scroll),
                false=> (0,0)
            }
            )
        .block(
            Block::default()
            .title(name)
            .title_alignment(Alignment::Left)
            .borders(Borders::BOTTOM)
            .border_type(BorderType::Plain)
            );
        frame.render_widget(new, size);
    };
    let select: bool= match app.info.current_entry{
        InfoEntries::USER=> true,
        _=> false
    };
    render_line(app.info.user.clone(),"User:",select,lines[1]);

    let select: bool= match app.info.current_entry{
        InfoEntries::PASS=> true,
        _=> false
    };
    render_line(std::iter::repeat('*')
                .take(app.info.password.len())
                .collect(),"Pass:",select,lines[2]);

    let select: bool= match app.info.current_entry{
        InfoEntries::HOST=> true,
        _=> false
    };
    render_line(app.info.host.clone(),"Host:",select,lines[3]);

    let select: bool= match app.info.current_entry{
        InfoEntries::DATABASE=> true,
        _=> false
    };
    render_line(app.info.database.clone(),"Database:",select,lines[4]);

    let select: bool= match app.info.current_entry{
        InfoEntries::PORT=> true,
        _=> false
    };
    render_line(app.info.port.clone(),"Port:",select,lines[5]);

}
*/
