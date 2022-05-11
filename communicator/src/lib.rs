// communicator
//  ├── client
//  └── network
//      └── server

pub mod client; // contents of client.rs

// mod 모듈명
pub mod network;

// 특별히 어노테이션이 붙어있을 뿐 tests 또한 모듈이다. 따라서 모듈 계층 구조는 다음과 같다.
// communicator
//  ├── client
//  ├── network
//  |   └── server
//  └── tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // 테스트 모듈 내이기 때문에 부모로 할번 올라갔다가 client로 접근할 필요가 있다.
        super::client::connect();
    }
}
