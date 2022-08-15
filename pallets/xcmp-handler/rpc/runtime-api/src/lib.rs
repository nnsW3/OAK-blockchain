// This file is part of OAK Blockchain.

// Copyright (C) 2022 OAK Network
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
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

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_core::Bytes;
use sp_runtime::AccountId32;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	pub trait XcmpHandlerApi<Balance> where
		Balance: Codec,
	{
		fn cross_chain_account(account_id: AccountId32) -> Result<AccountId32, Vec<u8>>;
		fn fees(encoded_xt: Bytes) -> Result<Balance, Vec<u8>>;
	}
}