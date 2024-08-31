// based on chuniio
// https://github.com/caxerx/chuniio_tasoller_new
// use dniel97 segatools
// https://gitea.tendokyu.moe/Dniel97/segatools/src/branch/develop/mai2io/mai2io.h
// follow example from chuniio
// https://gitea.tendokyu.moe/Dniel97/segatools/src/branch/develop/chuniio/chuniio.h
use std::sync::Arc;

use shared_memory::*;
use windows::core::HRESULT;

static mut GAMEBTNS: u16 = 0x0000;
static mut OPBTNS: u8 = 0x00;
static mut INPUT_SHMEM: Option<Arc<Shmem>> = None;

#[no_mangle]
pub extern "C" fn mai2_io_get_api_version() -> u16 {
    0x0100
}

fn create_input_shared_memory() -> Shmem {
    match ShmemConf::new().os_id("adx_button").open() {
        Ok(shmem) => shmem,
        Err(_) => {
            panic!("Failed to load shared memory: adx_button")
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mai2_io_init() -> HRESULT {
    INPUT_SHMEM = Some(Arc::new(create_input_shared_memory()));
    HRESULT(0)
}

// get button states
#[no_mangle]
pub extern "C" fn mai2_io_poll() -> HRESULT {
    let input_shmem = unsafe {
        match &INPUT_SHMEM {
            Some(t) => t,
            &None => todo!(),
        }
    };

    unsafe {
        let input = std::slice::from_raw_parts(input_shmem.as_ptr(), 3);
        GAMEBTNS = ((input[1] as u16) << 8) | input[0] as u16;
        OPBTNS = input[2];
    }

    HRESULT(0)
}

// read option buttons
#[no_mangle]
pub extern "C" fn mai2_io_get_opbtns(opbtn: *mut u8) {
    unsafe {
        *opbtn = OPBTNS;
    }
}

// read game buttons
#[no_mangle]
pub extern "C" fn mai2_io_get_gamebtns(player1: *mut u16, player2: *mut u16) {
    unsafe {
        *player1 = GAMEBTNS;
    }
}
