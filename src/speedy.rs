use crate::db::Db;
use crate::err::{FastCliErr, SMErr};
use crate::settings;
use crate::settings::Settings;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;
use tokio::process::Command;

#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SpeedTestEvent {
    #[serde(rename(deserialize = "buffer_bloat", serialize = "buffer_bloat"))]
    bufferBloat: f64,
    downloaded: u32,
    #[serde(rename(deserialize = "download_speed", serialize = "download_speed"))]
    downloadSpeed: u32,
    latency: u32,
    uploaded: u32,
    #[serde(rename(deserialize = "upload_speed", serialize = "upload_speed"))]
    uploadSpeed: u32,
    #[serde(rename(deserialize = "user_ip", serialize = "user_ip"))]
    userIp: String,
    #[serde(rename(deserialize = "user_location", serialize = "user_location"))]
    userLocation: String,
}

impl SpeedTestEvent {
    pub async fn from_cli() -> Result<Self, SMErr> {
        let proc = Command::new("fast")
            .args(&["-u", "--json"])
            .spawn()
            .map_err(|e| FastCliErr::General(e.to_string()))?;
        let status = proc
            .wait_with_output()
            .await
            .map_err(|e| FastCliErr::General(e.to_string()))?;
        match status.status.code() {
            Some(0) => {
                let stdout = std::str::from_utf8(&status.stdout).expect("failed to extract stdout");
                let res = serde_json::from_str(&stdout)
                    .map_err(|e| FastCliErr::ParseOutput(e.to_string()).into());
                res
            }
            _ => {
                let err = std::str::from_utf8(&status.stderr).unwrap_or("unknown failure");
                let sme: SMErr =
                    FastCliErr::General(format!("failed to get results from fast cli. {}", err))
                        .into();
                sme.into()
            }
        }
    }
}

pub struct SpeedTestOptions {
    is_dev: bool,
}

pub async fn run(opts: SpeedTestOptions, db: Arc<Db>) {
    Box::pin(async {
        println!("starting fast-cli ()...");
        match if opts.is_dev {
            Ok(SpeedTestEvent::default())
        } else {
            SpeedTestEvent::from_cli().await
        } {
            Ok(speed_result) => {
                let speed_str = serde_json::ser::to_string(&speed_result).expect("deserialization");
                println!("{}", speed_str);
                let pool = db.pool.acquire().await.expect("acquire db");
                sqlx::query!(
                    "insert into speed_samples (
  buffer_bloat,
  downloaded,
  download_speed,
  latency,
  uploaded,
  upload_speed,
  user_ip,
  user_location
) values (
  ?,
  ?,
  ?,
  ?,
  ?,
  ?,
  ?,
  ?
)",
                    speed_result.buffer_bloat,
                    speed_result.downloaded,
                    speed_result.download_speed,
                    speed_result.latency,
                    speed_result.uploaded,
                    speed_result.upload_speed,
                    speed_result.user_ip,
                    speed_result.user_location,
                    pool
                );
            }
            Err(e) => println!("{}", e),
        }
    })
    .await
}

impl From<Arc<Settings>> for SpeedTestOptions {
    fn from(s: Arc<Settings>) -> Self {
        Self {
            is_dev: s.mode == settings::Mode::Development,
        }
    }
}
