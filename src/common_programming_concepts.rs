pub mod common_programming_concepts {
    print!("\n\n --------------------------------- Variables and Mutability in Rust ---------------------------------\n\n");
    pub fn variables() {
        print!("Variables and Mutability in Rust\n\n");

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
}