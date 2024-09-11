// Define a struct called FilterCondition with a single field of the desired type for filtering.
struct FilterCondition {
    field: String,
}

pub trait FilterConditionTrait {
    fn is_match(&self, item: &Self) -> bool;
}

impl FilterConditionTrait for FilterCondition {
    fn is_match(&self, item: &Self) -> bool {
        item.field == self.field
    }
}

// Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments. The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.
pub fn custom_filter<T: FilterConditionTrait>(collection: Vec<T>, filter: &T) -> Vec<T> {
    collection
        .into_iter()
        .filter(|item| filter.is_match(item))
        .collect()
}
fn main() {
    // create a collection (e.g., a vector) with some elements and initialize a FilterCondition object with the desired value.
    let collection = vec![
        FilterCondition {
            field: "Alice".to_string(),
        },
        FilterCondition {
            field: "Bob".to_string(),
        },
        FilterCondition {
            field: "Chalie".to_string(),
        },
        FilterCondition {
            field: "Bob".to_string(),
        },
        FilterCondition {
            field: "Bob".to_string(),
        },
    ];

    let result = custom_filter(
        collection,
        &FilterCondition {
            field: "Bob".to_string(),
        },
    );

    for item in result {
        println!("{}", item.field);
    }
}
