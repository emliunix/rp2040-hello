## The target dependencies

It looks you have two mutually exclusive options:
1. defmt + defmt-rtt + panic-probe /w print-defmt
2. rtt-target + panic-probe /w print-rtt

I'll use defmt-rtt for now because it's the default in rp-rs's template and if I remember correctly, mentioned in probe-rs's doc.

## The host parts

To run with RTT, basically you have a program attached to the target with the probe along the runtime and keeps polling RTT messages from the target and print it on the host side.

There're two options:
1. probe-run
2. cargo-embed

The first problem I ran into was that both failed to attach. And found out that you have to turn connect_under_reset off. It times out at reset something.

The specific code that fails in probe-rs is `probe-rs::architecture::arm::sequences::ArmDebugSequence::reset_hardware_deassert`. I think it has something to do with the nSRST pin, which is absent from my debug probe.

I'll check CMSIS-DAP how it interacts with that pin.

*Update*: The probe I use and XIAO2040 just don't support nSRST. Per CMSIS-DAP, the full connector (SWJ SWD/JTAG Connector) pins consists of extra SWO/nTRST/nSRST pins.

Now back to the host programs:

cargo-embed prints gibberish on my settings, maybe because version mismatch on host/target defmt.

probe-run now works well, I'll continue with it.

### probe-run's modifying you binary

I think it's modifying to hook hard-fault handler and maybe some RTT setting up. probe-run does have done much to make RTT works more smoothly, decoding, hard-fault hooking, stacktraces, etc.

cargo-embed is a probe-rs project, it integrates with cargo, maybe more convenient, or maybe more hard to customize. eg. you can't just debug a binary file, it's coupled with your rust project.
