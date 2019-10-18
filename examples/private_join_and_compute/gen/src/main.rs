use std::io::Write;
use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use rand::Rng;

static DATA_FILE_A: &str = "bank_a.txt";
static DATA_FILE_B: &str = "bank_b.txt";
static DATA_FILE_C: &str = "bank_c.txt";
static DATA_FILE_D: &str = "bank_d.txt";

static RESULT_FILE_1: &str = "result1.txt";
static RESULT_FILE_2: &str = "result2.txt";
static RESULT_FILE_3: &str = "result3.txt";
static RESULT_FILE_4: &str = "result4.txt";

fn print_usage() {
    let msg = "
    ./gen root count

    count: data count per bank

    ";
    println!("usage: \n{}", msg);
}

fn remove_files(root: &str) {
    let _ = fs::remove_file(Path::new(root).join(DATA_FILE_A).to_str().unwrap());
    let _ = fs::remove_file(Path::new(root).join(DATA_FILE_B).to_str().unwrap());
    let _ = fs::remove_file(Path::new(root).join(DATA_FILE_C).to_str().unwrap());
    let _ = fs::remove_file(Path::new(root).join(DATA_FILE_D).to_str().unwrap());

    let _ = fs::remove_file(Path::new(root).join(RESULT_FILE_1).to_str().unwrap());
    let _ = fs::remove_file(Path::new(root).join(RESULT_FILE_2).to_str().unwrap());
    let _ = fs::remove_file(Path::new(root).join(RESULT_FILE_3).to_str().unwrap());
    let _ = fs::remove_file(Path::new(root).join(RESULT_FILE_4).to_str().unwrap());
}

fn gen_data(count: i32) -> HashMap<i32,i32> {
    let mut hm = HashMap::new();

    let mut base:i32 = 0;
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let r: i32 = rng.gen();
        let offset:i32 = r.abs() % 3 + 1;
        base += offset;
        hm.insert(base, r.abs()%100000);
    }

    return hm
}

fn merge(hm1: &HashMap<i32,i32>, hm2: &HashMap<i32,i32>) -> HashMap<i32,i32> {
    let mut hm = HashMap::new();

    for (k, v) in hm1{
        hm2.get(k).map(|v2| hm.insert(*k, *v + *v2));
    }

    return hm
}

fn write_data(hm: &HashMap<i32,i32>, path: &str){
    let mut f = fs::File::create(path).expect("create error");
    for (k, v) in hm {
        writeln!(f, "{}:{}", k, v).expect("write error");
    }
    f.flush().expect("flush file error")
}

fn write_result(bank: &str, hm: &HashMap<i32,i32>, path: &str){
    let mut f = fs::File::create(path).expect("create error");
    writeln!(f, "").expect("write error");
    writeln!(f, "[+] Task status is Finished").expect("write error");
    writeln!(f, "{} get result: ", bank).expect("write error");
    for (k, v) in hm {
        writeln!(f, "{} : {}", k, v).expect("write error");
    }
    f.flush().expect("flush file error");
}

fn main(){
    let args_string: Vec<String> = env::args().collect();
    let args: Vec<&str> = args_string.iter().map(|s| s.as_str()).collect();

    if args.len() < 3{
        print_usage();
        return;
    }

    let root: &str = args[1];
    println!("Root: {}", root);

    let count: i32 = args[2].parse().unwrap();
    println!("Generate {} records for 4 banks", count);
    
    remove_files(root);

    let data_a = gen_data(count);
    let data_b = gen_data(count);
    let data_c = gen_data(count);
    let data_d = gen_data(count);

    
    let result = merge(&data_a, &data_b);
    let result = merge(&result, &data_c);
    let result = merge(&result, &data_d);
    
    write_data(&data_a, Path::new(root).join("four_party_data").join(DATA_FILE_A).to_str().unwrap());
    write_data(&data_b, Path::new(root).join("four_party_data").join(DATA_FILE_B).to_str().unwrap());
    write_data(&data_c, Path::new(root).join("four_party_data").join(DATA_FILE_C).to_str().unwrap());
    write_data(&data_d, Path::new(root).join("four_party_data").join(DATA_FILE_D).to_str().unwrap());

    write_result("Bank_A", &result, Path::new(root).join("four_party_results").join(RESULT_FILE_1).to_str().unwrap());
    write_result("Bank_B", &result, Path::new(root).join("four_party_results").join(RESULT_FILE_2).to_str().unwrap());
    write_result("Bank_C", &result, Path::new(root).join("four_party_results").join(RESULT_FILE_3).to_str().unwrap());
    write_result("Bank_D", &result, Path::new(root).join("four_party_results").join(RESULT_FILE_4).to_str().unwrap());

}