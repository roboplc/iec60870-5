<h2>
  IEC 60870-5 101/104 Rust protocol implementation
  <a href="https://crates.io/crates/iec60870-5"><img alt="crates.io page" src="https://img.shields.io/crates/v/roboplc-io-ads.svg"></img></a>
  <a href="https://docs.rs/iec60870-5"><img alt="docs.rs page" src="https://docs.rs/roboplc-io-ads/badge.svg"></img></a>
</h2>

# Introduction

[IEC 60870-5](https://en.wikipedia.org/wiki/IEC_60870-5) is a set of standards
for telecontrol, teleprotection, and associated telecommunications for electric
power systems, widely used in the European Union, the United Kingdom and other
locations.

The crate provides a fully Rust-safe protocol-agnostic implementation for IEC
60870-5 101/104 telegrams and common data types. The library contains almost
the complete set of types and instruments required to easily write own client
or server, including time conversion between Rust and IEC 60870-5 formats.

The crate IS NOT FREE for any commercial or production use. Please refer to
<https://github.com/roboplc/iec60870-5/blob/main/LICENSE.md> for more
information.

IEC 60870-5 Rust library is a part of the [RoboPLC](https://roboplc.com)
project.

MSRV: 1.68.0

# Examples

## Creating a simple data transmission start IEC 60870-5 104 telegram

```rust
use iec60870_5::telegram104::Telegram104;

let telegram = Telegram104::new_start_dt();
// The telegram is ready to be sent in any way
let mut buffer = std::io::Cursor::new(Vec::new());
telegram.write(&mut buffer).unwrap();
```

## Creating a IEC 60870-5 104 command telegram

```rust
// A timestamp crate
use bma_ts::Timestamp;
use iec60870_5::types::datatype::{C_RC_TA_1, DataType, QU, RCO, RCS, SelectExecute};
use iec60870_5::types::COT;
use iec60870_5::telegram104::{ChatSequenceCounter, Telegram104, Telegram104_I};

// To communicate with the server, the client must have a chat sequence counter
let chat_sequence_counter = ChatSequenceCounter::new();

let mut telegram_i: Telegram104_I = Telegram104_I::new(
    DataType::C_RC_TA_1, // Data type, regulating step command with CP56Time2a time tag
    COT::Act, // Cause of transmission: activation
    15 // ASDU address
    );
telegram_i.append_iou(
    11, // Information object address,
    C_RC_TA_1 { // Regulating step command with CP56Time2a time tag
        rco: RCO {
            rcs: RCS::Increment,
            se: SelectExecute::Execute,
            qu: QU::Persistent,
        },
        time: Timestamp::now().try_into().unwrap(),
    },
);
let mut telegram: Telegram104 = telegram_i.into();
telegram.chat_sequence_apply_outgoing(&chat_sequence_counter);
// The telegram is ready to be sent in any way
let mut buffer = std::io::Cursor::new(Vec::new());
telegram.write(&mut buffer).unwrap();
```

## Reading a IEC 60870-5 104 telegram

```rust,no_run
// A timestamp crate
use bma_ts::Timestamp;
use iec60870_5::telegram104::{ChatSequenceCounter, Telegram104};
use iec60870_5::types::datatype::{DataType, M_EP_TA_1};

// For strict servers, the client must have a chat sequence counter
let chat_sequence_counter = ChatSequenceCounter::new();

// Consider that the buffer contains a valid telegram
let mut buffer = std::io::Cursor::new(Vec::new());
let telegram = Telegram104::read(&mut buffer).unwrap();
telegram.chat_sequence_validate_incoming(&chat_sequence_counter).unwrap();
if let Telegram104::I(i) = telegram {
    // This is an I-frame
    if i.data_type() == DataType::M_EP_TA_1 {
        // This is a M_EP_TA_1 telegram
        for iou in i.iou() {
            // Decode MP_EP_TA_1 information object value from the raw IOU buffer
            let val: M_EP_TA_1 = iou.value().into();
            // Convert the time tag to a timestamp
            let dt = Timestamp::try_from(val.time);
            // Output the event state and the timestamp
            dbg!(val.sep.es, dt);
        }
    }
}
```

# Troubleshooting

As IEC 60870-5 is a complex standard, the hardware/software vendors usually do
not implement it it full, some parts may require additional
development/workarounds.

The library has been fully tested in pair with [Beckhoff TwinCAT 3
TF6500](https://www.beckhoff.com/en-en/products/automation/twincat/tfxxxx-twincat-3-functions/tf6xxx-connectivity/tf6500.html)
as well as with certain embedded implementations, used in the European Union
power grids.

The most common problem is that the majority of the IEC 60870-5 104 servers
require require keep-alive frames. Such can be generated either using IEC
60870-5 104 S-Frames, or test U-frames out-of-the-box:

```rust
use iec60870_5::telegram104::Telegram104;

let telegram = Telegram104::new_test();
// write the telegram to the target
```
