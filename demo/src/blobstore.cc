#include "demo/include/blobstore.h"
#include "demo/src/main.rs.h"
#include <algorithm>
#include <functional>
#include <set>
#include <string>
#include <unordered_map>

namespace org {
namespace blobstore {

// 内存中blobstore的简易实现。
//
// 实际上，BlobstoreClient的实现可能是一个大型复杂的C++库。
class BlobstoreClient::impl {
  friend BlobstoreClient;
  using Blob = struct {
    std::string data; // 存储blob的数据
    std::set<std::string> tags; // 存储blob的标签
  };
  std::unordered_map<uint64_t, Blob> blobs; // 存储所有blob的映射
};

BlobstoreClient::BlobstoreClient() : impl(new class BlobstoreClient::impl) {}

// 上传一个新的blob并返回一个作为blob句柄的blobid。
uint64_t BlobstoreClient::put(MultiBuf &buf) const {
  std::string contents;

  // 遍历调用者的块迭代器。
  //
  // 实际上，blobstore的C++客户端可能会实现复杂的块批处理和/或并行上传。
  while (true) {
    auto chunk = next_chunk(buf); // 获取下一个数据块
    if (chunk.size() == 0) {
      break; // 如果数据块大小为0，结束循环
    }
    contents.append(reinterpret_cast<const char *>(chunk.data()), chunk.size()); // 将数据块追加到内容中
  }

  // 插入映射并提供给调用者句柄。
  auto blobid = std::hash<std::string>{}(contents); // 生成blobid
  impl->blobs[blobid] = {std::move(contents), {}}; // 插入新的blob到映射中
  return blobid;
}

// 添加标签到已有的blob。
void BlobstoreClient::tag(uint64_t blobid, rust::Str tag) const {
  impl->blobs[blobid].tags.emplace(tag); // 将标签添加到指定的blob中
}

// 检索关于blob的元数据。
BlobMetadata BlobstoreClient::metadata(uint64_t blobid) const {
  BlobMetadata metadata{};
  auto blob = impl->blobs.find(blobid); // 查找指定的blob
  if (blob != impl->blobs.end()) {
    metadata.size = blob->second.data.size(); // 获取blob的大小
    std::for_each(blob->second.tags.cbegin(), blob->second.tags.cend(),
                  [&](auto &t) { metadata.tags.emplace_back(t); }); // 获取blob的所有标签
  }
  return metadata;
}

// 创建一个新的BlobstoreClient实例。
std::unique_ptr<BlobstoreClient> new_blobstore_client() {
  return std::make_unique<BlobstoreClient>();
}

} // namespace blobstore
} // namespace org
