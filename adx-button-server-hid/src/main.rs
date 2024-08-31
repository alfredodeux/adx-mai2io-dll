extern crate hidapi;

use hidapi::HidApi;
use shared_memory::*;

async fn init_usb() {
    let mut adx_in = [0u8; 13];
    let mut input_status_smem = create_input_shared_memory();

    let api = HidApi::new().expect("Failed to create API Instance");
    println!("ADX Button Server Started");

    let device = api.open(0x2e3c, 0x5750).expect("Failed to open ADX HID");
    
    // b0+b1 = gamebtns, b2 = opbtns
    let mut btns = [0u8; 3];

    loop {
        // reset buttons
        btns[0] = 0x00;
        btns[1] = 0x00;
        btns[2] = 0x00;

        // Read input status from adx
        let res = device.read(&mut adx_in[..]).unwrap();

        // map front buttons
        if adx_in[4] == 1 {btns[0] |= 0x01;} // BTN1
        if adx_in[3] == 1 {btns[0] |= 0x02;} // BTN2
        if adx_in[2] == 1 {btns[0] |= 0x04;} // BTN3
        if adx_in[1] == 1 {btns[0] |= 0x08;} // BTN4
        if adx_in[8] == 1 {btns[0] |= 0x10;} // BTN5
        if adx_in[7] == 1 {btns[0] |= 0x20;} // BTN6
        if adx_in[6] == 1 {btns[0] |= 0x40;} // BTN7
        if adx_in[5] == 1 {btns[0] |= 0x80;} // BTN8

        // map side buttons
        if adx_in[9]  == 1 {btns[1] |= 0x01;} // SELECT (UP)
        if adx_in[10] == 1 {btns[2] |= 0x01;} // TEST (O1)
        if adx_in[11] == 1 {btns[2] |= 0x02;} // SERVICE (DOWN)
        if adx_in[12] == 1 {btns[2] |= 0x04;} // COIN (O2)

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
