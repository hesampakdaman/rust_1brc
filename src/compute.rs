use std::collections::HashMap;
use std::sync::mpsc::Sender;

use crate::pre_processing::Chunk;
use crate::record::Record;

type Statistics = HashMap<String, Record>;

pub fn stats(chunk: Chunk, tx: Sender<Statistics>) {}
