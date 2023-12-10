use std::fmt::format;

use lazy_static;
use wasmedge_sdk::{config::ConfigBuilder, params, NeverType, VmBuilder, WasmVal,Vm};
use wasmedge_sdk::{
    config::{CommonConfigOptions, HostRegistrationConfigOptions},
    dock::{Param, VmDock},
    plugin::{GraphEncoding,PluginManager,ExecutionTarget,NNPreload},
    Module,
};
use wasmedge_types::wat2wasm;
//pub const URL: &'static str  = "http://192.168.1.216:8000/wasm_test.wasm";
//pub const URL: &'static str  = "http://192.168.1.216:8000/wasm_lib.wasm";
//pub const URL: &'static str  = "http://192.168.1.216:8000/wasmedge-tcp-ggml-llama-interactive.wasm";
pub const URL: &'static str  = "http://192.168.1.216:8000/wasmedge-ws.wasm";
pub const MODEL_URL: &'static str = "http://192.168.1.216:8000/llama-2-7b-chat-q5_k_m.gguf"; 
const WAT: &'static [u8;121] = br#"
(module
    (func (export "addTwo") (param i32 i32) (result i32)
      local.get 0
      local.get 1
      i32.add))
  
"#;
lazy_static!{
    static ref VM :Vm = {
        let wasm_bytes = wat2wasm(
            WAT
        ).unwrap();
        let mut config = ConfigBuilder::default().build().unwrap();
       
        let v = VmBuilder::new().with_config(config).build().unwrap().register_module_from_bytes("wasm-lib",wasm_bytes).unwrap();
        v
    };
    pub static ref VM2 :Vm = {
        let wasm_bytes = wat2wasm(
            WAT
        ).unwrap();
        let mut config = ConfigBuilder::default().build().unwrap();
        
        let mut v = VmBuilder::new().with_config(config).build().unwrap().register_module_from_bytes("wasm-lib",wasm_bytes).unwrap();
        let args = vec!["arg1", "arg2"];
        let envs = vec!["ENV1=VAL1", "ENV2=VAL2", "ENV3=VAL3"];
        // the preopened directory is the current directory. You have to guarantee
        // the write permission if you wanna write something in this directory.
        let preopens = vec![(".:./data/user/0/dev.makepad.makepad_livspace/cache")];
        let wasi_module = v.wasi_module_mut().expect("Not found wasi module");
        wasi_module.initialize(Some(args), Some(envs), Some(preopens));
        v
    };
}
pub fn add_two(i:i32) -> Result<i32, Box<dyn std::error::Error>> {
    let res = VM.run_func(Some("wasm-lib"), "addTwo", params!(i,10))?;
    Ok(res[0].to_i32())
}
pub fn read_foo_text(wasm_bytes:&Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    let module = Module::from_bytes(None, wasm_bytes)?;    
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;
    let mut vm = VmBuilder::new()
        .with_config(config)
        .build()?
        .register_module(None, module)?;
    let args = vec!["arg1", "arg2"];
    let envs = vec!["ENV1=VAL1", "ENV2=VAL2", "ENV3=VAL3"];
    let preopens = vec![("/data/user/0/dev.makepad.makepad_livspace/cache")];
    let wasi_module = vm.wasi_module_mut().expect("Not found wasi module");
    wasi_module.initialize(Some(args), Some(envs), Some(preopens));
    let extern_instance = vm.named_module("extern")?;
    let vm: VmDock = VmDock::new(vm);
    
    match vm.run_func("read_foo_text", params!())?{
        Ok(mut res)=>{
            Ok(*res.pop().unwrap().downcast::<String>().unwrap())
            //Ok(String::from("nillz1"))
        },
        _=>{Ok(String::from("nillz2"))}
    }
}

use std::{
    fs::File,
    fs,
    io::prelude::*,
    mem,
    ptr,
    ffi::CStr,
    path::Path
};

pub fn tcp_test(wasm_bytes:&Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let module = Module::from_bytes(None, wasm_bytes)?;    

    Ok(())
}

use std::str::FromStr;
pub fn infer(wasm_bytes:&Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    // parse arguments
    let module = Module::from_bytes(None, wasm_bytes)?;    
    let dir_mapping ="/data/user/0/dev.makepad.makepad_livspace/cache:.";
    println!("load plugin");
    PluginManager::load(None)?;
    PluginManager::nn_preload(vec![NNPreload::from_str("default:GGML:AUTO:llama-2-7b-chat-q5_k_m.gguf")?]);
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;
    assert!(config.wasi_enabled());
    // create a Vm
    let mut vm = VmBuilder::new()
        .with_config(config)
        .with_plugin_wasi_nn()
        .build()?
        .register_module(Some("extern"), module)?;

    // init wasi module
    vm.wasi_module_mut()
        // .ok_or("Not found wasi module")?
        .expect("Not found wasi module")
        .initialize(
            Some(vec!["default","default"]),
            None,
            Some(vec![dir_mapping]),
        );

    vm.run_func(Some("extern"), "_start", params!())?;

    Ok(())
}

pub fn wasm_ws(wasm_bytes:&Vec<u8>) -> Result<(), Box<dyn std::error::Error>>{
    let module = Module::from_bytes(None, wasm_bytes)?;    
    PluginManager::load(None)?;
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;
    assert!(config.wasi_enabled());
    // create a Vm
    let mut vm = VmBuilder::new()
        .with_config(config)
        .with_plugin_wasi_nn()
        .build()?
        .register_module(Some("extern"), module)?;

    // init wasi module
    vm.wasi_module_mut()
        // .ok_or("Not found wasi module")?
        .expect("Not found wasi module")
        .initialize(
            Some(vec!["default","default"]),
            None,
            None,
        );
    vm.run_func(Some("extern"), "_start", params!())?;
    Ok(())
}