/*
    BladeWay Autoclicker Program
    Copyright (C) 2025  Evgeny K.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, Ordering};
use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

static HOOK: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

unsafe extern "system" fn mouse_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 {
        return unsafe { CallNextHookEx(Some(HHOOK(std::ptr::null_mut())), code, wparam, lparam) };
    }

    if wparam.0 as u32 == WM_MOUSEMOVE {
        return LRESULT(1);
    }

    unsafe { CallNextHookEx(Some(HHOOK(std::ptr::null_mut())), code, wparam, lparam) }
}

pub fn block_mouse() -> bool {
    unsafe {
        let hook = SetWindowsHookExA(
            WH_MOUSE_LL,
            Some(mouse_hook),
            Some(HINSTANCE(std::ptr::null_mut())),
            0,
        );

        if hook.clone().is_err() || hook.clone().unwrap().is_invalid() {
            return false;
        }

        HOOK.store(hook.clone().unwrap().0, Ordering::SeqCst);
        true
    }
}

pub fn unblock_mouse() {
    let hook_value = HOOK.swap(std::ptr::null_mut(), Ordering::SeqCst);
    if hook_value != std::ptr::null_mut() {
        unsafe {
            let _ = UnhookWindowsHookEx(HHOOK(hook_value));
        }
    }
    let _ = hook_value;
}
