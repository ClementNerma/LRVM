use crate::ids::gen_aux_id;

#[test]
fn ids() {
    assert!(gen_aux_id("test") == gen_aux_id("test"));
    assert!(gen_aux_id("\0") == gen_aux_id("\0"));

    assert!(gen_aux_id("test") != gen_aux_id("test2"));
    assert!(gen_aux_id("test") != gen_aux_id("2test"));
}
