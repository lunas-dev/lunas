build:
	cd ./crates/blve_compiler/ && wasm-pack build --target nodejs
	cp ./crates/blve_compiler/pkg/*.js ./npm-pkgs/blve/src/wasm-compiler/
	cp ./crates/blve_compiler/pkg/*.ts ./npm-pkgs/blve/src/wasm-compiler/
	cp ./crates/blve_compiler/pkg/*.wasm ./npm-pkgs/blve/src/wasm-compiler/
	cd ./npm-pkgs/blve/src/wasm-compiler && npm run build
	mkdir -p ./npm-pkgs/blve/dist/wasm-compiler
	cp -r ./npm-pkgs/blve/src/wasm-compiler/* ./npm-pkgs/blve/dist/wasm-compiler/

build-web:
	cd ./crates/blve_compiler/ && wasm-pack build --target web
