/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(improper_ctypes)]

use types::{IpfsGetPeerIdResult, IpfsPutResult, IpfsResult};

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;
use std::process::Command;
use std::fs;
use std::net::SocketAddr;
use std::fs::File;
use std::env::temp_dir;
use std::io::BufWriter;
use std::io::Write;
use eyre::{Result, WrapErr};
use std::env::args;
use std::fmt::Write as OtherWrite;

use std::io::Cursor;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

fn unwrap_mounted_binary_result(result: MountedBinaryResult) -> Result<String> {
    result
        .into_std()
        .ok_or(eyre::eyre!(
            "stdout or stderr contains non valid UTF8 string"
        ))?
        .map_err(|e| eyre::eyre!("ipfs cli call failed: {}", e))
}

#[inline]
fn get_timeout_string(timeout: u64) -> String {
    format!("{}s", timeout)
}

fn make_cmd_args(args: Vec<String>, api_multiaddr: String, timeout_sec: u64) -> Vec<String> {
    args.into_iter()
        .chain(vec![
            String::from("--timeout"),
            get_timeout_string(timeout_sec),
            String::from("--api"),
            api_multiaddr,
        ])
        .collect()
}

#[marine]
pub fn connect(multiaddr: String, api_multiaddr: String, timeout_sec: u64) -> IpfsResult {
    log::info!("connect called with multiaddr {}", multiaddr);

    let args = vec![String::from("swarm"), String::from("connect"), multiaddr];
    let cmd = make_cmd_args(args, api_multiaddr, timeout_sec);

    unwrap_mounted_binary_result(ipfs(cmd)).map(|_| ()).into()
}

/// Put file from specified path to IPFS and return its hash.
#[marine]
pub fn put(file_path: String, api_multiaddr: String, timeout_sec: u64) -> IpfsPutResult {
    log::info!("put called with file path {}", file_path);

    if !std::path::Path::new(&file_path).exists() {
        return IpfsPutResult {
            success: false,
            error: format!("path {} doesn't exist", file_path),
            hash: "".to_string(),
        };
    }

    println!("file path ---> {:?}", inject_vault_host_path(file_path.clone()));

    
    let args = vec![
        String::from("add"),
        String::from("-Q"),
        inject_vault_host_path(file_path.clone()),
    ];
    let cmd = make_cmd_args(args, api_multiaddr, timeout_sec);

    log::info!("ipfs put args {:?}", cmd);

    unwrap_mounted_binary_result(ipfs(cmd))
        .map(|res| res.trim().to_string())
        .into()
}

/// DAG put input to IPFS and return its hash.
#[marine]
pub fn dag_put(file_path: String, api_multiaddr: String, timeout_sec: u64) -> IpfsPutResult {
    log::info!("put called with file path {}", file_path);

    if !std::path::Path::new(&file_path).exists() {
        return IpfsPutResult {
            success: false,
            error: format!("path {} doesn't exist", file_path),
            hash: "".to_string(),
        };
    }

    let data = Cursor::new("Hello World!");

    let args = vec![
        String::from("dag"),
        String::from("put"),
        String::from("--input-codec"),
        String::from("raw"),
        // data
    ];

    let cmd = make_cmd_args(args, api_multiaddr, timeout_sec);

    log::info!("ipfs put args {:?}", cmd);

    unwrap_mounted_binary_result(ipfs(cmd))
        .map(|res| res.trim().to_string())
        .into()
}

/// Get file by provided hash from IPFS, saves it to a temporary file and returns a path to it.
#[marine]
pub fn get(hash: String, file_path: String, api_multiaddr: String, timeout_sec: u64) -> IpfsResult {
    log::info!("get called with hash {}", hash);

    // println!("file path ---> {:?}", file_path);

    // println!("file path injected ---> {:?}", inject_vault_host_path(file_path.clone()));

    // let mut dir = temp_dir();
    
    // File::create(file_path.clone());
    // fs::create_dir_all(file_path.clone());
    // fs::create_dir_all(inject_vault_host_path(file_path.clone()));

    // fs::write(file_path.clone(), "Muhammad Iqbal");
    // fs::write(inject_vault_host_path(file_path.clone()), "Muhammad Iqbal");

    // if !std::path::Path::new(&file_path.clone()).exists() {
    //     println!("Path does not exist");
    // }
    // if !std::path::Path::new(&inject_vault_host_path(file_path.clone())).exists() {
    //     println!("Path does not exist");
    // }

    // write!(inject_vault_host_path(inject_vault_host_path(file_path.clone())), "{}", "muhd iqbal");

    // let contents = fs::read_to_string(file_path.clone())
    //     .expect("Should have been able to read the file");
    // let contents = fs::read_to_string(inject_vault_host_path(file_path.clone()))
    //     .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    // let mut dir = temp_dir();
    // println!("{}", dir.to_str().unwrap());

    // let file_name = format!("{}.txt", "test_1");
    // println!("{}", file_name);
    // dir.push(file_name);

    // let file = File::create(dir);

    // let args = vec![
    //     String::from("add"),
    //     String::from("-Q"),
    //     String::from("-r"),
    //     String::from("/tmp/iqbal.txt")
    //     // file_path.clone()
    //     // inject_vault_host_path(file_path.clone())
    // ];

    
// std::fs::read_to_string(name);
    

    let args_create_file = vec![
        String::from("get"),
        String::from("-o"),
        inject_vault_host_path(file_path.clone()),
        String::from("QmfBRabun4FpaHV4wVXtnqtopUTro93XJHiWhNZscViCaq"),
    ];
    let cmd_create_file = make_cmd_args(args_create_file, api_multiaddr.clone(), timeout_sec);

    // let mut f = std::fs::OpenOptions::new().write(true).truncate(true).open(inject_vault_host_path(file_path.clone()));
    // f.write(b"muhd iqbal berjaya edit");
    // f.expect("successfully edit").flush();
    // Ok(())

    // fs::write(inject_vault_host_path(file_path.clone()), b"muhd iqbal");

    // let name =inject_vault_host_path(file_path.clone());
    // std::fs::write(name.clone(), "iqbal wrote").unwrap();
    // std::fs::read_to_string(name.clone());

    ipfs(cmd_create_file.clone());

    let name = "/tmp/vault/test3.txt";
    // std::fs::create_dir(name).expect("should create dir");
    std::fs::write(name, b"hello");

    let args_add_ipfs = vec![
        String::from("add"),
        String::from("-Q"),
        String::from(name)
    ];

    let cmd_add_ipfs = make_cmd_args(args_add_ipfs, api_multiaddr.clone(), timeout_sec);

    unwrap_mounted_binary_result(ipfs(cmd_add_ipfs))
        .map(|output| {
            log::info!("ipfs get output: {}", output);
        })
        .into()
    
    // let cmd = make_cmd_args(args, api_multiaddr, timeout_sec);

    // let bash_cmd = vec![
    //     String::from("touch"),
    //     String::from("/tmp/iqbal/test_iqbal.txt")
    // ];

    // bash(bash_cmd);

    
}

