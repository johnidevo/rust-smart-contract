#curl --head "82.193.101.137"
#curl --head "82.193.101.137"
#curl --head http://40.85.87.151
#curl --head http://3.86.76.191

#curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_accounts","params":[],"id":67}' "82.193.101.137"
#curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"web3_clientVersion","params":[],"id":1}' "92.104.106.246"

#curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_getBalance","params": ["0xb5e0b398bcc3ec1621816a176160959e73f53f5e", "latest"],"id":1}' http://127.0.0.1:8545
#curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_getStorageAt","params": ["0xb5e0b398bcc3ec1621816a176160959e73f53f5e", "0x0", "latest"],"id":1}' http://127.0.0.1:8545
#curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_getTransactionCount","params": ["0xb5e0b398bcc3ec1621816a176160959e73f53f5e", "latest"],"id":1}' http://127.0.0.1:8545
#mining
#curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_mining","params": [],"id":71}' http://127.0.0.1:8545
#curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_hashrate","params": [],"id":71}' http://127.0.0.1:8545
#3362396163613030
#curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_gasPrice","params": [],"id":73}' http://127.0.0.1:8545
#get code
#curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_getCode","params": ["0xb5e0b398bcc3ec1621816a176160959e73f53f5e", "latest"],"id":1}' http://127.0.0.1:8545
#eth_compileSolidity
curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_compileSolidity","params":["contract test { function multiply(uint a) returns(uint d) {   return a * 7;   } }"],"id":1}' http://127.0.0.1:8545



