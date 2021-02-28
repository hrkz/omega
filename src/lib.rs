/*
 Omega v0.0.1
 [lib]
 Copyright (c) 2020-present, Hugo (hrkz) Frezat
*/

/// @see book
/// http://cycle-research.org
use cycle::*;
use lang::{Ast, Interpreter, LangError};

use wasm_bindgen::prelude::*;

static mut VM: Option<Interpreter> = None;

#[wasm_bindgen(start)]
pub fn main() {
  let vm = unsafe { VM.get_or_insert(Interpreter::new(1)) };
  Prelude::new().use_into(vm, true).unwrap_or_else(|err| eprintln!("failed to load cycle prelude: {}", err));
}

#[wasm_bindgen]
pub fn interpret(
  //.
  line: u32,
  stmt: &str,
) -> String {
  let vm = unsafe { VM.as_mut().unwrap() };
  match vm.parse(&stmt.trim_end()) {
    Ok(None) => String::new(),

    Ok(Some(expr)) => {
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

///
/// Mathematical constants >
/// oo, pi, I
///
/// Arithmetic functions >
/// +, -, *, /, ^, !, sqrt
///
/// Elementary functions >
/// sin, cos, tan, arcsin, arccos, arctan, sinh, cosh, tanh, arsinh, arcosh, artanh, log, exp
///
struct Prelude {
  arg: Expr,
}

impl Prelude {
  fn new() -> Prelude {
    Prelude {
      arg: Expr::Sym(Symbol::new("_0", Set::SR)),
    }
  }

  fn def<F>(
    //.
    &self,
    vm: &mut Interpreter,
    name: &str,
    f: F,
  ) -> Result<Option<Expr>, LangError>
  where
    F: Fn(Expr) -> Expr,
  {
    let _0 = self.arg.clone();
    vm.eval(Ast::Def(Expr::map(name, vec![_0.clone()]), f(_0)))
  }

  fn use_into(
    //.
    &self,
    vm: &mut Interpreter,
    _op: bool,
  ) -> Result<(), LangError> {
    self.def(vm, "sqrt", |e| e.sqrt())?;

    self.def(vm, "sin", |e| e.sin())?;
    self.def(vm, "cos", |e| e.cos())?;
    self.def(vm, "tan", |e| e.tan())?;

    self.def(vm, "arcsin", |e| e.arcsin())?;
    self.def(vm, "arccos", |e| e.arccos())?;
    self.def(vm, "arctan", |e| e.arctan())?;

    self.def(vm, "sinh", |e| e.sinh())?;
    self.def(vm, "cosh", |e| e.cosh())?;
    self.def(vm, "tanh", |e| e.tanh())?;

    self.def(vm, "arsinh", |e| e.arsinh())?;
    self.def(vm, "arcosh", |e| e.arcosh())?;
    self.def(vm, "artanh", |e| e.artanh())?;

    self.def(vm, "exp", |e| e.exp())?;
    self.def(vm, "log", |e| e.log())?;

    vm.eval(Ast::Rule(Expr::Sym(Symbol::new("oo", Set::SR)), Expr::Cte(Constant::Infinity(1))))?;
    vm.eval(Ast::Rule(Expr::Sym(Symbol::new("pi", Set::SR)), Expr::Cte(Constant::pi)))?;
    vm.eval(Ast::Rule(Expr::Sym(Symbol::new("I", Set::SR)), Expr::Cte(Constant::I)))?;

    Ok(())
  }
}
