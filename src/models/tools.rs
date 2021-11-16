use crate::error::error::Error;

use super::data::CandleStick;

pub struct Analysis {
    data: Vec<CandleStick>,
    heikinashi: Vec<CandleStick>,
}

impl Analysis {
    pub fn get_value_back(&self, position: usize) -> Result<CandleStick, Error> {
        let position = self.data.len() as i32 - position as i32;
        if position >= 0 {
            Ok(self.data[position as usize])
        } else {
            Err(Error::IndexOutOfBounds)
        }
    }

    pub fn next(&mut self, c: CandleStick) -> Result<(), Error> {
        self.data.push(c);

        let close = c.ohlc_avg()?;
        let open = match self.heikinashi.len() {
            0 => 0.0,
            _ => {
                let v = self.heikinashi.last().unwrap();
                (v.open.unwrap() + v.close.unwrap()) / 2.0
            }
        };
        let high = match c.high {
            Some(v) => v.max(open).max(close),
            None => return Err(Error::MissingValue),
        };
        let low = match c.low {
            Some(v) => v.min(open).min(close),
            None => return Err(Error::MissingValue),
        };
        self.heikinashi.push(CandleStick {
            open: Some(open),
            high: Some(high),
            low: Some(low),
            close: Some(close),
            volume: c.volume,
        });

        Ok(())
    }
}
