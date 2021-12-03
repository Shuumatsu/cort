// #![no_std]
#![feature(asm)]
#![feature(llvm_asm)]

const SSIZE: isize = 48;

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

fn hello() -> ! {
    println!("Hello, world!");
    loop {}
}

// unsafe fn switch(new: *const ThreadContext) {
//     // asm!(
//     //     "
//     //     mov rsp, $0
//     //     ret"
//     // );
//     llvm_asm!("
//         mv     0x0($0), pc
//         ret
//        "
//     :
//     : "r"(new)
//     :
//     : "alignstack" // it will work without this now, will need it later
//     );
// }

// fn main() {
//     let mut ctx = ThreadContext::default();
//     let mut stack = vec![0_u8; SSIZE as usize];

//     unsafe {
//         let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
//         let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
//         std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);

//         ctx.rsp = sb_aligned.offset(-16) as u64;
//         switch(&mut ctx);
//     }
// }

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

// // the -> ! means that this function won't return
// #[panic_handler]
// fn panic(info: &core::panic::PanicInfo) -> ! {
//     if let Some(p) = info.location() {
//     } else {
//     }
//     abort();
// }

// // https://internals.rust-lang.org/t/why-rust-has-name-mangling/12503
// // turns off Rust's name mangling so the symbol is exactly eh_personality
// #[no_mangle]
// extern "C" fn abort() -> ! {
//     loop {
//         unsafe {
//             riscv::asm::wfi();
//         }
//     }
// }
