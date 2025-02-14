#[test]
fn test_exit_0() {
    let exit_0 = std::fs::read("res/exit_0").unwrap();
    let mut dl = ckbez::unittest::Resource::default();
    let mut px = ckbez::unittest::Pickaxer::default();

    let mut tx = ckbez::core::Transaction::default();
    let cell_meta_lock = px.create_cell(&mut dl, 0, ckbez::core::Script::default(), None, &exit_0);
    let cell_meta_i = px.create_cell(&mut dl, 0, px.create_script_by_data(&cell_meta_lock, &[]), None, &[]);
    tx.raw.cell_deps.push(px.create_cell_dep(&cell_meta_lock, 0));
    tx.raw.inputs.push(px.create_cell_input(&cell_meta_i));
    tx.raw.outputs.push(px.create_cell_output(0, ckbez::core::Script::default(), None));
    tx.raw.outputs_data.push(vec![]);

    let verifier = ckbez::unittest::Verifier::default();
    verifier.verify(&tx, &dl).unwrap();
}

#[test]
fn test_sighash_all() {
    let sighash_all = std::fs::read("res/sighash_all").unwrap();
    let mut dl = ckbez::unittest::Resource::default();
    let mut px = ckbez::unittest::Pickaxer::default();

    let mut tx = ckbez::core::Transaction::default();
    let cell_meta_lock = px.create_cell(&mut dl, 0, ckbez::core::Script::default(), None, &sighash_all);
    let cell_meta_i = px.create_cell(&mut dl, 0, px.create_script_by_data(&cell_meta_lock, &[]), None, &[]);
    tx.raw.cell_deps.push(px.create_cell_dep(&cell_meta_lock, 0));
    tx.raw.inputs.push(px.create_cell_input(&cell_meta_i));
    tx.raw.outputs.push(px.create_cell_output(0, ckbez::core::Script::default(), None));
    tx.raw.outputs_data.push(vec![]);
    tx.witnesses.push(ckbez::core::WitnessArgs::new(Some(vec![0; 32]), None, None).molecule());
    tx.witnesses[0] = ckbez::core::WitnessArgs::new(Some(tx.hash_sighash_all(0, &[]).to_vec()), None, None).molecule();

    let verifier = ckbez::unittest::Verifier::default();
    verifier.verify(&tx, &dl).unwrap();
}

#[test]
fn test_spawn() {
    let spawn_caller = std::fs::read("res/spawn_caller").unwrap();
    let spawn_callee = std::fs::read("res/spawn_callee").unwrap();
    let mut dl = ckbez::unittest::Resource::default();
    let mut px = ckbez::unittest::Pickaxer::default();

    let mut tx = ckbez::core::Transaction::default();
    let cell_meta_caller = px.create_cell(&mut dl, 0, ckbez::core::Script::default(), None, &spawn_caller);
    let cell_meta_callee = px.create_cell(&mut dl, 0, ckbez::core::Script::default(), None, &spawn_callee);
    let cell_meta_i = px.create_cell(&mut dl, 0, px.create_script_by_data(&cell_meta_caller, &[]), None, &[]);
    tx.raw.cell_deps.push(px.create_cell_dep(&cell_meta_caller, 0));
    tx.raw.cell_deps.push(px.create_cell_dep(&cell_meta_callee, 0));
    tx.raw.inputs.push(px.create_cell_input(&cell_meta_i));
    tx.raw.outputs.push(px.create_cell_output(0, ckbez::core::Script::default(), None));
    tx.raw.outputs_data.push(vec![]);

    let verifier = ckbez::unittest::Verifier::default();
    verifier.verify(&tx, &dl).unwrap();
}
