FROM --platform=riscv64 riscv64/debian:experimental

# RUN apk add --no-cache build-essential curl


# RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
# RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# COPY . .

# CMD ["cargo", "run"]