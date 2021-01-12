
fn run() {
    let i1: u32 = 1;
    let i2: u32 = 2;
    println!("biggest: {}", biggest(&i1, &i2));
}


fn biggest<'a, 'b>(x: &'a u32, y: &'b u32) -> &u32 {
    if x > y {
        x
    } else {
        y
    }
}






































































// fn biggest<'a 'a>(x: &'a u32, y: &'a u32) -> &'a u32 {
//     if x > y {
//         x
//     } else {
//         y
//     }
// }

// fn biggest<'a, 'b: 'a>(x: &'a u32, y: &'b u32) -> &'a u32 {
//     if x > y {
//         x
//     } else {
//         y
//     }
// }
