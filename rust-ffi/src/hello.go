package main

/*
#include <stdlib.h>
*/
import "C"
import (
	"fmt"
	"strings"
	"unsafe"
)

/// 库名：go_dyn
/// 构建动态库命令：go build -buildmode=c-shared -o ../libgo_dyn.dylib hello.go
/// 查看动态库中的函数： objdump -t libgo_dyn.dylib | grep print_hello

// 接收一个 C 字符串类型，并且输出它。
//
//export print_hello
func print_hello(rwaStr *C.char) {
	// 将 C 字符串转换为 Go 字符串
	str := C.GoString(rwaStr)
	fmt.Printf("%s \n", str)
}

// 将字符串转换后返回
//
//export transform
func transform(rwaStr *C.char) *C.char {
	// 转换为 Go 字符串
	str := C.GoString(rwaStr)
	upperStr := strings.ToUpper(str)
	return C.CString(upperStr)
}

// 用于释放 C.CString 分配的内存
//
//export free_str
func free_str(rawPtr *C.char) {
	ptr := unsafe.Pointer(rawPtr)
	C.free(ptr)
}

// main 函数必须存在，否则无法构建动态库
func main() {}
