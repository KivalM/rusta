use crate::error::error::Error;

#[derive(Debug, Clone, Copy)]
pub struct CandleStick {
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub volume: Option<f64>,
}

impl CandleStick {
    pub fn ohlc_avg(&self) -> Result<f64, Error> {
        let mut avg = 0.0;
        match self.close {
            Some(v) => avg += v,
            None => return Err(Error::MissingValue),
        };
        match self.open {
            Some(v) => avg += v,
            None => return Err(Error::MissingValue),
        };
        match self.high {
            Some(v) => avg += v,
            None => return Err(Error::MissingValue),
        };
        match self.low {
            Some(v) => avg += v,
            None => return Err(Error::MissingValue),
        };
        avg /= 4.0;
        Ok(avg)
    }

    pub fn hlc_avg(&self) -> Result<f64, Error> {
        let mut avg = 0.0;
        match self.close {
            Some(v) => avg += v,
            None => return Err(Error::MissingValue),
        };
        match self.high {
            Some(v) => avg += v,
            None => return Err(Error::MissingValue),
        };
        match self.low {
            Some(v) => avg += v,
            None => return Err(Error::MissingValue),
        };
        avg /= 3.0;
        Ok(avg)
    }
}
