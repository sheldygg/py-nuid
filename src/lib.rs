// Copyright 2018 Ivan Porto Carrero
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use pyo3::prelude::*;

use rand::distributions::Alphanumeric;
use rand::rngs::OsRng;
use rand::thread_rng;
use rand::Rng;

const BASE: usize = 62;
const ALPHABET: [u8; BASE] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const PRE_LEN: usize = 12;
const MAX_SEQ: u64 = 839_299_365_868_340_224; // (BASE ^ remaining bytes 22 - 12) == 62^10
const MIN_INC: u64 = 33;
const MAX_INC: u64 = 333;

/// The number of bytes/characters of a NUID.
pub const TOTAL_LEN: usize = 22;

/// NUID needs to be very fast to generate and truly unique, all while being entropy pool friendly.
/// We will use 12 bytes of crypto generated data (entropy draining), and 10 bytes of sequential data
/// that is started at a pseudo random number and increments with a pseudo-random increment.
/// Total is 22 bytes of base 62 ascii text :)
#[pyclass]
pub struct NUID {
    pre: [u8; PRE_LEN],
    seq: u64,
    inc: u64,
}

#[pymethods]
impl NUID {
    /// generate a new `NUID` and properly initialize the prefix, sequential start, and sequential increment.
    #[new]
    pub fn new() -> Self {
        let mut nuid = NUID {
            pre: [0; PRE_LEN],
            // the first call to `next` will cause the prefix and sequential to be regenerated
            seq: MAX_SEQ,
            inc: 0,
        };
        nuid.randomize_prefix();
        nuid
    }

    fn randomize_prefix(&mut self) {
        let rng = OsRng;
        for (i, n) in rng.sample_iter(&Alphanumeric).take(PRE_LEN).enumerate() {
            self.pre[i] = ALPHABET[n as usize % BASE];
        }
    }

    /// Generate the next `NUID` string.
    #[allow(clippy::should_implement_trait)]
    #[must_use]
    pub fn next(&mut self) -> String {
        let mut buffer = [0u8; TOTAL_LEN];

        self.seq += self.inc;
        if self.seq >= MAX_SEQ {
            self.randomize_prefix();
            self.reset_sequential();
        }

        let seq = self.seq as usize;
        for (i, n) in self.pre.iter().enumerate() {
            buffer[i] = *n;
        }

        let mut l = seq;
        for i in (PRE_LEN..TOTAL_LEN).rev() {
            buffer[i] = ALPHABET[l % BASE];
            l /= BASE;
        }

        unsafe { String::from_utf8_unchecked(buffer.to_vec()) }
    }

    fn reset_sequential(&mut self) {
        let mut rng = thread_rng();
        self.seq = rng.gen_range(0..MAX_SEQ);
        self.inc = rng.gen_range(MIN_INC..MAX_INC);
    }
}

#[pymodule]
fn nuid(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<NUID>()?;
    Ok(())
}
