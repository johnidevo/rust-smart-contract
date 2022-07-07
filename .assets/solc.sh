#solc --bin-runtime --optimize -o ./Artifacts ./Contract.sol --overwrite

#solc --bin-runtime --overwrite --asm --optimize -o ./Artifacts/Additionx.bin-runtime ./Contracts/Addition.sol

pwd 
solc --bin-runtime --overwrite --asm --optimize -o . ifelse.sol