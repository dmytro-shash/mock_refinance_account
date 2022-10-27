# login
#near login

# build & test
mkdir -p res && ./build.sh && ./test.sh

# clean up previuos deployment
echo 'y' | near delete mock.ref_finance.testnet ref_finance.testnet

# create corresponding accoutns
near create-account mock.ref_finance.testnet --masterAccount ref_finance.testnet --initialBalance 10

# redeploy contracts
near deploy mock.ref_finance.testnet \
  --wasmFile ./res/ref_contract_mock.wasm \
  --initFunction 'new' \
  --initArgs '{}'

# making sure its alive
near call mock.ref_finance.testnet set_balance '{"account": "some_account.testnet", "amount": "1000"}' --accountId mock.ref_finance.testnet
near view mock.ref_finance.testnet view_balance '{"account": "some_account.testnet"}'