use axum::body::Body;
use axum::extract::Query;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use sha3::{Digest, Sha3_256};
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::info;
use uuid::Uuid;
use crate::app::files::validator::{is_upload_complete, path_is_valid, sanitize_filename, validate_book_mime, MAX_UPLOAD_SIZE};
use crate::app::hashing::hash::hash_file;

#[derive(Deserialize)]
struct DownloadParams {
    file_name: String,
    offset: u64,
    total_chunks: usize,
}

// Function that downloads a file using a chunk mechanism
// pub fn download_file(Query(params): Query<DownloadParams>) -> impl IntoResponse {
//     let file_path = path_storage(&params.file_name);
//     let mut file = match File::open(file_path) {
//         Ok(file) => file,
//         Err(_) => return not_found_error("File not found"),
//     };
//
//     let total_size = match file.metadata() {
//         Ok(metadata) => metadata.len(),
//         Err(_) => return not_found_error("File metadata not found"),
//     };
//
//     if params.offset >= total_size {
//         return not_found_error("Offset out of bounds");
//     }
//
//     if file.seek(SeekFrom::Start(params.offset)).is_err() {
//         return not_found_error("File seek out of bounds");
//     }
//
//     let mut buffer = vec![0; params.total_chunks];
//     let bytes_read = match file.read(&mut buffer) {
//         Ok(bytes) => bytes,
//         Err(_) => return internal_error("Error reading file"),
//     };
//
//     if bytes_read == 0 {
//         return not_found_error("Empty file");
//     }
//
//     let mut headers = HeaderMap::new();
//     headers.insert("Content-Type", HeaderValue::from_static("application/octet-stream"));
//     headers.insert("Content-Disposition", HeaderValue::from_str(&format!(
//         "attachment; filename=\"{}\"", params.file_name
//     )).unwrap());
//
//     headers.insert("X-Chunk-Offset", HeaderValue::from_str(&params.offset.to_string()).unwrap());
//     headers.insert("X-Chunk-Size", HeaderValue::from_str(&params.total_chunks.to_string()).unwrap());
//     headers.insert("X-Total-Size", HeaderValue::from_str(&total_size.to_string()).unwrap());
//
//     Response::builder()
//         .status(StatusCode::PARTIAL_CONTENT)
//         .header("Content-Type", "application/octet-stream")
//         .header(
//             "Content-Disposition",
//             format!("attachment; filename=\"{}\"", params.file_name),
//         )
//         .header("X-Chunk-Offset", params.offset.to_string())
//         .header("X-Chunk-Size", bytes_read.to_string())
//         .body(Body::from(buffer[..bytes_read].to_vec()))
//         .unwrap_or_else(|_| {
//             Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .body(Body::from("Failed to build response"))
//                 .unwrap()
//         })
// }

/// Function that pointing to storage folder
pub fn path_storage(sub_path: &str) -> PathBuf {
    let base_path = std::env::current_dir().expect("Failed to get current directory");
    base_path.join("storage").join(sub_path)
}

/// Write a file as chunks
pub async fn write_file(
    upload_id: Uuid,
    file_name: &str,
    total_chunks: usize,
    output_dir: Option<&str>,
    chunk_number: usize,
    data_chunks: &[u8]
)
    -> Result<String, (StatusCode, String)>
{
    if (total_chunks as u64 * 2 * 1024 * 1024) > MAX_UPLOAD_SIZE {
        return Err((StatusCode::PAYLOAD_TOO_LARGE, "File exceeds limit".to_string()))
    }
    // if !path_is_valid(path) {
    //     info!("{:?}", path);
    //     return Err((StatusCode::NO_CONTENT, "Invalid path".to_owned()));
    // }

    let temp_dir_str = format!("temp/{}", upload_id);
    let temp_dir_path = path_storage(&temp_dir_str);

    if let Some(_parent) = temp_dir_path.parent() {
        fs::create_dir_all(&temp_dir_path).await.map_err(|e|
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Gagal buat temp dir: {}", e))
        )?;
    }

    if chunk_number == 0 {
        if let Err(e) = validate_book_mime(data_chunks) {
            return Err((StatusCode::UNSUPPORTED_MEDIA_TYPE, e));
        }
    }

    let chunk_path = temp_dir_path.join(chunk_number.to_string());
    fs::write(&chunk_path, data_chunks).await.map_err(|e|
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Gagal buat chunk: {}", e))
    )?;

    let is_complete = is_upload_complete(&temp_dir_path.to_str().unwrap(), total_chunks).await;
    if !is_complete {
        return Ok(format!("Chunk {} stored. Waiting for more...", chunk_number));
    }

    info!("Upload lengkap! Mulai menggabungkan file...");

    let timestamp = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let sanitized_name = sanitize_filename(&file_name);
    let final_file_name = format!("{}_{}", &timestamp, sanitized_name);


    let relative_path = match output_dir {
        Some(subdir) => format!("uploads/{}/{}", subdir, final_file_name),
        None => format!("uploads/{}", final_file_name)
    };

    let output_path = path_storage(&relative_path);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create output dir: {}", e))
        })?;
    }


    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&output_path).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;

    let mut hasher = Sha3_256::new();
    for i in 0..total_chunks {
        // let ck_path = format!("{}/chunk/{}", path, chunk_number);
        let current_chunk_path = temp_dir_path.join(i.to_string());

        // Buka Chunk
        let mut chunk_file = fs::File::open(&current_chunk_path).await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, format!("Chunk {} hilang!", i)))?;

        // Baca Chunk
        let mut buffer = Vec::new();
        chunk_file.read_to_end(&mut buffer).await.unwrap();

        // Tulis ke Final File
        output_file.write_all(&buffer).await.unwrap();

        // Update Hash
        hasher.update(&buffer);

        fs::remove_dir(current_chunk_path.as_path()).await.unwrap();
    }
    let hash_result = hex::encode(hasher.finalize());
    let hash_path = output_path.with_extension("hash");
    fs::write(&hash_path, hash_result.as_bytes()).await.unwrap();
    fs::remove_dir_all(&temp_dir_path).await.map_err(|e|
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Gagal cleanup temp: {}", e))
    )?;

    Ok(format!("File berhasil dibuat: {} (Hash: {})", relative_path, hash_result))

}

/// Generate unique id for directory
pub fn generate_chunk_dir_id() -> String {
    Uuid::new_v4().to_string()
}
