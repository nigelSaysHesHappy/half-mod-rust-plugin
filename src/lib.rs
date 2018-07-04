#[macro_use]
extern crate cpp;

cpp! {{
    #include <halfmod.h>
    
    int cmd_rusty(hmHandle& handle, const hmPlayer& client, std::string args[], int argc);
    extern "C" {
        void rust_rusty(const char* client, size_t length);
        
        int onPluginStart(hmHandle& handle, hmGlobal* global) {
            recallGlobal(global);
            handle.pluginInfo(
                "Rust Plugin Demo",
                "akr feat. nigel",
                "Calling HalfMod from Rust code!",
                "v0.1.3",
                "https://github.com/akars/half-mod-rust-plugin"
            );
            handle.regConsoleCmd("hm_rusty",&cmd_rusty,"Sorry, I'm a bit rusty :(");
            return 0;
        }
    }
    
    int cmd_rusty(hmHandle& handle, const hmPlayer& client, std::string args[], int argc)
    {
        rust_rusty(client.name.data(),client.name.size());
        return 0;
    }
}}

use std::str;
use std::slice;

#[no_mangle]
pub unsafe extern "C" fn rust_rusty(data: *const u8, length: usize) -> i32 {
    match str::from_utf8(slice::from_raw_parts(data, length)) {
        Ok(string) => {
            let rust_string = format!("say Hello {} I'm Rusty!", string);
            let data = &rust_string.as_bytes()[0];
            let len = rust_string.len();
            cpp!([data as "const char*", len as "size_t"] {
                std::string cpp_string(data, len);
                hmSendRaw(cpp_string);
            });
            0
        },
        Err(_invalid_utf8) => 1 /* ERROR */,
    }
}
