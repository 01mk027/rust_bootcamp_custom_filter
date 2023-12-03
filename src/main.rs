struct FilterCondition {
    filter_condition_val: i32
}

impl FilterCondition {
    fn is_match(&self, vect_item: &i32) -> bool
    {
        return *vect_item >= self.filter_condition_val;
    }
}

fn custom_filter(vect: &Vec<i32>, filter_condition: &FilterCondition) -> Vec<i32>
{
    let mut new_vector = Vec::new();
    for vi in vect 
    {
        if filter_condition.is_match(vi)
        {
            new_vector.push(*vi);
        }
    }

    return new_vector;
}

fn main()
{ 
    let vect_to_be_filtered = vec![48, 63, 22, 14, 95, 80];
    let filter_condition = FilterCondition{
        filter_condition_val: 70
    };

    println!("Filtered vector items: {:?}", custom_filter(&vect_to_be_filtered, &filter_condition))
}