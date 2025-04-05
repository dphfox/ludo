<h1>
    <img src="./assets/ludo.svg" alt="ludo" height=64>
</h1>

The **Lu**a **Do**er. An ultra-thin runtime for Luau written in Rust, with support for dynamic native extensions.

## Overview
Ludo largely supports the same feature set as Luau out of the box, except that all scripts are run in Luau's safe environment and unsafe features like `setfenv` or `loadstring` are forcibly disabled.

However, unlike Luau, Ludo supports interacting with native libraries. Any script can be accompanied by a `.ludorc` defining a natively compiled counterpart; this native library will be exposed to the Luau script for direct use.

This architecture allows any package to bundle native code dynamically. Extending the Ludo runtime is as easy as dropping new files into your project, just like adding a Luau library.

## Permissions

Scripts must have permission to define a native counterpart. If a `.ludorc` declares a native library without permission, the script will fail to load.

By default, only the main script (directly run by Ludo) has native permission.

Scripts with native permission can manually grant native permission to other scripts on a case-by-case basis via their own `.ludorc` files.

## Blessing

To protect against modified or unknown binaries, the user must acknowledge any binary that is being run for the first time. This process is called "blessing".

When the user blesses a native binary, a hash of that library's contents will be stored in the local user's configuration. When Ludo attempts to load the binary at runtime, the hash will be recalculated and compared to the stored binary; the two must match before proceeding.

If a native library is found which doesn't have a hash yet, the user will be prompted to bless it. If the hashes don't match, the user will be alerted to the discrepancy.

Blessing a binary does not guarantee that it is innocent; you should use your better judgement before blessing any binary.

## License

Licensed the same way as all of my open source projects: BSD 3-Clause + Security Disclaimer.

As with all other projects, you accept responsibility for choosing and using this project.

See [LICENSE](./LICENSE) or [the license summary](https://github.com/dphfox/licence) for details.