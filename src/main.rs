fn main() {
    // one();
    // two();
    three();
}

// Simple pattern matching
fn one() {
    let x = 1;


    let result = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "anything",
    };

    println!("{}", result)
}

// Pattern matching with parameters
fn two() {
    let x = 24;

    let result = match x {
        50 => "Got 50".to_string(),
        1 | 2 => "Matched 1 or 2".to_string(),
        y @ 20..=30 => format!("Matched {}", y),
        _ => "Everything else".to_string(),
    };

    println!("at the end: result={}", result);
}

fn three() {
    let calculation = divide(10, 5);

    let result = match calculation1 {
        Some(2.0) => "Got 2".to_string(),
        Some(x @ 3.0) => format!("Got {}", x),
        Some(x @ 5.0..=10.0) => format!("Got {}!!!", x),
        Some(_) => "Got Something".to_string(),
        None => "Got Nothing :(".to_string()
    };

    println!("at the end: result={}", result);

    // Won't compile, because not all cases are covered.
    // GOOD!!!
    //
    // match calculation1 {
    //    Some(_) => println!("Got some result")
    // }
}

fn divide(numerator: i32,
          denominator: i32) -> Option<f64> {
    if denominator == 0 {
        None
    } else {
        Some((numerator as f64) / (denominator as f64))
    }
}