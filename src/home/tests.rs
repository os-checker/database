use super::*;

#[test]
fn all() {
    let json = &crate::utils::ui_json();
    crate::print(&all_targets(json));
}

#[test]
fn by_target() {
    let json = &crate::utils::ui_json();
    crate::print(&split_by_target(json));
}
