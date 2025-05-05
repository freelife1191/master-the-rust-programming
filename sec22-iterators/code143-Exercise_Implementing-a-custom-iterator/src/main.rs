/* 작업에 할당할 수 있는 우선순위 수준을 나타냅니다.
   이는 중요도에 따라 작업을 정렬하는 데 사용됩니다.
   'High'이 가장 긴급하고 'Low'이 가장 긴급하지 않습니다. */
#[derive(Debug)]
enum Priority {
    Low,
    Medium,
    High,
}

/* 이 연습의 맥락에서 작업을 나타냅니다. */
struct Task {
    name: String,
    priority: Priority,
}

impl Task {
    fn new(name: &str, priority: Priority) -> Self {
        Task {
            name: name.to_string(),
            priority,
        }
    }
}

struct PriorityIterator<'a> {
    tasks: &'a [Task],
    /* 반복자는 이 필드를 사용하여 어떤 작업을 결정하는지 결정합니다.
       우선순위에 따라 다음 인스턴스를 생성합니다.

       반복자가 현재 처리 중인 우선순위 수준을 추적합니다. */
    current_priority: Priority,
    /* 현재 위치를 추적하는 상태 변수 역할을 합니다.
       반복되는 컬렉션 내에서 */
    next_index: usize,
}

impl<'a> PriorityIterator<'a> {
    fn new(tasks: &'a [Task]) -> Self {
        PriorityIterator {
            tasks,
            current_priority: Priority::Medium,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PriorityIterator<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO
    }
}

fn main() {
    let all_tasks = vec![
        Task::new("Laundry", Priority::Low),
        Task::new("Emails", Priority::Medium),
        Task::new("Homework", Priority::High),
        Task::new("Gym", Priority::Medium),
        Task::new("Taxes", Priority::High),
    ];

    let priority_tasks = PriorityIterator::new(&all_tasks);

    println!("Priority tasks:");
    for task in priority_tasks {
        println!("{:?}: {:?}", task.name, task.priority);
    }
}