use ::util::*;
use ::world::*;
use ::actor::*;

pub struct Player {
  pub actor : Actor
}

impl Player {
  pub fn new( pos : Position ) -> Player {
    Player {
      actor: Actor::new( pos )
    }
  }
  
  pub fn update( &mut self, world : &World, duration : u32 ) {
    self.actor.update( duration );
  }
}

