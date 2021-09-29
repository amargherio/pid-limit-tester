# PID Limit Tester

A one-off application written to test how the Kubelet responds to an application attempting to exceed the per-pod PID limits configured via [AKS custom node and kubelet configs](https://docs.microsoft.com/en-us/azure/aks/custom-node-configuration).

May not serve a real purpose unless you're interested in testing the behavior for yourself.


## Building the application and Docker image

### Prequisites
If you're planning on building a standalone binary, you'll need to have the Rust toolchain installed. The quickest method for installing the toolchain is [rustup](https://rustup.rs/).

### Standalone Binary
In order to build the standalone application binary, the Rust toolchain is required. Once the Rust toolchains are installed, the build for the standalone binary can be completed by running `cargo build --release`.

Building cross-platform hasn't been tested yet, but the standard process for cross-compilation should apply as well.

### Docker Image
The Dockerfile provided with the repository should contain everything required to build the binary and a deployable container image. The Dockerfile uses a multi-stage Docker build.