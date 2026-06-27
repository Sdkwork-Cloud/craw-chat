use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, ImageBuffer, ImageReader, Luma, Rgba};
use serde::Deserialize;
use std::io::Cursor;

const MAX_QR_IMAGE_BYTES: usize = 8 * 1024 * 1024;
const MAX_QR_IMAGE_DIMENSION: u32 = 4096;
const MAX_QR_CAMERA_FRAME_PIXELS: usize = 2_073_600;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeQrCodeImageRequest {
    image_base64: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeQrCodeRgbaRequest {
    data_base64: String,
    height: u32,
    width: u32,
}

pub fn decode_qr_code_from_dynamic_image(image: DynamicImage) -> Result<Option<String>, String> {
    let grayscale_image = image.to_luma8();
    let mut prepared_image = rqrr::PreparedImage::prepare(grayscale_image);
    for grid in prepared_image.detect_grids() {
        match grid.decode() {
            Ok((_meta, content)) => {
                let normalized_content = content.trim().to_string();
                if !normalized_content.is_empty() {
                    return Ok(Some(normalized_content));
                }
            }
            Err(_) => continue,
        }
    }

    Ok(None)
}

fn decode_base64_bytes(value: &str) -> Result<Vec<u8>, String> {
    let payload = value
        .split_once(',')
        .map(|(_, payload)| payload)
        .unwrap_or(value);
    general_purpose::STANDARD
        .decode(payload)
        .map_err(|error| format!("invalid base64 QR image data: {error}"))
}

fn decode_qr_code_from_rgba_bytes(
    data_base64: &str,
    width: u32,
    height: u32,
) -> Result<Option<String>, String> {
    if width == 0 || height == 0 {
        return Err("QR camera frame size is required".to_string());
    }

    let pixel_count = (width as usize)
        .checked_mul(height as usize)
        .ok_or_else(|| "QR camera frame size is too large".to_string())?;
    if pixel_count > MAX_QR_CAMERA_FRAME_PIXELS {
        return Err("QR camera frame size exceeds the native decoder limit".to_string());
    }

    let rgba_bytes = decode_base64_bytes(data_base64)?;
    let expected_len = pixel_count
        .checked_mul(4)
        .ok_or_else(|| "QR camera frame byte length is too large".to_string())?;
    if rgba_bytes.len() != expected_len {
        return Err(format!(
            "QR camera frame byte length mismatch: expected {expected_len}, got {}",
            rgba_bytes.len()
        ));
    }

    let rgba_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, rgba_bytes)
        .ok_or_else(|| "invalid QR camera frame buffer".to_string())?;
    let luma_image: ImageBuffer<Luma<u8>, Vec<u8>> =
        DynamicImage::ImageRgba8(rgba_image).to_luma8();
    decode_qr_code_from_dynamic_image(DynamicImage::ImageLuma8(luma_image))
}

fn decode_qr_code_from_image_bytes(image_bytes: &[u8]) -> Result<Option<String>, String> {
    if image_bytes.len() > MAX_QR_IMAGE_BYTES {
        return Err("QR image payload exceeds the native decoder limit".to_string());
    }

    let mut reader = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .map_err(|error| format!("invalid QR image data: {error}"))?;
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_QR_IMAGE_DIMENSION);
    limits.max_image_height = Some(MAX_QR_IMAGE_DIMENSION);
    reader.limits(limits);
    let image = reader
        .decode()
        .map_err(|error| format!("invalid QR image data: {error}"))?;
    decode_qr_code_from_dynamic_image(image)
}

#[tauri::command]
pub fn sdkwork_chat_pc_decode_qr_code_image(
    request: DecodeQrCodeImageRequest,
) -> Result<Option<String>, String> {
    let image_bytes = decode_base64_bytes(&request.image_base64)?;
    decode_qr_code_from_image_bytes(&image_bytes)
}

