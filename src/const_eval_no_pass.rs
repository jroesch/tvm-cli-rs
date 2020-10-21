use std::path::Path;
use tvm::ir::{relay, module::IRModule};
use tvm::runtime::object::IsObjectRef;
use tvm::ir::expr::as_text;

macro_rules! downcast_match {
    ($id:ident; { $($t:ty => $arm:expr $(,)? )+ , else => $default:expr }) => {
        $( if let Ok($id) = $id.downcast::<$t>() { $arm } else )+
        { $default }
    }
}

pub trait ExprVisitorMut<R> {
    fn visit(&mut self, expr: relay::Expr) -> R {
        downcast_match!(expr; {
            relay::Var => {
                self.visit_var(expr)
            },
            else => {
                todo!()
            }
        })
    }

    fn visit_function(&mut self, var: relay::Function) -> R;
    fn visit_var(&mut self, var: relay::Var) -> R;
}

struct ConstantFolder;

impl ExprVisitorMut<relay::Expr> for ConstantFolder {
    fn visit_function(&mut self, func: relay::Function) -> relay::Expr {
        return func.upcast();
    }

    fn visit_var(&mut self, var: relay::Var) -> relay::Expr {
        // TODO(@jroesch): we dont process yet
        return var.upcast();
    }
}

fn save_module(module: IRModule, output_path: &Path) -> std::io::Result<()> {
    Ok(())
}

// TODO(@jroesch): why does this require 'static?
pub fn run<P1: AsRef<Path>, P2: AsRef<Path>>(input: P1, output: P2) -> anyhow::Result<()> where P1: 'static, P2: 'static {
    let input_module = IRModule::parse_file(input)?;
    for (global, func) in input_module.functions.clone() {
        let func2 = func.clone();
        let new_func: tvm::ir::function::BaseFunc = downcast_match!(func2; {
            tvm::ir::relay::Function => {
                let new_func = ConstantFolder.visit_function(func2);
                // This is a bit of an annoyance but Rust doens't have subtyping and Max and I designed a up/down cast mechanism
                // to interact with C++ ast ergnomoically with zero-copy.
                let new_func = new_func.downcast::<relay::Function>().expect("this should be a function by invariant of the pass");
                new_func.upcast()
            },
            else => {
                func.clone()
            }
        });
        panic!("Need to clean up Module API a bit to fully support rebuilding a module\n{}", as_text(new_func));
    }
    Ok(())
}
