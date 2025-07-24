pub mod discovery;
pub mod error;

#[cfg(test)]
mod tests {
    use crate::discovery::client::discover;
    use std::time::Duration;

    #[tokio::test]
    async fn it_works() {
        println!("{:?}", discover(Duration::from_secs(10)).await.unwrap());
    }
}
