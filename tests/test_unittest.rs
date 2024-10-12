#[test]
fn test_init() {
    if !std::path::Path::new("res").exists() {
        std::fs::create_dir("res").unwrap();
    }
    if !std::path::Path::new("res/ckbes").exists() {
        let url = "https://github.com/mohanson/ckbes";
        std::process::Command::new("git").arg("clone").arg(url).current_dir("res").status().unwrap();
        std::process::Command::new("cargo").arg("build").arg("--examples").current_dir("res/ckbes").status().unwrap();
    }
}

#[test]
fn test_exit_0() {
    let data_root = std::path::Path::new("res/ckbes/target/riscv64imac-unknown-none-elf/debug/examples");
    let exit_0 = std::fs::read(data_root.join("exit_0")).unwrap();
    let mut dl = ckbez::unittest::Resource::default();
    let mut px = ckbez::unittest::Pickaxer::default();

    let mut tx = ckbez::core::Transaction::default();
    let cell_meta_lock = px.create_cell(&mut dl, 0, ckbez::core::Script::default(), None, &exit_0);
    let cell_meta_i = px.create_cell(&mut dl, 0, px.create_script_by_data(&cell_meta_lock, &[]), None, &[]);
    tx.raw.cell_deps.push(px.create_cell_dep(&cell_meta_lock));
    tx.raw.inputs.push(px.create_cell_input(&cell_meta_i));
    tx.raw.outputs.push(px.create_cell_output(0, ckbez::core::Script::default(), None));
    tx.raw.outputs_data.push(vec![]);

    let verifier = ckbez::unittest::Verifier::default();
    verifier.verify(&tx, &dl).unwrap();
}

#[test]
fn test_sighash_all() {
    let data_root = std::path::Path::new("res/ckbes/target/riscv64imac-unknown-none-elf/debug/examples");
    let sighash_all = std::fs::read(data_root.join("sighash_all")).unwrap();
    let mut dl = ckbez::unittest::Resource::default();
    let mut px = ckbez::unittest::Pickaxer::default();

    let mut tx = ckbez::core::Transaction::default();
    let cell_meta_lock = px.create_cell(&mut dl, 0, ckbez::core::Script::default(), None, &sighash_all);
    let cell_meta_i = px.create_cell(&mut dl, 0, px.create_script_by_data(&cell_meta_lock, &[]), None, &[]);
    tx.raw.cell_deps.push(px.create_cell_dep(&cell_meta_lock));
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
    let data_root = std::path::Path::new("res/ckbes/target/riscv64imac-unknown-none-elf/debug/examples");
    let spawn_caller = std::fs::read(data_root.join("spawn_caller")).unwrap();
    let spawn_callee = std::fs::read(data_root.join("spawn_callee")).unwrap();
    let mut dl = ckbez::unittest::Resource::default();
    let mut px = ckbez::unittest::Pickaxer::default();

    let mut tx = ckbez::core::Transaction::default();
    let cell_meta_caller = px.create_cell(&mut dl, 0, ckbez::core::Script::default(), None, &spawn_caller);
    let cell_meta_callee = px.create_cell(&mut dl, 0, ckbez::core::Script::default(), None, &spawn_callee);
    let cell_meta_i = px.create_cell(&mut dl, 0, px.create_script_by_data(&cell_meta_caller, &[]), None, &[]);
    tx.raw.cell_deps.push(px.create_cell_dep(&cell_meta_caller));
    tx.raw.cell_deps.push(px.create_cell_dep(&cell_meta_callee));
    tx.raw.inputs.push(px.create_cell_input(&cell_meta_i));
    tx.raw.outputs.push(px.create_cell_output(0, ckbez::core::Script::default(), None));
    tx.raw.outputs_data.push(vec![]);

    let verifier = ckbez::unittest::Verifier::default();
    verifier.verify(&tx, &dl).unwrap();
}
