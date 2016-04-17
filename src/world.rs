use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Read;
use std::io;
use std::num;
use std::error::Error;
use std::str;

use ::tcod::{RootConsole, Console};

use self::MapLoadingError::*;

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
    IOError( err )
  }
}

impl From<num::ParseIntError> for MapLoadingError {
  fn from( err : num::ParseIntError ) -> MapLoadingError {
    ParseIntError( err )
  }
}

pub fn load_map( filePath : &Path ) -> Result<Map, MapLoadingError> {
  let mut file = try!( File::open( filePath ) );
  
  let mut file_contents = String::new();
  try!( file.read_to_string( &mut file_contents ));
  
  let lines : Vec<&str> = file_contents.lines().collect();
  
  if lines.len() < 1 {
    return Err( EmptyFile )
  }
  
  let settings : Vec<&str> = lines[0].split( ',' ).map( str::trim ).collect();
  
  if settings.len() != 2 {
    return Err( InvalidSettings )
  }
  
  let width = try!( settings[0].parse::<usize>() );
  let height = try!( settings[1].parse::<usize>() );
  
  if lines.len() != height + 1 {
    return Err( InvalidHeight( lines.len(), height ) )
  }
  
  let mut tiles = Vec::with_capacity( width * height );
  
  let mut player_position = None;
  
  for (y, row) in lines[1..].iter().enumerate() {
    if row.len() != width {
      return Err( InvalidWidth( row.len(), width ) )
    }
    
    for (x, chr) in row.chars().enumerate() {
      if let Some( tile ) = Tile::from_config( chr ) {
        tiles.push( tile );
        continue;
      }
      
      if chr == '@' {
        tiles.push( Tile::Floor );
        player_position = Some( (x, y) );
      } else {
        return Err( InvalidTile( chr ) )
      }
    }
  }
  
  
  assert_eq!( tiles.len(), width * height );
  
  if player_position.is_none() {
    return Err( NoPlayerPosition )
  }
  
  Ok( Map {
    tiles: tiles,
    width: width,
    height: height,
    player_position: player_position.unwrap()
  } )
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
    use self::Tile::*;
    
    match self {
      Ground => '.',
      Floor => ' ',
      Wall => '#',
      Rock => chars::BLOCK1,
      Tree => chars::CLUB
    }
  }
}

impl Tile {
  fn from_config( chr : char ) -> Option<Tile> {
    use ::tcod::chars;
    use self::Tile::*;
    
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
  width  : usize,
  height : usize,
  player_position : (usize, usize)
}

impl Map {
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
    
    ctx.put_char_ex( self.player_position.0 as i32
                   , self.player_position.1 as i32
                   , '@'
                   , WHITE, BLACK );
  }
}
