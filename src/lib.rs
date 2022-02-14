mod image;
mod transform;

pub use crate::image::{Image, Format, DataSource};


#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn it_works() -> Result<()> {
        let mut im = Image::open("data/1024.png")?;
        im.scale(2.0)
            .save("data/2048.png");
        assert_eq!(2 + 2, 4);
        Ok(())
    }
}