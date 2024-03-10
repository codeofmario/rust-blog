use std::panic;

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

use crate::config::settings::Settings;

pub fn init_store(settings: Settings) -> Bucket {
  let credentials = Credentials::new(
    Some(settings.minio_access_key.as_str()),
    Some(settings.minio_secret_key.as_str()),
    None,
    None,
    None,
  ).unwrap();

  let region = Region::Custom {
    region: "us-east-1".to_owned(),
    endpoint: settings.minio_public_addr.to_string()
  };

  match Bucket::new(
    settings.minio_bucket_name.as_str(),
    region,
    credentials,
  ) {
    Ok(bucket) => bucket.with_path_style(),
    Err(err) => panic!("Cannot connect to minio: {:?}", err)
  }
}