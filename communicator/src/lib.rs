mod client {
    fn connect() {}
}

// mod 모듈명
mod network {
    // 이 내부의 요소들은 네임스페이스 network 안에 존재

    // network::connect()로 호출
    fn connect() {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
