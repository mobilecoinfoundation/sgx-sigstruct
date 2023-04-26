# MobileCoin Enclave Signature Structure (SIGSTRUCT)

[![Project Chat][chat-image]][chat-link]<!--
-->![License][license-image]<!--
-->[![Dependency Status][deps-image]][deps-link]<!--
-->[![CodeCov Status][codecov-image]][codecov-link]<!--
-->[![GitHub Workflow Status][gha-image]][gha-link]<!--
-->[![Contributor Covenant][conduct-image]][conduct-link]

`SIGSTRUCT` is a structure created and signed by the enclave developer that contains information about the enclave.
This crate contains utilities to parse the `SIGSTRUCT` structure, as created by the `sgx_sign` SDK utility.
It exposes the primary API, the `Signature` structure, which can be loaded from bytes or a file, as needed.
It includes Intel SGX SIGSTRUCT Dump Utility to read a CSS file and print its output
[Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 3D: System Programming Guide, Part 4, Section 38.13](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-3d-part-4-manual.pdf)

## Examples

```ignore
use hex_fmt::HexFmt;
use std::path::Path;
use sgx_css::Signature;

fn main() {
    let path = Path::from("/path/to/cssfile");
    let sig = Signature::try_from(&path).expect("Could not parse SIGSTRUCT");
    println!("{}:{}", HexFmt(sig.mrenclave()), HexFmt(sig.mrsigner()));
}
```

[chat-image]: https://img.shields.io/discord/844353360348971068?style=flat-square
[chat-link]: https://discord.gg/mobilecoin
[license-image]: https://img.shields.io/crates/l/mc-sgx-css?style=flat-square
[deps-image]: https://deps.rs/repo/github/mobilecoinfoundation/sgx-sigstruct/status.svg?style=flat-square
[deps-link]: https://deps.rs/repo/github/mobilecoinfoundation/sgx-sigstruct
[codecov-image]: https://img.shields.io/codecov/c/github/mobilecoinfoundation/sgx-sigstruct/main?style=flat-square
[codecov-link]: https://codecov.io/gh/mobilecoinfoundation/sgx-sigstruct
[gha-image]: https://img.shields.io/github/actions/workflow/status/mobilecoinfoundation/sgx-sigstruct/ci.yaml?branch=main&style=flat-square
[gha-link]: https://github.com/mobilecoinfoundation/sgx-sigstruct/actions/workflows/ci.yaml?query=branch%3Amain
[conduct-link]: CODE_OF_CONDUCT.md
[conduct-image]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=flat-square
