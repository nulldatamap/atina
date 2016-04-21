use std::path::{Path};
use std::cell::{RefCell};

use ::tcod::{Console};

use ::map::*;
use ::util::*;
use ::actor::*;
use ::player::*;

enum SpawnCommands {
  SpawnItem,
  SpawnActor( Position )
}

pub struct World {
  pub map : Map,
  pub player : RefCell<Player>,
  actors : Vec<RefCell<Actor>>,
  items  : Vec<RefCell<Item>>,
  spawns : Vec<SpawnCommands>
}

impl World {
  pub fn new( map_path : &str ) -> Result<World, MapLoadingError> {
    let map = try!( Map::load( map_path ) );
    let mut player = Player::new( map.player_position );
    player.actor.graphics.symbol = '@';
    
    Ok( World {
      map:    map,
      player: RefCell::new( player ),
      actors: Vec::new(),
      items:  Vec::new(),
      spawns: Vec::new()
    } )
  }
  
  pub fn update( &mut self ) {
    let duration = self.player.borrow().actor.action.duration;
    
    if duration == 0 {
      return
    }
    
    self.player.borrow_mut().update( &self, duration );
    
    self.actors.retain( |actor_cell| {
      let mut actor = actor_cell.borrow_mut();
      actor.update( duration );
      
      actor.active
    } );
    
    for cmd in self.spawns.drain( 0.. ) {
      match cmd {
        _ => {}
      }
    }
  }
  
  pub fn render<C : Console>( &self, ctx : &mut C ) {
    self.map.render( ctx );
    
    /*
    for item in self.items {
      item.render( ctx );
    }
    */
    
    for actor in &self.actors {
      actor.borrow().render( ctx );
    }
    
    self.player.borrow().actor.render( ctx );
  }
}

pub struct Item {
}
