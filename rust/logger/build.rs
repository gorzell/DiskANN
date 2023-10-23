/*
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT license.
 */
use std::env;

extern crate prost_build;

fn main() {
     if cfg!(target_os = "windows") {
         let protopkg = vcpkg::find_package("protobuf").unwrap();
         let protobuf_path = protopkg.link_paths[0].parent().unwrap();

         let protobuf_bin_path = protobuf_path
             .join("tools")
             .join("protobuf")
             .join("protoc.exe")
             .to_str()
             .unwrap()
             .to_string();
         env::set_var("PROTOC", protobuf_bin_path);
     }


    prost_build::compile_protos(&["src/indexlog.proto"], &["src/"]).unwrap();
}

