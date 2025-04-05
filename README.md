<h1>
    <img src="./assets/ludo.svg" alt="ludo">
</h1>

The **Lu**a **Do**er. An ultra-thin runtime for Luau written in Rust, with support for dynamic native extensions.

## Overview
Ludo largely supports the same feature set as Luau out of the box, except that all scripts are run in Luau's safe environment and unsafe features like `setfenv` or `loadstring` are forcibly disabled.

However, unlike Luau, Ludo supports interacting with native libraries. Any script can be accompanied by a `.ludorc` defining a natively compiled counterpart; this native library will be exposed to the Luau script for direct use.

This architecture allows any package to bundle native code dynamically. Extending the Ludo runtime is as easy as dropping new files into your project, just like adding a Luau library.

## Security

Ludo's security system is triple layered; through a combination of *encapsulation*, *permissions* and *blessing*, Ludo helps ensure the correct use of known native code.

### Encapsulation

Scripts cannot load arbitrary native libraries at runtime. Instead, they must be defined in a `.ludorc` statically. The native library must exist under that `.ludorc`'s parent directory.

When a native library is declared, it is exclusively exposed to the scripts in the directory via a `native` global. No other script outside of the directory can access this global by default.

Common loopholes like `getfenv()` are disabled by Ludo to prevent this global from being extracted from libraries.

This encapsulation helps ensure that a native library is only used by the package that introduces it.

### Permissions

Scripts must have permission to define a native counterpart. If a `.ludorc` declares a native library without permission, the script will fail to load.

By default, only the `.ludorc` in the main script's directory (directly run by Ludo) has native permission. Native libraries declared in other `.ludorc` files do not inherit this default permission.

Scripts with native permission can manually grant native permission to other scripts on a case-by-case basis via their own `.ludorc` files.

These permissions help codify where native code is *expected* to arise in a project.

### Blessing

To protect against modified or unknown binaries, the user must acknowledge any binary that is being run for the first time. This process is called "blessing".

When the user blesses a native binary, a hash of that library's contents will be stored in the local user's configuration. When Ludo attempts to load the binary at runtime, the hash will be recalculated and compared to the stored binary; the two must match before proceeding.

If a native library is found which doesn't have a hash yet, the user will be prompted to bless it. If the hashes don't match, the user will be alerted to the discrepancy.

Blessing ensures that the user is *aware* of the specific binaries being run and when they're modified. 

## License

Licensed the same way as all of my open source projects: BSD 3-Clause + Security Disclaimer.

As with all other projects, you accept responsibility for choosing and using this project.

See [LICENSE](./LICENSE) or [the license summary](https://github.com/dphfox/licence) for details.