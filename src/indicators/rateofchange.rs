use crate::{error::error::Error, models::tools::Analysis};

impl Analysis {
    pub fn rateofchange(&self, period: usize) -> Result<f64, Error> {
        let curr = self.get_value_back(0)?;
        let prev = self.get_value_back(period)?;

        Ok(((curr.ohlc_avg().unwrap() / prev.ohlc_avg().unwrap()) - 1.0) * 100.0)
    }
}
