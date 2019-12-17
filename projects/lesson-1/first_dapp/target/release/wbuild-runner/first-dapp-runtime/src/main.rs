
				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/Users/zhuqiang/Team3/projects/lesson-1/first_dapp/target/release/build/first-dapp-runtime-4c826ac39e10f40f/out/wasm_binary.rs",
						"/Users/zhuqiang/Team3/projects/lesson-1/first_dapp/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base",
					)
				}
			