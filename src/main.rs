// this stub is necessary because some platforms require building
// as dll (mobile / wasm) and some require to be built as executable
// unfortunately cargo doesn't facilitate this without a main.rs stub

#[tokio::main]
async fn main(){
    makepad_livspace::app::app_main()
}
