fn main() {
    // 러스트 코드에서는 스네이크 케이스로 변수나 함수명를 선언한다. 모든 문자는 소문자 단어 구분은 _로 사용.
    // 러스트는 함수가 어디에서 정의되었는지 위치를 신경쓰지 않는다. 어딘가에 정의되어 있다는 점만 고려한다.

    // 함수에 매개변수가 있는 경우 구체적인 값을 인수(전달인자)로 제공할 수 있다. 기술적으로 여기서 전달되는 상수를 인수(전달인자)라 부르지만 보통 인수와 매개변수를 혼용해서 부른다.
    another_function(5);

    print_labeled_measurement(5, 'h');

    // 구문(명령문)과 표현식
    // 함수의 본문은 필요에 따라 표현식으로 끝나는 구문으로 구성된다.
    // 지금까지는 종결 표현식이 없는 함수만 다뤘기에, 표현식이 구문의 일부로 여겨질 수 있다.
    // 러스트는 표현식 기반 언어로, 구문과 표현식이 어떤 차이가 있는지 알아야 한다. (다른 언어에서는 이와 같은 차이가 없다.)
    // - 구문(명령문): 일부 작업을 수행하고 값을 반환하지 않는 명령
    // - 표현식: 결과 값으로 평가된다.
    let y = 6; // 구문(명령문)
               // let x = (let y = 6); 구문은 반환이 없기 때문에 바로 할당이 불가함.
               // 다른 언어 같은 경우 x = y = 6;으로 x, y 모두에 6을 대입할 수 있다.
    println!("The value of y is: {}", y);

    // 위에서 6은 6의 값을 평가되는 표현식이다.
    // 이외 함수, 매크로, 중괄호 블록은 표현식이다.
    let y = {
        let x = 3;
        x + 1 // 표현식이지만 끝에 ;을 추가할 경우 구문이 되며 반환 값이 아니게 된다.
    };
    println!("The value of y is: {}", y); // 4

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// 매개변수를 추가하며 반드시 각 매개변수의 타입을 정의해야 한다.
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5 // 값을 반환하려는 표현식이기에 ; 없음
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
