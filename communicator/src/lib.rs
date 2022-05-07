// communicator
//  ├── client
//  └── network
//      └── server

pub mod client; // contents of client.rs

// mod 모듈명
mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
