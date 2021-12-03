FROM --platform=riscv64 riscv64/ubuntu

RUN apt-get update 
RUN apt-get install -y --no-install-recommends apt-utils  build-essential git curl wget 

COPY ./target/riscv64gc-unknown-linux-gnu/debug/cort ./cort

CMD ["./cort"]
