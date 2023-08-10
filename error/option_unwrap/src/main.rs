struct Person {
    job: Option<Job>,
}

#[derive(Copy, Clone)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Copy, Clone)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(1),
                number: 1,
            })
        })
    };

    assert_eq!(p.work_phone_area_code(), Some(1));

    let x1 = Some(1).map(|a| a + 1);
    println!("{:?}", x1);

    let x2 = Some(1).and_then(|a| Some(a + 1));
    println!("{:?}", x2);

    let x3 = None.or(Some(2));
    println!("{:?}", x3);

    let x4 = None.or_else(|| Some(1));
    println!("{:?}", x4);

    let mut binding = None;
    let x5 = binding.get_or_insert(1);
    println!("{:?}", x5);

    let x6 = binding.get_or_insert_with(|| 1);
    println!("{:?}", x6);

    let x7 = None.map_or(1, |a: i32| a + 1);
    println!("{:?}", x7);
}
