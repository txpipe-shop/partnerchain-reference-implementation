FROM docker.io/paritytech/ci-unified:latest AS builder

WORKDIR /polkadot
COPY . /polkadot

RUN cargo fetch
RUN cargo build --workspace --locked --release

RUN mv /polkadot/target/release/wbuild/minimal-template-runtime/minimal_template_runtime.wasm /polkadot/
RUN mv /polkadot/target/release/minimal-template-node /polkadot/
RUN rm -rf /polkadot/target/

FROM docker.io/paritytech/ci-unified:latest AS chain_spec_builder

WORKDIR /polkadot

COPY --from=builder  /polkadot .
RUN cargo fetch
RUN cargo install staging-chain-spec-builder

RUN /usr/local/cargo/bin/chain-spec-builder create --relay-chain "dev" --para-id 1000 --runtime minimal_template_runtime.wasm named-preset development

FROM docker.io/parity/base-bin:latest

COPY --from=builder /polkadot/minimal-template-node /usr/local/bin
COPY --from=builder /polkadot/dev_chain_spec.json /polkadot/
COPY --from=chain_spec_builder /polkadot/chain_spec.json /polkadot/

USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /polkadot polkadot && \
	mkdir -p /data /polkadot/.local/share && \
	chown -R polkadot:polkadot /data && \
	ln -s /data /polkadot/.local/share/polkadot && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/minimal-template-node --version

USER polkadot

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/minimal-template-node"]
