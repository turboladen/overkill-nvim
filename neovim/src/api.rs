pub(crate) mod dictionary;
pub(crate) mod mode;
pub(crate) mod object;
pub(crate) mod string;

pub use self::{dictionary::Dictionary, mode::Mode, object::Object, string::String};
pub use neovim_sys::api::vim::{Boolean, Float, Integer, LuaRef};

use neovim_sys::api::{
    buffer::{self, Buffer},
    helpers::cstr_to_string,
    vim,
};
use std::{borrow::Cow, os::raw::c_char};

pub fn nvim_get_var(name: &str) -> Object {
    unsafe {
        let api_name = cstr_to_string(name.as_ptr() as *const c_char);
        let mut out_err = vim::Error::default();

        Object::from(vim::nvim_get_var(api_name, &mut out_err))
    }
}

pub fn nvim_buf_get_var(name: &str) -> Object {
    unsafe {
        let api_name = cstr_to_string(name.as_ptr() as *const c_char);
        let mut out_err = vim::Error::default();

        Object::from(vim::nvim_buf_get_var(api_name, &mut out_err))
    }
}

pub fn nvim_feedkeys(keys: &str, mode: &str, escape_csi: bool) {
    unsafe {
        let api_keys = cstr_to_string(keys.as_ptr() as *const c_char);
        let api_mode = cstr_to_string(mode.as_ptr() as *const c_char);

        vim::nvim_feedkeys(api_keys, api_mode, escape_csi);
    }
}

pub fn nvim_get_current_buf() -> Buffer {
    unsafe { vim::nvim_get_current_buf() }
}

pub fn nvim_buf_get_option(buffer: Buffer, name: &str) -> Object {
    unsafe {
        let api_name = cstr_to_string(name.as_ptr() as *const c_char);
        let mut out_err = vim::Error::default();

        Object::from(buffer::nvim_buf_get_option(
            buffer,
            api_name,
            &mut out_err,
        ))
    }
}

pub fn nvim_get_mode() -> Mode {
    // @returns Dictionary { "mode": String, "blocking": Boolean }
    //
    let d = Dictionary::new(unsafe { vim::nvim_get_mode() });

    let mode = if let Some(Object::String(mode)) = d.get("mode") {
        Mode::from(mode.as_str())
    } else {
        Mode::Normal
    };

    mode
}

pub fn nvim_replace_termcodes(
    string_to_convert: &str,
    from_part: bool,
    do_lt: bool,
    special: bool,
) -> String {
    unsafe {
        let api_string = cstr_to_string(string_to_convert.as_ptr() as *const c_char);

        String::new(Cow::Owned(vim::nvim_replace_termcodes(
            api_string, from_part, do_lt, special,
        )))
    }
}

pub fn nvim_exec(src: &str, output: bool) -> String {
    unsafe {
        let api_src = cstr_to_string(src.as_ptr() as *const c_char);
        let mut out_err = vim::Error::default();

        String::new(Cow::Owned(vim::nvim_exec(api_src, output, &mut out_err)))
    }
}

pub fn nvim_set_hl(namespace_id: Integer, name: &str, val: Dictionary) {
    unsafe {
        let api_name = cstr_to_string(name.as_ptr() as *const c_char);
        let mut out_err = vim::Error::default();

        vim::nvim_set_hl(namespace_id, api_name, val.inner(), &mut out_err)
    }
}

pub fn nvim_get_namespaces() -> Dictionary {
    unsafe { Dictionary::new(vim::nvim_get_namespaces()) }
}

pub fn nvim_create_namespace(name: &str) -> Integer {
    unsafe {
        let api_name = cstr_to_string(name.as_ptr() as *const c_char);

        vim::nvim_create_namespace(api_name)
    }
}
