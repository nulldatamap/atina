#[macro_use(version)]
extern crate version;
extern crate tcod;

mod ui;
mod world;

use ui::*;
use world::*;

use std::error::Error;

use tcod::{RootConsole, Console};
use tcod::system::{sleep};

struct CoreState {
  title : String,
  root : RootConsole,
}

impl CoreState {
  
  fn new() -> CoreState {
    use std::path::Path;
    
    let title = format!( "Atina v{}", version!() );

    let root = RootConsole::initializer()
      .size( 80, 50 )
      .title( &title )
      .font( Path::new( "data/terminal.png" ), tcod::FontLayout::AsciiInCol )
      .init();
    
    tcod::system::set_fps( 60 );
    
    CoreState {
      title: title,
      root: root
    }
  }

  // Requires: no other state is ongoing
  fn starting_menu( &mut self ) {
    let title_field =
      TextField::new( self.title.clone(), self.x_centered( self.title.len() as i32, 2 ) );
    
    const PLAY_GAME : usize = 0;
    const OPTIONS   : usize = 1;
    const EXIT      : usize = 2;
    
    let mut menu =
      SelectionList::new( vec![ "Play Game".to_string()
                              , "Options".to_string()
                              , "Exit".to_string() ]
                        , self.x_centered( "Play Game".len() as i32, 5 )
                        , true );
    
    while !self.root.window_closed() {
      self.root.clear();
      
      title_field.render( &mut self.root );
      menu.render( &mut self.root );
      
      self.root.flush();
      
      match menu.update( &mut self.root ) {
        Some( PLAY_GAME ) => self.start_game(),
        Some( OPTIONS ) => {},
        Some( EXIT ) => break,
        _ => {}
      } 
    }
  }

  fn x_centered( &self, width : i32, y : i32 ) -> (i32, i32) {
    (self.root.width() / 2 - width / 2, y)
  }
  
  fn start_game( &mut self ) {
    use std::path::Path;
    
    let mut map;
    
    match load_map( Path::new( "data/test.map" ) ) {
      Result::Err( MapLoadingError::ParseIntError( err ) ) => { println!( "{:?}", err.description() ); return },
      Result::Err( err ) => { println!( "{:?}", err ); return },
      Result::Ok( m ) => map = m
    }
    
    self.root.clear();
    map.render( &mut self.root );
    self.root.flush();
    
    self.root.wait_for_keypress( true );
  }

  // Requires: no other state is ongoing 
  fn shutdown( self ) {
  }
}

fn main() {
  let mut core = CoreState::new();

  core.starting_menu();

  core.shutdown();
}