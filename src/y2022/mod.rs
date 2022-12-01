#[cfg(feature = "y2022d01")]
pub mod d01;

pub fn main() -> eyre::Result<()> {
    #[cfg(feature = "y2022d01")]
    d01::main()?;

    Ok(())
}
