use zero_to_production::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
