pub mod common_programming_concepts {
    pub fn variables() {
        print!("\n\n --------------------------------- Variables and Mutability in Rust ---------------------------------\n\n");

        //variable are immutable by default means there values can't be changed after they are initialized
        let _x = "abc";
        // x= "def"; //this will give error
        /* if you uncomment the above statement it will give you error like you can't assign
        a new value to x as it is immutable by default */

        // In order to make the variable we have to use the "mut" keyword with the data type
        let mut y = "abc";
        print!("current value of y is = {} \n Now changing the value of y to def", y);
        y = "def";
        print!("\n new value of y is = {}", y);

        // Constants are always immutable they are declared using the "const" keyword
        const VALUE_OF_G: f32 = 9.8;
        print!("\n\nThe value of gravity on earth is always = {}", VALUE_OF_G);
    }

    pub fn data_types() {
        print!("\n\n --------------------------------- Data Types in Rust ---------------------------------\n\n");

        // Rust is a statically typed language, which means that it must know the types of all variables at compile time
        // There are two types of data types in rust
        // 1. Scalar Types
        // 2. Compound Types

        // Scalar Types
        // 1. Integer Types
        // 2. Floating-Point Types
        // 3. Boolean Types
        // 4. Character Types

        // Integer Types
        // Length	Signed	Unsigned
        // 8-bit	i8	    u8
        // 16-bit	i16	    u16
        // 32-bit	i32	    u32
        // 64-bit	i64	    u64
        // 128-bit	i128	u128
        // arch	isize	usize
        let a = 5; // i32
        println!("The value of a is = {}", a);

        // Floating-Point Types
        // f32
        // f64
        let x = 2.0; // f64
        println!("The value of x is = {}", x);

        // Boolean Types
        // bool

        // Character Types
        // char
        let c = 'z';
        println!("The value of c is = {}", c);

        // Compound Types
        // 1. Tuple
        // 2. Array

        // Tuple
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        print!("The value of tuple is = {:?}", tup);

        // Array
        let arr = [1, 2, 3, 4, 5];
        print!("\nThe value of array is = {:?}", arr);
    }

    pub fn functions() {
        print!("\n\n --------------------------------- Functions in Rust ---------------------------------\n\n");

        // Functions are declared using the "fn" keyword
        // Functions can have parameters and return values
        // The return value is specified after an arrow ->, and the return value must be an expression
        // The return value of the function is the last expression in the function body
        // If you don't want to return anything from the function you can use the "()" as the return type

        // Function with parameters
        fn add(x: i32, y: i32) -> i32 {
            x + y
        }
        let sum = add(5, 10);
        print!("The sum of 5 and 10 is = {}", sum);

        // Function with return type as ()
        fn print_hello() {
            print!("\nHello from the function");
        }
        print_hello();
    }

    pub fn control_flow() {
        print!("\n\n --------------------------------- Control Flow in Rust ---------------------------------\n\n");

        // 1. If-Else
        let number = 3;
        if number < 5 {
            print!("The number is less than 5");
        } else {
            print!("The number is greater than 5");
        }

        // 2. Loop
        let mut counter = 0;
        loop {
            counter += 1;
            if counter == 10 {
                break;
            }
        }
        print!("\nCounter value is = {}", counter);

        // 3. While
        let mut number = 3;
        while number != 0 {
            print!("\n{}", number);
            number -= 1;
        }

        // 4. For
        let arr = [10, 20, 30, 40, 50];
        for element in arr.iter() {
            print!("\n{}", element);
        }
    }

    pub fn print_even_numbers(length: i32) {
        let mut length = length;
        print!("\n\n --------------------------------- Print Even Numbers ---------------------------------\n\n");
        loop {
            if length % 2 == 0 {
                print!("{} ", length);
            }
            if length == 0 {
                break;
            }
            length -= 1;
        }
    }

    // return value from loop example
    pub fn return_value_from_loop() -> i32 {
        let mut counter = 0;
        loop {
            counter += 1;
            if counter == 10 {
                break counter;
            }
        }
    }

    // loop labels example
    pub fn loop_labels() {
        print!("\n\n --------------------------------- Loop Labels in Rust ---------------------------------\n\n");
        'outer: for x in 0..10 {
            'inner: for y in 0..10 {
                if x % 2 == 0 {
                    continue 'outer;
                }
                if y % 2 == 0 {
                    continue 'inner;
                }
                print!("x = {}, y = {}\n", x, y);
            }
        }
    }

    pub fn while_loop() {
        print!("\n\n --------------------------------- While Loop in Rust ---------------------------------\n\n");
        let mut number = 3;
        while number != 0 {
            print!("\n{}", number);
            number -= 1;
        }
    }

    pub fn for_loop() {
        print!("\n\n --------------------------------- For Loop in Rust ---------------------------------\n\n");
        let arr = [10, 20, 30, 40, 50];
        for element in arr.iter() {
            print!("\n{}", element);
        }
    }
}