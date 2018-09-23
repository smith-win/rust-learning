
// Attempt to read Concept2 data using usb bindings ()


#[macro_use]
extern crate log;
extern crate env_logger;
extern crate libusb;

use libusb::{DeviceList, Device, Language};
use std::time::Duration;

fn main() {

    // logger initialization
    env_logger::Builder::from_env("APP_LOG").init();

    let context = libusb::Context::new().unwrap();

    let devices = context.devices().unwrap();

    for mut device in devices.iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("> Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id()
        );
    }
    //TODO: unwrap .. 
    let pm_device =  find_concept2_pm(&devices).unwrap();

    // Now open it and read from the device 

    let pm_device_handle_result = pm_device.open();
    if pm_device_handle_result.is_err() {
        error!("Failed to open PM4 ");
    }

    // Unwrap is dangerous here 
    let pm_device_handle = pm_device_handle_result.unwrap();

    // Need languages to read from it 
    let default_timeout = Duration::new(5, 0);
    let languages = pm_device_handle.read_languages(default_timeout).unwrap();
    debug!("Read languages {}", languages.len());

    let language_opt:Option<&Language> = match languages.len() {
        0 => None,
        _ => languages.get(0),
    };
    
    let language = language_opt.unwrap();
    let lang_id =  language.lang_id();
    info!("PM4 language {} {:?}, {:?}", lang_id, language.primary_language(), language.sub_language());

    //let devName = pm_device_handle.

    // So now we can start interrogating the device .. 
    // TODO: device descriptor 
    let product =  pm_device_handle.read_product_string(*language, &(pm_device.device_descriptor().unwrap()), default_timeout).unwrap();
    info!("Product: {:?}", product);
    let manuf =  pm_device_handle.read_manufacturer_string(*language, &(pm_device.device_descriptor().unwrap()), default_timeout).unwrap();
    info!("Manuf: {:?}", manuf);
}


/// Attemp to locate a Concept 2
/// My PM4 -- ls /usr/lib64/pkgconfig/
fn find_concept2_pm<'a> (devices : & DeviceList<'a>) -> Option<Device<'a>> {

    // TODO: change this to "functional" / "lamda" style ("stream") 

    for device in devices.iter() {
        // TODO : change to Result / and "?" to handle errors
        let device_desc = device.device_descriptor().unwrap();
        debug!("Checking {:04x}", device_desc.vendor_id());
        if device_desc.vendor_id() == 0x17a4u16 {
            info!("Found concept2");

            return Some(device);
        }

    }

    warn!("Did not find Concept2 / Performance Monitor");
    None
}


