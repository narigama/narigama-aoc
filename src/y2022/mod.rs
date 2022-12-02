#[cfg(feature = "y2022d01")]
pub mod d01;

#[cfg(feature = "y2022d02")]
pub mod d02;

pub fn main() -> eyre::Result<()> {
    #[cfg(feature = "y2022d01")]
    d01::main()?;

    #[cfg(feature = "y2022d02")]
    d02::main()?;

    Ok(())
}
