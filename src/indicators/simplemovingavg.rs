use crate::{error::error::Error, models::tools::Analysis};

impl Analysis {
    pub fn sma(&self, period: usize) -> Result<f64, Error> {
        let mut avg = 0.0;
        for i in 0..period {
            avg += self.get_value_back(i)?.ohlc_avg()?;
        }
        avg /= period as f64;
        Ok(avg)
    }
}
