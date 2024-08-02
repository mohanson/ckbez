use ckb_types::prelude::{Entity, IntoTransactionView};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Script {
    pub code_hash: [u8; 32],
    pub hash_type: u8,
    pub args: Vec<u8>,
}

impl Script {
    pub fn new(code_hash: [u8; 32], hash_type: u8, args: Vec<u8>) -> Self {
        Self { code_hash, hash_type, args }
    }

    pub fn molecule(&self) -> Vec<u8> {
        crate::molecule::encode_dynvec(vec![
            crate::molecule::Byte32::new(self.code_hash).molecule(),
            crate::molecule::Byte::new(self.hash_type).molecule(),
            crate::molecule::Bytes::new(self.args.clone()).molecule(),
        ])
    }

    pub fn molecule_decode(data: &[u8]) -> Self {
        let result = crate::molecule::decode_dynvec(data);
        Self {
            code_hash: crate::molecule::Byte32::molecule_decode(&result[0]),
            hash_type: crate::molecule::Byte::molecule_decode(&result[1]),
            args: crate::molecule::Bytes::molecule_decode(&result[2]),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutPoint {
    pub tx_hash: [u8; 32],
    pub index: u32,
}

impl OutPoint {
    pub fn new(tx_hash: [u8; 32], index: u32) -> Self {
        Self { tx_hash, index }
    }

    pub fn molecule(&self) -> Vec<u8> {
        crate::molecule::encode_seq(vec![
            crate::molecule::Byte32::new(self.tx_hash).molecule(),
            crate::molecule::U32::new(self.index).molecule(),
        ])
    }

    pub fn molecule_decode(data: &[u8]) -> Self {
        let result = crate::molecule::decode_seq(
            data,
            &[crate::molecule::Byte32::molecule_size(), crate::molecule::U32::molecule_size()],
        );
        Self {
            tx_hash: crate::molecule::Byte32::molecule_decode(&result[0]),
            index: crate::molecule::U32::molecule_decode(&result[1]),
        }
    }

    pub fn molecule_size() -> usize {
        crate::molecule::Byte32::molecule_size() + crate::molecule::U32::molecule_size()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CellInput {
    pub since: u64,
    pub previous_output: OutPoint,
}

impl CellInput {
    pub fn new(since: u64, previous_output: OutPoint) -> Self {
        Self { since, previous_output }
    }

    pub fn molecule(&self) -> Vec<u8> {
        crate::molecule::encode_seq(vec![
            crate::molecule::U64::new(self.since).molecule(),
            self.previous_output.molecule(),
        ])
    }

    pub fn molecule_decode(data: &[u8]) -> Self {
        let result =
            crate::molecule::decode_seq(data, &[crate::molecule::U64::molecule_size(), OutPoint::molecule_size()]);
        CellInput {
            since: crate::molecule::U64::molecule_decode(&result[0]),
            previous_output: OutPoint::molecule_decode(&result[1]),
        }
    }

    pub fn molecule_size() -> usize {
        crate::molecule::U64::molecule_size() + OutPoint::molecule_size()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CellOutput {
    pub capacity: u64,
    pub lock: Script,
    pub kype: Option<Script>,
}

impl CellOutput {
    pub fn new(capacity: u64, lock: Script, kype: Option<Script>) -> Self {
        Self { capacity, lock, kype }
    }

    pub fn molecule(&self) -> Vec<u8> {
        crate::molecule::encode_dynvec(vec![
            crate::molecule::U64::new(self.capacity).molecule(),
            self.lock.molecule(),
            match &self.kype {
                Some(kype) => kype.molecule(),
                None => vec![],
            },
        ])
    }

    pub fn molecule_decode(data: &[u8]) -> Self {
        let result = crate::molecule::decode_dynvec(data);
        CellOutput {
            capacity: crate::molecule::U64::molecule_decode(&result[0]),
            lock: Script::molecule_decode(&result[1]),
            kype: if !result[2].is_empty() { Some(Script::molecule_decode(&result[2])) } else { None },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CellDep {
    pub out_point: OutPoint,
    pub dep_type: u8,
}

impl CellDep {
    pub fn new(out_point: OutPoint, dep_type: u8) -> Self {
        Self { out_point, dep_type }
    }

    pub fn molecule(&self) -> Vec<u8> {
        crate::molecule::encode_seq(vec![
            self.out_point.molecule(),
            crate::molecule::Byte::new(self.dep_type).molecule(),
        ])
    }

    pub fn molecule_decode(data: &[u8]) -> Self {
        let result =
            crate::molecule::decode_seq(data, &[OutPoint::molecule_size(), crate::molecule::Byte::molecule_size()]);
        CellDep {
            out_point: OutPoint::molecule_decode(&result[0]),
            dep_type: crate::molecule::Byte::molecule_decode(&result[1]),
        }
    }

    pub fn molecule_size() -> usize {
        OutPoint::molecule_size() + crate::molecule::Byte::molecule_size()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionRaw {
    pub version: u32,
    pub cell_deps: Vec<CellDep>,
    pub header_deps: Vec<[u8; 32]>,
    pub inputs: Vec<CellInput>,
    pub outputs: Vec<CellOutput>,
    pub outputs_data: Vec<Vec<u8>>,
}

impl TransactionRaw {
    pub fn new(
        version: u32,
        cell_deps: Vec<CellDep>,
        header_deps: Vec<[u8; 32]>,
        inputs: Vec<CellInput>,
        outputs: Vec<CellOutput>,
        outputs_data: Vec<Vec<u8>>,
    ) -> Self {
        Self { version, cell_deps, header_deps, inputs, outputs, outputs_data }
    }

    pub fn molecule(&self) -> Vec<u8> {
        crate::molecule::encode_dynvec(vec![
            crate::molecule::U32::new(self.version).molecule(),
            crate::molecule::encode_fixvec(self.cell_deps.iter().map(|e| e.molecule()).collect()),
            crate::molecule::encode_fixvec(
                self.header_deps.iter().map(|e| crate::molecule::Byte32::new(*e).molecule()).collect(),
            ),
            crate::molecule::encode_fixvec(self.inputs.iter().map(|e| e.molecule()).collect()),
            crate::molecule::encode_dynvec(self.outputs.iter().map(|e| e.molecule()).collect()),
            crate::molecule::encode_dynvec(
                self.outputs_data.iter().map(|e| crate::molecule::Bytes::new(e.clone()).molecule()).collect(),
            ),
        ])
    }

    pub fn molecule_decode(data: &[u8]) -> Self {
        let result = crate::molecule::decode_dynvec(data);
        Self {
            version: crate::molecule::U32::molecule_decode(&result[0]),
            cell_deps: crate::molecule::decode_fixvec(&result[1])
                .iter()
                .map(|e| CellDep::molecule_decode(&e))
                .collect(),
            header_deps: crate::molecule::decode_fixvec(&result[2])
                .iter()
                .map(|e| crate::molecule::Byte32::molecule_decode(&e))
                .collect(),
            inputs: crate::molecule::decode_fixvec(&result[3]).iter().map(|e| CellInput::molecule_decode(&e)).collect(),
            outputs: crate::molecule::decode_dynvec(&result[4])
                .iter()
                .map(|e| CellOutput::molecule_decode(&e))
                .collect(),
            outputs_data: crate::molecule::decode_dynvec(&result[5])
                .iter()
                .map(|e| crate::molecule::Bytes::molecule_decode(&e))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transaction {
    pub raw: TransactionRaw,
    pub witnesses: Vec<Vec<u8>>,
}

impl Transaction {
    pub fn new(raw: TransactionRaw, witnesses: Vec<Vec<u8>>) -> Self {
        Self { raw, witnesses }
    }

    pub fn molecule(&self) -> Vec<u8> {
        crate::molecule::encode_dynvec(vec![
            self.raw.molecule(),
            crate::molecule::encode_dynvec(
                self.witnesses.iter().map(|e| crate::molecule::Bytes::new(e.clone()).molecule()).collect(),
            ),
        ])
    }

    pub fn molecule_decode(data: &[u8]) -> Self {
        let result = crate::molecule::decode_dynvec(data);
        Transaction {
            raw: TransactionRaw::molecule_decode(&result[0]),
            witnesses: crate::molecule::decode_dynvec(&result[1])
                .iter()
                .map(|e| crate::molecule::Bytes::molecule_decode(&e))
                .collect(),
        }
    }

    pub fn into_view(&self) -> ckb_types::core::TransactionView {
        let tx = ckb_types::packed::Transaction::new_unchecked(ckb_types::bytes::Bytes::from(self.molecule()));
        tx.into_view()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WitnessArgs {
    pub lock: Option<Vec<u8>>,
    pub input_type: Option<Vec<u8>>,
    pub output_type: Option<Vec<u8>>,
}

impl WitnessArgs {
    pub fn new(lock: Option<Vec<u8>>, input_type: Option<Vec<u8>>, output_type: Option<Vec<u8>>) -> Self {
        Self { lock, input_type, output_type }
    }

    pub fn molecule(&self) -> Vec<u8> {
        crate::molecule::encode_dynvec(vec![
            match &self.lock {
                Some(lock) => crate::molecule::Bytes::new(lock.clone()).molecule(),
                None => vec![],
            },
            match &self.input_type {
                Some(kype) => crate::molecule::Bytes::new(kype.clone()).molecule(),
                None => vec![],
            },
            match &self.output_type {
                Some(kype) => crate::molecule::Bytes::new(kype.clone()).molecule(),
                None => vec![],
            },
        ])
    }

    pub fn molecule_decode(data: &[u8]) -> Self {
        let result = crate::molecule::decode_dynvec(data);
        Self {
            lock: if !result[0].is_empty() { Some(crate::molecule::Bytes::molecule_decode(&result[0])) } else { None },
            input_type: if !result[1].is_empty() {
                Some(crate::molecule::Bytes::molecule_decode(&result[1]))
            } else {
                None
            },
            output_type: if !result[2].is_empty() {
                Some(crate::molecule::Bytes::molecule_decode(&result[2]))
            } else {
                None
            },
        }
    }
}
