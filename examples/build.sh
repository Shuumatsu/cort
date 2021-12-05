cp ../cort.h ./lib/cort.h 
cp ../target/debug/libcort.so ./lib/libcort.so

cd build && cmake .. && make 

# docker run --rm -it -v $(pwd):/project --entrypoint bash --platform=riscv64 riscv64/ubuntu