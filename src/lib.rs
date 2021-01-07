// Copyright 2021 Kenton Hamaluik
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use embedded_hal::blocking::i2c;

/// The default address for the PCF8591 (all address pins tied to ground)
pub const PCF8591_DEFAULT_ADDRESS: u8 = 0x48;

/// The PCF8591 has four ADC channels, represented here
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PCFADCNum {
    A0,
    A1,
    A2,
    A3,
}

/// A PCF8591 ADC using the underlying i2c channel
pub struct PCF8591<C: i2c::WriteRead> {
    channel: C,
    addr: u8,
}

impl<C: i2c::WriteRead> PCF8591<C> {
    /// Instantiate the device using the given channel
    pub fn new(channel: C, addr: u8) -> PCF8591<C> {
        PCF8591 { channel, addr }
    }

    /// Read a single ADC value from a single port
    pub fn read(&mut self, adc: PCFADCNum) -> Result<u8, C::Error> {
        // first trigger the measurement
        self.half_read(adc)?;
        // then communicate again to get the actual result
        self.half_read(adc)
    }

    fn half_read(&mut self, adc: PCFADCNum) -> Result<u8, C::Error> {
        let adc: u8 = match adc {
            PCFADCNum::A0 => 0,
            PCFADCNum::A1 => 1,
            PCFADCNum::A2 => 2,
            PCFADCNum::A3 => 3,
        };

        // TODO: take into account writing to the DAC
        let command: [u8; 2] = [adc, 0];
        let mut buffer: [u8; 2] = [0, 0];

        self.channel
            .write_read(self.addr, &command[..], &mut buffer[..])?;
        Ok(buffer[1])
    }

    /// Consume the driver, returning the underlying channel
    pub fn into_inner(self) -> C {
        self.channel
    }
}
