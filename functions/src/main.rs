fn main() {
    // 러스트 코드에서는 스네이크 케이스로 변수나 함수명를 선언한다. 모든 문자는 소문자 단어 구분은 _로 사용.
    // 러스트는 함수가 어디에서 정의되었는지 위치를 신경쓰지 않는다. 어딘가에 정의되어 있다는 점만 고려한다.

    // 함수에 매개변수가 있는 경우 구체적인 값을 인수(전달인자)로 제공할 수 있다. 기술적으로 여기서 전달되는 상수를 인수(전달인자)라 부르지만 보통 인수와 매개변수를 혼용해서 부른다.
    another_function(5);

    print_labeled_measurement(5, 'h');
}

// 매개변수를 추가하며 반드시 각 매개변수의 타입을 정의해야 한다.
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
