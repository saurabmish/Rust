fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    //x = 6; will produce an error because variables are mutable by default

    /*
    When a variable is immutable, value once bound to a name can't be chaged.
    Variables are immutable by default because it can lead to bugs, and the
    cause of this kind of bug can be difficult to track down. This makes the
    code easier to reason through.
    */

    /*
    But mutability can be very useful. The mut keyword conveys intent to code
    readers by indicating that other parts of the code will be changing this
    variable’s value.
    Sometimes, mutable variables make the code more convenient to write.
    There are multiple trade-offs to consider in addition to the prevention of
    bugs. For example, in cases where you’re using large data structures,
    mutating an instance in place may be faster than copying and returning
    newly allocated instances. With smaller data structures, creating new
    instances and writing in a more functional programming style may be easier
    to think through, so lower performance might be a worthwhile penalty for
    gaining that clarity.
    */

    let mut x = 7;
    println!("The value of x is {}", x);
    x = 8;
    println!("The value of x is {}", x);
}
