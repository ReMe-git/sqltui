use std::error;
use mysql::*;
use mysql::prelude::*;
use structopt::StructOpt;

pub type AppResult<T>= 
std::result::Result<T,Box<dyn error::Error>>;

pub enum AppMode{
    Normal,
    Editor,
    Message,
    Table,
    Login
}

pub struct App{
    pub state: bool,
    pub mode: AppMode,
    pub editor: Editor,
    pub message: Message,
    pub table: Table,
    pub info: DatabaseInfo
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: true,
            mode: AppMode::Normal,
            editor: Editor::new(),
            message: Message::new(),
            table: Table::new(),
            info: DatabaseInfo::from_args()
        }
    }
}

impl App {
    pub fn new()-> Self {
        Self::default()
    } 
    pub fn quit(&mut self) {
        self.state= false;
    }
    /*
    pub fn tick(&mut self) {
        if self.tick== 0xfffffffffffffffe{
            self.tick= 0;
        }
        self.tick+= 1;
    }
    */
    pub fn change_mode(&mut self,mode: AppMode) {
        self.mode= mode; 
    }
    pub fn send_query(&mut self) {
        let pool= match Pool::new(self.info.build_opts()){
            Ok(pool)=> pool,
            Err(err)=> {
                self.message.push(format!("{:?}\n",err));
                return
            }
        };
        let mut conn= match pool.get_conn() {
            Ok(conn)=> conn,
            Err(err)=> {
                self.message.push(format!("{:?}\n",err));
                return
            }
        };
        let querys= self.editor.context.concat();

        for query in querys.split(';'){
            if query.len()== 1 {
                continue;
            }
            let rows: Vec<Row>= match conn.query(query){
                Ok(rows)=> rows,
                Err(err)=> {
                    self.message.push(format!("{:?}\n",err));
                    return
                }
            };
            self.table.get_table(rows);
        }
    }
}

#[derive(StructOpt)]
pub struct DatabaseInfo{
    #[structopt(short,long,default_value= "root")]
    pub user: String,
    #[structopt(short,long,default_value= "")]
    pub password: String,
    #[structopt(short,long,default_value= "localhost")]
    pub host: String,
    #[structopt(short,long,default_value= "")]
    pub database: String,
    #[structopt(short="P",long,default_value= "3306")]
    pub port: u16,
}

impl DatabaseInfo{
    /*
    pub fn new()-> Self{
        Self{
            user: String::new(),
            password: String::new(),
            host: String::new(),
            database: String::new(),
            port: 0,
        }
    }
    */
    pub fn build_opts(&mut self)-> OptsBuilder{
        let user= self.user.clone();
        let password= self.password.clone();
        let host= self.host.clone();
        let database= self.database.clone();
        OptsBuilder::new()
            .user(Some(user))
            .pass(Some(password))
            .ip_or_hostname(Some(host))
            .db_name(Some(database))
            .tcp_port(self.port)
    }
}

pub struct Message{
    pub context: Vec<String>,
    pub current_line: u16,
    pub size: (u16,u16),
    pub scroll: (u16,u16)
}

impl Message{
    pub fn new()-> Self{
        Self{
            context: Vec::new(),
            current_line: 0,
            size: (0,0),
            scroll: (0,0)
        }
    }
    pub fn get_size(&mut self,size: (u16,u16)){
        self.size= size;
    }
    pub fn push(&mut self, context: String){
        self.context.push(context);
        self.scroll_down();
    }
    pub fn scroll_up(&mut self) {
        self.scroll.0= self.scroll.0.saturating_sub(1);
    }
    pub fn scroll_down(&mut self) {
        if self.scroll.0+ self.size.1< self.context.len() as u16{
            self.scroll.0= self.scroll.0.saturating_add(1);
        }
    }
    pub fn scroll_left(&mut self) {
        self.scroll.1= self.scroll.1.saturating_sub(1);
    }
    pub fn scroll_right(&mut self) {
        self.scroll.1= self.scroll.1.saturating_add(1);
    }
}

pub struct Editor{
    pub cursor_index: usize,
    pub line_index: usize,
    pub size: (u16,u16),
    pub scroll: (u16,u16),
    pub context: Vec<String>
}

impl Editor{
    pub fn new()-> Self{
        Self{
            cursor_index: 0,
            line_index: 0,
            size: (0,0),
            scroll: (0,0),
            context: vec![String::from("\n")]
        }
    }
    pub fn getsize(&mut self,size: (u16,u16)){
        self.size= size;
    }
    pub fn enter(&mut self, ch: char){
        let line= self.line_index;
        let index= self.cursor_index;

        self.context[line].insert(index, ch);
    }
    pub fn delete(&mut self)-> bool{
        let line= self.line_index;
        let height= self.context.len();
        let len= self.context[line].len();
        let mut remove_or_pop= || {
            if line< height- 1{
                self.context.remove(line);
            }else {
                self.context.pop();
            }
        };
        let mut delete_line= ||-> bool{
            if height> 1&& len<= 1{
                remove_or_pop();
                return true;
            } else{
                return false;
            }
        };

        if self.cursor_index> 0{
            let current_index= self.cursor_index;
            let new_index= current_index- 1;

            let before_chars= self.context[line]
                .chars()
                .take(new_index);
            let after_chars= self.context[line]
                .chars()
                .skip(current_index);

            self.context[line]= before_chars
                .chain(after_chars).collect();
            return false;
        }else{
            return delete_line();
        }
    }
    pub fn addline(&mut self){
        let index= self.cursor_index;

        if index== 0{
            self.context.insert(self.line_index, String::from("\n"));
            self.line_index+= 1;
        } else {
            let new:String = self.context[self.line_index]
                .split_off(self.cursor_index);
            self.context[self.line_index].push('\n');
            self.line_index+= 1; 
            self.context.insert(self.line_index, new);
        }
    }
    pub fn cursor_left(&mut self) {
        if self.cursor_index> 0{
            self.cursor_index-= 1;
        }
    }
    pub fn cursor_right(&mut self) {
        let line= self.line_index;

        if self.cursor_index< self.context[line].len()- 1&&
        self.context[line].len()< 0xfffe{
            self.cursor_index+= 1;
        }
    }
    pub fn cursor_up(&mut self) {
        if self.line_index> 0{
            self.line_index-= 1;
        }
    }
    pub fn cursor_down(&mut self) {
        if self.line_index< self.context.len()- 1{
            self.line_index+= 1;
        }
    }

