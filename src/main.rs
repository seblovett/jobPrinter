use escpos::driver::*;
use escpos::errors::Result as PrintResult;
use escpos::printer::Printer;
use escpos::utils::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct JobInfo {
    title: String,
    description: String,
}

impl JobInfo {
    fn dprint(&self) -> PrintResult<()> {
        println!("{}", &self.title);
        println!("{}", &self.description);
        Ok(())
    }

    fn print(&self) -> PrintResult<()> {
        let driver = UsbDriver::open(0x0416, 0x5011, None, None)?;
        Printer::new(driver, Protocol::default(), None)
            .debug_mode(Some(DebugMode::Dec))
            .init()?
            .justify(JustifyMode::CENTER)?
            .size(2, 2)?
            .underline(UnderlineMode::Single)?
            .bold(true)?
            .writeln(&self.title)?
            .feed()?
            .feed()?
            .reset_style_state()
            .justify(JustifyMode::LEFT)?
            .size(1, 1)?
            .underline(UnderlineMode::None)?
            .bold(false)?
            .writeln(&self.description)?
            .feed()?
            .print_cut()?;

        Ok(())
    }
}

fn lsusb() {
    // List of USB devices
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!(
            "Bus: {:03} Device: {:03} VID: {:04x} PID: {:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id(),
        );
    }
}

fn print_from_file(file: &str) ->  Result<(), Box<dyn std::error::Error>>{
 let data = fs::read_to_string("jobs.json")?;

    let j1: Vec<JobInfo> = serde_json::from_str(&data)?;
    for j in j1.iter() {
        j.dprint()?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // lsusb();
    print_from_file("jobs.json")?;
    Ok(())
   
}
