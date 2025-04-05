use anyhow::Result;
use libloading::{Library, Symbol};
use mlua::{lua_CFunction, Function, Lua};

pub fn load_extension_into_lua(
    lua: &Lua,
    library: Library
) -> Result<Function> {
    let ext_main: Symbol<lua_CFunction> = unsafe { library.get(b"ludo_ext_main") }?;
    unsafe { lua.create_c_function(*ext_main) }.into()
}