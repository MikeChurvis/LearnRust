fn main() {
    // --- MUTABILITY ---

    // [NOTE] This doesn't compile; you can't change the value of
    //  an immutable variable.
    // let x = 5;

    // [NOTE] This, however, is fine; the variable is declared as
    //  mutable.
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    // --- CONSTANTS ---

    // [NOTE] Constants are immutable by definition, and require a
    //  type declaration. Constants can be declared in any
    //  scope, including global. The main difference between
    //  constants and immutable variables is that constants
    //  are evaluated at compile-time, whereas variables (immutable
    //  or not) are evaluated at run-time.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // --- SHADOWING ---

    let y = 5;
    let y = y + 1;

    // [NOTE] Redeclaring a parent-scoped variable with a new value
    //  will technically create a new variable within that inner scope.
    //  Because that new variable is in the inner scope, it will
    //  fall out of scope at the end of the block, and the variable
    //  name will once again reference the variable in the parent
    //  scope. This is called Shadowing.
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // [NOTE] This works; you're declaring a new var with a new type:
    let spaces = "   ";
    let spaces = spaces.len()
    // -> spaces == 3

    // [NOTE] This doesn't; you can't mutate an existing variable's type:
    let mut mut_spaces = "   ";
    mut_spaces = mut_spaces.len()
}
