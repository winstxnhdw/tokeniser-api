mod v1;

use axum_test::TestServer;

pub fn get_client() -> TestServer {
    TestServer::new(tokeniser_api::app()).unwrap()
}
