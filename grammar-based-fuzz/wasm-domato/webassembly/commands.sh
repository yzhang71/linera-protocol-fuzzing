for i in {1..30}; do
  echo cargo fuzz run universal_cranelift_raw /home/jun/04-07-2023/wasmer-interface-fuzzing/grammar-based-fuzz/wasm-domato/webassembly/split_testcases/folder${i} --features='"'universal cranelift'"'
done

