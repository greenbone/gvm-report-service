FROM debian:bookworm-slim AS typst

ARG TARGETARCH
ARG TYPST_VERSION=0.14.2

ARG TYPST_SHA256_AMD64
ARG TYPST_SHA256_ARM64

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    tar \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

RUN case "$TARGETARCH" in \
      amd64) \
        TYPST_ARCH="x86_64-unknown-linux-musl"; \
        TYPST_SHA256="$TYPST_SHA256_AMD64" \
        ;; \
      arm64) \
        TYPST_ARCH="aarch64-unknown-linux-musl"; \
        TYPST_SHA256="$TYPST_SHA256_ARM64" \
        ;; \
      *) \
        echo "unsupported TARGETARCH=$TARGETARCH" >&2; \
        exit 1 \
        ;; \
    esac \
    && test -n "$TYPST_SHA256" \
    && curl -fsSL \
      "https://github.com/typst/typst/releases/download/v${TYPST_VERSION}/typst-${TYPST_ARCH}.tar.xz" \
      -o /tmp/typst.tar.xz \
    && echo "${TYPST_SHA256}  /tmp/typst.tar.xz" | sha256sum -c - \
    && mkdir -p /tmp/typst \
    && tar -xJf /tmp/typst.tar.xz -C /tmp/typst --strip-components=1 \
    && install -m 0755 /tmp/typst/typst /usr/local/bin/typst \
    && rm -rf /tmp/typst /tmp/typst.tar.xz


FROM rust:bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY crates/gvm-report-core/Cargo.toml crates/gvm-report-core/Cargo.toml
COPY crates/gvm-report-server/Cargo.toml crates/gvm-report-server/Cargo.toml
COPY crates/gvm-report-cli/Cargo.toml crates/gvm-report-cli/Cargo.toml

COPY crates crates

RUN cargo build --release -p gvm-report-server --bin gvm-report-server


FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    util-linux \
    zip \
    python3 \
    python3-pip \
    python3-cairocffi \
    python3-cairo \
    python3-lxml \
    xsltproc \
    xmlstarlet \
    graphviz \
    libcairo2 \
    texlive-latex-base \
    texlive-latex-recommended \
    texlive-fonts-recommended \
    texlive-pictures \
    texlive-latex-extra \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install --no-cache-dir --break-system-packages pycha

RUN useradd -m -u 10001 gvmr \
    && mkdir -p /tmp/gvmr/work \
    && chown -R gvmr:gvmr /tmp/gvmr

WORKDIR /app

COPY --from=builder \
    /app/target/release/gvm-report-server \
    /usr/local/bin/gvm-report-server

COPY --from=typst \
    /usr/local/bin/typst \
    /usr/local/bin/typst

COPY templates /app/templates

USER gvmr

EXPOSE 8084

ENTRYPOINT ["/usr/local/bin/gvm-report-server"]