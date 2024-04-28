mod cuda_cpi;
mod cpu_control;

use cuda_cpi::*;
use cpu_control::*;


extern "Rust" {
    #[link_name = "hello"]
    fn my_demo_function(a: u32) -> u32;
}

mod a {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    fn hello(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(1);
        }
    }
}