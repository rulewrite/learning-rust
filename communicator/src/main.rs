// communicator의 라이브러리 크레이트를 가져오기 위한 명령
// src/main.rs(현 파일)는 바이너리 크레이트의 루트 파일
// src/lib.rs는 라이브러리 크레이트의 루트 파일
// 만약 우리 프로젝트에 서브 모듈에서 외부 크레이트를 사용하더라도 `extern crate` 명령은 루트 파일에 존재해야 한다.
extern crate communicator;

fn main() {
  // 우리가 만들었던 모든 모듈은 communicator라는 루트 모듈아래 존재한다.
  communicator::client::connect();
}
