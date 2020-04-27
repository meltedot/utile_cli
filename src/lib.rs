pub mod cli;

#[cfg(test)]
mod tests {
  use super::*;
  use cli::*;
    #[test]
  fn term() {
    Terminal::new();
  }

  #[test]
  fn output() {
    let t = Terminal::new();
    t.out("Hello ".into());
    t.outln(" World!".into());
    t.outbr();
    t.outln("This is a test.".into());
  }

  #[test]
  fn input() {
    let t = Terminal::new();
    t.ask("> ".into());
  }

  #[test]
  fn delete() {
    let t = Terminal::new();
    t.out("Hello world!".into());
    t.raw_delete_to(5);
    t.raw_delete_offset(-2);
    t.raw_delete_prev();
    t.get_char();
  }

  #[test]
  fn choices() {
    let t = Terminal::new();
    t.outln("Choose...".into());
    let x = t.choices("-> ".into(), vec!["c1".into(), "c22".into(), "c333".into(), "c4444".into()]);
    t.outln(x);
    t.get_char();
  }

  #[test]
  fn get_char() {
    let t = Terminal::new();
    assert_eq!(t.get_char().unwrap(), Key::F3);
  }
}
