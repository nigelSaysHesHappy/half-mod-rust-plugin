#[macro_use]
extern crate cpp;

cpp! {{
    #include <halfmod.h>

    extern "C" {
        int onPluginStart(hmHandle& handle, hmGlobal* global) {
            recallGlobal(global);
            handle.pluginInfo(
                "Rust Plugin Demo",
                "akr",
                "Calling HalfMod from Rust code!",
                "v0.1.1",
                "https://github.com/akars/half-mod-rust-plugin"
            );

            return 0;
        }

        int onWorldInit(hmHandle& hmHandle) {
            hmSendRaw("say Hello World");
            return 0;
        }
    }
}}