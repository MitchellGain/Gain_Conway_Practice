fn num_neighbors(field: &Vec<Vec<i32>>, x: i32, y: i32, size: i32) -> i32{
    let mut ret: i32 = 0;
    let mut checks: Vec<Vec<i32>> = vec![];

    let x_zero: bool = x > 0;
    let x_end: bool = x < size - 1;
    let y_zero: bool = y > 0;
    let y_end: bool = y < size -1;

    if x_zero {
        if y_end {
            checks.push(vec![-1, 1]);
        }
        checks.push(vec![-1, 0]);
        if y_zero{
            checks.push(vec![-1, -1]);
        }
    }
    if x_end {
        if y_end {
            checks.push(vec![1, 1]);
        }
        checks.push(vec![1, 0]);
        if y_zero{
            checks.push(vec![1, -1]);
        }
    }

    if y_zero {
        checks.push(vec![0, -1]);
    }
    if y_end {
        checks.push(vec![0, 1]);
    }

    for check in checks.iter(){
        if field[(y+check[1]) as usize][(x+check[0]) as usize] == 1{
            ret  = ret + 1;
        }
    }

    return ret
}

pub fn run(cur: &Vec<Vec<i32>>, size: i32) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = Vec::with_capacity(size.try_into().unwrap());   
    for row in 0..cur.len(){
        let row_vec = &cur[row];
        let mut ret_row = Vec::with_capacity(size.try_into().unwrap()); 
        for col in 0..row_vec.len(){
            let neighbors = num_neighbors(&cur, col as i32, row as i32, size);
            if neighbors == 3{
                ret_row.push(1);
            }
            if neighbors == 2{
                ret_row.push(cur[row][col]);
            }
            if neighbors < 2 || neighbors > 3{
                ret_row.push(0);
            }
        }
        ret.push(ret_row);
    }
    return ret;
}