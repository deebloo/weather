use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::fmt;

#[async_trait]
pub trait Writer {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeatherReading {
    pub time: DateTime<Utc>,
    pub rain: Option<f32>,
    pub rain_delta: Option<f32>,
    pub wind_speed: Option<f32>,
    pub wind_dir: Option<f32>,
    pub out_temp: Option<f32>,
    pub out_humid: Option<u8>,
    pub wind_chill: Option<f32>,
    pub heat_index: Option<f32>,
    pub dew_point: Option<f32>,
}

impl fmt::Display for WeatherReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#### {:?} #### \nrain: {:?} rain_delta: {:?} \nwind_speed: {:?} wind_dir: {:?} \ntemp: {:?} humidity: {:?} \nwind_chill: {:?} heat_index: {:?} dew_point: {:?} \n###### END ######\n",
            self.time, self.rain, self.rain_delta, self.wind_speed, self.wind_dir, self.out_temp, self.out_humid, self.wind_chill, self.heat_index, self.dew_point
        )
    }
}

impl WeatherReading {
    pub fn new() -> WeatherReading {
        WeatherReading {
            time: Utc::now(),
            rain: None,
            rain_delta: None,
            wind_speed: None,
            wind_dir: None,
            out_temp: None,
            out_humid: None,
            wind_chill: None,
            heat_index: None,
            dew_point: None,
        }
    }
}
