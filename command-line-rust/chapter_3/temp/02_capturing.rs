fn main() {
    // Closures can captivate variables :-
    // 1. by reference: &T
    // 2. by mutable referecnce: &mut T
    // 3. by value: T

    // They preferentially capturing variables by reference and can only go lower when required.

    use std::mem;

    let color = String::from("green");

    // A closure to print `color` which immediately borrows ('&`) `color` and stores the borrow
    // and closure in the `print` variable. It will remain borrowed until `print` is used the
    // last time.

    // `println!` only requires arguments by immutable reference so it doesn't impose anything 
    // more restrictive.
    let print = || println!("`color`: {}", color);

    // `color` can be borrowed immutably again, because the closure only holds an immutable 
    // reference to `color`.
    let _reborrow = &color;
    print();    // print() borrows & of green

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count` byt `&mut count`
    // is less restritive so it takes that. Immediately borrows `count`
    let mut inc = || {
        count += 1 ;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error
    // let _reborrow = &count;
    // TODO: ^ try uncommenting this line

    // The closure no longer needs to borrow `&mut count`. Therefore, it is possible to reborrow
    // without an error
    let _count_reborrowed = &mut count;

    // A non-copy type
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type woudld copy into the 
    // closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();

    // consume()
    // TODO: ^ : Try uncommenting this line and run `rustc src/02_capturing.rs`

    //? To `move` before vertical pipes forces closure to take ownership

    // `Vec` has non-copy semantics.
    let haystack = vec![1,2,3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There'r {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error because borrow checker
    // doesn't allow re-using variable after it has been moved.

    // Removing `move` from  closure's signature will cause closure to borrow _haystack_ variable
    // immutably, hence _haystack_ is still available and uncommenting above line will not cause
    // an error
}