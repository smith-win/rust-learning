
// Attempt to read Concept2 data using usb bindings ()


#[macro_use]
extern crate log;
extern crate env_logger;
extern crate libusb;

use libusb::{DeviceList, Device, Language, Error, DeviceHandle};
use std::time::Duration;

fn main() {

    // logger initialization
    env_logger::Builder::from_env("APP_LOG").init();

    let context = libusb::Context::new().unwrap();

    let devices = context.devices().unwrap();

    // this was sample code
    // for mut device in devices.iter() {
    //     let device_desc = device.device_descriptor().unwrap();

    //     println!("> Bus {:03} Device {:03} ID {:04x}:{:04x}",
    //         device.bus_number(),
    //         device.address(),
    //         device_desc.vendor_id(),
    //         device_desc.product_id()
    //     );
    // }
    //TODO: unwrap .. 
    let pm_device =  find_concept2_pm(&devices).unwrap();

    // Now open it and read from the device 

    let pm_device_handle_result = pm_device.open();
    if pm_device_handle_result.is_err() {
        error!("Failed to open PM4 ");
    }

    // Unwrap is dangerous here 
    let mut pm_device_handle = pm_device_handle_result.unwrap();

    pm_device_handle.detach_kernel_driver(0).unwrap();

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


    // So now we can start interrogating the device .. 
    // TODO: device descriptor 
    let product =  pm_device_handle.read_product_string(*language, &(pm_device.device_descriptor().unwrap()), default_timeout).unwrap();
    info!("Product: {:?}", product);
    let manuf =  pm_device_handle.read_manufacturer_string(*language, &(pm_device.device_descriptor().unwrap()), default_timeout).unwrap();
    info!("Manuf: {:?}", manuf);
    info!("Active config: {}", pm_device_handle.active_configuration().unwrap());

    let x = device_list_interfaces(&pm_device, &pm_device_handle, &language);
    if x.is_err() {
        error!("device_list_interfaces {:?}", x);
    }
}


/// Attemp to locate a Concept 2
/// My PM4 -- ls /usr/lib64/pkgconfig/
fn find_concept2_pm<'a> (devices : & DeviceList<'a>) -> Option<Device<'a>> {

    // TODO: change this to "functional" / "lamda" style ("stream") 
    // You would use "find" on iterator and a closure on 

    let some_device = devices.iter().find( |d| d.device_descriptor().unwrap().vendor_id() == 0x17a4 );

    // for device in devices.iter() {
    //     // TODO : change to Result / and "?" to handle errors
    //     let device_desc = device.device_descriptor().unwrap();
    //     debug!("Checking {:04x}", device_desc.vendor_id());
    //     if device_desc.vendor_id() == 0x17a4u16 {
    //         info!("Found concept2");

    //         return Some(device);
    //     }

    // }

    if some_device.is_none() {
        warn!("Did not find Concept2 / Performance Monitor");
    }
    some_device
}


/// List out the interfaces
fn device_list_interfaces(device: &Device, dev_handle : &DeviceHandle, lang : &Language) -> Result<(), Error> {

    let config_desc = device.active_config_descriptor()?;

    for i in config_desc.interfaces() {
        let i_num = i.number();

        for desc in i.descriptors() {
            let desc_idx = desc.description_string_index();

            let some_string = dev_handle.read_interface_string(*lang, &desc, Duration::new(5, 0))?;

            info!("i={}, desc_idx {:?}, {}", i_num, desc_idx, some_string);
        }
    }
    Ok(())
}