use tcod::console::{TextAlignment, BackgroundFlag};
use tcod::{Console, RootConsole};

use ::util::ScreenPos;

pub struct TextField {
  position  : ScreenPos,
  text      : String,
  alignment : TextAlignment
}

impl TextField {
  pub fn new( text : String, position : ScreenPos
            , alignment : TextAlignment ) -> TextField {
              
    TextField {
      text:      text,
      position:  position,
      alignment: alignment
    }
  }

  pub fn render<C : Console>( &self, ctx : &mut C ) {
    ctx.print_ex( self.position.x, self.position.y
                , BackgroundFlag::None
                , self.alignment
                , &self.text );
  }
}

pub struct SelectionList {
  position  : ScreenPos,
  elements  : Vec<String>,
  selected  : usize,
  widest    : usize,
  wrapping  : bool,
  alignment : TextAlignment
}

impl SelectionList {
  
  pub fn new( elms : Vec<String>, position : ScreenPos
            , wrapping : bool, alignment : TextAlignment ) -> SelectionList {
    
    assert!( !elms.is_empty(), "Invariant: `non_empty` was false." );
    
    let widest =
      elms.iter().fold( 0, |x, e| if e.len() > x { e.len() } else { x } );
    
    SelectionList {
      position:  position,
      elements:  elms,
      selected:  0,
      widest  :  widest,
      wrapping:  wrapping,
      alignment: alignment
    }
  }
  
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
      ctx.print_ex( self.position.x, self.position.y + i as i32
                  , BackgroundFlag::None
                  , self.alignment
                  , field );
    }
    
    let x_pos = self.position.x - self.widest as i32 / 2;
    let y_pos = self.position.y + self.selected as i32;
    
    ctx.set_char_foreground( x_pos - 1, y_pos , ::tcod::colors::WHITE ); 
    ctx.set_char( x_pos - 1, y_pos , chars::ARROW2_E );
  }
  
  pub fn update( &mut self, root : &mut RootConsole ) -> Option<usize> {
    use ::tcod::input::{KeyCode};
    
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
