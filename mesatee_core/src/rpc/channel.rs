// Copyright 2019 MesaTEE Authors
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

// Use sgx_tstd to replace Rust's default std
#[cfg(feature = "mesalock_sgx")]
use std::prelude::v1::*;

use std::io::Write;
use crate::rpc::sgx;
use crate::rpc::RpcClient;
use crate::Result;
use net2::TcpBuilder;
use serde::{de::DeserializeOwned, Serialize};

pub struct SgxTrustedChannel<U: Serialize, V: DeserializeOwned> {
    client: sgx::PipeClient<U, V>,
}

impl<U, V> SgxTrustedChannel<U, V>
where
    U: Serialize,
    V: DeserializeOwned,
{
   fn _new(
        addr: std::net::SocketAddr,
        enclave_attr: sgx::EnclaveAttr,
        extension: u8,
    ) -> Result<SgxTrustedChannel<U, V>> {
        let tcp_builder = TcpBuilder::new_v4()?;
        tcp_builder.reuse_address(true)?;
        let mut stream = tcp_builder.connect(addr)?;
        stream.set_nodelay(true)?;

        let ext_data: [u8; 1] = [extension; 1];
        stream.write(&ext_data)?;

        let config = sgx::PipeClientConfig {
            tcp: stream,
            hostname: webpki::DNSNameRef::try_from_ascii_str(
                format!("{}-{}", "localhost", addr.port()).as_ref(),
            )
            .unwrap()
            .to_owned(),
            server_attr: enclave_attr,
        };
        let client = sgx::PipeClient::<U, V>::open(config)?;

        Ok(SgxTrustedChannel { client })
    }

    pub fn new(
        addr: std::net::SocketAddr,
        enclave_attr: sgx::EnclaveAttr,
    ) -> Result<SgxTrustedChannel<U, V>> {
        self._new(addr, enclave_attr, 0)
    }

    pub fn new_with_extension(
        addr: std::net::SocketAddr,
        enclave_attr: sgx::EnclaveAttr,
        extension: u8,
    ) -> Result<SgxTrustedChannel<U, V>> {
        self._new(addr, enclave_attr, extension)
    }

    pub fn invoke(&mut self, input: U) -> Result<V> {
        self.client.invoke(input)
    }
}
