use axum_test::multipart::{MultipartForm, Part};

// TODO: Remove unwrap + add local in case of failure
#[allow(dead_code)]
pub async fn get_multipart_random_image(image_name: &str, part_name: &str) -> MultipartForm {
    // This is a free online service
    let url = "https://picsum.photos/200";

    let image = reqwest::get(url).await.unwrap();
    let image_bytes = image.bytes().await.unwrap();

    let image_part = Part::bytes(image_bytes)
        .file_name(format!("{image_name}.jpeg"))
        .mime_type(&"image/jpeg");

    MultipartForm::new().add_part(part_name, image_part)
}
