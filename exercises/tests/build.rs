//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // 设置tests7需要的环境变量
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 设置TEST_FOO环境变量（注意格式）
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 启用tests8需要的pass特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}