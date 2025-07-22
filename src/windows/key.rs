use std::sync::Mutex;
use windows::{
    Win32::Foundation::*, 
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY, 
    Win32::UI::WindowsAndMessaging::*,
};
use std::thread;

struct HookState {
    special_key: VIRTUAL_KEY,
    special_callback: Box<dyn Fn() + Send + 'static>,
    other_keys_callback: Box<dyn Fn() + Send + 'static>,
}

static HOOK_STATE: Mutex<Option<HookState>> = Mutex::new(None);
thread_local! {
    static HOOK_HANDLE: std::cell::RefCell<Option<HHOOK>> = std::cell::RefCell::new(None);
}

pub struct HookKey;

impl HookKey {
    pub fn from_str(key: &str) -> Option<VIRTUAL_KEY> {
        if key.len() != 1 {
            return None;
        }

        let c = key.chars().next().unwrap().to_ascii_uppercase();

        if c.is_ascii_alphabetic() {
            return Some(VIRTUAL_KEY(c as u16 - 'A' as u16 + 0x41));
        }

        None
    }

    pub fn hook<F1, F2>(
        special_key: VIRTUAL_KEY,
        special_callback: F1,
        other_keys_callback: F2,
    ) -> windows::core::Result<()>
    where
        F1: Fn() + Send + 'static,
        F2: Fn() + Send + 'static,
    {
        let mut state = HOOK_STATE.lock().unwrap();
        *state = Some(HookState {
            special_key,
            special_callback: Box::new(special_callback),
            other_keys_callback: Box::new(other_keys_callback),
        });

        unsafe {
            let handle = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(keyboard_hook),
                Some(GetModuleHandleW(None).unwrap().into()),
                0,
            )?;

            HOOK_HANDLE.with(|h| {
                *h.borrow_mut() = Some(handle);
            });
        }

        Ok(())
    }

    pub fn unregister() {
        HOOK_HANDLE.with(|h| {
            if let Some(handle) = h.borrow_mut().take() {
                unsafe {
                    let _ = UnhookWindowsHookEx(handle);
                }
            }
        });

        let mut state = HOOK_STATE.lock().unwrap();
        *state = None;
    }
}

unsafe extern "system" fn keyboard_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= HC_ACTION as i32 {
        if wparam.0 == WM_KEYDOWN as usize {
            if let Some(state) = &*HOOK_STATE.lock().unwrap() {
                let kb = unsafe { &*(lparam.0 as *const KBDLLHOOKSTRUCT) };
                let vk_code = kb.vkCode;
                
                let is_alpha = vk_code >= 0x41 && vk_code <= 0x5A;
                
                if vk_code == state.special_key.0 as u32 {
                    (state.special_callback)();
                } else if is_alpha {
                    (state.other_keys_callback)();
                }
            }
        }
    }
    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

pub fn register_keys<F1, F2>(
    special_key: &str,
    special_func: F1,
    other_keys_func: F2,
) -> Result<(), String>
where
    F1: Fn() + Send + 'static,
    F2: Fn() + Send + 'static,
{
    if let Some(vk) = HookKey::from_str(special_key) {
        HookKey::hook(vk, special_func, other_keys_func).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("Невозможно распознать клавишу - {}", special_key))
    }
}

pub fn run_message_loop() {
    thread::spawn(|| unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    });
}