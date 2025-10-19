use windows::{
    core::s,
    Win32::{
        Foundation::{
            HINSTANCE,
            HWND
        },
        System::SystemServices,
        UI::WindowsAndMessaging::MessageBoxA
    } 
};

/*
- Windows crate has both s and w macros for creating strings
- DllMain is the entrypoint called when the DLL is attached / loaded
- Make note of the '#[unsafe(no_mangle)]'
  - If compiled without this, functions will not have the same names assigned in code. no_mangle tells the compiler not to mangle the name
*/

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_code: u32, _lp_reserved: &mut ()) -> bool {
    match call_code {
        // Runs when DLL is attached
        SystemServices::DLL_PROCESS_ATTACH => {
            unsafe {
                MessageBoxA(
                    Some(HWND(std::ptr::null_mut())), 
                    s!("Hi"),
                    s!("From: injected.dll"),
                    Default::default()
                );
            }
        },
        // Runs when DLL is detached
        SystemServices::DLL_PROCESS_DETACH => {
            unsafe {
                MessageBoxA(
                    Some(HWND(std::ptr::null_mut())), 
                    s!("Bye"),
                    s!("From: injected.dll"),
                    Default::default()
                );
            }
        },

        _ => {}
    }

    true
}

// This is an example of an exported function called 'hello' that can be called by other applications after loading the dll
#[unsafe(no_mangle)]
extern "system" fn hello() {
    unsafe {
        MessageBoxA(
            Some(HWND(std::ptr::null_mut())), 
            s!("hello"),
            s!("injected.dll"),
            Default::default()
        );
    }
}
