#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
extern  crate alloc;
use x86_64::VirtAddr;
use os::{task::{Task, executor::Executor, keyboard, shell}};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use os::allocator;
    use os::memory::{self, BootInfoFrameAllocator};

    os::init();


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap init failed");

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::init()));
    executor.spawn(Task::new(shell::init()));
    executor.spawn(Task::new(os::fs::init()));

    executor.run();

    #[allow(unreachable_code)]
    os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use os::println;

    println!("{}", info);
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}