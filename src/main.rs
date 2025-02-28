use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use objc::runtime::Class;
use objc::{msg_send, sel, sel_impl};

use anyhow::{Context, Result};

fn detect_macos_theme() -> Result<bool> {
    unsafe {
        let cls = Class::get("NSUserDefaults").context("Failed to get NSUserDefaults class")?;
        let defaults: *mut objc::runtime::Object = msg_send![cls, standardUserDefaults];

        let key = CFString::new("AppleInterfaceStyle");
        let appearance: *mut objc::runtime::Object =
            msg_send![defaults, objectForKey: key.as_concrete_TypeRef()];

        Ok(!appearance.is_null())
    }
}

fn main() -> Result<()> {
        println!(
        "Dark Mode is {}",
        if detect_macos_theme()? {
            "enabled"
        } else {
            "disabled"
        }
    );
    Ok(())
}
