//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

extern crate blake2_rfc;
extern crate serde;
extern crate serde_cbor;
extern crate time;
#[macro_use]
extern crate log;

mod cbor_codec;
mod file_protocol;

pub use cbor_codec::Protocol as CborProtocol;
pub use file_protocol::download;
pub use file_protocol::protocol::Protocol as FileProtocol;
pub use file_protocol::upload;
pub use file_protocol::Message;
