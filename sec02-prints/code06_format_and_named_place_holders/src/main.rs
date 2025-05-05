fn main() {
    let name = "John";
    let age = 30;

    //이름이 없는 자리 표시자( '{}' )
    let msg = format!("I am {} and I am {} years old", name, age);

    //주어진 이름이 'user_name' 및 'user_age'인 자리 표시자( '{}' )
    let msg = format!(
        "I am {user_name} and I am {user_age} years old",
        user_name = name,
        user_age = age
    );//이 코드는 아래와 같이 작성할 수도 있습니다.

    // 변수에 대한 단축 구문을 사용할 수 있습니다.
    // 변수 이름이 변수 이름과 같을 때 형식화된 문자열
    // placeholders
    let msg = format!(
        "I am {name} and I am {age} years old"
    );

    println!("{}", msg);

    //
    // named placeholders
    //
    let message = format!("My name is {user_name} and \
                                I am {user_age} years old", user_age = age, user_name = name);
    println!("{}", message);
}
