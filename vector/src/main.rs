fn main() {
    // 초기 값이 없기 때문에 타입 명시
    let v: Vec<i32> = Vec::new();

    // 매크로를 사용하여 초기화 하면 타입 추론이 가능하다.
    let v = vec![1, 2, 3];

    // 벡터를 업데이트 하려면 가변으로 선언해야 하며, 마찬가지로 타입 추론이 가능하다.
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];

        // v로 뭔가를 수행한다.
    } // v가 스코프 밖으로 벗어났고 해제된다.

    // 벡터 요소를 읽는 두가지 방법
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // panic!
    // let does_not_exist = &v[100];
    // None이 반환
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // 첫번째 요소를 참조하고 있는 동안에 요소를 추가하는 건 불가하다.
    // v.push(6);
    println!("The first element is: {}", first);

    // 벡터 값들 순횐
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    // 가변 참조로 순회하여 값 변경하기
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // * 역참조 연산자로 i값을 우선 가져와 += 연산을 수행한다.
        *i += 50;
    }

    // 열거형을 사용하여 다양한 타입을 하나의 벡터에 담기
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
