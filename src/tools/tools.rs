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

#![crate_id = "tools#0.1.0"]
#![crate_type="lib"]
#![license="Apache 2.0"]

#![feature(managed_boxes)]
#![feature(globs)]

extern crate libc;
extern crate collections;
extern crate sync;

pub mod md5;
pub mod stream;
pub mod ord_hash;
pub mod mockable;