/// Get content by provided hash from IPFS.
#[marine]
pub fn dag_get(hash: String, file_path: String, api_multiaddr: String, timeout_sec: u64){
    log::info!("get called with hash {}", hash);


    fs::write(file_path.clone(), "iqbal");

    // let args = vec![
    //     String::from("dag"),
    //     String::from("get"),
    //     inject_vault_host_path(file_path.clone()),
    //     hash,
    // ];

    // let cmd = make_cmd_args(args, api_multiaddr, timeout_sec);

    // log::info!("ipfs get args {:?}", cmd);

    // unwrap_mounted_binary_result(ipfs(cmd))
    //     .map(|output| {
    //         log::info!("ipfs get output: {}", output);
    //     })
    //     .into()
}

/// Get content by provided hash from IPFS.
#[marine]
pub fn upload_ipfs(input: String, file_path: String, api_multiaddr: String, timeout_sec: u64){
    log::info!("input value {}", input);

    fs::write(file_path, input);
    // fs::write("bar.txt", "dolor sit")?;
    // Ok(())

    // let args = vec![
    //     String::from("echo")
    // ];

    // let result = unwrap_mounted_binary_result(bash(args));

    // println!("{:?}", result);

    // Command::new("ls")
    //     .arg("-l")
    //     .arg("-a")
    //     .spawn()
    //     .expect("ls command failed to start");

    // unwrap_mounted_binary_result(bash(args))
        // .map(|output| {
        //     log::info!("ipfs get output: {}", output);
        // })
        // .into()

    // let output = Command::new("echo")
    //     .arg("Hello world")
    //     .output()
    //     .expect("Failed to execute command");

    // Command::new("sh")
    //         .arg("-c")
    //         .arg("echo hello")
    //         .output()
    //         .expect("failed to execute process");

    // Command::new("sh")
    //     .spawn()
    //     .expect("sh command failed to start");

    // let ipfs_args = vec![
    //     String::from("dag"),
    //     String::from("put"),
    //     inject_vault_host_path(file_path.clone()),
    //     String::from(input.clone())
    // ];

    // let ipfs_cmd = make_cmd_args(ipfs_args, api_multiaddr, timeout_sec);

    // log::info!("ipfs get args {:?}", ipfs_cmd);

    // unwrap_mounted_binary_result(ipfs(ipfs_cmd));
        // .map(|output| {
        //     log::info!("ipfs get output: {}", output);
        // })
        // .into()
}

#[marine]
pub fn get_peer_id(api_multiaddr: String, timeout_sec: u64) -> IpfsGetPeerIdResult {
    let result: Result<String> = try {
        let cmd = make_cmd_args(vec![String::from("id")], api_multiaddr, timeout_sec);

        let result = unwrap_mounted_binary_result(ipfs(cmd))?;
        let result: serde_json::Value =
            serde_json::from_str(&result).wrap_err("ipfs response parsing failed")?;
        result
            .get("ID")
            .ok_or(eyre::eyre!("ID field not found in response"))?
            .as_str()
            .ok_or(eyre::eyre!("ID value is not string"))?
            .to_string()
    };

    result
        .map_err(|e| eyre::eyre!("get_peer_id: {:?}", e))
        .into()
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    /// Execute provided cmd as a parameters of ipfs cli, return result.
    pub fn ipfs(cmd: Vec<String>) -> MountedBinaryResult;

    pub fn bash(cmd: Vec<String>) -> MountedBinaryResult;
}

fn inject_vault_host_path(path: String) -> String {
    let vault = "/tmp/vault";
    if let Some(stripped) = path.strip_prefix(&vault) {
        let host_vault_path = std::env::var(vault).expect("vault must be mapped to /tmp/vault");
        format!("{}/{}", host_vault_path, stripped)
    } else {
        path
    }
}
