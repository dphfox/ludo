use std::ffi::CString;
use anyhow::{anyhow, Result};
use libloading::{Library, Symbol};
use mlua::{lua_CFunction, Function, Lua};

pub fn load_extension(
    lua: &Lua,
    library: Library,
    entry_point: CString,
) -> Result<Function> {
    let ext_main: Symbol<lua_CFunction> = unsafe { library.get(entry_point.to_bytes()) }?;
    unsafe { lua.create_c_function(*ext_main) }.map_err(|e| anyhow!("Error while loading module: {}", e))
}