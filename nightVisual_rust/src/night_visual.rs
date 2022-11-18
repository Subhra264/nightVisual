pub struct Image {
    pub title: String,
    pub size: usize,
    data: String
}

impl Image {
    pub fn create_image(size: usize) -> Image {
        Image {
            title: String::from("image.jpg"),
            size,
            data: String::from("Some image data")
        }
    }
}