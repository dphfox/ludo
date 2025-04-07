use std::ffi::c_int;
use mlua::{lua_State, Lua};
use mlua::prelude::{LuaResult, LuaTable};

fn do_addition(
    _lua: &Lua,
    (a, b): (i64, i64)
) -> LuaResult<i64> {
    Ok(a + b)
}

fn entry_point(
    lua: &Lua
) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("do_addition", lua.create_function(do_addition)?)?;
    Ok(exports)
}

// Workaround: lua_module macro reimplemented for luau
pub unsafe extern "C-unwind" fn luaopen_native(
    state: *mut lua_State
) -> c_int {
    Lua::entrypoint1(state, entry_point)
}