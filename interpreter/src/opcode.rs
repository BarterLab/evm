/// Opcode enum. One-to-one corresponding to an `u8` value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(
	feature = "scale",
	derive(scale_codec::Encode, scale_codec::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Opcode(pub u8);

// Core opcodes.
impl Opcode {
	/// `STOP`
	pub const STOP: Opcode = Opcode(0x00);
	/// `ADD`
	pub const ADD: Opcode = Opcode(0x01);
	/// `MUL`
	pub const MUL: Opcode = Opcode(0x02);
	/// `SUB`
	pub const SUB: Opcode = Opcode(0x03);
	/// `DIV`
	pub const DIV: Opcode = Opcode(0x04);
	/// `SDIV`
	pub const SDIV: Opcode = Opcode(0x05);
	/// `MOD`
	pub const MOD: Opcode = Opcode(0x06);
	/// `SMOD`
	pub const SMOD: Opcode = Opcode(0x07);
	/// `ADDMOD`
	pub const ADDMOD: Opcode = Opcode(0x08);
	/// `MULMOD`
	pub const MULMOD: Opcode = Opcode(0x09);
	/// `EXP`
	pub const EXP: Opcode = Opcode(0x0a);
	/// `SIGNEXTEND`
	pub const SIGNEXTEND: Opcode = Opcode(0x0b);

	/// `LT`
	pub const LT: Opcode = Opcode(0x10);
	/// `GT`
	pub const GT: Opcode = Opcode(0x11);
	/// `SLT`
	pub const SLT: Opcode = Opcode(0x12);
	/// `SGT`
	pub const SGT: Opcode = Opcode(0x13);
	/// `EQ`
	pub const EQ: Opcode = Opcode(0x14);
	/// `ISZERO`
	pub const ISZERO: Opcode = Opcode(0x15);
	/// `AND`
	pub const AND: Opcode = Opcode(0x16);
	/// `OR`
	pub const OR: Opcode = Opcode(0x17);
	/// `XOR`
	pub const XOR: Opcode = Opcode(0x18);
	/// `NOT`
	pub const NOT: Opcode = Opcode(0x19);
	/// `BYTE`
	pub const BYTE: Opcode = Opcode(0x1a);

	/// `SHL`
	pub const SHL: Opcode = Opcode(0x1b);
	/// `SHR`
	pub const SHR: Opcode = Opcode(0x1c);
	/// `SAR`
	pub const SAR: Opcode = Opcode(0x1d);

	/// `CALLDATALOAD`
	pub const CALLDATALOAD: Opcode = Opcode(0x35);
	/// `CALLDATASIZE`
	pub const CALLDATASIZE: Opcode = Opcode(0x36);
	/// `CALLDATACOPY`
	pub const CALLDATACOPY: Opcode = Opcode(0x37);
	/// `CODESIZE`
	pub const CODESIZE: Opcode = Opcode(0x38);
	/// `CODECOPY`
	pub const CODECOPY: Opcode = Opcode(0x39);

	/// `POP`
	pub const POP: Opcode = Opcode(0x50);
	/// `MLOAD`
	pub const MLOAD: Opcode = Opcode(0x51);
	/// `MSTORE`
	pub const MSTORE: Opcode = Opcode(0x52);
	/// `MSTORE8`
	pub const MSTORE8: Opcode = Opcode(0x53);

	/// `JUMP`
	pub const JUMP: Opcode = Opcode(0x56);
	/// `JUMPI`
	pub const JUMPI: Opcode = Opcode(0x57);
	/// `PC`
	pub const PC: Opcode = Opcode(0x58);
	/// `MSIZE`
	pub const MSIZE: Opcode = Opcode(0x59);

	/// `JUMPDEST`
	pub const JUMPDEST: Opcode = Opcode(0x5b);
	/// `MCOPY`
	pub const MCOPY: Opcode = Opcode(0x5e);

	/// `PUSHn`
	pub const PUSH0: Opcode = Opcode(0x5f);
	pub const PUSH1: Opcode = Opcode(0x60);
	pub const PUSH2: Opcode = Opcode(0x61);
	pub const PUSH3: Opcode = Opcode(0x62);
	pub const PUSH4: Opcode = Opcode(0x63);
	pub const PUSH5: Opcode = Opcode(0x64);
	pub const PUSH6: Opcode = Opcode(0x65);
	pub const PUSH7: Opcode = Opcode(0x66);
	pub const PUSH8: Opcode = Opcode(0x67);
	pub const PUSH9: Opcode = Opcode(0x68);
	pub const PUSH10: Opcode = Opcode(0x69);
	pub const PUSH11: Opcode = Opcode(0x6a);
	pub const PUSH12: Opcode = Opcode(0x6b);
	pub const PUSH13: Opcode = Opcode(0x6c);
	pub const PUSH14: Opcode = Opcode(0x6d);
	pub const PUSH15: Opcode = Opcode(0x6e);
	pub const PUSH16: Opcode = Opcode(0x6f);
	pub const PUSH17: Opcode = Opcode(0x70);
	pub const PUSH18: Opcode = Opcode(0x71);
	pub const PUSH19: Opcode = Opcode(0x72);
	pub const PUSH20: Opcode = Opcode(0x73);
	pub const PUSH21: Opcode = Opcode(0x74);
	pub const PUSH22: Opcode = Opcode(0x75);
	pub const PUSH23: Opcode = Opcode(0x76);
	pub const PUSH24: Opcode = Opcode(0x77);
	pub const PUSH25: Opcode = Opcode(0x78);
	pub const PUSH26: Opcode = Opcode(0x79);
	pub const PUSH27: Opcode = Opcode(0x7a);
	pub const PUSH28: Opcode = Opcode(0x7b);
	pub const PUSH29: Opcode = Opcode(0x7c);
	pub const PUSH30: Opcode = Opcode(0x7d);
	pub const PUSH31: Opcode = Opcode(0x7e);
	pub const PUSH32: Opcode = Opcode(0x7f);

	/// `DUPn`
	pub const DUP1: Opcode = Opcode(0x80);
	pub const DUP2: Opcode = Opcode(0x81);
	pub const DUP3: Opcode = Opcode(0x82);
	pub const DUP4: Opcode = Opcode(0x83);
	pub const DUP5: Opcode = Opcode(0x84);
	pub const DUP6: Opcode = Opcode(0x85);
	pub const DUP7: Opcode = Opcode(0x86);
	pub const DUP8: Opcode = Opcode(0x87);
	pub const DUP9: Opcode = Opcode(0x88);
	pub const DUP10: Opcode = Opcode(0x89);
	pub const DUP11: Opcode = Opcode(0x8a);
	pub const DUP12: Opcode = Opcode(0x8b);
	pub const DUP13: Opcode = Opcode(0x8c);
	pub const DUP14: Opcode = Opcode(0x8d);
	pub const DUP15: Opcode = Opcode(0x8e);
	pub const DUP16: Opcode = Opcode(0x8f);

	/// `SWAPn`
	pub const SWAP1: Opcode = Opcode(0x90);
	pub const SWAP2: Opcode = Opcode(0x91);
	pub const SWAP3: Opcode = Opcode(0x92);
	pub const SWAP4: Opcode = Opcode(0x93);
	pub const SWAP5: Opcode = Opcode(0x94);
	pub const SWAP6: Opcode = Opcode(0x95);
	pub const SWAP7: Opcode = Opcode(0x96);
	pub const SWAP8: Opcode = Opcode(0x97);
	pub const SWAP9: Opcode = Opcode(0x98);
	pub const SWAP10: Opcode = Opcode(0x99);
	pub const SWAP11: Opcode = Opcode(0x9a);
	pub const SWAP12: Opcode = Opcode(0x9b);
	pub const SWAP13: Opcode = Opcode(0x9c);
	pub const SWAP14: Opcode = Opcode(0x9d);
	pub const SWAP15: Opcode = Opcode(0x9e);
	pub const SWAP16: Opcode = Opcode(0x9f);

	/// See [EIP-3541](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-3541.md)
	pub const EOFMAGIC: Opcode = Opcode(0xef);

	/// `RETURN`
	pub const RETURN: Opcode = Opcode(0xf3);

	/// `REVERT`
	pub const REVERT: Opcode = Opcode(0xfd);

	/// `INVALID`
	pub const INVALID: Opcode = Opcode(0xfe);
}

// External opcodes
impl Opcode {
	/// `SHA3`
	pub const SHA3: Opcode = Opcode(0x20);

	/// `ADDRESS`
	pub const ADDRESS: Opcode = Opcode(0x30);
	/// `BALANCE`
	pub const BALANCE: Opcode = Opcode(0x31);
	/// `ORIGIN`
	pub const ORIGIN: Opcode = Opcode(0x32);
	/// `CALLER`
	pub const CALLER: Opcode = Opcode(0x33);
	/// `CALLVALUE`
	pub const CALLVALUE: Opcode = Opcode(0x34);

	/// `GASPRICE`
	pub const GASPRICE: Opcode = Opcode(0x3a);
	/// `EXTCODESIZE`
	pub const EXTCODESIZE: Opcode = Opcode(0x3b);
	/// `EXTCODECOPY`
	pub const EXTCODECOPY: Opcode = Opcode(0x3c);
	/// `RETURNDATASIZE`
	pub const RETURNDATASIZE: Opcode = Opcode(0x3d);
	/// `RETURNDATACOPY`
	pub const RETURNDATACOPY: Opcode = Opcode(0x3e);
	/// `EXTCODEHASH`
	pub const EXTCODEHASH: Opcode = Opcode(0x3f);

	/// `BLOCKHASH`
	pub const BLOCKHASH: Opcode = Opcode(0x40);
	/// `COINBASE`
	pub const COINBASE: Opcode = Opcode(0x41);
	/// `TIMESTAMP`
	pub const TIMESTAMP: Opcode = Opcode(0x42);
	/// `NUMBER`
	pub const NUMBER: Opcode = Opcode(0x43);
	/// `DIFFICULTY`
	pub const DIFFICULTY: Opcode = Opcode(0x44);
	/// `GASLIMIT`
	pub const GASLIMIT: Opcode = Opcode(0x45);
	/// `CHAINID`
	pub const CHAINID: Opcode = Opcode(0x46);
	/// `SELFBALANCE`
	pub const SELFBALANCE: Opcode = Opcode(0x47);
	/// `BASEFEE`
	pub const BASEFEE: Opcode = Opcode(0x48);

	/// `SLOAD`
	pub const SLOAD: Opcode = Opcode(0x54);
	/// `SSTORE`
	pub const SSTORE: Opcode = Opcode(0x55);

	/// `GAS`
	pub const GAS: Opcode = Opcode(0x5a);

	/// `TLOAD`
	pub const TLOAD: Opcode = Opcode(0x5c);
	/// `TSTORE`
	pub const TSTORE: Opcode = Opcode(0x5d);

	/// `LOGn`
	pub const LOG0: Opcode = Opcode(0xa0);
	pub const LOG1: Opcode = Opcode(0xa1);
	pub const LOG2: Opcode = Opcode(0xa2);
	pub const LOG3: Opcode = Opcode(0xa3);
	pub const LOG4: Opcode = Opcode(0xa4);

	/// `CREATE`
	pub const CREATE: Opcode = Opcode(0xf0);
	/// `CALL`
	pub const CALL: Opcode = Opcode(0xf1);
	/// `CALLCODE`
	pub const CALLCODE: Opcode = Opcode(0xf2);

	/// `DELEGATECALL`
	pub const DELEGATECALL: Opcode = Opcode(0xf4);
	/// `CREATE2`
	pub const CREATE2: Opcode = Opcode(0xf5);

	/// `STATICCALL`
	pub const STATICCALL: Opcode = Opcode(0xfa);

	/// `SUICIDE`
	pub const SUICIDE: Opcode = Opcode(0xff);
}

impl Opcode {
	/// Whether the opcode is a push opcode.
	#[must_use]
	pub fn is_push(&self) -> Option<u8> {
		let value = self.0;
		if (0x60..=0x7f).contains(&value) {
			Some(value - 0x60 + 1)
		} else {
			None
		}
	}

	#[inline]
	#[must_use]
	pub const fn as_u8(&self) -> u8 {
		self.0
	}

	#[inline]
	#[must_use]
	pub const fn as_usize(&self) -> usize {
		self.0 as usize
	}


	pub const fn as_str(&self) -> &'static str {
		match self {
			&Self::STOP =>  "STOP",
			&Self::ADD =>  "ADD",
			&Self::MUL =>  "MUL",
			&Self::SUB =>  "SUB",
			&Self::DIV =>  "DIV",
			&Self::SDIV =>  "SDIV",
			&Self::MOD =>  "MOD",
			&Self::SMOD =>  "SMOD",
			&Self::ADDMOD =>  "ADDMOD",
			&Self::MULMOD =>  "MULMOD",
			&Self::EXP =>  "EXP",
			&Self::SIGNEXTEND =>  "SIGNEXTEND",
			&Self::LT =>  "LT",
			&Self::GT =>  "GT",
			&Self::SLT =>  "SLT",
			&Self::SGT =>  "SGT",
			&Self::EQ =>  "EQ",
			&Self::ISZERO =>  "ISZERO",
			&Self::AND =>  "AND",
			&Self::OR =>  "OR",
			&Self::XOR =>  "XOR",
			&Self::NOT =>  "NOT",
			&Self::BYTE =>  "BYTE",
			&Self::CALLDATALOAD =>  "CALLDATALOAD",
			&Self::CALLDATASIZE =>  "CALLDATASIZE",
			&Self::CALLDATACOPY =>  "CALLDATACOPY",
			&Self::CODESIZE =>  "CODESIZE",
			&Self::CODECOPY =>  "CODECOPY",
			&Self::SHL =>  "SHL",
			&Self::SHR =>  "SHR",
			&Self::SAR =>  "SAR",
			&Self::POP =>  "POP",
			&Self::MLOAD =>  "MLOAD",
			&Self::MSTORE =>  "MSTORE",
			&Self::MSTORE8 =>  "MSTORE8",
			&Self::JUMP =>  "JUMP",
			&Self::JUMPI =>  "JUMPI",
			&Self::PC =>  "PC",
			&Self::MSIZE =>  "MSIZE",
			&Self::JUMPDEST =>  "JUMPDEST",
			&Self::PUSH0 =>  "PUSH0",
			&Self::PUSH1 =>  "PUSH1",
			&Self::PUSH2 =>  "PUSH2",
			&Self::PUSH3 =>  "PUSH3",
			&Self::PUSH4 =>  "PUSH4",
			&Self::PUSH5 =>  "PUSH5",
			&Self::PUSH6 =>  "PUSH6",
			&Self::PUSH7 =>  "PUSH7",
			&Self::PUSH8 =>  "PUSH8",
			&Self::PUSH9 =>  "PUSH9",
			&Self::PUSH10 =>  "PUSH10",
			&Self::PUSH11 =>  "PUSH11",
			&Self::PUSH12 =>  "PUSH12",
			&Self::PUSH13 =>  "PUSH13",
			&Self::PUSH14 =>  "PUSH14",
			&Self::PUSH15 =>  "PUSH15",
			&Self::PUSH16 =>  "PUSH16",
			&Self::PUSH17 =>  "PUSH17",
			&Self::PUSH18 =>  "PUSH18",
			&Self::PUSH19 =>  "PUSH19",
			&Self::PUSH20 =>  "PUSH20",
			&Self::PUSH21 =>  "PUSH21",
			&Self::PUSH22 =>  "PUSH22",
			&Self::PUSH23 =>  "PUSH23",
			&Self::PUSH24 =>  "PUSH24",
			&Self::PUSH25 =>  "PUSH25",
			&Self::PUSH26 =>  "PUSH26",
			&Self::PUSH27 =>  "PUSH27",
			&Self::PUSH28 =>  "PUSH28",
			&Self::PUSH29 =>  "PUSH29",
			&Self::PUSH30 =>  "PUSH30",
			&Self::PUSH31 =>  "PUSH31",
			&Self::PUSH32 =>  "PUSH32",
			&Self::DUP1 =>  "DUP1",
			&Self::DUP2 =>  "DUP2",
			&Self::DUP3 =>  "DUP3",
			&Self::DUP4 =>  "DUP4",
			&Self::DUP5 =>  "DUP5",
			&Self::DUP6 =>  "DUP6",
			&Self::DUP7 =>  "DUP7",
			&Self::DUP8 =>  "DUP8",
			&Self::DUP9 =>  "DUP9",
			&Self::DUP10 =>  "DUP10",
			&Self::DUP11 =>  "DUP11",
			&Self::DUP12 =>  "DUP12",
			&Self::DUP13 =>  "DUP13",
			&Self::DUP14 =>  "DUP14",
			&Self::DUP15 =>  "DUP15",
			&Self::DUP16 =>  "DUP16",
			&Self::SWAP1 =>  "SWAP1",
			&Self::SWAP2 =>  "SWAP2",
			&Self::SWAP3 =>  "SWAP3",
			&Self::SWAP4 =>  "SWAP4",
			&Self::SWAP5 =>  "SWAP5",
			&Self::SWAP6 =>  "SWAP6",
			&Self::SWAP7 =>  "SWAP7",
			&Self::SWAP8 =>  "SWAP8",
			&Self::SWAP9 =>  "SWAP9",
			&Self::SWAP10 =>  "SWAP10",
			&Self::SWAP11 =>  "SWAP11",
			&Self::SWAP12 =>  "SWAP12",
			&Self::SWAP13 =>  "SWAP13",
			&Self::SWAP14 =>  "SWAP14",
			&Self::SWAP15 =>  "SWAP15",
			&Self::SWAP16 =>  "SWAP16",
			&Self::RETURN =>  "RETURN",
			&Self::REVERT =>  "REVERT",
			&Self::INVALID =>  "INVALID",
			&Self::EOFMAGIC =>  "EOFMAGIC",
			&Self::SHA3 =>  "SHA3",
			&Self::ADDRESS =>  "ADDRESS",
			&Self::BALANCE =>  "BALANCE",
			&Self::SELFBALANCE =>  "SELFBALANCE",
			&Self::BASEFEE =>  "BASEFEE",
			&Self::ORIGIN =>  "ORIGIN",
			&Self::CALLER =>  "CALLER",
			&Self::CALLVALUE =>  "CALLVALUE",
			&Self::GASPRICE =>  "GASPRICE",
			&Self::EXTCODESIZE =>  "EXTCODESIZE",
			&Self::EXTCODECOPY =>  "EXTCODECOPY",
			&Self::EXTCODEHASH =>  "EXTCODEHASH",
			&Self::RETURNDATASIZE =>  "RETURNDATASIZE",
			&Self::RETURNDATACOPY =>  "RETURNDATACOPY",
			&Self::BLOCKHASH =>  "BLOCKHASH",
			&Self::COINBASE =>  "COINBASE",
			&Self::TIMESTAMP =>  "TIMESTAMP",
			&Self::NUMBER =>  "NUMBER",
			&Self::DIFFICULTY =>  "DIFFICULTY",
			&Self::GASLIMIT =>  "GASLIMIT",
			&Self::SLOAD =>  "SLOAD",
			&Self::SSTORE =>  "SSTORE",
			&Self::GAS =>  "GAS",
			&Self::LOG0 =>  "LOG0",
			&Self::LOG1 =>  "LOG1",
			&Self::LOG2 =>  "LOG2",
			&Self::LOG3 =>  "LOG3",
			&Self::LOG4 =>  "LOG4",
			&Self::CREATE =>  "CREATE",
			&Self::CREATE2 =>  "CREATE2",
			&Self::CALL =>  "CALL",
			&Self::CALLCODE =>  "CALLCODE",
			&Self::DELEGATECALL =>  "DELEGATECALL",
			&Self::STATICCALL =>  "STATICCALL",
			&Self::SUICIDE =>  "SUICIDE",
			&Self::CHAINID =>  "CHAINID",
			_ => "UNKNOWN"
		}
	}
}
