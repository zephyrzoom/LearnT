fn main() {
    use std::collections::HashMap; // 他不像Vec和String默认引入了，HashMap需要手动引入

    let mut scores = HashMap::new();
    // HashMap<K, V>其中K和V是同质的，每个K和每个V都是相同类型
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 也可以通过zip两个vec，再用collect()方法创建HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); // 使用_，可以让编译器推断类型

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 之后field_name和field_value会失效，所有权移动了。用引用的话不会转移所有权，但是map有效时他们必须是有效的

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // 会返回Some(&10)，get返回的是Option<&V>，key为空时返回None

    // 类似Vec，但是结果是无序的
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 会覆盖上面的值，因为key是一样的
    println!("{:?}", scores);

    // 用entry设置值，entry是枚举类型的，如果没有值则插入50
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50); // or_insert会返回一个新易变值
    scores.entry(String::from("Blue")).or_insert(50); // 不会改变值，因为Blue已经有了

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // 计算单词出现次数
        let count = map.entry(word).or_insert(0);   // 返回类型是&mut V
        *count += 1; // 修改HashMap值时先解引用
    }
    println!("{:?}", map);

    // hash算法可以通过实现BuildHasher trait修改
}
