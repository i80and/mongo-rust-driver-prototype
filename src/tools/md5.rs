/* Copyright 2013 10gen Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::cast;
use std::int::range;
use std::io::extensions::u64_to_le_bytes;
use libc::c_int;

#[link(name = "md5")]
extern {
    fn md5_init(pms: *MD5State);
    fn md5_append(pms: *MD5State, data: *u8, nbytes: c_int);
    fn md5_finish(pms: *MD5State, digest: *[u8,..16]);
}

struct MD5State {
    count: [u32,..2],
    abcd: [u32,..4],
    buf: [u8,..64]
}

impl MD5State {
    fn new(len: u64) -> MD5State {
        let mut c: [u32,..2] = [0u32,0];
        u64_to_le_bytes(len, 8, |l| {
            c[0] |= l[0] as u32;
            c[0] |= (l[1] << 8) as u32;
            c[0] |= (l[2] << 16) as u32;
            c[0] |= (l[3] << 24) as u32;
            c[1] |= l[4] as u32;
            c[1] |= (l[5] << 8) as u32;
            c[1] |= (l[6] << 16) as u32;
            c[1] |= (l[7] << 24) as u32;
        });

        MD5State {
            count: c,
            abcd: [0u32,0,0,0],
            buf: [
                0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0
                ]
        }
    }
}

fn md5(msg: &str) -> ~str {
    let msg_bytes = msg.as_bytes();
    let m = MD5State::new(msg_bytes.len() as u64);
    let digest: [u8,..16] = [
        0,0,0,0,
        0,0,0,0,
        0,0,0,0,
        0,0,0,0
    ];

    unsafe {
        md5_init(cast::transmute(&m));
        md5_append(cast::transmute(&m), cast::transmute(&(msg_bytes[0])), msg_bytes.len() as i32);
        md5_finish(cast::transmute(&m), cast::transmute(&digest));
    }

    let mut result: ~str = ~"";
    for i in range(0u, 16u) {
        let mut byte = format!("{:x}", digest[i] as uint);
        if byte.len() == 1 {
            byte = (~"0").append(byte);
        }
        result.push_str(byte);
    }
    result
}

#[cfg(test)]
#[test]
fn md5_test() {
    assert_eq!(md5(~"hello"), ~"5d41402abc4b2a76b9719d911017c592");
    assert_eq!(md5(~"asdfasdfasdf"), ~"a95c530a7af5f492a74499e70578d150");
}
