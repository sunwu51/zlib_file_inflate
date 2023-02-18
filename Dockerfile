FROM scratch
COPY target/wasm32-wasi/release/decompress.wasm decompress.wasm
ENTRYPOINT ["decompress.wasm"]

# You need 
# - 1 a zlib file xx.zlib in the root dir
# - 2 a docker desktop with version > 4.15 and enable containerd image store feature
# - 3 rust
# then you should build the code to wasm: cargo b -r --target wasm32-wasi

# docker build --platform=wasi/wasm32 -t demo .
# docker run --platform=wasi/wasm32 --runtime=io.containerd.wasmedge.v1 -v `pwd`:/test demo /test/xx.zlib
