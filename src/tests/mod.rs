#[cfg(test)]
mod tests {
    // File: src/tests/mod.rs

    // Import the necessary items
    use rocket::{
        http::ContentType,
        http::Status
        ,
    };
    use rocket::futures::task::Spawn;

    // Import your Rocket application
    use crate::server::api_server::app;

    #[rocket::async_test]
    async fn test_api_enter() {
        use rocket::local::asynchronous::Client;

        // Create an instance of your Rocket application
        let rocket = app();

        // Create a client to use for the tests
        let client = Client::tracked(rocket).await.unwrap();

        // Create a mock request to the /api/enter route
        let response = client
            .get("/api/enter")
            .header(ContentType::JSON)
            .dispatch().await;
        // Check the response status
        assert_eq!(response.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn test_api_exit() {
        use rocket::local::asynchronous::Client;

        // Create an instance of your Rocket application
        let rocket = app();

        // Create a client to use for the tests
        let client = Client::tracked(rocket).await.unwrap();

        // Create a mock request to the /api/enter route
        let response = client
            .get("/api/exit")
            .header(ContentType::JSON)
            .dispatch().await;
        // Check the response status
        assert_eq!(response.status(), Status::Ok);
    }
}