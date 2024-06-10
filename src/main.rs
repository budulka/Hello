
use std::io;

fn main() {
    intro();
    let mut size : usize;
    loop {
        size = get_size();
        println!("{size}");
        let mut matrix : Vec<Vec<f32>> = vec![vec![0.0;size]; size];
        let mut free_terms : Vec<f32> = vec![0.0;size];
        let mut roots : Vec<f32> = vec![0.0;size];
        get_matrix(size, &mut matrix, &mut free_terms);
        if is_solvable(matrix.clone()) {
            roots = solve(matrix, free_terms);
            print_roots(&roots);
        } else {
            println!("Not solvable");
            continue;
        }
    }
}

fn intro() {
    println!("OWO OWO OWO OWO U LAUNCHED MY PROGRAM UWUUU :3 ~~ meow OWO");
    println!("Hehe, now enter matrix size");
}

fn get_size() -> usize{
    loop {
    let mut input = String::new();
    io :: stdin() .read_line(&mut input).expect("Invalid input");
    match input.trim().parse::<usize>() {

        Ok(num) => if num > 1 && num <= 5 { return num; } else {println!("Choort!");} 
        Err(_) => println!("Chort!")
        }
    }

}
fn get_number() -> f32{
    loop {
    let mut input = String::new();
    io :: stdin() .read_line(&mut input).expect("Invalid input");
    match input.trim().parse::<f32>() {
        Ok(num) => if num >= -100.0 && num <= 100.0 { return num; } else {println!("Choort!"); } 
        Err(_) => println!("Chort!")
        }
    }
}

fn get_matrix(size : usize,  matrix : &mut Vec<Vec<f32>>,  free_terms : &mut Vec<f32>){
    for i  in 0..size {
        for j in 0..size {
            println!("Enter coef [{i}][{j}]:");
            matrix[i][j] = get_number();
        }
        println!("Enter free term[{i}]:");
        free_terms[i] = get_number();
    }
}

fn print_matrix(matrix : &Vec<Vec<f32>>) {
    for i in matrix {
        for j in i {
            print!("{j} ")
        }
        println!();
    }
}

fn is_solvable(matrix : Vec<Vec<f32>>) -> bool {
    let size : usize = matrix.len();
    let mut sum;
    for i in 0..size {
        sum = 0.0;
        for j in 0..size {
            sum += matrix[i][j];
        }
        sum -= matrix[i][i];
        if sum.abs() > matrix[i][i].abs() {
            return false;
        }
    }
    true
}

fn solve(matrix : Vec<Vec<f32>>, free_terms : Vec<f32> ) -> Vec<f32> {
    let size : usize = free_terms.len();
    let mut solution : Vec<f32> = vec![0.0; size];
    let mut deltas:Vec<f32> = vec![0.0;size];
    let mut temp_sum : f32;
    let mut prev : f32;
    loop {
        for i in 0..size {
            prev = solution[i];
            temp_sum = isolate(& matrix, &solution , i);
            solution[i] = (free_terms[i] - temp_sum) / matrix[i][i];
            deltas[i] = (solution[i] - prev) / solution[i];
        }
        if max_val(&deltas) < 0.001 {
            print_matrix(&matrix);
            return solution;
        }
    }

    
}

fn max_val(array : &Vec<f32>) -> f32 {
     let mut max = array[0];
     for i in 1..array.len()-1 {
        max = if array[i] < array[i+1] {array[i+1]} else {array[i]};
     }
     return max;
}

fn isolate(matrix : & Vec<Vec<f32>>, prev_iteration: & Vec<f32>, i : usize ) -> f32 {
    let mut sum : f32 = 0.0;
    let size = prev_iteration.len();
    for j in 0..size {
        sum +=matrix[i][j] * prev_iteration[j];
    }
    sum -= matrix[i][i] * prev_iteration[i];
    sum
}
fn print_roots(roots: &Vec<f32>) {
    let size = roots.len();
    for i in 0..size {
        println!("x[{i}] = {}", roots[i]);
    }
}

fn print_vector(vec: &Vec<f32>) {
    for (i, element) in vec.iter().enumerate() {
        println!("Element {}: {}", i, element);
    }
}