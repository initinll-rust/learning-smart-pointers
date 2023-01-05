mod smart_pointer_box_concepts;
mod smart_pointer_deref_trait_concepts;
mod smart_pointer_drop_trait_concepts;
mod smart_pointer_rc_concepts;

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
}