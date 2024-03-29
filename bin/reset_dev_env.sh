echo 'Deploying contract to anvil...'
# This uses a private key that comes with anvil -- NEVER PUT YOUR OWN IN VERSION CONTROL
# But this key is fine for testing against a local development environment
# Also not the constructor args -- this will initialize the contract to point to an empty root CID
ADDRESS=$(forge \
	create \
	--rpc-url http://localhost:8545 \
	--chain 31337 \
	--private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
	src/eth/RootCid.sol:RootCid \
	--constructor-args "[0x0100000000000000000000000000000000000000000000000000000000000000,0x0000000000000000000000000000000000000000000000000000000000000000]" |
	grep -o 'Deployed to: [0-9a-fA-Fx]\+' | sed 's/Deployed to: //')

cargo run -- device create \
	--alias dev \
	--eth-rpc http://localhost:8545 \
	--contract-address ${ADDRESS} \
	--eth-chain-id 31337 \
	--ipfs-url http://localhost:5001 \
	--ipfs-gateway-url http://localhost:8080

cargo run -- device set dev


rm -rf bin/test/.fs
cargo run -- --dir bin/test init
cargo run -- --dir bin/test pull
cargo run -- --dir bin/test stage
cargo run -- --dir bin/test tag --name audio   --path audio/freak-mic-test.mp3        --value '{"title": "Freak on a Leash (Sample)", "project": "mic_test"}'
cargo run -- --dir bin/test tag --name writing --path writing/hello_world.md      --value '{"title": "Hello World", "description": "A lil hello!", "genre": "blog"}'
cargo run -- --dir bin/test tag --name visual  --path visual/petting_turtles.jpg --value '{"title": "Draw me, Naked, Petting the Turtles", "location": "New York", "medium": "blue ink on lined paper"}'
cargo run -- --dir bin/test  --admin-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 push

echo 'APP_NAME=Krondor.Org' > web.config.dev
echo 'APP_CONTRACT_ADDRESS='${ADDRESS} >> web.config.dev
echo 'APP_CHAIN_ID=31337' >> web.config.dev
echo 'APP_RPC_URL=http://localhost:8545' >> web.config.dev
echo 'APP_IPFS_GATEWAY_URL=http://localhost:8080' >> web.config.dev