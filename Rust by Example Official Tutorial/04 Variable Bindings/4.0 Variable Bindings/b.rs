fn main() {
    let an_int = 1u32;
    let a_bool = true;
    let unit = ();

    // copy 'an_int' into 'copied_int'
    let copied_int = an_int;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    // let noisy_unused_variable = 2u32;
}
