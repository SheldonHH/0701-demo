#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {
    // 共享结构体，字段在两种语言中都可见
    struct BlobMetadata {
        size: usize, // blob的大小
        tags: Vec<String>, // blob的标签
    }

    // 暴露给C++的Rust类型和函数签名
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8]; // 获取下一个数据块
    }

    // 暴露给Rust的C++类型和函数签名
    unsafe extern "C++" {
        include!("demo/include/blobstore.h"); // 包含C++头文件

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>; // 创建新的BlobstoreClient实例
        fn put(&self, parts: &mut MultiBuf) -> u64; // 上传blob
        fn tag(&self, blobid: u64, tag: &str); // 给blob添加标签
        fn metadata(&self, blobid: u64) -> BlobMetadata; // 获取blob的元数据
    }
}

// 对非连续文件对象的连续块进行迭代
//
// 简单实现使用Vec<Vec<u8>>，但实际上这可能是迭代一些更复杂的Rust数据结构，比如rope，或者从某个地方延迟加载块。
pub struct MultiBuf {
    chunks: Vec<Vec<u8>>, // 数据块集合
    pos: usize, // 当前块的位置
}

// 获取MultiBuf的下一个数据块
pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos); // 获取当前位置的数据块
    buf.pos += 1; // 更新位置
    next.map_or(&[], Vec::as_slice) // 返回数据块，如果没有更多块则返回空slice
}


/* 
main 函数的主要功能是创建一个 BlobstoreClient 实例，上传一个包含多个数据块的 blob，并为该 blob 添加标签，然后获取并打印 blob 的元数据。具体步骤如下：
1. 创建一个 BlobstoreClient 实例。
2. 定义一个包含两个数据块的向量，并创建一个 MultiBuf 实例来管理这些数据块。
3. 使用 BlobstoreClient 的 put 方法上传数据块，并获取一个 blobid。
4. 为上传的 blob 添加一个标签“rust”。
5. 获取该 blob 的元数据，并打印其中的标签信息。

*/ 
fn main() {
    let client = ffi::new_blobstore_client(); // 创建BlobstoreClient实例

    // 上传一个blob
    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()]; // 定义数据块
    let mut buf = MultiBuf { chunks, pos: 0 }; // 创建MultiBuf实例
    let blobid = client.put(&mut buf); // 上传数据块并获取blobid
    println!("blobid = {}", blobid);

    // 添加一个标签
    client.tag(blobid, "rust"); // 为blob添加标签"rust"

    // 读取标签
    let metadata = client.metadata(blobid); // 获取blob的元数据
    println!("tags = {:?}", metadata.tags); // 打印标签
}
