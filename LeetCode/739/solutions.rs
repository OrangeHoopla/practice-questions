impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {

        let n = temperatures.len();
        let mut result:Vec<i32> = vec![0; n];
        let mut st:Vec<usize> = Vec::new();   //Stack

        for i in 0..n {
            while let Some(&last) = st.last() {
                if temperatures[i] > temperatures[last] {
                    if let Some(prev_index) = st.pop() {
                        result[prev_index] = (i - prev_index) as i32;
                    }
                } else {
                    break;
                }
            }
            st.push(i);
        }
        return result;
        
    }
}