use ckb_types::prelude::{Entity, IntoTransactionView, Pack};

pub fn println_hex(name: &str, data: &[u8]) {
    println!("Tester(........): {}(len={}): {}", name, data.len(), hex::encode(data));
}

pub fn println_log(data: &str) {
    println!("Tester(........): {}", data);
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CellMeta {
    pub out_point: crate::core::OutPoint,
    pub cell_output: crate::core::CellOutput,
    pub data: Vec<u8>,
    pub data_hash: [u8; 32],
}

impl CellMeta {
    pub fn new(out_point: crate::core::OutPoint, cell_output: crate::core::CellOutput, data: &[u8]) -> Self {
        Self { out_point, cell_output, data: data.to_vec(), data_hash: ckb_hash::blake2b_256(data) }
    }

    pub fn pack(&self) -> ckb_types::core::cell::CellMeta {
        let cell_output = self.cell_output.pack();
        let data = ckb_types::bytes::Bytes::from(self.data.clone());
        ckb_types::core::cell::CellMetaBuilder::from_cell_output(cell_output, data)
            .out_point(self.out_point.pack())
            .build()
    }
}

#[derive(Clone, Default)]
pub struct Resource {
    pub cell: std::collections::HashMap<crate::core::OutPoint, CellMeta>,
}

impl ckb_traits::CellDataProvider for Resource {
    fn get_cell_data(&self, out_point: &ckb_types::packed::OutPoint) -> Option<ckb_types::bytes::Bytes> {
        let out_point = crate::core::OutPoint::molecule_decode(out_point.as_slice());
        self.cell.get(&out_point).and_then(|e| Some(ckb_types::bytes::Bytes::from(e.data.clone())))
    }

    fn get_cell_data_hash(&self, out_point: &ckb_types::packed::OutPoint) -> Option<ckb_types::packed::Byte32> {
        let out_point = crate::core::OutPoint::molecule_decode(out_point.as_slice());
        self.cell.get(&out_point).and_then(|e| Some(ckb_types::packed::Byte32::from_slice(&e.data_hash).unwrap()))
    }
}

impl ckb_traits::HeaderProvider for Resource {
    fn get_header(&self, _: &ckb_types::packed::Byte32) -> Option<ckb_types::core::HeaderView> {
        unimplemented!()
    }
}

impl ckb_traits::ExtensionProvider for Resource {
    fn get_block_extension(&self, _: &ckb_types::packed::Byte32) -> Option<ckb_types::packed::Bytes> {
        unimplemented!()
    }
}

impl ckb_types::core::cell::CellProvider for Resource {
    fn cell(&self, out_point: &ckb_types::packed::OutPoint, eager_load: bool) -> ckb_types::core::cell::CellStatus {
        let out_point = crate::core::OutPoint::molecule_decode(out_point.as_slice());
        let _ = eager_load;
        if let Some(data) = self.cell.get(&out_point) {
            ckb_types::core::cell::CellStatus::Live(data.pack())
        } else {
            ckb_types::core::cell::CellStatus::Unknown
        }
    }
}

impl ckb_types::core::cell::HeaderChecker for Resource {
    fn check_valid(&self, _: &ckb_types::packed::Byte32) -> Result<(), ckb_types::core::error::OutPointError> {
        unimplemented!()
    }
}

#[derive(Clone, Default)]
pub struct Verifier {}

impl Verifier {
    pub fn verify_prior(&self, tx: &crate::core::Transaction, _: &Resource) {
        assert_eq!(tx.raw.outputs.len(), tx.raw.outputs_data.len());
    }

    pub fn verify(
        &self,
        tx: &crate::core::Transaction,
        dl: &Resource,
    ) -> Result<ckb_types::core::Cycle, ckb_error::Error> {
        self.verify_prior(tx, dl);
        let tx_view = tx.pack().into_view();
        let tx_resolved =
            ckb_types::core::cell::resolve_transaction(tx_view, &mut std::collections::HashSet::new(), dl, dl).unwrap();
        let hardfork = ckb_types::core::hardfork::HardForks {
            ckb2021: ckb_types::core::hardfork::CKB2021::new_dev_default(),
            ckb2023: ckb_types::core::hardfork::CKB2023::new_dev_default(),
        };
        let consensus = ckb_chain_spec::consensus::ConsensusBuilder::default().hardfork_switch(hardfork).build();
        let mut verifier = ckb_script::TransactionScriptsVerifier::new(
            std::sync::Arc::new(tx_resolved.clone()),
            dl.clone(),
            std::sync::Arc::new(consensus),
            std::sync::Arc::new(ckb_script::TxVerifyEnv::new_submit(
                &ckb_types::core::HeaderView::new_advanced_builder()
                    .epoch(ckb_types::core::EpochNumberWithFraction::new(0, 0, 1).pack())
                    .build(),
            )),
        );
        verifier.set_debug_printer(|script: &ckb_types::packed::Byte32, msg: &str| {
            let str = format!("Script({})", hex::encode(&script.as_slice()[..4]));
            println!("{}: {}", str, msg);
        });
        let result = verifier.verify(u64::MAX);
        if result.is_ok() {
            let cycles = (*result.as_ref().unwrap() as f64) / 1024.0 / 1024.0;
            println_log(&format!("cycles is {:.1} M ", cycles));
        }
        result
    }
}

#[derive(Clone, Default)]
pub struct Pickaxer {
    outpoint_hash: [u8; 32],
    outpoint_i: u32,
}

impl Pickaxer {
    pub fn create_type_id(&self) -> crate::core::Script {
        let mut args = vec![0u8; 32];
        args[28..].copy_from_slice(&self.outpoint_i.to_be_bytes());
        crate::core::Script::new_type_id(args)
    }

    pub fn create_cell(
        &mut self,
        dl: &mut Resource,
        capacity: u64,
        lock: crate::core::Script,
        kype: Option<crate::core::Script>,
        data: &[u8],
    ) -> CellMeta {
        let cell_out_point = crate::core::OutPoint::new(self.outpoint_hash.clone(), self.outpoint_i);
        let cell_output = crate::core::CellOutput { capacity: capacity, lock: lock, kype: kype };
        let cell_meta = CellMeta::new(cell_out_point.clone(), cell_output, data);
        dl.cell.insert(cell_out_point, cell_meta.clone());
        self.outpoint_i += 1;
        cell_meta
    }

    pub fn create_cell_dep(&self, cell_meta: &CellMeta) -> crate::core::CellDep {
        crate::core::CellDep { out_point: cell_meta.out_point.clone(), dep_type: ckb_types::core::DepType::Code.into() }
    }

    pub fn create_cell_input(&self, cell_meta: &CellMeta) -> crate::core::CellInput {
        crate::core::CellInput::new(0, cell_meta.out_point.clone())
    }

    pub fn create_cell_output(
        &self,
        capacity: u64,
        lock: crate::core::Script,
        kype: Option<crate::core::Script>,
    ) -> crate::core::CellOutput {
        crate::core::CellOutput { capacity: capacity, lock: lock, kype: kype }
    }

    pub fn create_script_by_data(&self, cell_meta: &CellMeta, args: &[u8]) -> crate::core::Script {
        crate::core::Script {
            code_hash: cell_meta.data_hash,
            hash_type: ckb_types::core::ScriptHashType::Data1.into(),
            args: args.to_vec(),
        }
    }

    pub fn create_script_by_type(&self, cell_meta: &CellMeta, args: &[u8]) -> crate::core::Script {
        crate::core::Script {
            code_hash: cell_meta.cell_output.kype.clone().unwrap().hash(),
            hash_type: ckb_types::core::ScriptHashType::Type.into(),
            args: args.to_vec(),
        }
    }
}