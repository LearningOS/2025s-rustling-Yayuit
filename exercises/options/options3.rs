// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref a) => println!("Co-ordinates are {},{} ", a.x, a.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
