#[test]
fn test_unittest() {
    let exit_0 = std::fs::read("res/exit_0").unwrap();
    let mut dl = ckbez::unittest::Resource::default();
    let mut px = ckbez::unittest::Pickaxer::default();

    let mut tx = ckbez::core::Transaction::default();
    // Create cell meta
    let cell_meta_lock = px.create_cell(&mut dl, 0, ckbez::core::Script::default(), Some(px.create_type_id()), &exit_0);
    let cell_meta_i = px.create_cell(&mut dl, 0, px.create_script_by_data(&cell_meta_lock, &[]), None, &[]);
    // Create cell dep
    tx.raw.cell_deps.push(px.create_cell_dep(&cell_meta_lock));
    // Create input
    tx.raw.inputs.push(px.create_cell_input(&cell_meta_i));
    // Create output
    tx.raw.outputs.push(px.create_cell_output(0, ckbez::core::Script::default(), None));
    // Create output data
    tx.raw.outputs_data.push(vec![]);

    let verifier = ckbez::unittest::Verifier::default();
    verifier.verify(&tx, &dl).unwrap();
}
