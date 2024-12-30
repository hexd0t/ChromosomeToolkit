use crate::error::*;
use crate::helpers::*;

use serde::{Deserialize, Serialize};
use std::io::Read;
use std::io::Write;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

const FILE_TIMES_PER_SEC: u64 = 10_000_000;
const UNIX_EPOCH_OFFSET: u64 = 134_774 * 86400;

#[derive(Debug, Deserialize, Serialize)]
pub struct DateTime(pub u64);

impl DateTime {
    pub fn load<R: Read>(src: &mut R) -> Result<Self> {
        let val = read_u64(src)?;
        Ok(Self(val))
    }
    pub fn save<W: Write>(&self, dst: &mut W) -> Result<()> {
        write_u64(dst, self.0)?;
        Ok(())
    }

    pub fn now() -> Self {
        Self::new(SystemTime::now())
    }

    pub fn new(time: SystemTime) -> Self {
        let unix_stamp = time.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let windows_time_in_s = unix_stamp + UNIX_EPOCH_OFFSET;
        Self(windows_time_in_s * FILE_TIMES_PER_SEC)
    }

    pub fn decode(&self) -> SystemTime {
        let windows_time_in_s = self.0 / FILE_TIMES_PER_SEC;
        let unix_stamp = windows_time_in_s - UNIX_EPOCH_OFFSET;
        UNIX_EPOCH + Duration::from_secs(unix_stamp)
    }
}
