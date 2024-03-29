use serde::{Deserialize, Serialize};

use crate::api::todo_list_api::ViewModel;

#[derive(Default, Serialize, Deserialize, Debug)]
pub(crate) struct TodoListModel {
    pub(crate) items: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    AddTodo(String),
    RemoveTodo(usize),
    CleanList,
}

// requests to the shell, aka Capabilities aka Effects
#[derive(Debug, PartialEq, Eq)]
pub enum Effect {
    Render(ViewModel),
}

pub(crate) fn process_mod_event(event: Event, model: &mut TodoListModel) -> Vec<Effect> {
    match event {
        Event::AddTodo(todo) => model.items.push(todo),
        Event::RemoveTodo(todo_pos) => {
            model.items.remove(todo_pos - 1);
        }
        Event::CleanList => model.items = vec![],
    }
    vec![Effect::Render(view(model))]
}

// fn process_read_event(event: Event, model: &TodoListModel) -> Vec<Effect> {
// }

pub(crate) fn view(model: &TodoListModel) -> ViewModel {
    let count = model.items.len();
    ViewModel {
        items: model.items.clone(),
        count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crux_core::{render::RenderOperation, testing::AppTester};

    #[test]
    fn add_todo() {
        //        let todo_list = AppTester::<TodoList, _>::default();
        let mut model = TodoListModel::default();

        // Call 'add'
        let effects = process_mod_event(Event::AddTodo("test the list".into()), &mut model);

        // Check update asked us to `Render`
        let actual_effect = &effects[0];
        let expected_effect = Effect::Render(view(&model));
        assert_eq!(actual_effect, &expected_effect);

        // Make sure the view matches our expectations
        let actual_view = /*&todo_list.*/view(&model);
        let expected_view = ViewModel {
            items: vec!["test the list".into()],
            count: 1,
        };
        assert_eq!(actual_view, expected_view);
    }
    #[test]
    fn remove_todo() {
        // let todo_list = AppTester::<TodoList, _>::default();
        let mut model = TodoListModel {
            items: vec!["remove me".into()],
        };

        // Call 'add'
        let effects = process_mod_event(Event::RemoveTodo(1), &mut model);

        // Check update asked us to `Render`
        let actual_effect = &effects[0];
        let expected_effect = Effect::Render(view(&model));
        assert_eq!(actual_effect, &expected_effect);

        // Make sure the view matches our expectations
        let actual_view = /*&todo_list.*/view(&model);
        let expected_view = ViewModel {
            items: vec![],
            count: 0,
        };
        assert_eq!(actual_view, expected_view);
    }
    #[test]
    fn clean_list() {
        // let todo_list = AppTester::<TodoList, _>::default();
        let mut model = TodoListModel::default();
        model.items.push("remove me".into());
        model.items.push("clean me".into());

        // Call 'add'
        let effects = process_mod_event(Event::CleanList, &mut model);

        // Check update asked us to `Render`
        let actual_effect = &effects[0];
        let expected_effect = Effect::Render(view(&model));
        assert_eq!(actual_effect, &expected_effect);

        // Make sure the view matches our expectations
        let actual_view = /*&todo_list.*/view(&model);
        let expected_view = ViewModel {
            items: vec![],
            count: 0,
        };
        assert_eq!(actual_view, expected_view);
    }
}
