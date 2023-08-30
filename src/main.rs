use std::collections::HashMap;

fn main() {
    for n in 1..100{
        if n % 15 == 0 {
            println!("fizzbuzz")
        }
        else if n % 5 == 0 {
             println!("buzz")
        }
        else if n % 3 == 0 {
            println!("fizz")
        }
        else{
            let str = n.to_string();
            println!("{}", str)
        }
    }
    let vec = vec![1..100];
    fn somefunc<'life>(i: i32) -> &'life str {
        let mut ret = "";
        if (i % 3 == 0) {ret = "fizz"}
        if (i % 5 == 0) {ret = "buzz"}
        if (ret == ""){ ret = &i.to_string()}
        return ret;
    }
    let results = vec.iter().map(|x| somefunc).collect();
    println!(results)
}

