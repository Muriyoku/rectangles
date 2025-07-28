// this attribute allows to print it using 
// println! macro with {:?}.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// a struct can have multiples implementation at same time. 
// implementations allows to add associated functions and 
// methods. Appearentaly, traits and some other things. 

impl Rectangle {
    // self works as "this" in JavaScritt. 
    // it's a shorthand to "self: &Self".
    // Self is an alias for the type that the 
    // impl is for. 

    // the borrow is optional, and you can pass
    // "&mut" references. But, in this context
    // it's better to use &self

    // the method name could be the same than 
    // one field name. For example: witdh.

    // Rust knows that if there is a parenthesis
    // is the method, otherwise, it's the field.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rec: &Rectangle) -> bool {
        return self.width > other_rec.width && self.height > other_rec.height
    }

    // this is an associated function. An associated function is a function associated to a "impl"
    // but it does not use a "self" as its parameter. 

    // According with the docs, the associated methods are often used to return a new instance 
    // of current implementation. For example, the self below is a new instance for Rectangle. 
    fn square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };

    // an associated function is called using "::". It's awesome, for to diferentiate 
    // methods from associated functions. 
    let square: Rectangle = Rectangle::square(30);

    println!(
        "The area of rectangle is {} square pixels",
        rect1.area()
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
    dbg!(&square);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
