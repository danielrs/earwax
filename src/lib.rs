#![feature(unique)]

extern crate libc;
extern crate num;

pub mod error;
pub mod ffi;
pub mod log;
pub mod timestamp;

use error::{Error, Result};
pub use log::LogLevel;
pub use timestamp::Timestamp;

use num::rational::Rational64;

use std::ffi::CString;
use std::ptr;
use std::ptr::Unique;

/**
 * Earwax context. This struct contains the stream data
 * and main methods getting and seeking data.
 */
pub struct Earwax {
    earwax_context: Unique<ffi::EarwaxContext>,
    info: Info,
}

impl Earwax {
    /// Creates a new Earwax instance from the given url.
    /// # Returns
    /// Some(Earwax) if everything went fine.
    /// None if something went wrong with ffmpeg.
    pub fn new(url: &str) -> Result<Self> {
        let url = try!(CString::new(url));
        let mut earwax_context = ptr::null_mut();
        unsafe {
            ffi::earwax_init();
            let res = ffi::earwax_new(&mut earwax_context, url.as_ptr());
            if res == 0 {
                let mut info = ffi::EarwaxInfo::new();
                ffi::earwax_get_info(earwax_context, &mut info);
                let time_base = Rational64::new(info.time_base.num, info.time_base.den);

                Ok(Earwax {
                       earwax_context: Unique::new(earwax_context),
                       info: Info {
                           bitrate: info.bitrate,
                           sample_rate: info.sample_rate,
                           start_time: Timestamp::from_pts(time_base, info.start_time),
                           duration: Timestamp::from_pts(time_base, info.duration),
                           time_base: time_base,
                       },
                   })
            } else {
                Err(Error::FFI(res.into()))
            }
        }
    }

    /// Returns the information for this Earwax stream.
    pub fn info(&self) -> &Info {
        &self.info
    }

    /// Returns the next decoded chunk of PCM data. None if
    /// the end of the stream was reached.
    pub fn spit(&mut self) -> Option<Chunk> {
        unsafe {
            let mut chunk = ffi::EarwaxChunk::new();
            if ffi::earwax_spit(self.earwax_context.as_mut(), &mut chunk) > 0 {
                let slice = std::slice::from_raw_parts(chunk.data, chunk.size);
                Some(Chunk {
                         data: slice,
                         time: Timestamp::from_pts(self.info().time_base, chunk.time),
                     })
            } else {
                None
            }
        }
    }

    /// Seeks to the given seconds.
    pub fn seek(&mut self, seconds: i64) {
        let time_base = self.info.time_base;
        self.seek_pts(seconds * time_base.denom() / time_base.numer())
    }

    /// Seeks to the given pts.
    pub fn seek_pts(&mut self, pts: i64) {
        unsafe {
            ffi::earwax_seek(self.earwax_context.as_mut(), pts);
        }
    }

    pub fn log_level() -> LogLevel {
        unsafe { LogLevel::from_int(ffi::av_log_get_level()) }
    }

    /// Sets the log level for all earwax objects. Internally,
    /// this sets the log level of ffmpeg.
    pub fn set_log_level(level: LogLevel) {
        unsafe { ffi::av_log_set_level(level.to_int()) }
    }
}

impl Drop for Earwax {
    fn drop(&mut self) {
        unsafe {
            let mut ctx = self.earwax_context.as_mut() as *mut ffi::EarwaxContext;
            ffi::earwax_drop(&mut ctx);
            ffi::earwax_shutdown();
        }
    }
}

/// A chunk represents a piece of decoded PCM data in 16-bit signed
/// format.
#[derive(Debug)]
pub struct Chunk<'a> {
    pub data: &'a [i8],
    pub time: Timestamp,
}

/// Information about the Earwax context it is attached to.
#[derive(Debug)]
pub struct Info {
    pub bitrate: i32,
    pub sample_rate: i32,
    pub start_time: Timestamp,
    pub duration: Timestamp,
    pub time_base: Rational64,
}
