use cpython::{PyResult, Python, py_module_initializer, py_fn, PyErr};
use cpython::exc::TypeError;
use num_bigint::{ToBigUint,BigUint};
use num_traits::One;
use num_integer::Integer;
use std::str::FromStr;
use std::time::Instant;
use num_format::{Locale, ToFormattedString};


// add bindings to the generated python module
// N.B: names: "pysyr" must be the name of the `.so` or `.pyd` file
py_module_initializer!(pysyr, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.\n Collatz compute the sequence for a given integer given by string and return the tuple: (total iterations, number of multiply operations, number of division operations\n collatz_pow(a,b,i) give the same results for the number a^b+i")?;
    m.add(py, "collatz", py_fn!(py, collatz_py(a: String)))?;
    m.add(py, "collatz_pow", py_fn!(py, collatz_pow_py(a: u64, exponent: u32,i:i64, verbose: bool)))?;
    m.add(py, "collatz_inc", py_fn!(py, collatz_inc_py(from: String, to: String)))?;
    m.add(py, "find_next", py_fn!(py, find_next_py(a: u64, exponent: u32)))?;
    Ok(())
});

fn crop_biguint(n: &BigUint, size: usize, force: bool) -> String {
    let mut repr = "..".to_owned();
    let two: BigUint = 2.to_biguint().unwrap();

    if !force {
        let max_pow: u32 = 250_000;
        if n > &BigUint::pow(&two,max_pow) {
            repr = "Too big... representation would take some time we don't have...".to_owned();
        }
    }
    else {
        let max_pow: u32 = 169;
        if n < &BigUint::pow(&two,max_pow) {
            let mut s = (*n).to_formatted_string(&Locale::fr);
            let pos = s.len() - size;
            if &s.len() > &size {
                s.drain(..pos);
            }
            repr.push_str(&s);
        }
        else {
            let mut s = n.to_str_radix(10);
            let pos = s.len() - size;
            match s.char_indices().nth(pos) {
                Some((pos, _)) => {
                    s.drain(..pos);
                }
                None => {}
            }
            repr.push_str(&s);
        }
    }
    repr
}

fn collatz_py(_:Python, a: String) -> PyResult<(u64,u64,u64,String)> {
    let out = optimum_syracuse(BigUint::from_str(&a).unwrap());
    Ok(out)
}

fn collatz_pow_py(py:Python, a: u64, exponent: u32,i: i64, verbose: bool) -> PyResult<(u64,u64,u64,String)> {
    let mut n:BigUint = BigUint::pow(&a.to_biguint().unwrap(), exponent);
    let abs_i = i.abs().to_biguint().unwrap();

    if num::signum(i) == -1 {
        if abs_i > n {
            return Err(PyErr::new::<TypeError, _>(py, "Not defined for negative integers"));
        }
        else {
            n -= abs_i;
        }
    }
    else {
        n += abs_i;
    }

    let now = Instant::now();
    if verbose {
        let s = crop_biguint(&n, 100, false);
        println!("\n{}",s);
    }
    let out = optimum_syracuse(n);
    if verbose {
        println!("\t\t...elapsed: {:.2?}", now.elapsed());
    }

    Ok(out)

}

fn find_next_py(_: Python, a: u64,exponent: u32) ->PyResult<usize>{
    let mut n:BigUint = BigUint::pow(&a.to_biguint().unwrap(), exponent);
    let one: BigUint = One::one();
    let two: BigUint = 2.to_biguint().unwrap();
    let mut decay: usize = 0;
    if n.is_even(){
        n += &one;
        decay += 1;
    }
    let res = optimum_syracuse(n.clone()).0;
    loop {
        n += &two;
        let tmp = optimum_syracuse(n.clone()).0;
        if tmp != res {
            break;
        }
    }
    Ok(decay)
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

//fn collatz_pow_par()

fn optimum_syracuse(n: BigUint) -> (u64, u64, u64, String) {
    let one: BigUint = One::one();
    let mut i: BigUint = n.clone();
    let mut div: u64= 0;
    let mut mul: u64 = 0;
    let mut max: BigUint = n.clone();
    if i.is_even() {
        let a: u64 = i.trailing_zeros().unwrap();
        i = &i >> &a;
        div += &a;
    }
    if i == one {
        return (div, 0, div, one.to_string())
    }
    loop {
        i = (&i << 1) + &i + &one;
        if i > max {
            max = i.clone();
        }
        i >>= 1;
        mul += 1;
        div += 1;
        let a: u64 = i.trailing_zeros().unwrap();
        i = &i >> &a;
        div += &a;
        if i == one{
            break;
        }
    }
    let counter = &mul+&div;
    //let s = crop_biguint(&max, 100, true);
    //println!("\n{}",s);
    let max_str = max.to_string();
    (counter, mul, div, max_str)
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
