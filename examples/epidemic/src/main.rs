use std::rc::Rc;
use std::cell::RefCell;
use kernel::{
    Model,
    Entity,
    EntityType,
    Value,
    Function,
    Process,
    ExecutionResult,
    EntityCreationInfo,
    ExecutionContext,
    ReadOnlyEntity,
    ReadOnlyRelation,
};
use rand::Rng;

fn main() {
    // モデルの作成
    let model = Rc::new(RefCell::new(Model::new()));

    // エンティティの作成
    let john = model.borrow_mut().create_entity("John".to_string(), EntityType::Agent);
    let mary = model.borrow_mut().create_entity("Mary".to_string(), EntityType::Agent);

    // 状態の設定
    john.get_state().borrow_mut().set("age".to_string(), Value::Integer(30));
    mary.get_state().borrow_mut().set("age".to_string(), Value::Integer(28));

    // 関数とプロセスの追加
    add_age_increment_function(&john);
    add_birth_function(&john);
    add_death_function(&john);

    add_age_increment_function(&mary);
    add_birth_function(&mary);
    add_death_function(&mary);


    add_processes_to_model(&model, &john);
    add_processes_to_model(&model, &mary);

    // シミュレーションの実行
    println!("Initial state:");
    print_model_state(&model.borrow());

    for step in 1..=5 {
        println!("\nSimulating step {}:", step);
        model.borrow_mut().simulate();
        print_model_state(&model.borrow());
    }
}

fn add_age_increment_function(entity: &Rc<Entity>) {
    let entity_clone = Rc::clone(entity);
    let age_function = Rc::new(Function::new("age_increment".to_string(), Rc::downgrade(entity)));
    let age_process = Rc::new(Process::new(
        "increment_age".to_string(),
        Rc::downgrade(&age_function),
        Box::new(move |_context: &ExecutionContext| {
            let mut results = Vec::new();
            if let Some(age) = entity_clone.get_state().borrow().get("age") {
                if let Value::Integer(current_age) = age {
                    let new_age = current_age + 1;
                    println!("  Incrementing age of {} from {} to {}", entity_clone.get_name(), current_age, new_age);
                    results.push(ExecutionResult::UpdateEntityState(entity_clone.id, "age".to_string(), Value::Integer(new_age)));
                }
            }
            results
        }),
    ));
    age_function.add_process(Rc::clone(&age_process));
    entity.add_function(Rc::clone(&age_function));
    age_function.activate();
}

fn add_birth_function(entity: &Rc<Entity>) {
    let entity_clone = Rc::clone(entity);
    let birth_function = Rc::new(Function::new("birth".to_string(), Rc::downgrade(entity)));
    let birth_process = Rc::new(Process::new(
        "give_birth".to_string(),
        Rc::downgrade(&birth_function),
        Box::new(move |_context: &ExecutionContext| {
            let mut results = Vec::new();
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.1) {  // 10% chance of giving birth
                println!("  {} is giving birth!", entity_clone.get_name());
                let new_entity_info = EntityCreationInfo {
                    name: format!("Baby of {}", entity_clone.get_name()),
                    entity_type: EntityType::Agent,
                    initial_state: vec![("age".to_string(), Value::Integer(0))].into_iter().collect(),
                    functions: vec![],
                    relations: vec![],
                };
                results.push(ExecutionResult::CreateEntity(new_entity_info));
            }
            results
        }),
    ));
    birth_function.add_process(Rc::clone(&birth_process));
    entity.add_function(Rc::clone(&birth_function));
    birth_function.activate();
}

fn add_death_function(entity: &Rc<Entity>) {
    let entity_clone = Rc::clone(entity);
    let death_function = Rc::new(Function::new("death".to_string(), Rc::downgrade(entity)));
    let death_process = Rc::new(Process::new(
        "die".to_string(),
        Rc::downgrade(&death_function),
        Box::new(move |_context: &ExecutionContext| {
            let mut results = Vec::new();
            if let Some(age) = entity_clone.get_state().borrow().get("age") {
                if let Value::Integer(current_age) = age {
                    if *current_age >= 80 {
                        println!("  {} has died at age {}", entity_clone.get_name(), current_age);
                        results.push(ExecutionResult::DeleteEntity(entity_clone.id));
                    }
                }
            }
            results
        }),
    ));
    death_function.add_process(Rc::clone(&death_process));
    entity.add_function(Rc::clone(&death_function));
    death_function.activate();
}

fn print_model_state(model: &Model) {
    for entity in model.get_all_entities() {
        println!("Entity: {} ({:?})", entity.get_name(), entity.get_entity_type());
        println!("  State:");
        let state = entity.get_state().borrow();
        for (key, value) in state.iter() {
            println!("    {}: {:?}", key, value);
        }
        println!("  Relations:");
        for relation in entity.get_all_relations() {
            if let Some(other_entity) = relation.get_entity2() {
                println!("    {} -> {}", relation.get_name(), other_entity.get_name());
            }
        }
        println!();
    }
}

fn add_processes_to_model(model: &Rc<RefCell<Model>>, entity: &Rc<Entity>) {
    for function in entity.get_all_functions() {
        for process in function.get_all_processes() {
            model.borrow_mut().add_process(Rc::clone(&process));
        }
    }
}