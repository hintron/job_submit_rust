// https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html#convert-date-to-unix-timestamp-and-vice-versa
// https://docs.rs/chrono/0.4.10/chrono/
extern crate chrono;
use chrono::{DateTime, Local};

extern crate libc;
use libc::{c_void, c_int};

use std::fs::{OpenOptions};
use std::io::prelude::*;

const SLURM_SUCCESS: c_int = 0;
const SLURM_FAILURE: c_int = -1;

const JOB_SUBMIT_FILE: &str = "job_submit_rust.log";

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
    println!("Rust has infected Slurm! Submitted by user {}\n", submit_uid);

    let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(JOB_SUBMIT_FILE);
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return SLURM_FAILURE,
    };

    let now: DateTime<Local> = Local::now();
    let result = write!(file, "{}: Job Submit Rust: Slurm has called job_submit() in Rust! uid={}\n", now, submit_uid);
    match result {
        Ok(_) => {},
        Err(_) => return SLURM_FAILURE,
    };

    SLURM_SUCCESS
}

// C prototype:
// extern int job_modify(job_desc_msg_t *job_desc, job_record_t *job_ptr,
//               u32 submit_uid)
#[no_mangle]
pub extern "C" fn job_modify(_job_desc: *mut c_void, _job_ptr: *mut c_void,
                             submit_uid: u32) -> c_int {
    println!("Rust has infected Slurm! Modified by user {}\n", submit_uid);

    let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(JOB_SUBMIT_FILE);
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return SLURM_FAILURE,
    };

    let now: DateTime<Local> = Local::now();
    let result = write!(file, "{}: Job Submit Rust: Slurm has called job_modify() in Rust! uid={}\n", now, submit_uid);
    match result {
        Ok(_) => {},
        Err(_) => return SLURM_FAILURE,
    };
    SLURM_SUCCESS
}

#[no_mangle]
pub extern "C" fn fini() {
    let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(JOB_SUBMIT_FILE);
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return,
    };

    let now: DateTime<Local> = Local::now();
    let result = write!(file, "{}: Job Submit Rust: fini\n", now);
    match result {
        Ok(_) => {},
        Err(_) => return,
    };
}

#[no_mangle]
pub extern "C" fn init() -> c_int {
    let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(JOB_SUBMIT_FILE);
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return SLURM_FAILURE,
    };

    let now: DateTime<Local> = Local::now();
    let result = write!(file, "{}: Job Submit Rust: init\n", now);
    match result {
        Ok(_) => {},
        Err(_) => return SLURM_FAILURE,
    };

    SLURM_SUCCESS
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
