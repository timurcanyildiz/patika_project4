struct FilterCondition{
    value: i32
}
impl FilterCondition {
    fn is_match(&self, item: i32) -> bool{
        (item*&self.value)% 2 == 0
    }
}
fn custom_filter(collection: Vec<i32>, condition: &FilterCondition) -> Vec<i32>{
    collection
        .into_iter()
        .filter(|item| condition
        .is_match(*item))
        .collect()
       
}

fn main() {
   let collection:Vec<i32>=vec![1,2,3,4,5,6,7,10,15,20,29,31,45,66];

   let filter_condition: FilterCondition=FilterCondition{value: 7};

   let final_collection:Vec<i32>=custom_filter(collection, &filter_condition);
   println!("Numbers {:#?}",final_collection);
   

}
