extern crate hidapi;

use hidapi::HidApi;
use shared_memory::*;

async fn init_usb() {
    let mut adx_in = [0u8; 32];
    let mut input_status_smem = create_input_shared_memory();

    let api = HidApi::new().expect("Failed to create API Instance");
    println!("ADX Button Server Started");

    let device = api.open(0x0ca3, 0x0021).expect("Failed to open ADX HID");
    
    // b0+b1 = gamebtns, b2 = opbtns
    let mut btns = [0u8; 3];

    loop {
        // reset buttons
        btns[0] = 0x00;
        btns[1] = 0x00;
        btns[2] = 0x00;

        // Read input status from adx
        let res = device.read(&mut adx_in[..]).unwrap();

        // map front buttons (FLIP 1 and 4 to see if working)
        if !((adx_in[29] & 0x04) == 0x04) {btns[0] |= 0x01;} // BTN1
        if !((adx_in[29] & 0x08) == 0x08) {btns[0] |= 0x02;} // BTN2
        if !((adx_in[29] & 0x01) == 0x01) {btns[0] |= 0x04;} // BTN3
        if !((adx_in[30] & 0x80) == 0x80) {btns[0] |= 0x08;} // BTN4
        if !((adx_in[30] & 0x40) == 0x40) {btns[0] |= 0x10;} // BTN5
        if !((adx_in[30] & 0x20) == 0x20) {btns[0] |= 0x20;} // BTN6
        if !((adx_in[30] & 0x10) == 0x10) {btns[0] |= 0x40;} // BTN7
        if !((adx_in[30] & 0x08) == 0x08) {btns[0] |= 0x80;} // BTN8

        // map side buttons
        if (adx_in[29] & 0x02) == 0x02 {btns[1] |= 0x01;} // SELECT (UP)
        if (adx_in[30] & 0x02) == 0x02 {btns[2] |= 0x01;} // TEST (O1)
        if (adx_in[29] & 0x40) == 0x40 {btns[2] |= 0x04;} // COIN (DOWN)
        if (adx_in[26] & 0x01) == 0x01 {btns[2] |= 0x02;} // SERVICE (O2)

        // Write button status to shared memory
        unsafe {
            let input_status_mut = input_status_smem.as_slice_mut();
            for (i, el) in btns.iter().enumerate() {
                input_status_mut[i] = *el;
            }
        }
    }
}

fn create_input_shared_memory() -> Shmem {
    let mut shmem = match ShmemConf::new().size(3).os_id("adx_button").create() {
        Ok(m) => m,
        Err(ShmemError::MappingIdExists) => {
            ShmemConf::new().os_id("adx_button").open().unwrap()
        }
        Err(_) => {
            println!("Failed to create shared memory: adx_button");
            panic!()
        }
    };

    shmem.set_owner(true);

    return shmem;
}

#[tokio::main]
async fn main() {
    let task = tokio::spawn(init_usb());

    task.await.ok();
}
