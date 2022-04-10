fn main() {
    println!("Hello, world!");

    // 러스트 코드에서는 스네이크 케이스로 변수나 함수명를 선언한다. 모든 문자는 소문자 단어 구분은 _로 사용.
    // 러스트는 함수가 어디에서 정의되었는지 위치를 신경쓰지 않는다. 어딘가에 정의되어 있다는 점만 고려한다.
    another_function();
}

fn another_function() {
    println!("Another function.");
}
