use cpython::{PyResult, Python, py_module_initializer, py_fn};
use num_bigint::{ToBigUint,BigUint};
use num_traits::One;
use num_integer::Integer;
use std::str::FromStr;
use std::time::Instant;

// add bindings to the generated python module
// N.B: names: "pysyr" must be the name of the `.so` or `.pyd` file
py_module_initializer!(pysyr, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "collatz", py_fn!(py, collatz_py(a: String)))?;
    m.add(py, "collatz_inc", py_fn!(py, collatz_inc_py(from: String, to: String)))?;
    Ok(())
});

fn collatz_py(_:Python, a: String) -> PyResult<(u64,u64,u64)> {
    let out = optimum_syracuse(BigUint::from_str(&a).unwrap());
    Ok(out)
}

fn collatz_inc_py(_:Python, from: String, to: String) -> PyResult<bool> {
    let one: BigUint = One::one();
    let two: BigUint = 2.to_biguint().unwrap();
    let mut from = BigUint::from_str(&from).unwrap();
    if from.is_even() {
        from += &one;
    }

    let to = BigUint::from_str(&to).unwrap();
    while from <= to {
        match incremental_syracuse(&from){
            true => {},
            false => return Ok(false),
        }
        from += &two;

    }
    Ok(true)
}

fn optimum_syracuse(n: BigUint) -> (u64, u64, u64) {
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let mut div: u64= 0;
    let mut mul: u64 = 0;
    if i.is_even() {
        let a: u64 = i.trailing_zeros().unwrap();
        i = &i >> &a;
        div += a;
    }
    if i == one {
        return (div, 0, div)
    }
    loop {
        i = (&i << 1) + &i + &one >> 1;
        mul += 1;
        div += 1;
        // the following line is worse :
        //i = &i >> &i.trailing_zeros().unwrap();
        let a: u64 = i.trailing_zeros().unwrap();
        i = &i >> &a;
        div += a;
        if i == one{
            break;
        }
    }
    //println!("{}", counter);
    let counter = &mul+&div;
    (counter, mul, div)
}

fn incremental_syracuse(n: &BigUint) -> bool{
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let min: BigUint = i.clone();
    let now = Instant::now();
    if i < (&one << 64) {
        return true;
    }
    if i.is_even() {
        let a: u64 = i.trailing_zeros().unwrap();
        i = &i >> &a;
    }
    loop {
        if now.elapsed().as_secs() > 10*60 {
            println!("Timeout for n= {min}");
        }

        i = ((&i << 1) + &i + &one) >> 1;
        let a: u64 = i.trailing_zeros().unwrap();
        //i = i >> a; is longer !
        i = &i >> &a;
        if i == one || i < min{
            break;
        }
    }
    return true;
}
