#[rocket::main]
async fn main() {
    omnirss::server_start().await
}
