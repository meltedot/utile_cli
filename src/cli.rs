extern crate pancurses;
use pancurses::{Window, Input, initscr};

pub struct Terminal {
  win: Window
}

#[derive(Debug, PartialEq)]
pub enum Key {
  Alpha(char),
  Enter,
  Space,
  Backspace,
  ArrowUp,
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12
}

/// A terminal containing a pancurses window.
/// 
/// Contains multiple abstractions over pancurses.
/// 
/// # Examples
/// Creating a terminal:
/// ```
/// let t = Terminal::new()
/// ```
impl Terminal {

  /// Creates a new terminal
  pub fn new() -> Terminal {
    let win = initscr();
    win.keypad(true);
    Terminal { win }
  }
  
  /// Outputs a string over the cursor.
  pub fn out(&self, s: String) {
    self.win.printw(s);
    self.win.refresh();
  }

  /// Outputs a string over the cursor and outputs a break / newline.
  pub fn outln(&self, s: String) {
    self.win.printw(s);
    self.outbr();
  }

  /// Outputs a newline.
  pub fn outbr(&self) {
    self.win.printw(String::from("\n"));
    self.win.refresh();
  }

  /// Outputs a string without refreshing the terminal.
  pub fn raw_out(&self, s: String) {
    self.win.printw(s);
  }

  /// Outputs a string and a break / newline without refreshing the terminal.
  pub fn raw_outln(&self, s: String) {
    self.win.printw(s);
    self.raw_br();
  }

  /// Outputs a newline without refreshing the terminal.
  pub fn raw_br(&self) {
    self.win.printw(String::from("\n"));
  }

  /// Refreshes the terminal, flushing the output and displaying all changes to the screen.
  pub fn raw_refresh(&self) {
    self.win.refresh();
  }

  /// Returns a tuple containing the current cursor position in the form (x, y)
  pub fn raw_posxy(&self) -> (i32, i32) {
    (self.win.get_cur_x(), self.win.get_cur_y())
  }

  /// Returns the x position of the current cursor position.
  pub fn raw_posx(&self) -> i32 {
    self.win.get_cur_x()
  }

  /// Returns the y position of the current cursor position.
  pub fn raw_posy(&self) -> i32 {
    self.win.get_cur_y()
  }

  /// Moves the cursor to position x and y.
  pub fn raw_move(&self, x: i32, y: i32) {
    self.win.mv(y, x);
  }

  /// Offsets the cursor by x and y.
  pub fn raw_move_offset(&self, xoffs: i32, yoffs: i32) {
    self.raw_move(self.raw_posx() + xoffs, self.raw_posy() + yoffs);
  }

  /// Moves the cursor to the start of the line.
  pub fn raw_move_first(&self) {
    self.raw_move(0, self.raw_posy());
  }

  /// Moves one position back (x - 1).
  pub fn raw_move_prev(&self) {
    self.raw_move_offset(-1, 0);
  }

  /// Moves one position forwards (x + 1).
  pub fn raw_move_next(&self) {
    self.raw_move_offset(1, 0);
  }

  /// Deletes the character that the current cursor is on.
  pub fn raw_delete(&self) {
    self.win.delch();
  }

  /// Deletes the last character (not letter).
  pub fn raw_delete_prev(&self) {
    self.raw_move_prev();
    self.raw_delete();
  }

  /// Deletes all the characters from the cursor until the current position is offset position.
  /// 
  /// # Examples
  /// ```
  /// t.out("Hello world!".into());
  /// t.raw_delete_offset(-6);
  /// ```
  /// Output: `Hello `
  pub fn raw_delete_offset(&self, xoffs: i32) {
    let offs = xoffs / xoffs.abs(); // -1 or 1
    let here = self.raw_posx();
    while self.raw_posx() != here + xoffs {
      self.raw_move_offset(offs, 0);
      self.raw_delete();
    }
  }

  /// Deletes, from the start of the line, all the characters until `chars` position.
  /// 
  /// # Examples
  /// ```
  /// t.out("Hello world!".into());
  /// t.raw_delete_from(5);
  /// ```
  /// Output: ` world!`
  pub fn raw_delete_from(&self, chars: usize) {
    self.raw_move_first();
    self.raw_move_offset(chars as i32, 0);
    self.raw_delete_to(0);
  }

