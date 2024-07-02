fn main() {
    // 使用cxx_build构建桥接代码，将src/main.rs与src/blobstore.cc连接起来
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cc") // 添加C++源文件
        .std("c++14") // 使用C++14标准
        .compile("cxxbridge-demo"); // 编译生成名为cxxbridge-demo的库

    // 当src/main.rs发生改变时重新运行构建
    println!("cargo:rerun-if-changed=src/main.rs");
    // 当src/blobstore.cc发生改变时重新运行构建
    println!("cargo:rerun-if-changed=src/blobstore.cc");
    // 当include/blobstore.h发生改变时重新运行构建
    println!("cargo:rerun-if-changed=include/blobstore.h");
}
