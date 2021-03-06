use std::io;
use std::io::Read;
use std::num;
use std::path::Path;
use std::fs::File;

use rustc_serialize::Decodable;

use tcod::{Console};

use util::*;
use description::*;

use self::Tile::*;

#[derive(Debug)]
pub enum MapLoadingError {
  IOError( io::Error ),
  ParseIntError( num::ParseIntError ),
  EmptyFile,
  InvalidSettings,
  InvalidWidth( usize, usize ),
  InvalidHeight( usize, usize ),
  InvalidTile( char ),
  NoPlayerPosition
}

impl From<io::Error> for MapLoadingError {
  fn from( err : io::Error ) -> MapLoadingError {
    MapLoadingError::IOError( err )
  }
}

impl From<num::ParseIntError> for MapLoadingError {
  fn from( err : num::ParseIntError ) -> MapLoadingError {
    MapLoadingError::ParseIntError( err )
  }
}

#[derive(Clone, Copy)]
pub enum Tile {
  Ground,
  Floor,
  Wall,
  Rock,
  Tree,
}

impl Into<char> for Tile {
  fn into( self ) -> char {
    use ::tcod::chars;
    
    match self {
      Ground => '.',
      Floor => ' ',
      Wall => '#',
      Rock => chars::BLOCK1,
      Tree => chars::CLUB
    }
  }
}

impl Describe for Tile {
  fn desc_id( &self ) -> String {
    match *self {
      Ground => "tile.ground",
      Floor => "tile.floor",
      Wall => "tile.wall",
      Rock => "tile.rock",
      Tree => "tile.tree"
    }.to_string()
  }
}

impl Tile {
  pub fn is_solid( self ) -> bool {
    match self {
      Ground | Floor => false,
      Wall | Rock | Tree => true
    }
  }
  
  fn from_config( chr : char ) -> Option<Tile> {
        
    Some( match chr {
      '.' => Ground,
      ' ' => Floor,
      '#' => Wall,
      '+' => Rock,
      'T' => Tree,
      _   => return None
    } )
  }
}

pub struct Map {
  tiles  : Vec<Tile>,
  pub width  : usize,
  pub height : usize,
  pub player_position : Position
}

#[derive(RustcDecodable)]
struct MapConfig {
  dimensions : Pos<usize>,
  player_position : Position,
  layout : String,
}

impl Map {
  pub fn load( filename : &str ) -> Result<Map, MapLoadingError> {
    use toml::{Value, Decoder};
    use self::MapLoadingError::*;
    
    let mut data = load_data_file( filename );
    
    let mut map_tbl = match data.remove( "map" ) {
      Some( t@Value::Table( _ ) ) => t,
      _ => panic!( "Failed to decode data file: '{}'", filename )  
    };
    
    let config = MapConfig::decode( &mut Decoder::new( map_tbl ) ).unwrap();
    
    let lines : Vec<&str> = config.layout.lines().collect();
    let (width, height) = config.dimensions.into();
    let mut tiles = Vec::with_capacity( width * height );
    
    if lines.len() != height {
      panic!( "Failed to decode data file: '{}'", filename )
    }
    
    for (y, row) in lines[..].iter().enumerate() {
      if row.len() != width {
        return Err( InvalidWidth( row.len(), width ) )
      }
      
      for (x, chr) in row.chars().enumerate() {
        if let Some( tile ) = Tile::from_config( chr ) {
          tiles.push( tile );
          continue;
        }
        
        return Err( InvalidTile( chr ) )
      }
    }
    
    assert_eq!( tiles.len(), width * height );
    
    Ok( Map {
      tiles: tiles,
      width: width,
      height: height,
      player_position: config.player_position
    } )
  }
  
  pub fn tile_at( &self, pos : Position ) -> Tile {
    let x = pos.x as usize;
    let y = pos.y as usize;
    
    assert!( x < self.width && y < self.height
           , "the given position is outside the map bounds" );
    
    self.tiles[ x + y * self.width ]
  }
  
  pub fn render<C : Console>( &self, ctx : &mut C ) {
    use ::tcod::colors::{WHITE, BLACK};
    
    let tile_poses = self.tiles
      .iter()
      .enumerate()
      .map( |(i, t)| ( (i % self.width, i / self.width), t) );
    
    for ((x, y), &tile) in tile_poses {
      ctx.put_char_ex( x as i32, y as i32
                     , tile.into()
                     , WHITE, BLACK );
    }
    
  }
}
