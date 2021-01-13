   Compiling rmc v0.1.0 (/home/accusitive/Projects/personal/scratches/rmc)
warning: unused import: `std::fmt::Write`
 --> src/network.rs:1:5
  |
1 | use std::fmt::Write;
  |     ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `byteorder::ByteOrder`
 --> src/network.rs:4:5
  |
4 | use byteorder::ByteOrder;
  |     ^^^^^^^^^^^^^^^^^^^^

warning: unused import: `BigEndian`
 --> src/network.rs:5:17
  |
5 | use byteorder::{BigEndian, ReadBytesExt};
  |                 ^^^^^^^^^

warning: unused import: `WriteBytesExt`
 --> src/packets.rs:3:53
  |
3 | use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
  |                                                     ^^^^^^^^^^^^^

error[E0609]: no field `3` on type `packets::HandshakeC2S`
  --> src/main.rs:28:28
   |
28 |                 state = hs.3;
   |                            ^ unknown field

error[E0599]: no method named `write_var_i32` found for struct `std::io::Cursor<&mut [u8; 2048]>` in the current scope
  --> src/packets.rs:29:16
   |
29 |         cursor.write_var_i32();
   |                ^^^^^^^^^^^^^ method not found in `std::io::Cursor<&mut [u8; 2048]>`
   | 
  ::: /home/accusitive/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/io/cursor.rs:75:1
   |
75 | pub struct Cursor<T> {
   | --------------------
   | |
   | doesn't satisfy `std::io::Cursor<&mut [u8; 2048]>: VarIntWrite`
   | doesn't satisfy `std::io::Cursor<&mut [u8; 2048]>: std::io::Write`
   |
   = note: the method `write_var_i32` exists but the following trait bounds were not satisfied:
           `std::io::Cursor<&mut [u8; 2048]>: std::io::Write`
           which is required by `std::io::Cursor<&mut [u8; 2048]>: VarIntWrite`

warning: unused import: `ByteOrder`
 --> src/packets.rs:3:28
  |
3 | use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
  |                            ^^^^^^^^^

warning: unused import: `VarIntWrite`
 --> src/packets.rs:4:36
  |
4 | use minecraft_varint::{VarIntRead, VarIntWrite};
  |                                    ^^^^^^^^^^^

error: aborting due to 2 previous errors; 6 warnings emitted

Some errors have detailed explanations: E0599, E0609.
For more information about an error, try `rustc --explain E0599`.
error: could not compile `rmc`

To learn more, run the command again with --verbose.
