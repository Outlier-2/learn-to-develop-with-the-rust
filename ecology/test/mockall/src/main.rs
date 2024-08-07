use mockall::predicate::*;
use mockall::*;
pub struct User {
    pub id: u32,
    pub name: String,
}
#[automock]
pub trait UserService {
    fn get_user(&self, user_id: u32) -> User;
}


#[automock]
trait MyTrait<T> {
    fn foo(&self, x: u32) -> T;
    // fn hello() -> str;
}

fn call_with_four(x: &dyn MyTrait) -> u32 {
    x.foo(5)
}

fn main() {
    let mut mock = MockMyTrait::new();
    let a = mock
        .expect_foo()
        // .with(predicate::eq(4))
        .times(1)
        .returning({
           
        });
    assert_eq!(6, call_with_four(&mock));
    // println!("{:?}", a);


    // 创建一个模拟的 UserService 对象
    let mut mock_service = MockUserService::new();

    // 设置期望，返回自定义结构体 User
    mock_service.expect_get_user()
        .with(mockall::predicate::eq(1)) // 期望参数为1
        .times(1)                          // 期望被调用一次
        .returning(|_| User {               // 返回一个 User 实例
            id: 1,
            name: String::from("Alice"),
        });

    // 调用被测试代码
    let user = mock_service.get_user(1);

    // 验证返回的 User 实例
    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Alice");
}
