// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This becomes problematic
// for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
// which also allows us to wrap a recursive type.
// 컴파일 타임에 러스트는 타입이 차지하는 공간을 알아야 한다.
// 이는 값의 일부로 같은 타입의 다른 값을 가질 수 있는 재귀 타입의 경우 문제가 된다.
// 이 문제를 해결하기 위해 힙에 데이터를 저장하는데 사용되는 스마트 포인터인 `Box`를 사용할 수 있다.
// 아울러 재귀 타입을 래핑할 수 있다.
//
// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
// 이 연습에서 구현하는 재귀 타입은 `cons list`(데이터 구조체)이다.
// 함수형 프로그래밍 언어에서 자주 발견된다.
// cons, list의 각 아이템에는 두 개의 요소: 현재 아이템과 다음 아이템의 값이며 마지막 아이템은 `Nil`이라는 값이다.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `unimplemented!()`
// Step 1: 열거형 정의에서 `Box`를 사용하여 코드를 컴파일한다.
// Step 2: `unimplemented!()`를 대체하여 비어 있거나 비어있지 않은 cons 리스트를 모두 생성한다.
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` for hints :)
// Step 1
// 컴파일러의 메세지는 도움이 될 것이다. 실제 타입의 값을 저장할 수 없기 때문이다.
// 재귀 타입으로 작업할 때 해당 값에 대한 참조(포인터)를 저장해야 한다.
// 그러므로 우리는 `List`를 `Box` 안에 넣어야 한다.
// 자세한 내용은 여기를 참조하자.
// https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes
// Step 2
// 빈 리스트를 만드는 것은 매우 간단하다. (힌트: assertions 살펴보기)
// 비어있지 않은 리스트의 경우 Cons "list Builder"를 사용하려 한다.
// 현재 리스트는 정수(i32)중 하나지만 자유롭게 변경해도 된다.
// 다른 타입도 시도해보자.

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    Cons(1, Box::new(Cons(2, Box::new(List::Nil))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