    pub fn cursor_check(&mut self,last: bool) {
        let index= self.cursor_index;
        let max= self.context[self.line_index].len()- 1;

        if index> max|| last{
            self.cursor_index= max;
        }
    }
    pub fn scroll_check(&mut self) {
        let x= self.cursor_index as u16;
        let y= self.line_index as u16;
        self.scroll.0= y.saturating_sub(self.size.0- 5);
        self.scroll.1= x.saturating_sub(self.size.1- 5);
    }
}

pub struct Table{
    pub rows: Vec<Row>,
    pub headers: Vec<String>,
    pub items: Vec<Vec<String>>,
    pub scroll: (u16,u16),
    pub size: (u16,u16)
}

impl Table {
    pub fn new()-> Self{
        Self {
            rows: Vec::new(),
            headers: Vec::new(),
            items: Vec::new(),
            scroll: (0,0),
            size: (0,0)
        }
    }
    pub fn get_size(&mut self,size: (u16,u16)){
        self.size= size;
    }
    pub fn get_table(&mut self,rows: Vec<Row>){
        if !rows.is_empty() {
            self.rows= rows.clone();
            self.headers.clear();
            self.items.clear();
            for column in rows[0].columns_ref(){
                self.headers.push(column.name_str().to_string());
            }
            for row in rows{
                let mut item: Vec<String>= Vec::new();
                for value in row.unwrap(){
                    item.push(value.as_sql(false));
                }
                self.items.push(item);
            }
            self.scroll= (0,0);
        }
    }
    pub fn next_col(&mut self){
        if self.scroll.0 as usize+ 3< self.headers.len(){
            self.scroll.0= self.scroll.0.saturating_add(1);
        }
    }
    pub fn prev_col(&mut self){
        self.scroll.0= self.scroll.0.saturating_sub(1);
    }
    
    pub fn next_row(&mut self){
        if (self.scroll.1 as usize+ self.size.1 as usize)< self.items.len()+ 3{
            self.scroll.1= self.scroll.1.saturating_add(1);
        }
    }
    pub fn prev_row(&mut self){
        self.scroll.1= self.scroll.1.saturating_sub(1);
    }
}
//use structopt::StructOpt;

/*
*/


//thease code is no longer used
/*
pub scroll_check{
        match x{
            true=>{
                if self.cursor_index as u16+ 5> self.size.1{
                    self.scroll.1= self.cursor_index as u16- (self.size.1- 5);
                }else {
                    self.scroll.1= 0;
                }
            },
            false=>{
                if self.line_index as u16+ 5>= self.size.0{
                    self.scroll.0= self.line_index as u16- (self.size.0- 5);
                }else {
                    self.scroll.0= 0;
                }
            }
        }
}
 */
/*
#[derive(Clone, Copy)]
pub enum InfoEntries{
    USER,
    PASS,
    HOST,
    DATABASE,
    PORT
}

impl InfoEntries {
    pub fn next(&mut self)-> Self{
        match self{
            Self::USER=> Self::PASS,
            Self::PASS=> Self::HOST,
            Self::HOST=> Self::DATABASE,
            Self::DATABASE=> Self::PORT,
            Self::PORT=> Self::PORT
        }
    }
    pub fn prev(&mut self)-> Self{
        match self{
            Self::USER=> Self::USER,
            Self::PASS=> Self::USER,
            Self::HOST=> Self::PASS,
            Self::DATABASE=> Self::HOST,
            Self::PORT=> Self::DATABASE
        }
    }
}
*/
/*
    pub fn get_length(&mut self,length: u16) {
        self.length= length;
    }
    pub fn next_entry(&mut self) {
        self.current_entry= self.current_entry.next();
    }
    pub fn prev_entry(&mut self) {
        self.current_entry= self.current_entry.prev();
    }
    pub fn enter(&mut self,ch: char) {
        match self.current_entry {
            InfoEntries::USER=> self.user.push(ch),
            InfoEntries::PASS=> self.password.push(ch),
            InfoEntries::HOST=> self.host.push(ch),
            InfoEntries::DATABASE=> self.database.push(ch),
            InfoEntries::PORT=> self.port.push(ch),
        };
    }
    pub fn delete(&mut self) {
        match self.current_entry {
            InfoEntries::USER=> self.user.pop(),
            InfoEntries::PASS=> self.password.pop(),
            InfoEntries::HOST=> self.host.pop(),
            InfoEntries::DATABASE=> self.database.pop(),
            InfoEntries::PORT=> self.port.pop(),
        };
    }
    pub fn scroll_check(&mut self){

        let mut scroll= |len: u16| {
            self.scroll= len.saturating_sub(self.length- 5);
        };
        match self.current_entry {
            InfoEntries::USER=> scroll(self.user.len() as u16),
            InfoEntries::PASS=> scroll(self.password.len() as u16),
            InfoEntries::HOST=> scroll(self.host.len() as u16),
            InfoEntries::DATABASE=> scroll(self.database.len() as u16),
            InfoEntries::PORT=> scroll(self.port.len() as u16),
        }
    }
*/
