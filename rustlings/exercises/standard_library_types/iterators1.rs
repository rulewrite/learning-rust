// iterators1.rs
//
// Make me compile by filling in the `???`s
// `???`를 채워 컴파일되도록 하자.
//
// When performing operations on elements within a collection, iterators are essential.
// This module helps you get familiar with the structure of using an iterator and
// how to go through elements within an iterable collection.
// 컬렉션 내의 요소에 대한 작업을 수행할 때 이터레이터는 필수다.
// 이 모듈은 이터레이터를 사용하는 구조와
// 반복 가능한 컬렉션 내에서 요소를 살펴보는 방법에 익숙해지도록 도와준다.
//
// Execute `rustlings hint iterators1` for hints :D

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);
}
