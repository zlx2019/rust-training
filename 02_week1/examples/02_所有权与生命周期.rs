/// 所有权与生命周期
///
/// 基本规则：
/// 1. 一个值被一个唯一的scope拥有，它们共存亡，值会随着scope结束而消亡；
/// 2. 一个值可以被 Move 到另一个scope中，新的scope将拥有这个值，而旧的scope将不再拥有；
/// 3. 一个值可以存在多个 '不可变引用' 或一个 '可变引用'，它们之间是互斥关系；
/// 4. 引用的生命周期，不可以超过值的存活期；
///
/// 所有权解决了堆内存的生存难题：它究竟该活多久？
///
///   总结，所有权的本质思想就是：将堆内存与栈内存的生命周期绑定，堆内存随着栈内存一起消亡，而栈的内存与 scope 绑定，
/// 这样能够将堆内存的存活周期，取一个折中的方案，不依赖于开发者或运行时(GC)，也能很好的控制堆内存。
///
///   而所谓的生命周期，其实指的就是'栈内存'的生命周期，生命周期的标注其实是在帮助编译器理解各个函数的参数的生命周期
/// 之间的关系。

fn main(){

}
