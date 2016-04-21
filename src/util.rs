
use map::Map;

pub type Position = Pos<u32>;
pub type ScreenPos = Pos<i32>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pos<N> {
  pub x : N,
  pub y : N
}

impl<N> Pos<N> {
  pub fn new( x : N, y : N ) -> Pos<N> {
    Pos {
      x: x,
      y: y
    }
  }
}

impl<M : Into<N>, N> Into<(N, N)> for Pos<M> {
  fn into( self ) -> (N, N) {
    (self.x.into(), self.y.into())
  }
}

impl<N> From<(N, N)> for Pos<N> {
  fn from( crds : (N, N) ) -> Pos<N> {
    Pos::new( crds.0, crds.1 )
  }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
  North,
  East,
  South,
  West,
  NorthEast,
  NorthWest,
  SouthEast,
  SouthWest
}

impl Direction {
  pub fn try_offset_position( self, pos : Position, map : &Map )
    -> Option<Position> {
    use self::Direction::*;
    
    let (ox, oy) = match self {
      North => (0, -1),
      East => (1, 0),
      South => (0, 1),
      West => (-1, 0),
      NorthEast => (1, -1),
      NorthWest => (-1, -1),
      SouthEast => (1, 1),
      SouthWest => (-1, 1)
    };
    
    if ( pos.x != 0 || ox >= 0 )
       && ( pos.x as i32 + ox < map.width as i32 )
       && ( pos.y != 0 || oy >= 0 )
       && ( pos.y as i32 + oy < map.height as i32 ) {
         
      Some( Position::new( (pos.x as i32 + ox) as u32
                         , (pos.y as i32 + oy) as u32 ) )
    } else {
      None
    }
  }
  
  pub fn offset_position( self, pos : Position, map : &Map ) -> Position {
    self.try_offset_position( pos, map ).expect( "offset out of bounds" )
  } 
}
