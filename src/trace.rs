use chrono::{DateTime, Utc};
use miniseed::ms_record;

#[derive(Debug)]
pub struct Trace {
    rec: ms_record,
    pub xdata: Vec<f64>,
    pub ydata: Vec<f64>,
}

impl Trace {
    pub fn xydata(&self) -> Vec<(f64, f64)> {
        let mut out = Vec::with_capacity(self.ydata.len());
        for (i, value) in self.ydata.iter().enumerate() {
            out.push((i as f64, *value));
        }
        out
    }

    pub fn ymin(&self) -> f64 {
        self.rec.min()
    }

    pub fn ymax(&self) -> f64 {
        self.rec.max()
    }

    pub fn tmin(&self) -> DateTime<Utc> {
        self.rec.start()
    }

    pub fn tmax(&self) -> DateTime<Utc> {
        self.rec.end()
    }

    pub fn read_mseed(file: &str) -> Trace {
        let rec = ms_record::read(file);
        let ydata_i32 = rec.data_i32().to_owned();

        let ydata: Vec<f64> = ydata_i32.into_iter().map(|x| x as f64).collect();
        let xdata: Vec<f64> = (0..ydata.len()).map(|x| x as f64).collect();

        let trace = Trace { rec, xdata, ydata };

        trace
    }
}

#[cfg(test)]
mod tests {
    use crate::trace::Trace;

    #[test]
    fn load_mseed() {
        let file = "tests/test.mseed";
        let trace = Trace::read_mseed(&file);
        println!("{}", trace.tmin())
    }
}
