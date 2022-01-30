FROM gitpod/workspace-full

# Install custom tools, runtime, etc.
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.9.5/install)"

