use clap_v3::{App, Arg};
use colored::Colorize;
use core::panic;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

fn compile_the_program(algo_file: &str) {
    let pth = format!("{}.cpp", algo_file);
    if Path::new(&pth).try_exists().is_ok_and(|x| x == false) {
        panic!("No file at {}", pth);
    }
    fs::create_dir("cppchetmp").unwrap_or_default();
    Command::new("sh").arg("-c").arg("rm cppchetmp/program");
    Command::new("sh")
        .arg("-c")
        .arg(format!("g++ {}.cpp -o cppchetmp/program", algo_file))
        .output()
        .expect("failed to execute process");
    Command::new("-c").arg("mv program cppchetmp/program");
    if Path::new("cppchetmp/program")
        .try_exists()
        .is_ok_and(|x| x == false)
    {
        panic!("could not compile");
    }
}
fn check_if_program_output_if_fine(amt: u32, input_file: &str, output_file: &str) -> Option<f32> {
    let tm = Instant::now();
    if Path::new(&format!("{}{}.in", input_file, amt))
        .try_exists()
        .is_ok_and(|x| x == false)
    {
        panic!("Could not find; {}{}.in", input_file, amt);
    }

    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "cppchetmp/program < {}{}.in > cppchetmp/tmp{}.out",
            input_file, amt, amt
        ))
        .output()
        .expect("failed to execute process");

    let outfile = fs::read_to_string(format!("{}{}.out", output_file, amt));

    if outfile.is_err() {
        panic!("Could not find; {}{}.out", output_file, amt);
    }
    if Path::new(&format!("cppchetmp/tmp{}.out", amt))
        .try_exists()
        .is_ok_and(|x| x == false)
    {
        let txt = format!("Test wrong (hasn't returned an output). (test no: {})", amt);
        println!("{}", txt.red());
        return None;
    }
    let tmpfile = fs::read_to_string(format!("cppchetmp/tmp{}.out", amt));

    let oout = outfile.unwrap();
    let otmp = tmpfile.unwrap();
    let out = oout
        .clone()
        .split_whitespace()
        .fold("".to_owned(), |a, b| format!("{}{}", a, b));

    let tmp = otmp
        .clone()
        .split_whitespace()
        .fold("".to_owned(), |a, b| format!("{}{}", a, b));

    if !(out == tmp) {
        println!("{} at test n: {}", "Wrong Anwser".red(), amt);
        println!("{}: {}", "Expected".blue(), oout);
        println!("{}: {}", "Got".blue(), otmp);
        return None;
    }

    Some(tm.elapsed().as_secs_f32())
}

fn main() {
    let matches = App::new("cpp tester.\n
        Example Usage:\n
        checkr -a algo -i testyin/t -o testyo/t -b 0 -e 20 -n 5\n
        Will compile the algo.cpp file, run it with input testyin/ti.in (i being a number between 0 and 20)\n
        And test the output to a file testyo/ti.out, while reporting every 5 tests (to show it hasn't halted)
        ")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .value_name("input file path")
                .help("input directory/nameprefix ( fe: testy/test )")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .value_name("output file path")
                .help("output directory/nameprefix ( fe: testy/test )")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bg")
                .short('b')
                .long("begin")
                .value_name("number")
                .help("first test number")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("en")
                .short('e')
                .long("end")
                .value_name("number")
                .help("last test number")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("algo")
                .short('a')
                .long("algo")
                .value_name("algorythm file path")
                .help("the algorythm file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("all_pass")
                .short('p')
                .long("allpass")
                .value_name("bool")
                .help("makes it that all tests pass :3")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("stop if wrong")
                .short('w')
                .long("stw")
                .value_name("bool")
                .help("Halts if a test was incorect")
                .takes_value(true),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap_or("testy/test");
    let iclone = input.to_owned().clone();
    let output = matches.value_of("output").unwrap_or(&iclone);
    let algo = matches.value_of("algo").unwrap_or("algo");
    let all_pas = matches
        .value_of("all_pass")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0);
    let halt = matches.value_of("stw").unwrap_or("0").parse().unwrap_or(0);

    let begin: usize = matches.value_of("bg").unwrap_or("1").parse().unwrap_or(1);
    let end: usize = matches.value_of("en").unwrap_or("10").parse().unwrap_or(10);
    compile_the_program(algo);
    let mut inc = 0;

    (begin..end + 1).for_each(|i| {
        println!("Testing {i}");
        let out = check_if_program_output_if_fine(i as u32, input, output);
        if out.is_none() && all_pas == 0 {
            inc += 1;
            if halt == 1 {
                panic!("{}", format!("Test {} incorect!", i).red());
            }
        } else {
            let out = out.unwrap();
            println!(
                "{}",
                format!("Test {} correct! it took: {:.3}s", i, out).green()
            );
        }
    });

    if inc == 0 {
        println!("{}", "All Correct :3".green());
        return;
    }
    let crt = (end - begin + 1) - inc;
    println!("{} {}", crt, "Correct".green());
    println!("{} {} out of {}", inc, "Incorect".red(), end - begin + 1);
    println!(
        "That is {:.1}%",
        (inc as f32) / (end - begin + 1) as f32 * 100.
    );
}
