# r8169-rust-driver-poc

Experimental out-of-tree Proof-of-Concept for rewriting the Linux kernel Realtek `r8169` Gigabit Ethernet driver (`drivers/net/ethernet/realtek/r8169_main.c`) using `Rust-for-Linux` abstractions.

## Roadmap & Current Progress
- [x] Repository scaffold: crate layout, PCI ID constants, DMA ring / HW state skeleton
- [ ] QEMU `rtl8169` virtual device testbench setup
- [ ] `bindgen` C header bindings generation (`pci`, `netdevice`, `sk_buff`)
- [ ] **Phase 1:** Decouple `rtl_hw_start` hardware state machine
- [ ] **Phase 2:** Safe `Owned` lifecycle wrapper for `dma_alloc_coherent` ring buffers
- [ ] **Phase 3:** Zero-copy `sk_buff` transmission path (`rtl8169_start_xmit`) & `netperf` benchmarks
