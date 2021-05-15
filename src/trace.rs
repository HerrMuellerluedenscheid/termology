// file handling
use libmseed_sys::ms_record;

pub fn read_mseed(file: &str) {
    let m = ms_record::read(file);
    println!("...... record {}", m);
}
