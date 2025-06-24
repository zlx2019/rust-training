#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>

/// @brief 将字符串全部转换为大写后返回
/// @param str content
/// @return after to upper
char* str_to_upper(const char* str){
    if (str == NULL) {
        return NULL;
    }
    // 为要返回的字符串分配内存
    size_t len = strlen(str);
    char *ret = (char*) malloc(len + 1);
    for (size_t i = 0; i < len; i++){
        ret[i] = toupper(str[i]);
    }
    ret[len] = '\0';
    return ret;
}


/// @brief 释放C字符串内存
/// @param str 
void free_c_str(char* str){
    if (str != NULL){
        free(str);
    }
}