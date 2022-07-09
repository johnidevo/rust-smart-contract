
//use primitive_types::U256;

impl Opcode {

	pub fn describe(&self) {
		match self {

			Opcode::STOP(line) => println!("0x{:x}\tSTOP\t\tHalts execution", line),
			Opcode::ADD(line) => println!("0x{:x}\tADD\t\tAddition operation", line),
			Opcode::MUL(line) => println!("0x{:x}\tMUL\t\tMultiplication operation", line),
			Opcode::SUB(line) => println!("0x{:x}\tSUB\t\tSubtraction operation", line),
			Opcode::DIV(line) => println!("0x{:x}\tDIV\t\tInteger division operation", line),
			Opcode::SDIV(line) => println!("0x{:x}\tSDIV\t\tSigned integer division operation (truncated)", line),
			Opcode::MOD(line) => println!("0x{:x}\tMOD\t\tModulo remainder operation", line),
			Opcode::SMOD(line) => println!("0x{:x}\tSMOD\t\tSigned modulo remainder operation", line),
			Opcode::ADDMOD(line) => println!("0x{:x}\tADDMOD\t\tModulo addition operation", line),
			Opcode::MULMOD(line) => println!("0x{:x}\tMULMOD\t\tModulo multiplication operation", line),
			Opcode::EXP(line) => println!("0x{:x}\tEXP\t\tExponential operation", line),
			Opcode::SIGNEXTEND(line) => println!("0x{:x}\tSIGNEXTEND\tExtend length of two’s complement signed integer", line),
			Opcode::LT(line) => println!("0x{:x}\tLT\t\tLess-than comparison", line),
			Opcode::GT(line) => println!("0x{:x}\tGT\t\tGreater-than comparison", line),
			Opcode::SLT(line) => println!("0x{:x}\tSLT\t\tSigned less-than comparison", line),
			Opcode::SGT(line) => println!("0x{:x}\tSGT\t\tSigned greater-than comparison", line),
			Opcode::EQ(line) => println!("0x{:x}\tEQ\t\tEquality comparison", line),
			Opcode::ISZERO(line) => println!("0x{:x}\tISZERO\t\tSimple not operator", line),
			Opcode::AND(line) => println!("0x{:x}\tAND\t\tBitwise AND operation", line),
			Opcode::OR(line) => println!("0x{:x}\tOR\t\tBitwise OR operation", line),
			Opcode::XOR(line) => println!("0x{:x}\tXOR\t\tBitwise XOR operation", line),
			Opcode::NOT(line) => println!("0x{:x}\tNOT\t\tBitwise NOT operation", line),
			Opcode::BYTE(line) => println!("0x{:x}\tBYTE\t\tRetrieve single byte from word", line),
			Opcode::SHA3(line) => println!("0x{:x}\tSHA3\t\tCompute Keccak-256 hash", line),
			Opcode::ADDRESS(line) => println!("0x{:x}\t\tADDRESS\tGet address of currently executing account", line),
			Opcode::BALANCE(line) => println!("0x{:x}\t\tBALANCE\tGet balance of the given account", line),
			Opcode::ORIGIN(line) => println!("0x{:x}\tORIGIN\t\tGet execution origination address", line),
			Opcode::CALLER(line) => println!("0x{:x}\tCALLER\t\tGet caller address", line),
			Opcode::CALLVALUE(line) => println!("0x{:x}\tCALLVALUE\tGet deposited value by the instruction/transaction responsible for this execution", line),
			Opcode::CALLDATALOAD(line) => println!("0x{:x}\tCALLDATALOAD\tGet input data of current environment", line),
			Opcode::CALLDATASIZE(line) => println!("0x{:x}\tCALLDATASIZE\tGet size of input data in current environment", line),
			Opcode::CALLDATACOPY(line) => println!("0x{:x}\tCALLDATACOPY\tCopy input data in current environment to memory", line),
			Opcode::CODESIZE(line) => println!("0x{:x}\tCODESIZE\tGet size of code running in current environment", line),
			Opcode::CODECOPY(line) => println!("0x{:x}\tCODECOPY\tCopy code running in current environment to memory", line),
			Opcode::GASPRICE(line) => println!("0x{:x}\tGASPRICE\tGet price of gas in current environment", line),
			Opcode::EXTCODESIZE(line) => println!("0x{:x}\tEXTCODESIZE\tGet size of an account’s code", line),
			Opcode::EXTCODECOPY(line) => println!("0x{:x}\tEXTCODECOPY\tCopy an account’s code to memory", line),
			Opcode::BLOCKHASH(line) => println!("0x{:x}\tBLOCKHASH\tGet the hash of one of the 256 most recent complete blocks", line),
			Opcode::COINBASE(line) => println!("0x{:x}\tCOINBASE\t\tGet the block’s beneficiary address", line),
			Opcode::TIMESTAMP(line) => println!("0x{:x}\tTIMESTAMP\t\tGet the block’s timestamp", line),
			Opcode::NUMBER(line) => println!("0x{:x}\tNUMBER\t\tGet the block’s number", line),
			Opcode::DIFFICULTY(line) => println!("0x{:x}\tDIFFICULTY\tGet the block’s difficulty", line),
			Opcode::GASLIMIT(line) => println!("0x{:x}\tGASLIMIT\t\tGet the block’s gas limit", line),
			Opcode::POP(line) => println!("0x{:x}\tPOP\t\tRemove item from stack", line),
			Opcode::MLOAD(line) => println!("0x{:x}\tMLOAD\t\tLoad word from memory", line),
			Opcode::MSTORE(line) => println!("0x{:x}\tMSTORE\t\tSave word to memory", line),
			Opcode::MSTORE8(line) => println!("0x{:x}\tMSTORE8\t\tSave byte to memory", line),
			Opcode::SLOAD(line) => println!("0x{:x}\tSLOAD\t\tLoad word from storage", line),
			Opcode::SSTORE(line) => println!("0x{:x}\tSSTORE\t\tSave word to storage", line),
			Opcode::JUMP(line) => println!("0x{:x}\tJUMP\t\tAlter the program counter", line),
			Opcode::JUMPI(line) => println!("0x{:x}\tJUMPI\t\tConditionally alter the program counter", line),
			Opcode::PC(line) => println!("0x{:x}\tPC\t\tGet the value of the program counter prior to the increment corresponding to this instruction", line),
			Opcode::MSIZE(line) => println!("0x{:x}\tMSIZE\t\tGet the size of active memory in bytes", line),
			Opcode::GAS(line) => println!("0x{:x}\tGAS\t\tGet the amount of available gas, including the corresponding reduction for the cost of this instruction", line),
			Opcode::JUMPDEST(line) => println!("0x{:x}\tJUMPDEST\tMark a valid destination for jumps", line),
			Opcode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\t\tPlace 1-byte item on stack 0x{:x}", line, x),
			//Opcode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\tPlace 1 byte item on stack", line),
			//Opcode::PUSH2(line) => println!("0x{:x}\tPUSH2\tPlace 2 byte item on stack", line),
			Opcode::PUSH2(line, x0, x1) => println!("0x{:x}\tPUSH2\t\tPlace 2-bytes item on stack 0x{:x} 0x{:x}", line, x0, x1),
			Opcode::PUSH3(line, x0, x1, x2) => println!("0x{:x}\tPUSH3\t\tPlace 3 byte item on stack 0x{:x} 0x{:x} 0x{:x}", line, x0, x1, x2),
			Opcode::PUSH4(line, x0, x1, x2, x3) => println!("0x{:x}\tPUSH4\t\tPlace 4 byte item on stack 0x{:x} 0x{:x} 0x{:x} 0x{:x}", line, x0, x1, x2, x3),
			Opcode::PUSH5(line, x0, x1, x2, x3, x4) => println!("0x{:x}\tPUSH5\t\tPlace 5 byte item on stack 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x}", line, x0, x1, x2, x3, x4),
			Opcode::PUSH6(line, x0, x1, x2, x3, x4, x5) => println!("0x{:x}\tPUSH6\t\tPlace 6 byte item on stack 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x}", line, x0, x1, x2, x3, x4, x5),
			Opcode::PUSH7(line, x0, x1, x2, x3, x4, x5, x6) => println!("0x{:x}\tPUSH7\t\tPlace 7 byte item on stack 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x}", line, x0, x1, x2, x3, x4, x5, x6),
			Opcode::PUSH8(line, x0, x1, x2, x3, x4, x5, x6, x7) => println!("0x{:x}\tPUSH8\t\tPlace 8 byte item on stack 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x}", line, x0, x1, x2, x3, x4, x5, x6, x7),
			Opcode::PUSH9(line, x0, x1, x2, x3, x4, x5, x6, x7, x8) => println!("0x{:x}\tPUSH9\t\tPlace 9 byte item on stack 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x} 0x{:x}", line, x0, x1, x2, x3, x4, x5, x6, x7, x8),
			Opcode::PUSH10(line) => println!("0x{:x}\tPUSH10\t\tPlace 10 byte item on stack", line),
			Opcode::PUSH11(line) => println!("0x{:x}\tPUSH11\t\tPlace 11 byte item on stack", line),
			Opcode::PUSH12(line) => println!("0x{:x}\tPUSH12\t\tPlace 12 byte item on stack", line),
			Opcode::PUSH13(line) => println!("0x{:x}\tPUSH13\t\tPlace 13 byte item on stack", line),
			Opcode::PUSH14(line) => println!("0x{:x}\tPUSH14\t\tPlace 14 byte item on stack", line),
			Opcode::PUSH15(line) => println!("0x{:x}\tPUSH15\t\tPlace 15 byte item on stack", line),
			Opcode::PUSH16(line) => println!("0x{:x}\tPUSH16\t\tPlace 16 byte item on stack", line),
			Opcode::PUSH17(line) => println!("0x{:x}\tPUSH17\t\tPlace 17 byte item on stack", line),
			Opcode::PUSH18(line) => println!("0x{:x}\tPUSH18\t\tPlace 18 byte item on stack", line),
			Opcode::PUSH19(line) => println!("0x{:x}\tPUSH19\t\tPlace 19 byte item on stack", line),
			Opcode::PUSH20(line) => println!("0x{:x}\tPUSH20\t\tPlace 20 byte item on stack", line),
			Opcode::PUSH21(line) => println!("0x{:x}\tPUSH21\t\tPlace 21 byte item on stack", line),
			Opcode::PUSH22(line) => println!("0x{:x}\tPUSH22\t\tPlace 22 byte item on stack", line),
			Opcode::PUSH23(line) => println!("0x{:x}\tPUSH23\t\tPlace 23 byte item on stack", line),
			Opcode::PUSH24(line) => println!("0x{:x}\tPUSH24\t\tPlace 24 byte item on stack", line),
			Opcode::PUSH25(line) => println!("0x{:x}\tPUSH25\t\tPlace 25 byte item on stack", line),
			Opcode::PUSH26(line) => println!("0x{:x}\tPUSH26\t\tPlace 26 byte item on stack", line),
			Opcode::PUSH27(line) => println!("0x{:x}\tPUSH27\t\tPlace 27 byte item on stack", line),
			Opcode::PUSH28(line) => println!("0x{:x}\tPUSH28\t\tPlace 28 byte item on stack", line),
			Opcode::PUSH29(line) => println!("0x{:x}\tPUSH29\t\tPlace 29 byte item on stack", line),
			Opcode::PUSH30(line) => println!("0x{:x}\tPUSH30\t\tPlace 30 byte item on stack", line),
			Opcode::PUSH31(line) => println!("0x{:x}\tPUSH31\t\tPlace 31 byte item on stack", line),
			Opcode::PUSH32(line) => println!("0x{:x}\tPUSH32\t\tPlace 32 byte (full word) item on stack", line),
			Opcode::DUP1(line) => println!("0x{:x}\tDUP1\t\tDuplicate 1st stack item", line),
			Opcode::DUP2(line) => println!("0x{:x}\tDUP2\t\tDuplicate 2nd stack item", line),
			Opcode::DUP3(line) => println!("0x{:x}\tDUP3\t\tDuplicate 3rd stack item", line),
			Opcode::DUP4(line) => println!("0x{:x}\tDUP4\t\tDuplicate 4th stack item", line),
			Opcode::DUP5(line) => println!("0x{:x}\tDUP5\t\tDuplicate 5th stack item", line),
			Opcode::DUP6(line) => println!("0x{:x}\tDUP6\t\tDuplicate 6th stack item", line),
			Opcode::DUP7(line) => println!("0x{:x}\tDUP7\t\tDuplicate 7th stack item", line),
			Opcode::DUP8(line) => println!("0x{:x}\tDUP8\t\tDuplicate 8th stack item", line),
			Opcode::DUP9(line) => println!("0x{:x}\tDUP9\t\tDuplicate 9th stack item", line),
			Opcode::DUP10(line) => println!("0x{:x}\tDUP10\t\tDuplicate 10th stack item", line),
			Opcode::DUP11(line) => println!("0x{:x}\tDUP11\t\tDuplicate 11th stack item", line),
			Opcode::DUP12(line) => println!("0x{:x}\tDUP12\t\tDuplicate 12th stack item", line),
			Opcode::DUP13(line) => println!("0x{:x}\tDUP13\t\tDuplicate 13th stack item", line),
			Opcode::DUP14(line) => println!("0x{:x}\tDUP14\t\tDuplicate 14th stack item", line),
			Opcode::DUP15(line) => println!("0x{:x}\tDUP15\t\tDuplicate 15th stack item", line),
			Opcode::DUP16(line) => println!("0x{:x}\tDUP16\t\tDuplicate 16th stack item", line),
			Opcode::SWAP1(line) => println!("0x{:x}\tSWAP1\t\tExchange 1st and 2nd stack items", line),
			Opcode::SWAP2(line) => println!("0x{:x}\tSWAP2\t\tExchange 1st and 3rd stack items", line),
			Opcode::SWAP3(line) => println!("0x{:x}\tSWAP3\t\tExchange 1st and 4th stack items", line),
			Opcode::SWAP4(line) => println!("0x{:x}\tSWAP4\t\tExchange 1st and 5th stack items", line),
			Opcode::SWAP5(line) => println!("0x{:x}\tSWAP5\t\tExchange 1st and 6th stack items", line),
			Opcode::SWAP6(line) => println!("0x{:x}\tSWAP6\t\tExchange 1st and 7th stack items", line),
			Opcode::SWAP7(line) => println!("0x{:x}\tSWAP7\t\tExchange 1st and 8th stack items", line),
			Opcode::SWAP8(line) => println!("0x{:x}\tSWAP8\t\tExchange 1st and 9th stack items", line),
			Opcode::SWAP9(line) => println!("0x{:x}\tSWAP9\t\tExchange 1st and 10th stack items", line),
			Opcode::SWAP10(line) => println!("0x{:x}\tSWAP10\t\tExchange 1st and 11th stack items", line),
			Opcode::SWAP11(line) => println!("0x{:x}\tSWAP11\t\tExchange 1st and 12th stack items", line),
			Opcode::SWAP12(line) => println!("0x{:x}\tSWAP12\t\tExchange 1st and 13th stack items", line),
			Opcode::SWAP13(line) => println!("0x{:x}\tSWAP13\t\tExchange 1st and 14th stack items", line),
			Opcode::SWAP14(line) => println!("0x{:x}\tSWAP14\t\tExchange 1st and 15th stack items", line),
			Opcode::SWAP15(line) => println!("0x{:x}\tSWAP15\t\tExchange 1st and 16th stack items", line),
			Opcode::SWAP16(line) => println!("0x{:x}\tSWAP16\t\tExchange 1st and 17th stack items", line),
			Opcode::LOG0(line) => println!("0x{:x}\tLOG0\t\tAppend log record with no topics", line),
			Opcode::LOG1(line) => println!("0x{:x}\tLOG1\t\tAppend log record with one topic", line),
			Opcode::LOG2(line) => println!("0x{:x}\tLOG2\t\tAppend log record with two topics", line),
			Opcode::LOG3(line) => println!("0x{:x}\tLOG3\t\tAppend log record with three topics", line),
			Opcode::LOG4(line) => println!("0x{:x}\tLOG4\t\tAppend log record with four topics", line),
			Opcode::CREATE(line) => println!("0x{:x}\tCREATE\t\tCreate a new account with associated code", line),
			Opcode::CALL(line) => println!("0x{:x}\tCALL\t\tMessage-call into an account", line),
			Opcode::CALLCODE(line) => println!("0x{:x}\tCALLCODE\t\tMessage-call into this account with alternative account’s code", line),
			Opcode::RETURN(line) => println!("0x{:x}\tRETURN\t\tHalt execution returning output data", line),
			Opcode::DELEGATECALL(line) => println!("0x{:x}\tDELEGATECALL\tMessage-call into this account with an alternative account’s code, but persisting the current values for sender and value", line),
			Opcode::INVALID(line) => println!("0x{:x}\tINVALID\t\tDesignated invalid instruction", line),
			Opcode::SELFDESTRUCT(line) => println!("0x{:x}\tSELFDESTRUCT\tHalt execution and register account for later deletion", line),

			Opcode::PRINT(line) => println!("0x{:x}\tPRINT\tHalts execution", line),

			/*
			Opcode::CALLDATASIZE(line) => println!("0x{:x}\tCALLDATASIZE\tHalts execution", line),
			Opcode::CALLDATALOAD(line) => println!("0x{:x}\tCALLDATALOAD\tHalts execution", line),
			
			Opcode::MLOAD(line) => println!("0x{:x}\tPRINT\tHalts execution", line),
			Opcode::MSTORE(line) => println!("0x{:x}\tJUMPI\tHalts execution", line),
			Opcode::MSTORE8(line) => println!("0x{:x}\tSLT\tAddition operation", line),
			
			Opcode::PRINT(line) => println!("0x{:x}\tPRINT\tHalts execution", line),
			Opcode::JUMPI(line) => println!("0x{:x}\tJUMPI\tHalts execution", line),
			Opcode::SLT(line) => println!("0x{:x}\tSLT\tAddition operation", line),
			
			Opcode::JUMP(line) => println!("0x{:x}\tJUMP\tHalts execution", line),
		
			Opcode::STOP(line) => println!("0x{:x}\tSTOP\tHalts execution", line),
			Opcode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
			Opcode::MUL(line) => println!("0x{:x}\tMUL\tMultiplication operation", line),
			Opcode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}", line, x),
			Opcode::PUSH2(line, x0, x1) => println!("0x{:x}\tPUSH2\tPlace 2-bytes item on the stack 0x{:x} 0x{:x}", line, x0, x1),
			
			*/
			_ => println!("Unknown opcode")
		}
	}
}

