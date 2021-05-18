use miniseed::ms_record;

pub fn read_mseed(file: &str) -> Vec<i32> {
    let rec = ms_record::read(file);
    let data = rec.data_i32().to_owned();
    data
}

#[cfg(test)]
mod tests {
    use crate::trace::read_mseed;

    #[test]
    fn load_mseed() {
        let file = "../libmseed-sys/tests/sample.miniseed";
        read_mseed(&file);
    }
}
