fn slow_hello() -> String {
    "hello from slow function".to_string()
}

pub async fn root() -> &'static str {
    "task root"
}