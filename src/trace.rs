use chrono::{DateTime, Utc};
use miniseed::{ms_input, ms_record};

pub struct Input {
    input: ms_input,
}

impl Input {
    pub fn traces(self) -> Vec<Trace> {
        let mut output = Vec::new();
        for inp in self.input {
            let itr = Trace::read(inp);
            output.push(itr);
        }
        output
    }

    pub fn read(filename: &str) -> Input {
        let input = ms_input::open(&filename);
        Input { input }
    }
}

#[derive(Debug)]
pub struct Trace {
    pub rec: ms_record,
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

    pub fn read(rec: ms_record) -> Trace {
        let ydata_i32 = rec.data_i32().clone();

        let ydata: Vec<f64> = ydata_i32.into_iter().map(|x| *x as f64).collect();
        let xdata: Vec<f64> = (0..ydata.len()).map(|x| x as f64).collect();

        let trace = Trace { rec, xdata, ydata };

        trace
    }

    pub fn nslc_id(&self) -> String {
        format!(
            "{}.{}.{}",
            self.rec.network(),
            self.rec.station(),
            self.rec.location()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::trace::{Input, Trace};
    use miniseed::ms_input;

    #[test]
    fn test_input() {
        let file = "tests/test.mseed";
        let inp = Input::read(file);
        let trs = inp.traces();
        let x = &trs[0];
    }
}
