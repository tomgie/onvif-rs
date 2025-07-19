pub mod soap;
mod error;
mod discovery;

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::discovery::client::discover;

    #[tokio::test]
    async fn it_works() {
        println!("{:?}", discover(Duration::from_secs(10)).await.unwrap());
    }
}
