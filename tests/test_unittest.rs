static LOCK_ALWAYS_SUCCESS: &[u8] = include_bytes!("../res/always_success");

#[test]
fn test_unittest() {
    let mut dl = ckb_ez::unittest::Resource::default();
    let mut px = ckb_ez::unittest::Pickaxer::default();

    let mut tx = ckb_ez::core::Transaction::default();
    // Create cell meta
    let cell_meta_lock =
        px.create_cell(&mut dl, 0, ckb_ez::core::Script::default(), Some(px.create_type_id()), LOCK_ALWAYS_SUCCESS);
    let cell_meta_i = px.create_cell(&mut dl, 0, px.create_script_by_data(&cell_meta_lock, &[]), None, &[]);
    // Create cell dep
    tx.raw.cell_deps.push(px.create_cell_dep(&cell_meta_lock));
    // Create input
    tx.raw.inputs.push(px.create_cell_input(&cell_meta_i));
    // Create output
    tx.raw.outputs.push(px.create_cell_output(0, ckb_ez::core::Script::default(), None));
    // Create output data
    tx.raw.outputs_data.push(vec![]);

    let verifier = ckb_ez::unittest::Verifier::default();
    verifier.verify(&tx, &dl).unwrap();
}
