fn main() {
    // prelude되지 않으므로 use로 가져와야 한다.
    use std::collections::HashMap;

    // 해시맵 생성
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    // collect 메소드로 해시 맵 생성하기
    let terms = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // collect는 다양한 컬렉션 타입으로 변환이 가능하기 때문에 타입 명시가 필요하다.
    let mut scores: HashMap<_, _> = terms.into_iter().zip(initial_scores.into_iter()).collect();

    // 해시맵의 소유권
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name, field_value는 이 시점에선 유효하지 않다.
    // println!("{}", field_value);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    // 해시 맵 값에 접근하기
    let score = scores.get(&team_name);

    // 해시 맵 키, 값 쌍 순회하기
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 해시맵 값 덮어쓰기
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores); // {"Blue": 25}

    // 해시맵에 해당 키 값이 없을때만 삽입하기
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // .entry는 Entry<std::string::String, i32>를 반환
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores); // {"Yellow": 50, "Blue": 10}

    // 기존 값을 기반으로 새 값으로 갱신하기
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map); // {"world": 2, "wonderful": 1, "hello": 1}
}
