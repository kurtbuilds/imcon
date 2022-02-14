mod image;
mod transform;
pub use image::{Image, Format};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let im = Image::open("data/1024.png").unwrap();
        im.scale(2)
            .save("data/2048.png");
        assert_eq!(2 + 2, 4);
    }
}