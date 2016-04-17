use super::tcod::{Color};
use super::tcod::{Console, RootConsole};

pub type Position = (i32, i32);

pub struct TextField {
  position : Position,
  text : String
}

impl TextField {
  pub fn new( text : String, position : Position ) -> TextField {
    TextField {
      text: text,
      position: position
    }
  }

  pub fn render<C : Console>( &self, ctx : &mut C ) {
    ctx.print( self.position.0, self.position.1, &self.text );
  }
}

// Invariants:
//  non_empty: elements.len() >= 1
//  in_bounds: 0 <= selected < elements.len()   
pub struct SelectionList {
  position : Position,
  elements : Vec<String>,
  selected : usize,
  wrapping : bool
}

impl SelectionList {
  
  // Assures:
  //  Invariants non_empty & in_bounds
  //  selected == 0
  pub fn new( elms : Vec<String>, position : Position, wrapping : bool ) -> SelectionList {
    assert!( !elms.is_empty(), "Invariant: `non_empty` was false." );
    
    SelectionList {
      position: position,
      elements: elms,
      selected: 0,
      wrapping: wrapping
    }
  }
  
  // Assures:
  //  Invariant in_bounds
  //  abs( direction ) == 1
  fn move_selection( &mut self, direction : isize ) {
    use ::std::cmp::{min, max};
    
    assert!( direction.abs() == 1, "`direction` must be other 1 or -1" );
    
    let sel = self.selected as isize + direction;
    
    if self.wrapping {
      self.selected = (self.elements.len() as isize + sel) as usize % self.elements.len();
    } else {
      self.selected = min( 0, max( (sel as usize), self.elements.len() - 1 ) );
    }
  }
  
  fn next_selection( &mut self ) {
    self.move_selection( 1 );
  }
  
  fn prev_selection( &mut self ) {
    self.move_selection( -1 );
  }
  
  pub fn get_selection( &self ) -> usize {
    self.selected
  }
  
  pub fn render<C : Console>( &self, ctx : &mut C ) {
    use ::tcod::chars;
    
    for (i, field) in self.elements.iter().enumerate() {
      ctx.print( self.position.0, self.position.1 + i as i32, field );
    }
    
    let x_pos = self.position.0;
    let y_pos = self.position.1 + self.selected as i32;
    
    ctx.set_char_foreground( x_pos - 1, y_pos , ::tcod::colors::WHITE ); 
    ctx.set_char( x_pos - 1, y_pos , chars::ARROW2_E );
  }
  
  pub fn update( &mut self, root : &mut RootConsole ) -> Option<usize> {
    use ::tcod::input::{Key, KeyCode};
    
    let mkey = root.check_for_keypress( ::tcod::input::KEY_PRESSED );
    
    if mkey.is_none() {
      return None;
    }
    
    match mkey.unwrap().code {
      KeyCode::Up => self.prev_selection(),
      KeyCode::Down => self.next_selection(),
      KeyCode::Enter => return Some( self.get_selection() ),
      _ => {}
    }
    
    None
  }
}
