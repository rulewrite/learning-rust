fn main() {
  // 변수는 기본적으로 불변. `mut`을 선언하여 가변성 변수로 설정 가능
  let mut x = 5;
  println!("The value of x is : {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  // 상수는 표현식으로만 선언될 수 있다. (런타임 중에 계산되는 함수 결과 등에 사용 불가)
  // 타입 선언을 필히 해야 함. 추론이 안되는 듯?
  // 컴파일 중에 결과를 평가할 수 있으므로 10,800처럼 난해하지 않고, 이해하기 쉽도록 선언.
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("The value of x is: {}", THREE_HOURS_IN_SECONDS);

  // Shadowing
  // 같은 이름의 새 변수를 선언할 수 있다. 이전 변수를 가리게(Shadowing) 됨.
  let y = 5;
  let y = y + 1;
  {
    let y = y * 2;
    println!("내부 스코프에서 y 변수의 값은: {}", y); // 12
  }
  println!("y 변수의 값은: {}", y); // 6
}
