# Syscon-rs
A basic syscon-poweroff and syscon-reboot driver
Relies on `log` and `fdt` crates

# Usage
```extern "C" fn kmain(_hartid: u64, devicetree_ptr: *const u8) {
    // Initialize logger
    
    syscon_rs::init(devicetree_ptr);
    
    syscon_rs::power_off().expect("Failed to power off");
}```
