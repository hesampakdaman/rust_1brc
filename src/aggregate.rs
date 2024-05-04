use std::sync::mpsc::Receiver;
use std::collections::HashMap;

use crate::record::Record;

pub fn reduce(rx: Receiver<HashMap<String, Record>>) {}
