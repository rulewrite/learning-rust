fn main() {
  // 변수는 기본적으로 불변. `mut`을 선언하여 가변성 변수로 설정 가능
  let mut x = 5;
  println!("The value of x is : {}", x);
  x = 6;
  println!("The value of x is: {}", x);
}
