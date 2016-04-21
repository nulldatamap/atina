use std::io::{BufReader, BufRead};
use std::io;
use std::path::Path;
use std::fs::File;
use std::cmp::{min, max};

use tcod::Console;

pub type Message = (String, u32);

pub const MAX_MESSAGES : usize = 100;

pub struct MessageLog {
  messages : Vec<Message>,
  log_file : Option<File>,
  scroll   : usize
}

impl MessageLog {
  pub fn new() -> MessageLog {
    MessageLog {
      messages: Vec::new(),
      log_file: None,
      scroll  : 0
    }
  }
  
  pub fn from_file( p : &Path ) -> Result<MessageLog, io::Error> {
    let f = try!( File::open( p ) );
    
    let mut log = MessageLog {
      messages: Vec::new(),
      log_file: Some( f ),
      scroll  : 0
    };
    
    try!( log.load_messages() );
    
    Ok( log )
  }
  
  fn load_messages( &mut self ) -> Result<(), io::Error> {
    if !self.messages.is_empty() {
      self.messages.drain( .. );
    }
    
    let log_file = self.log_file.as_mut().expect(
      "tried to read messages from a log file which isn't bound to a file" );
    
    let reader = BufReader::new( log_file );
    
    for line in reader.lines() {
      self.messages.push( (try!( line ), 1) );
    }
    
    Ok( () )
  }
  
  pub fn add_message( &mut self, msg : String ) {
    if !self.messages.is_empty() {
      let lidx = self.messages.len() - 1;
      let elm = &mut self.messages[lidx];
      if elm.0 == msg {
        elm.1 += 1;
        return
      }
    }
    
    self.messages.push( (msg, 1) );
  }
  
  pub fn render<C : Console>( &self, ctx : &mut C ) {
    ctx.clear();
    
    if self.messages.is_empty() {
      return;
    }
    
    let width = ctx.width();
    let height = ctx.height() as usize;
    
    let last_idx = self.messages.len();
    
    let first_item_dx =
      max( last_idx as isize - height as isize - self.scroll as isize, 0 ) as usize;
    
    let last_item_idx = min( first_item_dx + height, last_idx );
    
    let to_be_shown = &self.messages[first_item_dx..last_item_idx];
    
    for (y, message) in to_be_shown.iter().enumerate() {
      match message {
        &(ref m, 1) => ctx.print( 0, y as i32, m ),
        &(ref m, x) => ctx.print( 0, y as i32, format!( "{} x{}", m, x ) )
      }
    }
  }
}

