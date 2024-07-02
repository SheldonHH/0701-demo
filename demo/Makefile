# 指定编译器
CXX = scan-build clang++
CXXFLAGS = -Wall -Wextra -Werror -g -std=c++14

# 包含头文件目录
INCLUDES = -Iinclude -Itarget/cxxbridge -I../

# 源文件
SRCS = src/main.cpp src/blobstore.cc
OBJS = src/main.o src/blobstore.o
#TODO: cargo build, rustc 的编译过程
#TODO变成cargo的一个plugin 
# 编译命令变成scan-build，缺点是
# 可执行文件名
EXEC = demo

# 默认目标
all: src/main.o src/blobstore.o

# 链接目标
# $(EXEC): $(OBJS) 
# 	$(CXX)  --save-temps $(CXXFLAGS) $(INCLUDES) -o $@ $^

# 编译目标
src/main.o: src/main.cpp
	$(CXX) --save-temps $(CXXFLAGS) $(INCLUDES) -c $< -o $@

src/blobstore.o: src/blobstore.cc
	$(CXX) --save-temps  $(CXXFLAGS) $(INCLUDES) -c $< -o $@

# 清理目标
clean:
	rm -f $(OBJS) $(EXEC)
