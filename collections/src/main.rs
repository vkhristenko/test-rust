fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    let vv = vec![1,2,3,4,5];

    println!("{:?}", v);
    println!("{:?}", vv);

    {
        let vvv = vec![1,2,3,4,5];
        // vvv goes out of scope -> gets destroyed
    }

    {
        let v = vec![1,2,3,4,5];
        let third: &i32 = &v[2];
        let third: i32 = v[2];
        let third: Option<&i32> = v.get(2);
    }

    {
        let mut s = String::new();

        let data = "initial contents";
        let s = data.to_string();
        let s = "initial contents".to_string();
    }
}
