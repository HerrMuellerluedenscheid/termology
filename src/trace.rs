use miniseed::ms_record;

pub fn read_mseed(file: &str) -> Vec<(f64, f64)> {
    let rec = ms_record::read(file);
    let ydata_i32 = rec.data_i32().to_owned();

    let ydata: Vec<f64> = ydata_i32.into_iter().map(|x| x as f64).collect();
    let xdata: Vec<f64> = (0..ydata.len()).map(|x| x as f64).collect();

    let data = xdata.into_iter().zip(ydata).collect::<Vec<(f64, f64)>>();

    data
}

#[cfg(test)]
mod tests {
    use crate::trace::read_mseed;

    #[test]
    fn load_mseed() {
        let file = "tests/test.mseed";
        let data = read_mseed(&file);
        println!("{:?}", data);
    }
}
