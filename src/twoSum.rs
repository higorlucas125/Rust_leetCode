use std::collections::HashMap;

pub fn two_sum( nums: Vec<i32>, target:i32 ) -> Vec<i32>{
     let mut num_map = HashMap::new();

    for i in 0..nums.len(){
        let value =  target - nums[i];
        if let Some(&index) = num_map.get(&value){
            return vec![index as i32, i as i32];
        }

        num_map.insert(nums[i],i);
    }
    
    vec![]
}

// Se eu quiser retornar com 3
pub fn three_sum(nums : Vec<i32>, target:i32) -> Vec<i32>{
    for i in 0..nums.len(){
        for j in (i + 1)..nums.len(){
            for z in (j + 1)..nums.len(){
                if (nums[i] + nums[j] + nums[z]) == target {
                    return vec![i as i32, j as i32, z as i32];
                }
            }
        }
    }

    vec![]
}
