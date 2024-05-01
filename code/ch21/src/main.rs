use ch21::{
    field_access_demo, new_drink_demo, newtype_pattern_demo, order_super_meal, struct_update_demo,
    tuple_struct_demo, tuple_struct_demo_2, type_alias_demo, unit_struct_demo_1, unit_struct_demo_2,
};

fn main() {
    new_drink_demo();
    order_super_meal();
    struct_update_demo();
    field_access_demo();
    tuple_struct_demo();
    tuple_struct_demo_2();
    type_alias_demo();
    newtype_pattern_demo();
    unit_struct_demo_1();
    unit_struct_demo_2();
}
