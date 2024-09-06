use super::*;
use crate::Result;

#[test]
fn print() -> Result<()> {
    let json = crate::utils::ui_json();

    home(&json).print();
    repos(&json).iter().for_each(|(r, b)| {
        r.print();
        b.print();
    });

    Ok(())
}