  /// Deletes all characters from the cursor until the x position is `x`
  /// 
  /// # Examples
  /// ```
  /// t.out("Hello world!".into());
  /// t.raw_delete_to(5);
  /// ```
  /// Output: `Hello`
  pub fn raw_delete_to(&self, x: i32) {
    let offs = if x > self.raw_posx() {
      1
    } else {
      -1
    };

    while self.raw_posx() != x {
      self.raw_move_offset(offs, 0);
      self.raw_delete();
    }
  }

  /// Gets a character from input.
  pub fn get_char(&self) -> Option<Key> {
    match self.win.getch() {
      Some(Input::Character('\n')) | Some(Input::Character('\r')) => Some(Key::Enter),
      Some(Input::Character('\x08')) => Some(Key::Backspace),
      Some(Input::Character(' ')) => Some(Key::Space),
      Some(Input::Character(c)) => Some(Key::Alpha(c)),
      Some(Input::KeyUp) => Some(Key::ArrowUp),
      Some(Input::KeyDown) => Some(Key::ArrowDown),
      Some(Input::KeyLeft) => Some(Key::ArrowLeft),
      Some(Input::KeyRight) => Some(Key::ArrowRight),
      Some(Input::KeyF1) => Some(Key::F1),
      Some(Input::KeyF2) => Some(Key::F2),
      Some(Input::KeyF3) => Some(Key::F3),
      Some(Input::KeyF4) => Some(Key::F4),
      Some(Input::KeyF5) => Some(Key::F5),
      Some(Input::KeyF6) => Some(Key::F6),
      Some(Input::KeyF7) => Some(Key::F7),
      Some(Input::KeyF8) => Some(Key::F8),
      Some(Input::KeyF9) => Some(Key::F9),
      Some(Input::KeyF10) => Some(Key::F10),
      Some(Input::KeyF11) => Some(Key::F11),
      Some(Input::KeyF12) => Some(Key::F12),
      None => None,
      _ => None,
    }
  }

  /// Asks the user for input, prefixing the question with `prefix`
  /// 
  /// # Examples
  /// ```
  /// t.ask("> ".into())
  /// ```
  pub fn ask(&self, prefix: String) -> String {
    self.out(prefix);
    let mut r = String::new();
    let mut x = 0;
    while let Some(i) = self.get_char() {
      match i {
        Key::Enter => break,
        Key::Backspace => {
          if x != 0 { x -= 1; } else { self.raw_move_next(); continue; }
          self.raw_delete_prev();
        },
        Key::Alpha(c) => {
          x += 1;
          r.push(c);
          self.raw_out(c.to_string());
        },
        _ => continue
      }
    }
    r
  }

  /// Gives the user choices between strings.
  /// Takes in a `prefix` to be added as a prefix on the current choice, and vector `strs` as the choices.
  /// The result output is a list in which options can be highlighted by using the arrow keys.
  /// # Examples
  /// ```
  /// let x = t.choices("-> ".into(), vec!["c1".into(), "c22".into(), "c333".into(), "c4444".into()]);
  /// t.outln(x);
  /// ```
  /// Output if selected was `c2`: `c2`
  pub fn choices(&self, prefix: String, strs: Vec<String>) -> String {
    let mut y: i32 = 0;
    let here = self.raw_posxy();
    let mut i = 0;
    for str in strs.clone() {
      if i == 0 {
        self.out(prefix.clone());
      }
      self.outln(str);
      i += 1;
    }
    self.raw_move(here.0 + (prefix.len() + strs[0].clone().len()) as i32, here.1);
    while let Some(i) = self.get_char() {
      match i {
        Key::Enter => break,
        Key::ArrowDown => {
          if y + 1 >= strs.len() as i32 {
            continue;
          }
          y += 1;
          self.raw_delete_to(0);
          self.out(strs[(y - 1) as usize].clone());
          self.raw_move_offset(0, 1);
          self.raw_delete_from(prefix.clone().len() + strs[y as usize].clone().len());
          self.out(prefix.clone());
          self.out(strs[y as usize].clone());
        },
        Key::ArrowUp => {
          if (y - 1) < 0 {
            continue;
          }
          y -= 1;
          self.raw_delete_to(0);
          self.out(strs[(y + 1) as usize].clone());
          self.raw_move_offset(0, -1);
          self.raw_delete_from(prefix.clone().len() + strs[y as usize].clone().len());
          self.out(prefix.clone());
          self.out(strs[y as usize].clone());
        },
        _ => continue,
      }
    }
    self.raw_move_offset(0, strs.len() as i32 - y - 1);
    strs[y as usize].clone()
  }
}