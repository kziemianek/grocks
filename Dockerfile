# syntax=docker/dockerfile:1

### Gramine Builder
##################################################
FROM gramineproject/gramine:stable-jammy AS build

ARG SGX_DEBUG=0

WORKDIR /usr/src/grocks
COPY . .

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl \
    libssl-dev \
    pkg-config \
    clang

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo --version

RUN echo $USER

RUN --mount=type=secret,id=signer,target=/root/.config/gramine/enclave-key.pem \
    make SGX=1 SGX_DEBUG=$SGX_DEBUG

### Gramine image
##################################################
FROM gramineproject/gramine:stable-jammy AS runner

WORKDIR /usr/src/grocks

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl \
    libssl-dev \
    ca-certificates \ 
    clang

COPY --from=build /usr/src/grocks/target/release/grocks /usr/src/grocks/target/release/grocks
COPY --from=build /usr/src/grocks/grocks.sig /usr/src/grocks/grocks.sig
COPY --from=build /usr/src/grocks/grocks.manifest.sgx /usr/src/grocks/grocks.manifest.sgx

ENTRYPOINT ["gramine-sgx", "grocks"]