#[macro_use(version)]
extern crate version;
extern crate tcod;
extern crate fnv;
extern crate toml;
extern crate rustc_serialize;

mod util;
mod ui;
mod log;
mod description;
mod map;
mod world;
mod actor;
mod player;
mod input;

use ui::*;
use log::*;
use map::MapLoadingError;
use world::*;
use input::*;

use std::error::Error;
use std::cell::RefCell;

use tcod::{RootConsole, Console};
use tcod::console::{TextAlignment};

struct Game {
  title : String,
  root  : RootConsole,
  input : RefCell<Input>,
  world : RefCell<World>,
  message_log : RefCell<MessageLog>
}

impl Game {
  fn new( title : String, root : RootConsole ) -> Game {
    use std::path::Path;
    
    let mut world;
    
    match World::new( "data/test.toml" ) {
      Result::Err( MapLoadingError::ParseIntError( err ) ) => panic!( "{:?}", err.description() ),
      Result::Err( err ) => panic!( "{:?}", err ),
      Result::Ok( w ) => world = w
    }
    
    Game {
      title: title,
      root: root,
      input: RefCell::new( Input::new() ),
      world: RefCell::new( world ),
      message_log: RefCell::new( MessageLog::new() )
    }
  }

  fn start( &mut self ) {
    let mut msg_log_console = tcod::console::Offscreen::new( 80, 10 );
    
    while !self.root.window_closed() {
      self.input.borrow_mut().update( self );
      
      while self.world.borrow().player_is_performing_action() {
        self.world.borrow_mut().update();
      }
      
      self.root.clear();
      self.world.borrow().render( &mut self.root );
      self.message_log.borrow_mut().render( &mut msg_log_console );
      tcod::console::blit( &msg_log_console, (0, 0), (0, 0), &mut self.root, (0, 40), 1.0, 1.0 );
      self.root.flush();
    }
  }

}

#[derive(PartialEq, Eq)]
enum MenuChoice {
  StartGame,
  Exit
}

fn starting_menu( title : &str, root : &mut RootConsole ) -> MenuChoice {
  let title_field =
    TextField::new( title.to_string()
                  , (root.width() / 2, 2).into()
                  , TextAlignment::Center );
  
  const PLAY_GAME : usize = 0;
  const OPTIONS   : usize = 1;
  const EXIT      : usize = 2;
  
  let mut menu =
    SelectionList::new( vec![ "Play Game".to_string()
                            , "Options".to_string()
                            , "Exit".to_string() ]
                      , (root.width() / 2 , 5 ).into()
                      , true
                      , TextAlignment::Center );
  
  while !root.window_closed() {
    root.clear();
    title_field.render( root );
    menu.render( root );
    
    root.flush();
    
    match menu.update( root ) {
      Some( PLAY_GAME ) => return MenuChoice::StartGame,
      Some( OPTIONS ) => {},
      Some( EXIT ) => return MenuChoice::Exit,
      _ => {}
    } 
  }
  
  MenuChoice::Exit
}

fn main() {
  use std::path::Path;
  
  let title = format!( "Atina v{}", version!() );

  let mut root = RootConsole::initializer()
    .size( 80, 50 )
    .title( &title )
    .font( Path::new( "data/terminal.png" ), tcod::FontLayout::AsciiInCol )
    .init();
  
  tcod::system::set_fps( 60 );
  
  description::load_descriptions();
  
  if starting_menu( &title, &mut root ) == MenuChoice::StartGame {
    let mut game = Game::new( title, root );
    game.start();
  }
}