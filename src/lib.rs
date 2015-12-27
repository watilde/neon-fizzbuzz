extern crate neon;

use neon::vm::{Call, Result, JS, Module};
use neon::value::{String, Integer};
use neon::mem::Handle;

fn fizzbuzz(call: Call) -> JS<String> {
    let scope = call.scope;
    let input: u32 = try!(try!(call.arguments.require(scope, 0)).check::<Integer>());
    if input == 0 {
        Ok(input.to_string());
    }
    let result = if input % 15 == 0 {
        "FizzBuzz".to_string()
    } else if input % 5 == 0 {
        "Buzz".to_string()
    } else if input % 3 == 0 {
        "Fizz".to_string()
    } else {
        input.to_string()
    };

    Ok(result);
}

#[no_mangle]
pub fn node_main(mut module: Module) -> Result<()> {
    module.export("fizzbuzz", fizzbuzz)
}
