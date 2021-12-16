use acurite_core::{ReadResult, Reader};

pub struct TestReader {
    pub readings: Vec<Vec<u8>>, // A list of readings to iterate through when .read() is called
    pub current_reading: usize,
}

impl Reader<[u8; 10]> for TestReader {
    fn read(&mut self, buf: &mut [u8; 10]) -> ReadResult {
        for i in 0..=9 {
            buf[i] = self.readings[self.current_reading][i];
        }

        self.current_reading = if self.current_reading < self.readings.len() - 1 {
            self.current_reading + 1
        } else {
            0
        };

        Ok(())
    }
}

pub struct RTL433TestReader {
    pub readings: Vec<String>, // A list of readings to iterate through when .read() is called
    pub current_reading: usize,
}

impl Reader<String> for RTL433TestReader {
    fn read(&mut self, buf: &mut String) -> ReadResult {
        let value = self.readings[self.current_reading].clone();

        buf.insert_str(0, value.as_str());

        self.current_reading = if self.current_reading < self.readings.len() - 1 {
            self.current_reading + 1
        } else {
            0
        };

        Ok(())
    }
}
