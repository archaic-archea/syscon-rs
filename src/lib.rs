#![no_std]
#![feature(pointer_byte_offsets)]
#![feature(never_type)]

use log::{log, Level};

static mut MMIO: mmio::VolBox<u32, mmio::Deny, mmio::Allow> = unsafe {mmio::VolBox::new(core::ptr::null_mut())};
static mut OFF: u32 = 0;
static mut REBOOT: u32 = 0;

pub fn init(fdt_ptr: *const u8) ->  Option<()> {
    unsafe {
        let fdt = fdt::Fdt::from_ptr(fdt_ptr).expect("Invalid Fdt pointer");

        let node = fdt.find_compatible(&["syscon-poweroff"])?;
        let offset = node.property("offset")?.as_usize()?;
        OFF = node.property("value")?.as_usize()? as u32;

        let node = fdt.find_compatible(&["syscon-reboot"])?;
        let offset = node.property("offset")?.as_usize()?;
        REBOOT = node.property("value")?.as_usize()? as u32;

        let syscon_phandle = node.property("regmap")?.as_usize()? as u32;
        let syscon_node = fdt.find_phandle(syscon_phandle)?;
        let syscon_mmio = syscon_node.reg()?.next()?.starting_address.cast::<u32>().cast_mut();

        let mmio: mmio::VolBox<u32, mmio::Deny, mmio::Allow> = mmio::VolBox::new(syscon_mmio);

        MMIO = mmio;
    }

    Some(())
}

pub fn power_off() -> ! {
    log!(Level::Info, "Power off requested");

    unsafe {
        MMIO.write(OFF);
    }

    unreachable!("ERROR OCCURED WHILE POWERING OFF");
}

pub fn reboot() -> ! {
    log!(Level::Info, "Reboot requested");
    
    unsafe {
        MMIO.write(REBOOT);
    }

    unreachable!("ERROR OCCURED WHILE REBOOTING");
}