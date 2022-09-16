use windows_sys::Win32::System::Threading::WakeByAddressSingle;

fn main() {
    unsafe { WakeByAddressSingle(std::ptr::null()) };
}
