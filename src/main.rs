struct Point {
    x: i32,
    y: i32,
}

fn foo(_: i32, y: i32) {
    println!("This code only uses y parameter: {y}");
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background.");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as background color.");
        } else {
            println!("Using orange as background color.");
        }
    } else {
        println!("Using blue as the background color.");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        1..=5 => println!("One through five"),
        _ => println!("something else"),
    }

    let p = Point {x: 0, y: 7};
    let Point { x: a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    match p {
        Point {x, ..} => println!("x is {x}"),
    }

    foo(3, 4);

    // Underscore at start will prevent warnings from compiler for unused variables.
    let _unused_var = 3;

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
}
