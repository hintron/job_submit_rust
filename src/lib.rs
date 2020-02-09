// https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html#convert-date-to-unix-timestamp-and-vice-versa
// https://docs.rs/chrono/0.4.10/chrono/
extern crate chrono;
use chrono::{DateTime, Local};

extern crate libc;
use libc::{c_int, c_void};

use std::fs::OpenOptions;
use std::io::prelude::*;

const SLURM_SUCCESS: c_int = 0;
const SLURM_FAILURE: c_int = -1;

const JOB_SUBMIT_FILE: &str = "job_submit_rust.log";
const LOG_PREFIX: &str = "Job Submit Rust: ";

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

/*
 * Create a print-to-file wrapper that also adds a custom prefix and a time
 * stamp. Importantly, on failure, the value for rc found in the local scope is
 * set to SLURM_FAILURE.
 *
 * See:
 * https://doc.rust-lang.org/reference/macros-by-example.html
 * https://github.com/rust-lang/rust/blob/07a34df18b437319a7ff510077bbab95cf7ec6bc/src/libstd/macros.rs#L62-L64
 * https://doc.rust-lang.org/std/macro.format_args.html
 * https://doc.rust-lang.org/rust-by-example/macros/variadics.html
 * https://doc.rust-lang.org/book/ch19-06-macros.html
 * https://stackoverflow.com/questions/34373169/how-do-i-create-a-rust-macro-with-optional-parameters-using-repetitions
 */
macro_rules! log_file {
    ($rc:expr, $($arg:tt)*) => ({
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(JOB_SUBMIT_FILE);
        match file {
            Ok(file) => {
                let mut file = file;
                let now: DateTime<Local> = Local::now();
                let result = write!(
                    file,
                    "{}: {}{}\n",
                    now, LOG_PREFIX, format_args!($($arg)*)
                );
                match result {
                    Ok(_) => {}
                    Err(_) => {
                        $rc = SLURM_FAILURE;
                    },
                };
            },
            Err(_) => {
                $rc = SLURM_FAILURE;
            },
        };
    });
}

// C prototype:
// extern int job_submit(job_desc_msg_t *job_desc, u32 submit_uid,
//               u8 **err_msg)
#[no_mangle]
pub extern "C" fn job_submit(
    _job_desc: *mut c_void,
    submit_uid: u32,
    _err_msg: *mut c_void,
) -> c_int {
    let mut rc = SLURM_SUCCESS;
    log_file!(
        rc,
        "Slurm has called job_submit() in Rust! uid={}",
        submit_uid
    );
    rc
}

// C prototype:
// extern int job_modify(job_desc_msg_t *job_desc, job_record_t *job_ptr,
//               u32 submit_uid)
#[no_mangle]
pub extern "C" fn job_modify(
    _job_desc: *mut c_void,
    _job_ptr: *mut c_void,
    submit_uid: u32,
) -> c_int {
    let mut rc = SLURM_SUCCESS;
    log_file!(
        rc,
        "Slurm has called job_submit() in Rust! uid={}",
        submit_uid
    );
    rc
}

#[no_mangle]
pub extern "C" fn fini() {
    let mut _rc = SLURM_SUCCESS;
    log_file!(_rc, "fini");
}

#[no_mangle]
pub extern "C" fn init() -> c_int {
    let mut rc = SLURM_SUCCESS;
    log_file!(rc, "init");
    rc
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
