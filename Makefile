build:
	cd ./crates/blve_compiler/ && wasm-pack build --target nodejs
	cp ./crates/blve_compiler/pkg/*.js ./npm-pkg/src/wasm-compiler/
	cp ./crates/blve_compiler/pkg/*.ts ./npm-pkg/src/wasm-compiler/
	cp ./crates/blve_compiler/pkg/*.wasm ./npm-pkg/src/wasm-compiler/
	cd ./npm-pkg/src/wasm-compiler && npm run build
	mkdir -p ./npm-pkg/dist/wasm-compiler
	cp -r ./npm-pkg/src/wasm-compiler/* ./npm-pkg/dist/wasm-compiler/

build-web:
	cd ./crates/blve_compiler/ && wasm-pack build --target web && open ./
