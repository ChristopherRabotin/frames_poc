extern crate bytes;
extern crate nalgebra;
extern crate prost;
#[macro_use]
extern crate prost_derive;
use bytes::IntoBuf;
use prost::Message;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

/// Returns the provided angle bounded between 0.0 and 360.0
pub fn between_0_360(angle: f64) -> f64 {
    let mut bounded = angle;
    while bounded > 360.0 {
        bounded -= 360.0;
    }
    while bounded < 0.0 {
        bounded += 360.0;
    }
    bounded
}

/// Returns the provided angle bounded between -180.0 and +180.0
pub fn between_pm_180(angle: f64) -> f64 {
    let mut bounded = angle;
    while bounded > 180.0 {
        bounded -= 360.0;
    }
    while bounded < -180.0 {
        bounded += 360.0;
    }
    bounded
}

mod axb;
mod exb;
mod fxb;
mod hermite;

/// Loads the provided input_filename as an EXB
///
/// This function may panic!
pub fn load_ephemeris(input_filename: &str) -> Vec<exb::Ephemeris> {
    let mut input_exb_buf = Vec::new();

    File::open(input_filename)
        .expect(&format!("could not open EXB file {}", input_filename))
        .read_to_end(&mut input_exb_buf)
        .expect("something went wrong reading the file");

    if input_exb_buf.len() == 0 {
        panic!("EXB file {} is empty (zero bytes read)", input_filename);
    }

    println!("Decoding EXB (this may take a while for large files).");

    let decode_start = Instant::now();

    let ephcnt =
        exb::EphemerisContainer::decode(input_exb_buf.into_buf()).expect("could not decode EXB");

    let ephemerides = ephcnt.ephemerides;
    let num_eph = ephemerides.len();
    if num_eph == 0 {
        panic!("no ephemerides found in EXB");
    }
    println!(
        "Loaded {} ephemerides in {} seconds.",
        num_eph,
        decode_start.elapsed().as_secs()
    );
    ephemerides
}

/// Loads the provided input_filename as an FXB
///
/// This function may panic!
pub fn load_frames(input_filename: &str) -> Vec<fxb::Frame> {
    let mut input_fxb_buf = Vec::new();

    File::open(input_filename)
        .expect(&format!("could not open FXB file {}", input_filename))
        .read_to_end(&mut input_fxb_buf)
        .expect("something went wrong reading the file");

    if input_fxb_buf.len() == 0 {
        panic!("FXB file {} is empty (zero bytes read)", input_filename);
    }

    println!("Decoding FXB (this may take a while for large files).");

    let decode_start = Instant::now();

    let cnt = fxb::FrameContainer::decode(input_fxb_buf.into_buf()).expect("could not decode FXB");

    let frames = cnt.frames;
    let num_eph = frames.len();
    if num_eph == 0 {
        panic!("no frames found in FXB");
    }
    println!(
        "Loaded {} frames in {} seconds.",
        num_eph,
        decode_start.elapsed().as_secs()
    );
    frames
}

pub mod cosm;
pub mod frames;
pub mod state;
