extern crate pancurses;
use pancurses::{Window, Input, initscr};

/// A terminal containing a pancurses window.
/// 
/// Contains multiple abstractions over pancurses.
/// 
/// # Examples
/// Creating a terminal:
/// ```
/// let t = Terminal::new()
/// ```
pub struct Terminal {
  win: Window,
  layers: Vec<Layer>
}

/// An arbitrary type that contains a position and content.
/// A layer can be positioned anywhere in the console, and it can instead be edited as a string without manual manipulation of the cursor.
/// 
/// A layer is one dimensional, meaning that it does not fit multiple lines.
/// 
/// Layers may shrink and grow, however have allocated space in the terminal, for example, a layer with content "Hello" which then is changed to "Bye" will actually appear on the console as "Bye  " (with two spaces).
/// This means that they have their own space depending on their longest length string.
/// This trait is useful for hiding certain objects from view, however can be reset using `shrink` which removes trailing whitespace.
/// 
/// # Examples
/// A layer can be initialized using `new`:
/// ```
/// let layer = Layer::new(0, 0);
/// layer.set_content("Hello world!".into());
/// ```
/// 
/// A layer can then be displayed from a terminal either by being added using `add_layer` and then displayed using `refresh`:
/// ```
/// let new: &Layer = t.add_layer(layer);
/// t.refresh();
/// ```
/// 
/// Or it can be displayed externally by using `draw_layer`:
/// ```
/// t.draw_layer(&layer);
/// t.draw_layer_static(&layer); // <- a layer can also be drawn without editting the cursor position
/// ```
/// 
/// A layer also contains another type of content called `inner_content` which will *never* be displayed to the terminal, however may contain useful data about the layer:
/// ```
/// layer.inner_content = "Hello rust!".into();
/// layer.inner_to_outer(); // <- replaces the outer content with the inner content.
/// ```
pub struct Layer {
  pub posx: i32,
  pub posy: i32,
  pub inner_content: String,
  content: String,
  length: usize
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


impl Layer {
  /// Returns a new layer at `posx`, `posy`
  pub fn new(posx: i32, posy: i32) -> Layer {
    Layer { posx, posy, content: String::new(), inner_content: String::new(), length: 0 }
  }

  /// Returns a clone of the outer content of the layer.
  pub fn get_content(&self) -> String {
    self.content.clone()
  }

  /// Sets the current outer content of the layer.
  pub fn set_content(&mut self, c: String) {
    self.content = c;
    if self.content.len() > self.length {
      self.length = self.content.len();
    }
  }

  /// Replaces the outer content with the inner content.
  pub fn inner_to_outer(&mut self) {
    self.set_content(self.inner_content.clone());
  }

  /// Removes any hiding content.
  pub fn shrink(&mut self) {
    self.length = self.content.len();
  }
}

impl Terminal {

  /// Creates a new terminal
  pub fn new() -> Terminal {
    let win = initscr();
    win.keypad(true);
    Terminal { win, layers: vec![] }
  }

  pub fn add_layer(&mut self, layer: Layer) -> &Layer {
    let idx = self.layers.len() - 1;
    self.layers.push(layer);
    &self.layers[idx]
  }

  /// Refreshes and re-draws all layers.
  pub fn refresh(&self) {
    self.win.refresh();
    let here = self.raw_posxy();
    for layer in &self.layers {
      self.raw_move(layer.posx, layer.posy);
      self.raw_out(layer.get_content());
    }
    self.raw_move(here.0, here.1);
    self.win.refresh();
  }

  /// Draws a layer to the console.
  pub fn draw_layer(&self, layer: &Layer) {
    self.raw_move(layer.posx, layer.posy);
    self.out_static(" ".repeat(layer.length)); // clear layer
    self.out(layer.get_content());
  }

  /// Draws a layer to the console however does not affect the cursor.
  pub fn draw_layer_static(&self, layer: &Layer) {
    let here = self.raw_posxy();
    self.draw_layer(layer);
    self.raw_move(here.0, here.1);
  }
  
  /// Outputs a string over the cursor.
  pub fn out(&self, s: String) {
    self.win.printw(s);
    self.refresh();
  }

  /// Outputs a string that does not affect the cursor position.
  pub fn out_static(&self, s: String) {
    let here = self.raw_posxy();
    self.out(s);
    self.raw_move(here.0, here.1);
  }

  /// Outputs a string over the cursor and outputs a break / newline.
  pub fn outln(&self, s: String) {
    self.win.printw(s);
    self.outbr();
  }

