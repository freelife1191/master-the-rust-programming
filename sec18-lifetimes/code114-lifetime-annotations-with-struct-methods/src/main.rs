#[derive(Debug)]
struct MyStruct<'a, 'b> {
    data1: &'a str,
    data2: &'b str,
}

// impl MyStruct {
/*
impl<'a, 'b> MyStruct<'a, 'a> 블록은 구현이 data1 및 data2 필드 모두 동일한 수명 'a를 갖는 MyStruct 인스턴스에 적용되도록 지정한다
 */
impl<'a, 'b> MyStruct<'a, 'b> {
// impl<'a, 'b> MyStruct<'_, '_> {
// impl<'a, 'b> MyStruct<'a, '_> {
    fn get_data1(&self) -> &'a str {
        self.data1
    }

    fn set_data(&mut self, s1: &'a str, s2: &'b str) {
        self.data1 = s1;
        self.data2 = s2;
    }

    fn get_longest<'c>(&'c self, s: &'c str) -> &str {
        let longest_self = if self.data1.len() > self.data2.len() {
            self.data1
        } else {
            self.data2
        };

        if longest_self.len() > s.len() {
            longest_self
        } else {
            s
        }
    }
}
fn main() {
    /*
    Lifetime annotation for struct methods
     */
    let mut struct_ins = MyStruct {
        data1: "Hi",
        data2: "World",
    };
    let ret = struct_ins.get_data1();
    println!("{}", ret);

    let s1 = "Good";
    let s2 = "Morning";

    struct_ins.set_data(s1, s2);
    println!("{:?}", struct_ins);

    println!("{}", struct_ins.get_longest("Evening"));

    /*
    let ret;
    {
        let mut struct_ins = MyStruct {
            data1: "Hi",
            data2: "World",
        };

        ret = struct_ins.get_data1();
    }

    println!("{}", ret);
    */

}