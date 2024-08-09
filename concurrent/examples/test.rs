
fn main(){
    let data = vec![1,2,3,4];
    let ref_data = &data;

    println!("{:p}, {:p}", &data, ref_data);
    println!("{:p}, {:p}", &&data, &ref_data);

    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
    sum(ref_data);
}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}