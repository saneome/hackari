use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{
    Client,
    config::Credentials,
    error::SdkError,
    operation::put_object::PutObjectError,
    primitives::ByteStream,
};
use tokio::sync::OnceCell;

use crate::utils::error::AppError;

static S3_CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn init_s3_client() -> &'static Client {
    S3_CLIENT.get_or_init(|| async {
        let endpoint = std::env::var("AWS_ENDPOINT")
            .expect("AWS_ENDPOINT must be set");
        let access_key = std::env::var("AWS_ACCESS_KEY_ID")
            .expect("AWS_ACCESS_KEY_ID must be set");
        let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY")
            .expect("AWS_SECRET_ACCESS_KEY must be set");
        let region = std::env::var("AWS_REGION")
            .expect("AWS_REGION must be set");

        let credentials = Credentials::new(
            access_key,
            secret_key,
            None,
            None,
            "manual",
        );

        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(region))
            .endpoint_url(endpoint)
            .credentials_provider(credentials)
            .load()
            .await;

        Client::new(&config)
    }).await
}

pub async fn upload_file(
    key: &str,
    data: Vec<u8>,
    content_type: &str,
) -> Result<String, AppError> {
    let client = init_s3_client().await;
    let bucket = std::env::var("AWS_BUCKET_NAME")
        .map_err(|_| AppError::Internal("AWS_BUCKET_NAME not set".to_string()))?;

    client
        .put_object()
        .bucket(&bucket)
        .key(key)
        .body(ByteStream::from(data))
        .content_type(content_type)
        .send()
        .await
        .map_err(|e: SdkError<PutObjectError>| {
            AppError::Internal(format!("Failed to upload file: {}", e))
        })?;

    let endpoint = std::env::var("AWS_ENDPOINT")
        .map_err(|_| AppError::Internal("AWS_ENDPOINT not set".to_string()))?;

    Ok(format!("{}/{}/{}", endpoint, bucket, key))
}

pub async fn generate_presigned_url(key: &str, expires_in: u64) -> Result<String, AppError> {
    let client = init_s3_client().await;
    let bucket = std::env::var("AWS_BUCKET_NAME")
        .map_err(|_| AppError::Internal("AWS_BUCKET_NAME not set".to_string()))?;

    let presigned = client
        .get_object()
        .bucket(&bucket)
        .key(key)
        .presigned(aws_sdk_s3::presigning::PresigningConfig::builder()
            .expires_in(std::time::Duration::from_secs(expires_in))
            .build()
            .map_err(|e| AppError::Internal(format!("Failed to build presigned config: {}", e)))?)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to generate presigned URL: {}", e)))?;

    Ok(presigned.uri().to_string())
}

pub async fn delete_file(key: &str) -> Result<(), AppError> {
    let client = init_s3_client().await;
    let bucket = std::env::var("AWS_BUCKET_NAME")
        .map_err(|_| AppError::Internal("AWS_BUCKET_NAME not set".to_string()))?;

    client
        .delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to delete file: {}", e)))?;

    Ok(())
}
