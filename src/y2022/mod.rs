#[cfg(feature = "y2022d01")]
pub mod d01;

#[cfg(feature = "y2022d02")]
pub mod d02;

#[cfg(feature = "y2022d03")]
pub mod d03;

#[cfg(feature = "y2022d04")]
pub mod d04;

#[cfg(feature = "y2022d05")]
pub mod d05;

#[cfg(feature = "y2022d06")]
pub mod d06;

#[cfg(feature = "y2022d07")]
pub mod d07;

pub fn main() -> eyre::Result<()> {
    #[cfg(feature = "y2022d01")]
    d01::main()?;

    #[cfg(feature = "y2022d02")]
    d02::main()?;

    #[cfg(feature = "y2022d03")]
    d03::main()?;

    #[cfg(feature = "y2022d04")]
    d04::main()?;

    #[cfg(feature = "y2022d05")]
    d05::main()?;

    #[cfg(feature = "y2022d06")]
    d06::main()?;

    #[cfg(feature = "y2022d07")]
    d07::main()?;

    Ok(())
}
