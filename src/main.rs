use windows_sys::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*,
};

fn main() {
    unsafe {
        let event = CreateEventW(std::ptr::null(), 1, 0, std::ptr::null());
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);

        MessageBoxA(0, s!("YAY!"), s!("Title here!"), MB_OK);
    }
}
