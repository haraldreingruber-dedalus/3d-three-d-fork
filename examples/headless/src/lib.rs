use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue>
{
    console_log::init_with_level(log::Level::Debug).unwrap();

    use log::info;
    info!("This example is not supported on the web!");

    Ok(())
}
