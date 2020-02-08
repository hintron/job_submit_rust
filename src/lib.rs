
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

// extern int job_submit(job_desc_msg_t *job_desc, uint32_t submit_uid,
//               char **err_msg)
#[no_mangle]
pub extern "C" fn job_submit() {
    println!("Rust has infected Slurm!");
}

// extern int job_modify(job_desc_msg_t *job_desc, job_record_t *job_ptr,
//               uint32_t submit_uid)
#[no_mangle]
pub extern "C" fn job_modify() {
    println!("Rust has infected Slurm!");
}









// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
