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
  use std::path::Path;
  use std::io::Read;
  use std::fs::File;
  use toml::{Parser, Value};
  
  let mut content = String::new();
  
  File::open( Path::new( "data/tile.toml" ) ).unwrap().read_to_string( &mut content );
  
  let mut parser = Parser::new( &content );
  match parser.parse() {
    None => panic!( "{:?}", parser.errors ),
    Some( v ) => {
      DESCRIPTIONS.with( |descs| {
        let mut d = descs.borrow_mut();
        
        for (k, w) in v.into_iter() {
          let t = w.as_table().unwrap();
          let qualname = format!( "tile.{}", k );
          let name = t.get( "name" ).unwrap().as_str().unwrap().to_string();
          let desc = t.get( "description" ).unwrap().as_str().unwrap().to_string();
          d.insert( qualname, Description::new( name, desc ) );
        }
      } );
    }
  }
  
}
