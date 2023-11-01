use task_manager::{ TaskManager, Weight };

mod task_manager;

fn main() {
    let mut tm = TaskManager::new();
    
    tm.add_task("0".into(), "0".into(), 0, 0, Weight::Low);
    tm.add_task("1".into(), "1".into(), 0, 0, Weight::High);
    tm.add_task("2".into(), "2".into(), 0, 1, Weight::High);
   
    print!("{}", tm);

    tm.edit_task(1, "3".into(), "3".into(), 0, 0, Weight::High);

    print!("{}", tm);
}
