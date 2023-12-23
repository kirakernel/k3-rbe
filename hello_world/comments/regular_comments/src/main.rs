// Regular comments which are ignored by the compiler:
//    - // Line comments which go to the end of the line.
//    - /* Block comments which go to the closing delimiter. */

fn main() {
    // This is an example of a line comment.
    // There are two slashes at the beginning of the line.
    // And nothing written after these will be read by the compiler.

    // println!("Hello, world!");
    
    // Run it. See? Now try deleting the two slashes, and run it again.

    /* 
     * This is another type of comment, a block comment. In general, 
     * line comments are the recommended style. But block comments 
     * are extremely useful for temporarily disabling chuncks of code.
     * /* Block comments can be /* nested. */ */ so it takes only a few
     * keystrokes to comment out everything in this man() function.
     * /*/*/* Try it yourself */*/*/ 
     */
     
    /*
    Note: The previous column of `*`was entirely for style. There's 
    no actual need for it.
    */

    // You can manipulate expressions more easily with block comments
    // than line comments. Try deleting the comments delimiters
    // to chnage the result:
    // let x = 5 + /* 90+*/ 5;
    let x = 5 + 90 + 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
