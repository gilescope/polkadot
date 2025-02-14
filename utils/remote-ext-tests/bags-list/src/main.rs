// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Remote tests for bags-list pallet.

use clap::arg_enum;
use std::convert::TryInto;
use structopt::StructOpt;

arg_enum! {
	#[derive(Debug)]
	enum Command {
		CheckMigration,
		SanityCheck,
		Snapshot,
	}
}

arg_enum! {
	#[derive(Debug)]
	enum Runtime {
		Polkadot,
		Kusama,
		Westend,
	}
}

#[derive(StructOpt)]
struct Cli {
	#[structopt(long, short, default_value = "wss://kusama-rpc.polkadot.io:443")]
	uri: String,
	#[structopt(long, short, case_insensitive = true, possible_values = &Runtime::variants(), default_value = "kusama")]
	runtime: Runtime,
	#[structopt(long, short, case_insensitive = true, possible_values = &Command::variants(), default_value = "SanityCheck")]
	command: Command,
	#[structopt(long, short)]
	snapshot_limit: Option<usize>,
}

#[tokio::main]
async fn main() {
	let options = Cli::from_args();
	sp_tracing::try_init_simple();

	log::info!(
		target: "remote-ext-tests",
		"using runtime {:?} / command: {:?}",
		options.runtime,
		options.command
	);

	use pallet_bags_list_remote_tests::*;
	match options.runtime {
		Runtime::Polkadot => sp_core::crypto::set_default_ss58_version(
			<polkadot_runtime::Runtime as frame_system::Config>::SS58Prefix::get()
				.try_into()
				.unwrap(),
		),
		Runtime::Kusama => sp_core::crypto::set_default_ss58_version(
			<kusama_runtime::Runtime as frame_system::Config>::SS58Prefix::get()
				.try_into()
				.unwrap(),
		),
		Runtime::Westend => sp_core::crypto::set_default_ss58_version(
			<westend_runtime::Runtime as frame_system::Config>::SS58Prefix::get()
				.try_into()
				.unwrap(),
		),
	};

	match (options.runtime, options.command) {
		(Runtime::Kusama, Command::CheckMigration) => {
			use kusama_runtime::{Block, Runtime};
			use kusama_runtime_constants::currency::UNITS;
			migration::execute::<Runtime, Block>(UNITS as u64, "KSM", options.uri.clone()).await;
		},
		(Runtime::Kusama, Command::SanityCheck) => {
			use kusama_runtime::{Block, Runtime};
			use kusama_runtime_constants::currency::UNITS;
			sanity_check::execute::<Runtime, Block>(UNITS as u64, "KSM", options.uri.clone()).await;
		},
		(Runtime::Kusama, Command::Snapshot) => {
			use kusama_runtime::{Block, Runtime};
			use kusama_runtime_constants::currency::UNITS;
			snapshot::execute::<Runtime, Block>(
				options.snapshot_limit,
				UNITS.try_into().unwrap(),
				options.uri.clone(),
			)
			.await;
		},

		(Runtime::Westend, Command::CheckMigration) => {
			use westend_runtime::{Block, Runtime};
			use westend_runtime_constants::currency::UNITS;
			migration::execute::<Runtime, Block>(UNITS as u64, "WND", options.uri.clone()).await;
		},
		(Runtime::Westend, Command::SanityCheck) => {
			use westend_runtime::{Block, Runtime};
			use westend_runtime_constants::currency::UNITS;
			sanity_check::execute::<Runtime, Block>(UNITS as u64, "WND", options.uri.clone()).await;
		},
		(Runtime::Westend, Command::Snapshot) => {
			use westend_runtime::{Block, Runtime};
			use westend_runtime_constants::currency::UNITS;
			snapshot::execute::<Runtime, Block>(
				options.snapshot_limit,
				UNITS.try_into().unwrap(),
				options.uri.clone(),
			)
			.await;
		},

		(Runtime::Polkadot, Command::CheckMigration) => {
			use polkadot_runtime::{Block, Runtime};
			use polkadot_runtime_constants::currency::UNITS;
			migration::execute::<Runtime, Block>(UNITS as u64, "DOT", options.uri.clone()).await;
		},
		(Runtime::Polkadot, Command::SanityCheck) => {
			use polkadot_runtime::{Block, Runtime};
			use polkadot_runtime_constants::currency::UNITS;
			sanity_check::execute::<Runtime, Block>(UNITS as u64, "DOT", options.uri.clone()).await;
		},
		(Runtime::Polkadot, Command::Snapshot) => {
			use polkadot_runtime::{Block, Runtime};
			use polkadot_runtime_constants::currency::UNITS;
			snapshot::execute::<Runtime, Block>(
				options.snapshot_limit,
				UNITS.try_into().unwrap(),
				options.uri.clone(),
			)
			.await;
		},
	}
}
