use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use windows::Win32::System::Threading::{OpenProcess, QueryFullProcessImageNameW, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::Foundation::MAX_PATH;

pub fn get_active_application() -> String {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            return "Unknown".into();
        }

        let mut process_id = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        if process_id == 0 {
            return "Unknown".into();
        }

        let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id);
        
        if let Ok(handle) = process_handle {
            let mut buffer = [0u16; MAX_PATH as usize];
            let mut size = MAX_PATH;
            
            let result = QueryFullProcessImageNameW(
                handle,
                windows::Win32::System::Threading::PROCESS_NAME_FORMAT(0),
                windows::core::PWSTR(buffer.as_mut_ptr()),
                &mut size,
            );
            
            let _ = windows::Win32::Foundation::CloseHandle(handle);

            if result.is_ok() {
                let path = String::from_utf16_lossy(&buffer[..size as usize]);
                // Extract just the executable name
                if let Some(idx) = path.rfind('\\') {
                    return path[idx + 1..].to_lowercase();
                } else {
                    return path.to_lowercase();
                }
            }
        }
        
        "Unknown".into()
    }
}
