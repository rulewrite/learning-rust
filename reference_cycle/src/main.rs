use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a 초기 rc count = {}", Rc::strong_count(&a));
    println!("a 다음 아이템 = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("b 생성 후 a의 rc count = {}", Rc::strong_count(&a));
    println!("b 초기 rc count = {}", Rc::strong_count(&b));
    println!("b 다음 아이템 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("a를 변경한 후 b의 rc count = {}", Rc::strong_count(&b));
    println!("a를 변경한 후 a의 rc count = {}", Rc::strong_count(&a));

    // 다음 줄의 주석을 제거하여 순환이 있는지 확인한다.
    // 이것은 스택 오버플로가 발생한다.
    // println!("a next item = {:?}", a.tail());
}
