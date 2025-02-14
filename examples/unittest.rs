// Create a transaction whose input is locked by the exit_0 contract, and execute it.

fn main() {
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
