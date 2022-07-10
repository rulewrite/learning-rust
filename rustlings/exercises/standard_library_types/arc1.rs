// arc1.rs
// In this exercise, we are given a Vec of u32 called "numbers" with values ranging
// from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ]
// We would like to use this set of numbers within 8 different threads simultaneously.
// Each thread is going to get the sum of every eighth value, with an offset.
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
// 이 연습에서는 "numbers"라고 하는 u32의 Vec이 제공되며 값은 다음과 같다.
// 0에서 99까지 -- [ 0, 1, 2, ..., 98, 99 ]
// 우리는 8개의 다른 스레드 내에서 이 숫자 세트를 동시에 사용하고자 한다.
// 각 스레드는 오프셋과 함께 모든 8번째 값의 합계를 가져온다.
// 첫번째 스레드 (오프셋 0)은 0, 8, 16, ...
// 두번째 스레드 (오프셋 1)은 1, 9, 17, ...
// 세번째 스레드 (오프셋 2)은 2, 10, 18, ...
// ...
// 여덟번째 스레드 (오프셋 7)은 7, 15, 23, ...

// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
// 스레드를 사용하기 때문에 값은 스레드로부터 안전해야 한다. 그러므로,
// 우리는 Arc를 사용하고 있다. 두 TODO를 각각 변경해야 한다.

// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the `numbers` Vec!
// `shared_numbers`에 값을 입력하여 이 코드를 컴파일 한다.
// 첫번째 TODO 주석은 `child_numbers`에 대한 초기 바인딩을 생성한다.
// 두번째 TODO 주석에서 `numbers` Vec의 복사본을 만들지 마라.

// Execute `rustlings hint arc1` for hints :)
// `shared_numbers`를 숫자 벡터의 `Arc`로 만든다. 그 후 순서대로
// `numbers`의 사본을 생성하지 않으려면 `child_numbers`를 생성해야 한다.
// 루프 내부에 있지만 여전히 메인 스레드에 있다.
//
// `child_numbers`는 숫자 대신 Arc의 복제본이어야 한다.
// 숫자의 스레드 로컬 복사본이다.
//
// 기본 개념을 이해하면 간단한 예제이지만,
// 너무 많은 어려움을 겪고 있다면 16장을 모두 읽어보자.
// https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 8;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
