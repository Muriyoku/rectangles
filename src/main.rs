// this attribute allows to print it using 
// println! macro with {:?}.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of rectangle is {} square pixels",
        area(&rect1)
    );

    // {:?} allows to print values for debugging
    // questions. By default, the macro 
    // "println!" uses a trait "Display",
    // intented for end-users.
    // So, the {:?} allows to show "items"
    // for debugger questions. 

    // the {:#?} show break lines for a better
    // view of object. 

    // Go to the struct Rectangle for more 
    // information. 
    println!("Rect1 is {rect1:#?}");
    // another way to print values for debug
    // questions. Read more about in on docs 
    // for ownership questions. 
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}


