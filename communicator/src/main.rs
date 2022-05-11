// communicator의 라이브러리 크레이트를 가져오기 위한 명령
// src/main.rs(현 파일)는 바이너리 크레이트의 루트 파일
// src/lib.rs는 라이브러리 크레이트의 루트 파일
// 만약 우리 프로젝트에 서브 모듈에서 외부 크레이트를 사용하더라도 `extern crate` 명령은 루트 파일에 존재해야 한다.
extern crate communicator;

pub mod a {
  pub mod series {
    pub mod of {
      pub fn nested_modules() {}
    }
  }
}

// of:: 를 사용해서 접근 가능
// use a::series::of;
// use 구문에 함수를 명시하여 스코프 내로 함수를 가져올 수도 있다.
use a::series::of::nested_modules;

enum TrafficLight {
  Red,
  Yellow,
  Green,
}

// enum 또한 모듈과 비슷한 네임스페이스를 형성하기에 열거형의 variant도 use를 사용하여 스코프내로 가져올 수 있다.
// use TrafficLight::{Red, Yellow};
// *(glob)를 사용하여 네임스페이스 내의 모든 아이템을 가져올 수 있다.
// 단, 글롭은 예상한것보다 많은 아이템을 끌고오기 때문에 이름 충돌의 원인이 될 수 있다.
use TrafficLight::*;

fn main() {
  // 우리가 만들었던 모든 모듈은 communicator라는 루트 모듈아래 존재한다.
  communicator::client::connect();

  // 모듈 이름으로 모듈 내에 정의된 함수 가져오기, 하지만 너무 길 수 있다.
  // a::series::of::nested_modules();
  // of::nested_modules();
  nested_modules();

  let red = Red;
  let yellow = Yellow;
  // let green = TrafficLight::Green;
  let green = Green;
}
