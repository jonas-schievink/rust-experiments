#![feature(trace_macros)]
#![feature(box_syntax)]
#![feature(log_syntax)]

use execution_engine::{
  self, dval, errors, eval, expr::*, ivec, runtime,
};

fn main() -> Result<(), errors::Error> {
  let tlid = runtime::TLID::TLID(7);
  let program = elet(
                     "range",
                     esfn(
    "Int",
    "range",
    0,
    ivec![eint(1), eint(100),],
  ),
                     esfn(
    "List",
    "map",
    0,
    ivec![(evar("range")),

              elambda(ivec!["i"],
               eif(
                 ebinop(
                 ebinop(
                   evar("i"),"Int",
                   "%", 0,
                   eint(15)
                  ), "Int", "==", 0, eint(0)), estr("fizzbuzz"),

                eif(
                 ebinop(
                 ebinop(
                   evar("i"),
                   "Int",
                   "%",0,
                   eint(5)
                  ), "Int", "==", 0, eint(0)), estr("buzz"),

                eif(
                 ebinop(
                 ebinop(
                   evar("i"),
                   "Int",
                   "%", 0,
                   eint(3)
                  ), "Int", "==", 0, eint(0)), estr("fizz"), esfn("Int", "toString", 0, ivec![evar("i")])   )


                )

                ))  ],
  ),
  );

  let state =
    eval::ExecState { caller: runtime::Caller::Toplevel(tlid), };
  let result = eval::run(&state, program);
  match &*result {
    dval::Dval_::DSpecial(dval::Special::Error(_, err)) => {
      use std::io::Write;
      let stderr = &mut ::std::io::stderr();
      let errmsg = "Error writing to stderr";

      writeln!(stderr, "error: {}", err).expect(errmsg);
      ::std::process::exit(1);
    }
    _ => {
      println!("{:?}", result);
      Ok(())
    }
  }
}
