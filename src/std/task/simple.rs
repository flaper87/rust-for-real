use std::task;

fn main() {
    //! This function shows 
    //! simple spawn.

    do task::spawn {
        // This code will be
        // scheduled by the runtime.
        assert!(1 == 1);
        println("1 is indeed equal to 1");
    }

    println("Simple spawn example!");
}
