# Fully homomorphic encryption in ZK

Utilizing the Risc Zero zkVM and the zama.ai concrete FHE library, execute simple FHE tasks inside of zero knowledge circuits.

## Reasons this doesn't work yet

- Risc Zero runs on rustc 1.66, tfhe-rs needs at least rustc 1.67.
- concrete crates target system level bindings (x86, aarch) that are unlikely to be available in risc0 vm.

## When might this work

I think it is worth exploring this again whenever risc zero upgrades the version of rustc that they freeze. Otherwise the effort to do this myself is non-trivial.
