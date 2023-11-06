use lazy_static;
use wasmedge_sdk::{config::ConfigBuilder, params, NeverType, VmBuilder, WasmVal,Vm};
use wasmedge_types::wat2wasm;
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
        let config = ConfigBuilder::default().build().unwrap();
        let v = VmBuilder::new().with_config(config).build().unwrap().register_module_from_bytes("wasm-lib",wasm_bytes).unwrap();
        v
    };
}
pub fn add_two(i:i32) -> Result<i32, Box<dyn std::error::Error>> {
    let res = VM.run_func(Some("wasm-lib"), "addTwo", params!(i,10))?;
    Ok(res[0].to_i32())
}