#[tauri::command]
pub fn sdkwork_chat_pc_decode_qr_code_rgba(
    request: DecodeQrCodeRgbaRequest,
) -> Result<Option<String>, String> {
    decode_qr_code_from_rgba_bytes(&request.data_base64, request.width, request.height)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_QR_PNG_BASE64: &str = "iVBORw0KGgoAAAANSUhEUgAAAKAAAACgCAYAAACLz2ctAAAAAklEQVR4AewaftIAAAQLSURBVO3BMW4kSRAEQffE/P/LcZRWaLRQRHMmebthZr5QtWSoWjRULRqqFg1Vi4aqRUPVoqFq0VC1aKhaNFQtGqoWvfgmld8oCe+g8lQS3kHlN0rCqaFq0VC1aKhaNFQtevEDkvBJKk+onErCHZWrJNxROZWEJ5LwSSpPDFWLhqpFQ9WioWrRULXoxRupPJGEp1R+oyR8ksoTSXiHoWrRULVoqFo0VC168Q9KwjuonFI5lYS/1VC1aKhaNFQtGqoWDVWLXtQfKldJuJOEK5U7STil8i8ZqhYNVYuGqkVD1aIXb5SEv5XKKZVtSfiNhqpFQ9WioWrRULVoqFr04geoFCThjspVEu6oPKHyfzJULRqqFg1Vi4aqReYLfzGVU0l4B5VTSfiXDFWLhqpFQ9WioWrRULXoxTepXCXhjsqnJOFOEq5UnlJ5B5WrJNxR+ZQkvMNQtWioWjRULRqqFr34ASpPJeEJlVNJOKXyVBJOqZxKwpXKnSRcqWwbqhYNVYuGqkVD1aKhatGLH5CEp1SuknBH5SoJd1ROqVwl4Y7KEypPqVwl4Y7KVRLuqFypnErCqaFq0VC1aKhaNFQtevFGKldJeCoJVyqnVO4k4Ykk3FF5QuWUylNJOKXyxFC1aKhaNFQtGqoWDVWLzBceUrmThHdQuUrCHZUnkrBN5VQS7qhcJeGOyhNJODVULRqqFg1Vi4aqRS++SWVbEk4l4UrlThJOqZxKwjsk4UrlThJOJeFThqpFQ9WioWrRULVoqFpkvvANKldJOKXySUk4pXKVhFMqv1USTqk8kYRTQ9WioWrRULVoqFr04n8oCVcqn6RyKglXKqeScEfllMpVEu4k4UrlThKeGKoWDVWLhqpFQ9WioWrRizdSeQeVqyScUrmThCeScEflCZV3UDmVhDsqV0k4NVQtGqoWDVWLhqpF5gt/MZUnkvAOKqeS8JTKVRLuqJxKwhND1aKhatFQtWioWjRULXrxTSq/URLuJOEJlVNJOJWEOyqnVK6S8EkqV0k4NVQtGqoWDVWLhqpFL35AEj5J5ZTKqSScSsIplaskPJWEd0jCKZUnhqpFQ9WioWrRULVoqFr04o1UnkjCOyThk5JwpXJK5ZNUPmWoWjRULRqqFg1Vi17UHypXSbij8hsl4Y7KlcqdJFypvMNQtWioWjRULRqqFg1Vi17UH0k4lYQrlVNJ+KQkXKncUblKwh2VJ4aqRUPVoqFq0VC16MUbJWFbEt5B5VQSTqlcJeGUyqkkbBuqFg1Vi4aqRUPVoqFq0YsfoPJbqZxKwhNJuKNylYQ7SbhSOZWEOypXKtuGqkVD1aKhatFQtch8oWrJULVoqFo0VC0aqhYNVYuGqkVD1aKhatFQtWioWvQfs4FjbtWqRnwAAAAASUVORK5CYII=";

    fn load_test_qr_png() -> DynamicImage {
        let png_bytes = general_purpose::STANDARD
            .decode(TEST_QR_PNG_BASE64)
            .expect("test QR PNG base64 must decode");
        image::load_from_memory(&png_bytes).expect("test QR PNG must load")
    }

    #[test]
    fn native_qr_decoder_reads_png_qr_payload() {
        let png_bytes = general_purpose::STANDARD
            .decode(TEST_QR_PNG_BASE64)
            .expect("test QR PNG base64 must decode");
        let decoded =
            decode_qr_code_from_image_bytes(&png_bytes).expect("native QR decode should not error");

        assert_eq!(decoded.as_deref(), Some("sdkwork-native-qr-ok"));
    }

    #[test]
    fn native_qr_decoder_reads_rgba_camera_frame_payload() {
        let image = load_test_qr_png().to_rgba8();
        let width = image.width();
        let height = image.height();
        let frame_base64 = general_purpose::STANDARD.encode(image.into_raw());
        let decoded = decode_qr_code_from_rgba_bytes(&frame_base64, width, height)
            .expect("native QR frame decode should not error");

        assert_eq!(decoded.as_deref(), Some("sdkwork-native-qr-ok"));
    }

    #[test]
    fn native_qr_decoder_rejects_oversized_image_payload() {
        let image_bytes = vec![0; MAX_QR_IMAGE_BYTES + 1];
        let result = decode_qr_code_from_image_bytes(&image_bytes);

        assert!(
            matches!(result, Err(ref message) if message.contains("payload exceeds")),
            "expected oversized image payload error, got {result:?}"
        );
    }

    #[test]
    fn native_qr_decoder_rejects_oversized_image_dimensions() {
        let image = ImageBuffer::<Luma<u8>, Vec<u8>>::from_pixel(
            MAX_QR_IMAGE_DIMENSION + 1,
            1,
            Luma([255]),
        );
        let mut image_bytes = Vec::new();
        DynamicImage::ImageLuma8(image)
            .write_to(&mut Cursor::new(&mut image_bytes), image::ImageFormat::Png)
            .expect("oversized test PNG must encode");

        let result = decode_qr_code_from_image_bytes(&image_bytes);

        assert!(
            matches!(result, Err(ref message) if message.contains("invalid QR image data")),
            "expected oversized image dimensions error, got {result:?}"
        );
    }

    #[test]
    fn native_qr_decoder_rejects_rgba_frame_length_mismatch() {
        let result = decode_qr_code_from_rgba_bytes("AAAA", 10, 10);

        assert!(
            matches!(result, Err(ref message) if message.contains("byte length mismatch")),
            "expected RGBA length mismatch error, got {result:?}"
        );
    }

    #[test]
    fn native_qr_decoder_rejects_oversized_rgba_frame_dimensions() {
        let result = decode_qr_code_from_rgba_bytes("", 10_000, 10_000);

        assert!(
            matches!(result, Err(ref message) if message.contains("decoder limit")),
            "expected oversized RGBA frame error, got {result:?}"
        );
    }
}
