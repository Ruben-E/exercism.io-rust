use std::ops::Add;

pub fn raindrops(n: u32) -> String {
    let is_factor = |x| n % x == 0;

//    Not really Scalable
//
//    return match (dev(n, 3), dev(n, 5), dev(n, 7)) {
//        (true, true, true) => format!("PlingPlangPlong"),
//        (true, true, _) => format!("PlingPlang"),
//        (_, true, true) => format!("PlangPlong"),
//        (true, _, true) => format!("PlingPlong"),
//        (true, _, _) => format!("Pling"),
//        (_, true, _) => format!("Plang"),
//        (_, _, true) => format!("Plong"),
//        _ => format!("{}", n)
//    }


    let mut result = String::new();

    if is_factor(3) { result.push_str("Pling"); }
    if is_factor(5) { result.push_str("Plang"); }
    if is_factor(7) { result.push_str("Plong"); }

    if result.is_empty() { result.push_str(&format!("{}", n)) }

    return result;
}
