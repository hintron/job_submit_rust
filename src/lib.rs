// https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html#convert-date-to-unix-timestamp-and-vice-versa
// https://docs.rs/chrono/0.4.10/chrono/
extern crate chrono;
use chrono::{DateTime, Local};

extern crate libc;
use libc::{c_void, c_int};
// use libc::{time_t, c_int};

use std::fs::File;
use std::io::prelude::*;

// Export strings for plugin interface
// https://users.rust-lang.org/t/how-to-export-string-symbol-not-function-to-c/26039
// https://stackoverflow.com/questions/31701655/can-a-rust-constant-static-be-exposed-to-c
#[no_mangle]
#[used]
pub static plugin_name: [u8; 23] = *b"Job submit Rust plugin\0";
#[no_mangle]
#[used]
pub static plugin_type: [u8; 16] = *b"job_submit/rust\0";
#[no_mangle]
#[used]
pub static plugin_version: u32 = 0x140200; /* i.e. Slurm 20.02.0 */

// C prototype:
// extern int job_submit(job_desc_msg_t *job_desc, u32 submit_uid,
//               u8 **err_msg)
#[no_mangle]
pub extern "C" fn job_submit(_job_desc: *mut c_void, submit_uid: u32,
                             _err_msg: *mut c_void) -> c_int {
    println!("Rust has infected Slurm! Submitted by user {}", submit_uid);
    let file = File::create("job_submit_rust.log");
    let now: DateTime<Local> = Local::now();
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return -1,
    };
    let result = write!(file, "{}: Rust's job_submit has been called from Slurm! uid={}", now, submit_uid);
    match result {
        Ok(_) => {},
        Err(_) => return -1,
    };

    0
}

// C prototype:
// extern int job_modify(job_desc_msg_t *job_desc, job_record_t *job_ptr,
//               u32 submit_uid)
#[no_mangle]
pub extern "C" fn job_modify(_job_desc: *mut c_void, _job_ptr: *mut c_void,
                             submit_uid: u32) -> c_int {
    println!("Rust has infected Slurm! Modified by user {}", submit_uid);
    let file = File::create("job_submit_rust.log");
    let now: DateTime<Local> = Local::now();
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return -1,
    };
    let result = write!(file, "{}: Rust's job_modify has been called from Slurm! uid={}", now, submit_uid);
    match result {
        Ok(_) => {},
        Err(_) => return -1,
    };
    0
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
