FROM gitpod/workspace-full

# Install custom tools, runtime, etc.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.9.5/install)"
