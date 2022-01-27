use crate::models::category::Category;

pub fn category_fixture(id: usize) -> Category {
    Category {
        id: id as i32,
        name: String::from("food"),
    }
}

pub fn categoies_fixture(num: usize) -> Vec<Category> {
    let mut categories = vec![];
    for i in 1..num + 1 {
        categories.push(category_fixture(i));
    }
    categories
}
