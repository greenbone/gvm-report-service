#[tokio::main]
async fn main() {
    gvm_report_server::run().await.expect("application failed");
}
