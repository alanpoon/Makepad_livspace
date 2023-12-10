use lazy_static;
use std::{
    collections::HashMap,
    io::{self,Read, Write},
};
use std::sync::mpsc::Receiver;
use bytes::{Buf, BufMut};
use wasmedge_sdk::{
    r#async::{
        vm::Vm,
        wasi::{
            async_wasi::{
                snapshots::env::{
                    vfs::{WasiStdin, WasiStdout},
                    VFD,
                },
                WasiCtx,
            },
            
            AsyncWasiModule,
        },
    },
    plugin::{GraphEncoding,PluginManager,ExecutionTarget,NNPreload},
    // dock::{Param, VmDock},
    Module, Store, AsInstance,
};
use makepad_widgets::ToUISender;
pub const URL: &'static str  = "http://10.0.2.2:8000/wasmedge-app.wasm";
pub const MODEL_URL: &'static str  = "http://192.168.1.216:8000/tinyllama-1.1b-chat-v0.3.Q5_K_M.gguf";

struct SenderWriter {
    sender: ToUISender<String>,
}
impl Write for SenderWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let message = String::from_utf8_lossy(buf).into_owned();
        match self.sender.send(message){
            Ok(_)=>{
                Ok(buf.len())
            }
            _=>{
                Ok(0)
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
struct ReceiverReader{
    receiver: Receiver<String>
}
impl Read for ReceiverReader {
    fn read(&mut self, buf: &mut[u8]) -> io::Result<usize> {
        // match self.receiver.recv(){
        //     Ok(d)=>{
        //         buf.writer().write(d.as_bytes())
              
        //     }
        //     _=>{
        //         Ok(0)
        //     }
        // }
        let message = self.receiver.recv().map_err(|_| io::ErrorKind::Other)?;

        // Copy the message into the buffer
        let bytes_written = std::cmp::min(message.len(), buf.len());
        buf[..bytes_written].copy_from_slice(&message.as_bytes()[..bytes_written]);

        Ok(bytes_written)
    }
}
use std::str::FromStr;
pub async fn zzz(sender:ToUISender<String>,receiver: Receiver<String>) -> Result<String, Box<dyn std::error::Error>> {
    let wasm_file = include_bytes!("../wasm_test_server/wasm-app2.wasm");
    //let wasm_file = include_bytes!("../wasm_test_server/llama-chat.wasm");
    let mut wasi_ctx = WasiCtx::new();
    wasi_ctx.push_preopen("/data/user/0/dev.makepad.makepad_livspace/cache".into(),".".into());
    //let wasm_stdout_file = std::fs::File::create("/data/user/0/dev.makepad.makepad_livspace/cache/wasm_stdout.txt")?;
    if let Ok(wasi_stdin) = wasi_ctx.get_mut_vfd(0) {
        let virtual_stdin: Box<dyn Read + Send> = Box::new(ReceiverReader{receiver:receiver});
        *wasi_stdin = VFD::Inode(WasiStdin::from(virtual_stdin).into());
    }

    if let Ok(wasi_stdout) = wasi_ctx.get_mut_vfd(1) {
       let virtual_stdout: Box<dyn Write + Send> = Box::new(SenderWriter{sender:sender});
        *wasi_stdout = VFD::Inode(WasiStdout::from(virtual_stdout).into());
    }

    let mut wasi_module = AsyncWasiModule::create_from_wasi_context(wasi_ctx).unwrap();
    let module = Module::from_bytes(None, wasm_file)?;

    let mut instance_map = HashMap::new();
    instance_map.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    let p = PluginManager::load(None)?;
    
    let wasi_nn = PluginManager::nn_preload(vec![NNPreload::from_str("default:GGML:AUTO:tinyllama-1.1b-chat-v0.3.Q5_K_M.gguf")?]);
    // let mut wasi_nn = PluginManager::load_plugin_wasi_nn()?;
    
    // instance_map.insert(wasi_nn.name().unwrap(), &mut wasi_nn.into());
    let store = Store::new(None, instance_map)?;
    // create a Vm
    let mut vm = Vm::new(store);
    
    vm.register_module(None, module)?;
    let _ = vm
        .run_func(None, "_start", [])
        .await
        .expect("failed to run func from file");
    // run the wasm function from a specified wasm file

    println!("exit_code: {}", wasi_module.exit_code());
    // let contents = std::fs::read_to_string("/data/user/0/dev.makepad.makepad_livspace/cache/wasm_stdout.txt")
    // .expect("Should have been able to read the file");
    Ok(format!("exit_code:{}",wasi_module.exit_code()))
    // Ok(contents)
}
use std::fs;
use std::path::Path;
pub fn fetch_if_noexist(url:String,cache_dir:String)->  bool{
    let mut arr:Vec<&str> = url.split("/").collect();
    if let Some(file) = arr.pop(){
        match fs::metadata(Path::new(&cache_dir).join(file)) {
            Ok(_) => println!("File exists!"),
            Err(_) => {
                return true
            },
        }
    }
    false
}