#[derive(Debug)]
pub enum Opcode {
	STOP(usize), // 0x00
	ADD(usize), // 0x01
	MUL(usize), // 0x02
	SUB(usize), // 0x03
	DIV(usize), // 0x04
	SDIV(usize), // 0x05
	MOD(usize), // 0x06
	SMOD(usize), // 0x07
	ADDMOD(usize), // 0x08
	MULMOD(usize), // 0x09
	EXP(usize), // 0x0a
	SIGNEXTEND(usize), // 0x0b
	LT(usize), // 0x10
	GT(usize), // 0x11
	SLT(usize), // 0x12
	SGT(usize), // 0x13
	EQ(usize), // 0x14
	ISZERO(usize), // 0x15
	AND(usize), // 0x16
	OR(usize), // 0x17
	XOR(usize), // 0x18
	NOT(usize), // 0x19
	BYTE(usize), // 0x1a
	SHA3(usize), // 0x20
	ADDRESS(usize), // 0x30
	BALANCE(usize), // 0x31
	ORIGIN(usize), // 0x32
	CALLER(usize), // 0x33
	CALLVALUE(usize), // 0x34
	CALLDATALOAD(usize), // 0x35
	CALLDATASIZE(usize), // 0x36
	CALLDATACOPY(usize), // 0x37
	CODESIZE(usize), // 0x38
	CODECOPY(usize), // 0x39
	GASPRICE(usize), // 0x3a
	EXTCODESIZE(usize), // 0x3b
	EXTCODECOPY(usize), // 0x3c
	BLOCKHASH(usize), // 0x40
	COINBASE(usize), // 0x41
	TIMESTAMP(usize), // 0x42
	NUMBER(usize), // 0x43
	DIFFICULTY(usize), // 0x44
	GASLIMIT(usize), // 0x45
	POP(usize), // 0x50
	MLOAD(usize), // 0x51
	MSTORE(usize), // 0x52
	MSTORE8(usize), // 0x53
	SLOAD(usize), // 0x54
	SSTORE(usize), // 0x55
	JUMP(usize), // 0x56
	JUMPI(usize), // 0x57
	PC(usize), // 0x58
	MSIZE(usize), // 0x59
	GAS(usize), // 0x5a
	JUMPDEST(usize), // 0x5b
	PUSH1(usize, u8), // 0x60
	PUSH2(usize, u8, u8), // 0x61
	PUSH3(usize, u8, u8, u8), // 0x62
	PUSH4(usize, u8, u8, u8, u8), // 0x63
	PUSH5(usize, u8, u8, u8, u8, u8), // 0x64
	PUSH6(usize, u8, u8, u8, u8, u8, u8), // 0x65
	PUSH7(usize, u8, u8, u8, u8, u8, u8, u8), // 0x66
	PUSH8(usize, u8, u8, u8, u8, u8, u8, u8, u8), // 0x67
	PUSH9(usize, u8, u8, u8, u8, u8, u8, u8, u8, u8), // 0x68
	PUSH10(usize), // 0x69
	PUSH11(usize), // 0x6a
	PUSH12(usize), // 0x6b
	PUSH13(usize), // 0x6c
	PUSH14(usize), // 0x6d
	PUSH15(usize), // 0x6e
	PUSH16(usize), // 0x6f
	PUSH17(usize), // 0x70
	PUSH18(usize), // 0x71
	PUSH19(usize), // 0x72
	PUSH20(usize), // 0x73
	PUSH21(usize), // 0x74
	PUSH22(usize), // 0x75
	PUSH23(usize), // 0x76
	PUSH24(usize), // 0x77
	PUSH25(usize), // 0x78
	PUSH26(usize), // 0x79
	PUSH27(usize), // 0x7a
	PUSH28(usize), // 0x7b
	PUSH29(usize), // 0x7c
	PUSH30(usize), // 0x7d
	PUSH31(usize), // 0x7e
	PUSH32(usize), // 0x7f
	DUP1(usize), // 0x80
	DUP2(usize), // 0x81
	DUP3(usize), // 0x82
	DUP4(usize), // 0x83
	DUP5(usize), // 0x84
	DUP6(usize), // 0x85
	DUP7(usize), // 0x86
	DUP8(usize), // 0x87
	DUP9(usize), // 0x88
	DUP10(usize), // 0x89
	DUP11(usize), // 0x8a
	DUP12(usize), // 0x8b
	DUP13(usize), // 0x8c
	DUP14(usize), // 0x8d
	DUP15(usize), // 0x8e
	DUP16(usize), // 0x8f
	SWAP1(usize), // 0x90
	SWAP2(usize), // 0x91
	SWAP3(usize), // 0x92
	SWAP4(usize), // 0x93
	SWAP5(usize), // 0x94
	SWAP6(usize), // 0x95
	SWAP7(usize), // 0x96
	SWAP8(usize), // 0x97
	SWAP9(usize), // 0x98
	SWAP10(usize), // 0x99
	SWAP11(usize), // 0x9a
	SWAP12(usize), // 0x9b
	SWAP13(usize), // 0x9c
	SWAP14(usize), // 0x9d
	SWAP15(usize), // 0x9e
	SWAP16(usize), // 0x9f
	LOG0(usize), // 0xa0
	LOG1(usize), // 0xa1
	LOG2(usize), // 0xa2
	LOG3(usize), // 0xa3
	LOG4(usize), // 0xa4
	CREATE(usize), // 0xf0
	CALL(usize), // 0xf1
	CALLCODE(usize), // 0xf2
	RETURN(usize), // 0xf3
	DELEGATECALL(usize), // 0xf4
	INVALID(usize), //0xfe
	SELFDESTRUCT(usize), // 0xff
	
	PRINT(usize), // 0x00
	
/*
	CALLDATASIZE(usize), // 0x00
	CALLDATALOAD(usize), // 0x01
	
	MLOAD(usize), // 0x00
	MSTORE(usize), // 0x01
	MSTORE8(usize), // 0x02
	
	PRINT(usize), // 0x00
	
	JUMP(usize), // 0x01
	JUMPI(usize), // 0x01
	SLT(usize), // 0x02
	
	STOP(usize), // 0x00
	ADD(usize), // 0x01
	MUL(usize), // 0x02
	
	// Push operations
	PUSH1(usize, U256), // 0x60
	PUSH2(usize, U256), // 0x61
	PUSH32(usize, U256),
	
	PUSH1(usize, u8), // 0x60
	PUSH2(usize, u8, u8), // 0x61
	PUSH32(usize, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8), // 0x7f 
*/
	// test commit mesage terminal
	EOF,
}
