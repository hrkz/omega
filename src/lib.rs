/*
 Omega v0.0.1
 [lib]
 Copyright (c) 2020-present, Hugo (hrkz) Frezat
*/

/// @see book
/// http://cycle-research.org
use cycle::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn interpret(
  //.
  line: u32,
  stmt: &str,
) -> String {
  match lang::parse(&stmt.trim_end()) {
    Ok(expr) => {
      let expr = expr.trivial();
      expr.map_or_else(|err| format!("{} (undefined)", err), |expr| expr.to_string())
    }

    Err(err) => {
      format!(
        "[error: {}] {}",
        line, //.
        err
      )
    }
  }
}
