extern crate tui;

use tui::{ButtonA, ButtonB, Widget, WidgetRef};

pub fn main() {
    println!("I am main() in src/separate.rs");

    let btn_a: WidgetRef = Box::new(ButtonA { text: String::from("ButtonA: using generics")} ).into();
    let btn_a_ref = btn_a.weak_ref();

    let btn_b: WidgetRef = Box::new(ButtonB { text: String::from("ButtonB: simple")} ).into();
    let btn_b_ref = btn_b.weak_ref();

    btn_a.render(&btn_a_ref);
    btn_b.render(&btn_a_ref);

    println!("Switching focus");

    btn_a.render(&btn_b_ref);
    btn_b.render(&btn_b_ref);

}
