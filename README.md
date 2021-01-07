# Rust Embedded-HAL PCF8591 Driver

For interfacing with the [PCF8591 8-bit A/D and D/A converter](https://www.nxp.com/docs/en/data-sheet/PCF8591.pdf). Currently only handles reading a single single-ended ADC at a time.

## Example

With the PCF8591 connected to a raspberry pi:

```rust
use linux_embedded_hal::I2cdev;
use pcf8591_hal::*;

pub fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").expect("can open i2c device");
    let mut adc = PCF8591::new(i2c, PCF8591_DEFAULT_ADDRESS);

    loop {
        let a0 = adc.read(PCFADCNum::A0).expect("can read ADC0");
        println!("a0: {}", a0);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
```

