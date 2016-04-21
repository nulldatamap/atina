use ::tcod::input::{Key, KEY_PRESSED};

use util::*;
use Game;

enum InputState {
  Toplevel
}

pub struct Input {
  state      : InputState,
  key_config : (), // Placeholder
}

impl Input {
  pub fn new() -> Input {
    Input {
      state: InputState::Toplevel,
      key_config: ()
    }
  }
  
  pub fn update( &mut self, game : &Game ) {
    if let Some( new_state ) = match self.state {
      InputState::Toplevel => self.update_toplevel( game )
    } {
      self.state = new_state;
    }
  }
  
  fn update_toplevel( &mut self, game : &Game ) -> Option<InputState> {
    while let Some( key ) = game.root.check_for_keypress( KEY_PRESSED ) {
      // Check for movements
      if let Some( direction ) = Input::directional_key( key ) {
        let world = game.world.borrow();
        let mut player = world.player.borrow_mut();
        
        let maybe_reason = player.actor.move_direction( direction, &world );
        
        if let Some( reason ) = maybe_reason {
          game.message_log.borrow_mut().add_message(
            format!( "You're blocked by {}", reason ) );
        }
      }
    }
    
    None
  }
  
  fn directional_key( key : Key ) -> Option<Direction> {
    use ::tcod::input::KeyCode::*;
    use util::Direction::*;
    
    Some( match key.code {
      Up | NumPad8 => North,
      Down | NumPad2 => South,
      Left | NumPad4 => West,
      Right | NumPad6 => East,
      NumPad7 => NorthWest,
      NumPad9 => NorthEast,
      NumPad1 => SouthWest,
      NumPad3 => SouthEast,
      _ => return None
    } )
  }
}
