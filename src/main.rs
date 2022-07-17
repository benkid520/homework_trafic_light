fn main() {
    //声明和定义一个TraficLight类型枚举light变量
    let light = TraficLight::Red;
    //输出相关枚举的时间
    println!("{}",light.show_time());
}
//声明定义TraficLight枚举
enum TraficLight {
    Red,
    Green,
    Yellow,
}
//声明定义一个trait Timer
trait Timer{
    //声明一个 函数show_time参数为调用对象自身 返回一个u8数据类型
    fn show_time(&self) -> u8;
}
//为TraficLight实现Timer trait
impl Timer for TraficLight {
    //定义和实现show_time
    fn show_time(&self)->u8{
        //利用match 返回相关枚举对应的时间值
        match self {
            TraficLight::Red => 10,
            TraficLight::Green => 20,
            TraficLight::Yellow => 30,
        }
    }
}
