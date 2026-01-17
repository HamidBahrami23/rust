#[derive(Debug)]
enum Job {
    Worker,
    Engineer,
    Teacher,
    HR,
    Editor,
}

#[derive(Debug)]
struct Employee {
    job: Job,
    worktime: i32,
    salaryratio: f64,
}

fn print_info(num: i32 ,empl: Employee) {
    println!("======= Employee number: {:?}",num);
    println!("{:?}",empl);
}

fn main() {
    let workers = vec![
        Employee {
            job: Job::Engineer,
            worktime: 40 ,
            salaryratio: 1.17,
        },
        Employee {
            job: Job::Worker,
            worktime: 27 ,
            salaryratio: 0.92
        },
        Employee {
            job: Job::HR,
            worktime: 20 ,
            salaryratio: 0.72 // I hate HRs
        },
        Employee {
            job: Job::Editor,
            worktime: 45 ,
            salaryratio: 1.0
        },
    ];
    let mut num = 1;

    for person in workers {
        print_info(num, person);
        num = num + 1 ;
    }


}