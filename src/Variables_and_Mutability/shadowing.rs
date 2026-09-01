pub fn shadowing_var () {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

/*
    * This program first binds x to a value of 5.
    * Then, it creates a new variable x by repeating let x =,
    * taking the original  value and adding 1 so that the value of x is 6.
    * Then, within an inner scope created with the curly brackets, the third
    * let statement also shadows x and creates a new variable, multiplying the previous value by 2 to give x a value of 12.
    * When that scope is over, the inner shadowing ends and x returns to being 6.
    * When we run this program, it will output the folowing:



    * Shadowing is different from marking a variable as mut because we'll geta compile-time error if we accidently try to reassign to this variable without
    * using the let keyword. By using let, we can perform a few transformations have completed.


    * The other difference between mut and shadowing is that because we're
    * effectively creating a new variable when we use the let keyword again, we can
    * change the type of the value but reuse the same name. For example say our program
    * asks a user to show how many spaces they want between some text by inputting space characters, and
    * then we want to store that input as a number::

    ```rs
        let spaces = "   ";
        let spaces = spaces.len();
    ```

    * The first spaces variale is a string type, and the second sapces variable isa number type.
    * Shadowing thus spaces us from having to come up with different names, such as spaces_str
    and spaces_num; instead, we can reuse the simpler sapces name. However, if we try to use mut for this,
    as shown here,
    we'll get a compile-time error:

    ```rs
        let mut spaces = "   ";
        spaces = spaces.len();
    ```

    * The error says we're not allowed to mutate a variable's type:


*/

// Uncomment to see warnings and runn to see compile time error
// pub fn shadowing_var () {
//     let mut spaces = "   ";
//         spaces = spaces.len(); // Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword

//     println!("The value of spaces is: {spaces}");
// }