use crate::sbi;
use crate::debug::kdebug;

/// Rust kernel entrypoint
///
/// Accepts a `hartid` as a parameter. This should be passed by OpenSBI in `a0` register
#[no_mangle]
pub extern "C" fn kernel_boot(hart_id: i32) -> ! {
    if hart_id != 0 {
        loop {}
    }

    kdebug!(include_str!("logo_fmt.txt"));
    panic!("no further instructions");
}
