use crate::sbi;

fn writeln(string: &str) {
    let Some(console) = sbi::DebugConsole::get_if_available() else { return; };

    for byte in string.bytes() {
        console.write_byte(byte)
    }
    console.write_byte('\n' as u8);
}

/// Rust kernel entrypoint
///
/// Accepts a `hartid` as a parameter. This should be passed by OpenSBI in `a0` register
#[no_mangle]
pub extern "C" fn kernel_boot(hart_id: i32) {
    if hart_id != 0 {
        loop {}
    }

    writeln(r"|\     /|");
    writeln(r"| \___/ |  /|");
    writeln(r"| u   u |_/ |    losGatOS, master branch");
    writeln(r"| >=u=< | o |    (c) Miroslaw Blazej 2024");
    writeln(r"\_______v=< |");
    writeln(r"    \_______/");
    loop {}
}
