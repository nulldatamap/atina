use std::collections;
use std::hash;
use std::cell::RefCell;
use std::mem;
use fnv::FnvHasher;

use util::load_data_file;

type HashMap<K, V> =
  collections::HashMap<K, V, hash::BuildHasherDefault<FnvHasher>>;

type DescMap = RefCell<HashMap<String, Description>>;

thread_local!( static DESCRIPTIONS : DescMap =
  RefCell::new( collections::HashMap::default() ) );

#[derive(RustcDecodable)]
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
  use toml::Value;
  use toml::decode;
  
  let filename = format!( "data/{}.toml", name );
  
  let mut data = load_data_file( &filename );
  
  for (entry_name, entry_value) in data.into_iter() {
    let desc = decode( entry_value ).unwrap();
    
    let qualifying_name = format!( "{}.{}", name, entry_name );
    
    descs.insert( qualifying_name, desc );
  }
}
