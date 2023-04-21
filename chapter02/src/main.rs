mod arithmetic;
mod assign;
mod bit;
mod comparison;
mod logical;

fn main() {
    // arithmetic::symbol();
    // arithmetic::methods(20, 10);
    // arithmetic::overflow();
    // arithmetic::ignore_overflow();
    // arithmetic::check_overflow();
    // arithmetic::check_bool_overflow();
    // arithmetic::return_max_overflow()

    // assign::assign_value(10, 12_567)
    // assign::compound_assign(13, 3)
    // assign::compound_assign_method(13, 3)

    // comparison::symbol(10, 6);
    // comparison::methods(10, 6);

    // logical::use_symbol(10, 6);

    // bit::use_symbol(10, 6)
    // bit::use_format_specifier();
    // bit::use_method(10, 6);
    // bit::compound_assign(10, 6);
    bit::compound_assign_method(10, 6);
}
