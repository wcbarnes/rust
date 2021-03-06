fn main() {
    println!("Hello, world!");
    formatted_print();
    debug();
}

fn formatted_print() {
    /* Adding a variable to a println */
    println!("{} days!", 31);

    /* Using positional variables in println! */
    println!("{0} This is {1}. {1} This is {0}", "George", "Thai");

    /* using naming conventions for printing */
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    /* using formatting to change a variable to binary */
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    /* using width formmating to start 5 spaces indented */
    println!("{number:>width$}", number=1, width=5);

    /* adding leading zeroes */
    println!("{number:>0width$}", number=1, width=6);
}

fn debug() {
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    /* Printing with `{:?}` is similar to with `{}`. */
    println!("{:?} months in a year.", 12);

    /* Look at the quotes in the next println! */
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    /* Structure is now printable! */
    println!("Now {:?} will print!", Structure(3));

    /* not just printing seven printing the whole structure */
    println!("Now {:?} will print!", Deep(Structure(7)));
}

