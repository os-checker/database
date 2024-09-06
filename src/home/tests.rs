use super::*;

#[test]
fn all() {
    let json = &crate::utils::ui_json();
    crate::print(&nodes(json));
}
