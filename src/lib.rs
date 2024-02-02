use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{engine::general_purpose::STANDARD, Engine};
use image::{load_from_memory, ImageOutputFormat::Png};
use std::io::Cursor;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
  log(&"Grayscale called".into());

  let base64_to_vector = STANDARD.decode(encoded_file).unwrap();
  log(&"Image decoded".into());

  let mut img = load_from_memory(&base64_to_vector).unwrap();
  log(&"Image loaded".into());

  img = img.grayscale();
  log(&"Grayscale effect applied".into());

  let mut buffer = vec![];
  img.write_to(&mut Cursor::new(&mut buffer), Png).unwrap(); // Use Cursor
  log(&"New image written".into());

  let encoded_img = STANDARD.encode(&buffer);
  let data_url = format!("data:image/png;base64,{}", encoded_img);

  data_url
}
