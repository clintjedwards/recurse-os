// We're skipping the 'create a bootloader' phase of writing an operating system
// due to it being somewhat uneventful, mostly you're just moving things into registers
// and following a bunch of rules in order to load the kernel. Instead we're focusing on writing
// the kernel.

// Don't link the standard library. Since we're writing an OS we can't use any OS
// specific features.
#![no_std]
// Since we don't have std and don't have access to the underlying c runtime and rust
// runtime that makes jumping into main work we instead we must define our own set up
// to the OS. As such we tell rust to not attempt to jump us to main.
#![no_main]

use core::panic::PanicInfo;

// By default the std lib contains a panic handler that is automatically given to us.
// Since we aren't using that we have to create our own or the compiler wont know what
// to do when it has to panic.

// Language items are special functions and types that are required internally by the compiler.
// For example the copy trait tells the compiler how to handle types with copy semantics
// They have special headers like #[lang = "copy"]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Compilers usually take the function names that we give it and edit it to some degree
// for a variety of reasons, one of which being that they need to be unique to be called
// correctly. no_mangle tells the compiler to fuck off since we need this to be called
// exactly this in order to reference it in the linker.
//
// The extern C part of this means that the compiler should use the C calling convention
// for this function. A calling convention is just a specified way to pass parameters into and
// call a function.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
