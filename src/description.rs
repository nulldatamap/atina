use std::collections;
use std::hash;
use std::cell::RefCell;
use std::mem;
use fnv::FnvHasher;

type HashMap<K, V> =
  collections::HashMap<K, V, hash::BuildHasherDefault<FnvHasher>>;

type DescMap = RefCell<HashMap<String, Description>>;

thread_local!( static DESCRIPTIONS : DescMap =
  RefCell::new( collections::HashMap::default() ) );

pub struct Description {
  name        : String,
  description : String
}

impl Description {
  fn new( name : String, desc : String ) -> Description {
    Description {
      name:        name,
      description: desc
    }
  }
  
  pub fn name( &self ) -> &str {
    &self.name
  }
}

pub trait Describe {
  fn desc_id( &self ) -> String;
  
  fn description<'a>( &'a self ) -> &'a Description {
    let id = self.desc_id();
    
    DESCRIPTIONS.with( &mut |descs : &DescMap| {
      let d = descs.borrow();
      let ret = d.get( &id )
        .expect( &format!( "No entry `{}` was found in the descriptions.", id ) );
      
      // The lifetime of `d` is currently at most the same as `descs.borrow()`
      // which doesn't actually represent the true lifetime of the parent
      // structure. As long as the contents of the `DESCRIPTIONS` map is not
      // mutated beyond initialization, an assumption of an lifetime bound to
      // the lifetime of the caller should be safe:
      unsafe { mem::transmute( ret ) }
    } )
  }
}

pub fn load_descriptions() {
  DESCRIPTIONS.with( |descs| {
        let mut d = descs.borrow_mut();
        load_file( "tile", &mut d );
      } );
}

fn load_file( name : &str, descs : &mut HashMap<String, Description> ) {
  use std::path::Path;
  use std::io::Read;
  use std::fs::File;
  use toml::{Parser, Value};
  
  let filename = format!( "data/{}.toml", name );
  
  let mut data_file = File::open( Path::new( &filename ) )
    .expect( &format!( "Failed to load data file: '{}'", filename ) );
  
  let file_size = data_file.metadata().map( |md| md.len() )
    .expect( &format!( "Failed to get file size of data file '{}'", filename ) );
  
  let mut source = String::with_capacity( file_size as usize );
  
  data_file.read_to_string( &mut source )
    .expect( &format!( "Failed to read data file: '{}'", filename ) );
  
  let mut parser = Parser::new( &source );
  
  let mut data;
  
  match parser.parse() {
    None => panic!( "Failed to parse data file: '{}'", filename ),
    Some( d ) => data = d
  }
  
  for (entry_name, entry_value) in data.into_iter() {
    let mut entry_table;
    
    match entry_value {
      Value::Table( tbl ) => entry_table = tbl,
      _ => panic!( "Failed to decode data file: '{}'", filename )
    }
    
    let desc_name;
    
    match entry_table.remove( "name" ) {
      Some( Value::String( s ) ) => desc_name = s,
      _ => panic!( "Failed to decode data file: '{}'", filename )
    }
    
    let desc_description;
    
    match entry_table.remove( "description" ) {
      Some( Value::String( s ) ) => desc_description = s,
      _ => panic!( "Failed to decode data file: '{}'", filename )
    }
    
    let qualifying_name = format!( "{}.{}", name, entry_name );
    
    descs.insert( qualifying_name
                , Description::new( desc_name, desc_description ) );
    
  }
}
