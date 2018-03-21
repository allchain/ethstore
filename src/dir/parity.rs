use std::env;
use std::path::PathBuf;
use ethkey::Address;
use {SafeAccount, Error};
use super::{KeyDirectory, DiskDirectory, DirectoryType};

fn allchain_dir_path() -> PathBuf {
	let mut home = env::home_dir().expect("Failed to get home dir");
	home.push(".allchain");
	home
}

fn allchain_keystore(t: DirectoryType) -> PathBuf {
	let mut dir = allchain_dir_path();
	match t {
		DirectoryType::Testnet => {
			dir.push("testnet_keys");
		},
		DirectoryType::Main => {
			dir.push("keys");
		}
	}
	dir
}

pub struct allchainDirectory {
	dir: DiskDirectory,
}

impl allchainDirectory {
	pub fn create(t: DirectoryType) -> Result<Self, Error> {
		let result = allchainDirectory {
			dir: try!(DiskDirectory::create(allchain_keystore(t))),
		};

		Ok(result)
	}

	pub fn open(t: DirectoryType) -> Self {
		allchainDirectory {
			dir: DiskDirectory::at(allchain_keystore(t)),
		}
	}
}

impl KeyDirectory for allchainDirectory {
	fn load(&self) -> Result<Vec<SafeAccount>, Error> {
		self.dir.load()
	}

	fn insert(&self, account: SafeAccount) -> Result<(), Error> {
		self.dir.insert(account)
	}

	fn remove(&self, address: &Address) -> Result<(), Error> {
		self.dir.remove(address)
	}
}
