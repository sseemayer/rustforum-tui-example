use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref};
use std::fmt::{Display, Debug};

pub trait Widget {
    fn render(&self, focus: &WidgetWeakRef);
}

pub struct WidgetRef (Rc<RefCell<Box<Widget>>>);
pub struct WidgetWeakRef (Weak<RefCell<Box<Widget>>>);

impl WidgetRef {
    pub fn weak_ref(&self) -> WidgetWeakRef {
        WidgetWeakRef(Rc::downgrade(&self.0))
    }

    pub fn render(&self, focus: &WidgetWeakRef) {
        self.0.borrow().render(focus);
    }
}

impl<W: Widget> PartialEq<W> for WidgetWeakRef {
    fn eq(&self, other: &W) -> bool {
        if let Some(strong_ref) = Weak::upgrade(&self.0) {
            &WidgetRef(strong_ref) == other
        } else {
            false
        }
    }
}

impl<W: Widget> PartialEq<W> for WidgetRef {
    fn eq(&self, other: &W) -> bool {

        use std::ops::Deref;
        use std::borrow::Borrow;

        let rc_ref: &RefCell<Box<Widget>> = self.0.deref();
        let ref_ref: Ref<Box<Widget>> = rc_ref.borrow();
        let self_ref: &Widget = ref_ref.deref().deref();

        let is_same = self_ref as *const Widget == other as *const Widget;

        is_same
    }
}

impl<T: Widget + 'static> From<Box<T>> for WidgetRef {
    fn from(w: Box<T>) -> WidgetRef {
        WidgetRef(Rc::new(RefCell::new(w)))
    }
}


pub struct ButtonA<T: Display+Debug> {
    pub text: T,
}

impl<T: Display+Debug> Widget for ButtonA<T> {
    fn render(&self, focus: &WidgetWeakRef) {
        println!("{} (focus={})", self.text, focus == self);
    }
}

pub struct ButtonB {
    pub text: String,

}

impl Widget for ButtonB {
    fn render(&self, focus: &WidgetWeakRef) {
        println!("{} (focus={})", self.text, focus == self);
    }
}

pub fn main() {
    println!("I am main() in src/lib.rs");

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
