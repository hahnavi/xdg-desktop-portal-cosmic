// SPDX-License-Identifier: GPL-3.0-or-later

use std::backtrace::Backtrace;
use std::{panic, thread};

pub fn init() {
    panic::set_hook(Box::new(|info| {
        let thread = thread::current();
        let thread_name = thread.name().unwrap_or("<unnamed>");
        let message = info
            .payload()
            .downcast_ref::<&'static str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
            .unwrap_or("Box<Any>");
        let backtrace = Backtrace::force_capture();

        match info.location() {
            Some(location) => log::error!(
                target: "panic",
                "thread '{}' panicked at '{}': {}:{}{:?}",
                thread_name,
                message,
                location.file(),
                location.line(),
                backtrace
            ),
            None => log::error!(
                target: "panic",
                "thread '{}' panicked at '{}'{:?}",
                thread_name,
                message,
                backtrace
            ),
        }
    }));
}
