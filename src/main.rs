mod single_threaded;

use single_threaded::{
    smart_pointer_box_concepts, 
    smart_pointer_deref_trait_concepts, 
    smart_pointer_drop_trait_concepts, 
    smart_pointer_rc_concepts,
    smart_pointer_cell_concepts,
    smart_pointer_refcell_concepts,
    smart_pointer_rc_refcell_concepts
};

fn main() {
    // Box Smart Pointer
    smart_pointer_box_concepts::normal_reference();
    smart_pointer_box_concepts::box_reference();
    // Deref trait for Smart Pointer
    smart_pointer_deref_trait_concepts::custom_box_reference();
    smart_pointer_deref_trait_concepts::deref_coercion();
    // Drop trait for Smart Pointer
    smart_pointer_drop_trait_concepts::drop_resource_automatically();
    smart_pointer_drop_trait_concepts::drop_resource_manually();
    // Reference count for Smart Pointer
    smart_pointer_rc_concepts::rc_list_immutable();
    smart_pointer_rc_concepts::rc_string_immutable();
    // Interior Mutability Pattern - Cell<T>
    smart_pointer_cell_concepts::mutate_var_without_mut_keyword();
    smart_pointer_cell_concepts::mutate_struct_var_without_mut_keyword();
    // Interior Mutability Pattern - RefCell<T>
    smart_pointer_refcell_concepts::mutate_var_without_mut_keyword_1();
    smart_pointer_refcell_concepts::mutate_var_without_mut_keyword_2();
    smart_pointer_refcell_concepts::mutate_struct_var_without_mut_keyword_1();
    smart_pointer_refcell_concepts::mutate_struct_var_without_mut_keyword_2();
    // Interior Mutability Pattern - Rc<RefCell<T>>
    smart_pointer_rc_refcell_concepts::mutate_var_without_mut_keyword_1();
}