  /// Outputs a newline.
  pub fn outbr(&self) {
    self.win.printw(String::from("\n"));
    self.refresh();
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

  /// Returns a character however hides it from input.
  pub fn get_char_hidden(&self) -> Option<Key> {
    let here = self.raw_posxy();
    let ret = self.get_char();
    match ret {
      Some(Key::Alpha(c)) => { self.raw_delete_prev(); },
      _ | None => ()
    }
    self.raw_move(here.0, here.1);
    ret
  }

  /// Asks the user for input, prefixing the question with `prefix`
  /// 
  /// # Examples
  /// ```
  /// t.ask("> ".into())
  /// ```
  pub fn ask(&self, prefix: String) -> String {
    self.out(prefix);
    let mut r = Layer::new(self.raw_posx(), self.raw_posy());
    while let Some(i) = self.get_char_hidden() {
      match i {
        Key::Enter => break,
        Key::Backspace => {
          if let Some(i) = r.get_content().pop() {
            let mut content = r.get_content();
            content.pop();
            r.set_content(content);
            self.draw_layer(&r);
          } else {
            self.raw_move_next();
            continue;
          }
        },
        Key::Alpha(c) => {
          let mut content = r.get_content();
          content.push(c);
          r.set_content(content);
          self.draw_layer(&r);
        },
        _ => continue,
      }
    }
    r.get_content()
  }

  /// Asks the user for input, however the input is masked by a series of `mask` to hide the input.
  pub fn mask(&self, prefix: String, mask: char) -> String {
    self.out(prefix);
    let mut r = Layer::new(self.raw_posx(), self.raw_posy());
    let mut s = String::new();
    while let Some(i) = self.get_char_hidden() {
      match i {
        Key::Enter => break,
        Key::Backspace => {
          if let Some(i) = r.get_content().pop() {
            let mut content = r.get_content();
            content.pop();
            r.set_content(content);
            self.draw_layer(&r);
            s.pop();
          } else {
            self.raw_move_next();
            continue;
          }
        },
        Key::Alpha(c) => {
          let mut content = r.get_content();
          content.push(mask);
          s.push(c);
          r.set_content(content);
          self.draw_layer(&r);
        },
        _ => continue,
      }
    }
    s
  }

  /// Asks a y/n question to the user, returning a boolean (true if yes).
  /// 
  /// The `suffix` parameter must be specified like "exampley/examplen" (must contain a '/')
  /// The `default` parameter is the default highlighted y/n
  /// 
  /// You can choose a yes or a no using the left and right arrow keys.
  /// # Examples
  /// ```
  /// t.yesno("y/n".into(), true)
  /// ```
  /// Outputs: (Y/n)
  pub fn yesno(&self, suffix: String, default: bool) -> bool {
    let yn: Vec<String> = suffix.split('/').map(|s| String::from(s)).collect();
    if yn.len() == 1 {
      panic!("Expected a '/' character separating a yes no question!");
    }
    let mut ynl = Layer::new(self.raw_posx(), self.raw_posy());
    let mut y = yn[0].clone();
    let mut n = yn[1].clone();
    match default {
      true => { y = y.to_uppercase(); },
      false => { n = n.to_uppercase(); },
    }
    ynl.set_content(format!("({}/{})", y.clone(), n.clone()));
    self.draw_layer(&ynl);
    let mut ret = default;
    while let Some(i) = self.get_char_hidden() {
      match i {
        Key::Enter => break,
        Key::ArrowRight => { ret = false; },
        Key::ArrowLeft => { ret = true; },
        _ => continue
      }
      match ret {
        true => { y = { n = n.to_lowercase(); y.to_uppercase() } },
        false => { n = { y = y.to_lowercase(); n.to_uppercase() } },
      }
      ynl.set_content(format!("({}/{})", y.clone(), n.clone()));
      self.draw_layer(&ynl);
    }
    ret
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
    self.outbr();
    let mut layers: Vec<Layer> = vec![];
    for (i , str) in strs.iter().enumerate() {
      let mut l = Layer::new(self.raw_posx(), self.raw_posy() + i as i32);
      l.inner_content = str.clone();
      if i == 0 {
        l.set_content(format!("{}{}", prefix.clone(), l.inner_content.clone()));
      } else { l.inner_to_outer(); }
      self.draw_layer_static(&l);
      layers.push(l);
    }

    self.raw_move_offset(0, layers.len() as i32);
    let mut y = 0;
    while let Some(i) = self.get_char_hidden() {
      match i {
        Key::Enter => break,
        Key::ArrowDown => {
          if (y < layers.len() - 1) {
            y += 1;
          }
        },
        Key::ArrowUp => {
          if (y > 0) {
            y -= 1;
          }
        }
        _ => continue
      }
      for (i, l) in layers.iter_mut().enumerate() {
        if i == y {
          l.set_content(format!("{}{}", prefix.clone(), l.inner_content.clone()));
        } else { l.inner_to_outer(); }
        self.draw_layer_static(&l);
      }
    }
    
    layers[y].inner_content.clone()
  }
}