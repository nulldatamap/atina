use tcod::{Color, Console};
use tcod::colors;

use description::*;
use map::*;
use world::*;
use util::*;

use std::fmt;
use std::fmt::{Display, Formatter};

pub struct Stats {
  health     : u32,
  max_health : u32,
  speed      : u32
}

pub struct Graphics {
  pub symbol : char,
  pub fg     : Color,
  pub bg     : Color
}

pub enum ActionFailureReason<'a> {
  BlockedByTile( Tile ),
  BlockedByActor( &'a Actor )
}

impl<'a> Display for ActionFailureReason<'a> {
  fn fmt( &self, fmtr : &mut Formatter ) -> Result<(), fmt::Error> {
    use self::ActionFailureReason::*;
    
    match self {
      &BlockedByTile( t ) =>
        write!( fmtr, "You were blocked by {}", t.description().name() ),
      &BlockedByActor( a ) =>
        write!( fmtr, "{} is in your way!", a.description().name() )
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ActionKind {
  None,
  MoveTo( Position )
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Action {
  pub duration : u32,
  kind : ActionKind
}

impl Action {
  pub fn none() -> Action {
    Action { duration: 0, kind: ActionKind::None }
  }
}

pub struct Actor {
  pub active   : bool,
  pub action   : Action,
  pub pos          : Position,
  pub stats        : Stats,
  pub graphics     : Graphics
}

impl Describe for Actor {
  fn desc_id( &self ) -> String {
    "placeholer".to_string()
  }
}

impl Actor {
  pub fn new( pos : Position ) -> Actor {
    Actor {
      active: true,
      action: Action::none(),
      pos: pos,
      stats: Stats { health: 10, max_health: 10, speed: 100 },
      graphics: Graphics { symbol: 'a'
                         , fg: colors::WHITE
                         , bg: colors::BLACK },
    }
  }
  
  pub fn move_direction( &mut self, dir : Direction, world : &World )
    -> Option<ActionFailureReason> {
    
    assert_eq!( self.action, Action::none() );
    
    let move_pos = dir.offset_position( self.pos, &world.map );
    let tile = world.map.tile_at( move_pos );
    
    if tile.is_solid() {
      Some( ActionFailureReason::BlockedByTile( tile ) )
    } else {
      self.action = Action {
        duration: self.stats.speed,
        kind: ActionKind::MoveTo( move_pos )
      };
      
      None
    }
  }
  
  pub fn update( &mut self, duration : u32 ) {
    
    if self.action.kind != ActionKind::None {
      if self.action.duration <= duration {
        match self.action.kind {
          ActionKind::MoveTo( pos ) => self.pos = pos,
          _ => {}
        }
        
        self.action = Action::none();
      } else {
        self.action.duration -= duration;
      }
    }
  }
  
  pub fn render<C : Console>( &self, ctx : &mut C ) {
    ctx.put_char_ex( self.pos.x as i32, self.pos.y as i32
                   , self.graphics.symbol
                   , self.graphics.fg
                   , self.graphics.bg );
  }
}

