use std::rc::Rc;
use std::cell::RefCell;
use kernel::{
    Model,
    EntityType,
    RelationType,
    Value,
    Function,
    Process,
};

fn main() {
    // モデルの作成
    let model = Rc::new(RefCell::new(Model::new()));

    // エンティティの作成
    let john = model.borrow_mut().create_entity("John".to_string(), EntityType::Person);
    let mary = model.borrow_mut().create_entity("Mary".to_string(), EntityType::Person);
    let acme_corp = model.borrow_mut().create_entity("Acme Corporation".to_string(), EntityType::Organization);
    let central_park = model.borrow_mut().create_entity("Central Park".to_string(), EntityType::Location);

    // 関係の追加
    let _ = model.borrow_mut().add_relation(
        "works_at".to_string(),
        RelationType::ManyToOne,
        &john.id,
        &acme_corp.id,
    );
    let _ = model.borrow_mut().add_relation(
        "works_at".to_string(),
        RelationType::ManyToOne,
        &mary.id,
        &acme_corp.id,
    );
    let _ = model.borrow_mut().add_relation(
        "visits".to_string(),
        RelationType::ManyToMany,
        &john.id,
        &central_park.id,
    );

    // 状態の設定
    john.get_state().borrow_mut().set("age".to_string(), Value::Integer(30));
    mary.get_state().borrow_mut().set("age".to_string(), Value::Integer(28));
    acme_corp.get_state().borrow_mut().set("employee_count".to_string(), Value::Integer(2));

    // 関数とプロセスの追加
    let john_clone = Rc::clone(&john);
    let age_function = Rc::new(Function::new("age_increment".to_string(), Rc::downgrade(&john)));
    let age_process = Rc::new(Process::new(
        "increment_age".to_string(),
        Rc::downgrade(&age_function),
        Box::new(move || {
            if let Some(age) = john_clone.get_state().borrow().get("age") {
                if let Value::Integer(current_age) = age {
                    john_clone.get_state().borrow_mut().set("age".to_string(), Value::Integer(current_age + 1));
                }
            }
        }),
    ));
    age_function.add_process(age_process);
    john.add_function(age_function);
    if let Some(age_func) = john.get_function("age_increment") {
        age_func.activate();
    }

    // シミュレーションの実行
    println!("Initial state:");
    print_model_state(&model.borrow());

    for _ in 0..5 {
        model.borrow_mut().proceed();
        // ここで各エンティティの関数を実行する
        for entity in model.borrow().get_all_entities() {
            if let Some(age_func) = entity.get_function("age_increment") {
                for (_name, process) in age_func.processes.borrow().iter() {
                    process.execute();
                }
            }
        }
    }

    println!("\nAfter 5 time steps:");
    print_model_state(&model.borrow());
}

fn print_model_state(model: &Model) {
    for entity in model.get_all_entities() {
        println!("Entity: {} ({})", entity.name, entity.entity_type);
        println!("  State:");
        let state = entity.get_state().borrow();
        for (key, value) in state.iter() {
            println!("    {}: {:?}", key, value);
        }
        println!("  Relations:");
        for relation in entity.get_all_relations() {
            if let Some(other_entity) = relation.get_other_entity(&entity) {
                if let Some(other) = other_entity.upgrade() {
                    println!("    {} -> {}", relation.name, other.name);
                }
            }
        }
        println!();
    }
}