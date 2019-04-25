fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // s is not usable because ownership was passed and wasn't given back?

    let x = 5;

    makes_copy(x);

    // x is still usable because a uint32_t has a known size at compile time
    // and is Copy (trait). it sits on the stack and no performance penalty is
    // incurred, so a copy is made (like with a new variable let y = x;)
    // before passing to makes_copy().

    println!("main(): x + 2 = {}", x + 2);

    // ---------------------
    println!();

    let s = gives_ownership(); // `s` from above is shadowed to another var

    let s2 = gives_and_takes_ownership(s);

    println!("{}", s2);

    // ---------------------
    println!();

    // first way to get heap-allocated variables back after passing ownership:
    // return a tuple containing the passed variable and the return value.
    let s = String::from("Initial string 1");

    let (s2, s_len) = calculate_length_owned(s);

    println!("main(): \"{}\" has length {}", s2, s_len);

    // ---------------------
    println!();

    let s = String::from("Initial string 2");

    // ref's are passed by using C's "addres-of" operator (&) on both the fn
    // prototype (in the type annotation) and in the calling function.
    // this does NOT pass ownership, so it won't be drop'd when it leaves the
    // callee's scope. main() still has ownership.
    let len = calculate_length_reference(&s);

    println!("main(): \"{}\" has length {}", s, len);

    // ---------------------
    println!();

    let mut s = String::from("Initial string 3");

    change_string_mutable_ref(&mut s);

    println!("main(): s = {}", s);

    // ---------------------
    println!();

    slices();
}

fn takes_ownership(s: String) {
    println!("own():  s = {}", s);
    // s drops out of scope and heap alloc is freed automatically.
}

fn makes_copy(x: u32) {
    println!("copy(): x + 1 = {}", x + 1);
    // this function's copy of x goes out of scope (popped off the stack).
    // the variable that was passed is not affected.
}

fn gives_ownership() -> String {
    // 'creates' a variable and returns that variable, passing ownership of it
    // to the calling function
    String::from("gives(): giving ownership to calling fn")
}

fn gives_and_takes_ownership(s: String) -> String {
    // s comes into scope here, and its ownership is passed to this function

    // return s and its ownership to the calling function
    s
}

fn calculate_length_owned(s: String) -> (String, usize) {
    // (s, s.len())  // won't work: s goes out of scope just before s.len()
    // is called :(

    let len = s.len();

    (s, len)
}

fn calculate_length_reference(s: &String) -> usize {
    // s comes into scope

    // s is a reference to some immutable String on the heap:
    //    s (ptr) -> some_String(ptr + len + capacity) -> Heap['s', 'o', ...]
    // this function does not own it (the calling function does), so we
    // shouldn't be able to modify it, either

    // s.push_str("some text"); // won't work, since s is a reference

    s.len()
    // s goes out of scope here, but since this function doesn't own it,
    // nothing happens.
}

fn change_string_mutable_ref(s: &mut String) {
    // this function accepts a mutable reference to a String on the heap.
    // even tho we don't own the data, we should have the ability to change it.

    s.push_str(", world!");
    // s goes out of scope here, but since it's a ref, it won't be drop'd.
    // the String at &s will be changed though. Note: this is a statement, not
    // and expression (not the ';') and this function returns nothing.
}

/*********************************************************/

fn slices() {
    let s1 = String::from("hello world"); // new heap pointer from data

    let word = first_word(&s1[..]); // pass a slice of a String

    println!("{}", word);

    let s2 = "hello world"; // literal; on data

    let word = first_word(&s2[..]);

    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &el) in bytes.iter().enumerate() {
        if el == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
