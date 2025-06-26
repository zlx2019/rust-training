package main

import "C"
import "fmt"

/// 库名：go_dyn
/// 构建动态库命令：go build -buildmode=c-shared -o ../libgo_dyn.dylib hello.go

//export say_hello
func say_hello(rwaStr *C.char) {
	// 将 C 字符串转换为 Go 字符串
	str := C.GoString(rwaStr)
	fmt.Printf("%s \n", str)
}

// main 函数必须存在，否则无法构建动态库
func main() {}
