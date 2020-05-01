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
    let a = t.ask("> ".into());
    println!("{}", a);
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

  #[test]
  fn mask() {
    let t = Terminal::new();
    println!("{}", t.mask("> ".into(), '?'));
  }

  #[test]
  fn yes_no() {
    let t = Terminal::new();
    println!("{}", t.yesno("y/n".into(), true));
  }

  #[test]
  fn layer2d() {
    let mut t = Terminal::new();
    let mut l = Layer::new(0, 0);
    l.set_content("Xana".into());
    let l2d = Layer2D::new(0, 0, 5, 5, l);
    let l2d = t.add_layer2d(l2d);
    t.refresh();
    t.get_char();
  }
}
