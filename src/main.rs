use my_rtd_for_learn_rust::Service;

fn main() {
    let service = Service::new("todo.csv".to_string());

    // 添加测试项
    service.add_item("Learn Rust".to_string());

    // 列出所有项
    println!("All TODOs:");
    service.list_all();

    // 其他操作可以在此测试，例如:
    // service.complete_item(1);
    // service.list_completed();
}

