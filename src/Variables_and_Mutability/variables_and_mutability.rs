pub fn var_and_mut() {
//     let x = 5;
//     println!("The value of x is: {}", x);
//    /*
//          * we will get error like:
//          * error[E0384]: cannot assign twice to immutable variable `x`
//          * help: consider making this binding mutable
//          * 2 |    => let mut x = 5;
//          * For more information about this error, try `rustc --explain E0384`.
//          * error: could not compile `learning_rust` (bin "learning_rust") due to 1 previous error
//          *
//          * Fix:
//          * Although variables are immutable by default, you can make them mutable by adding mut in front of the variable name
   
//     */
//     x = 6;
//     println!("The value of x is: {}", x);


// The Fix 
    // Variables: let, const
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
