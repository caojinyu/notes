const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}
