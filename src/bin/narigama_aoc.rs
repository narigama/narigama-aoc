fn main() -> eyre::Result<()> {
    // setup env and logging
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // run years
    narigama_aoc::y2022::main()?;

    Ok(())
}
