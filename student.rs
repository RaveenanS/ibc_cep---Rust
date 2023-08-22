struct Student {
    name: String,
    email: String,
    phone: String,
    id: u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    
    students.push(Student {
        name: String::from("Ravee"),
        email: String::from("ravee@gmail.com"),
        phone: String::from("7912912358"),
        id: 1,
    });

    students.push(Student {
        name: String::from("pooja"),
        email: String::from("rpooja@gmail.com"),
        phone: String::from("9638875121"),
        id: 2,
    });

    students.push(Student {
        name: String::from("sham"),
        email: String::from("sham@gmail.com"),
        phone: String::from("9344751630"),
        id: 3,
    });

    students.push(Student {
        name: String::from("karthi"),
        email: String::from("karthi@example.com"),
        phone: String::from("8514955751"),
        id: 4,
    });

    students.push(Student {
        name: String::from("Ajai"),
        email: String::from("ajai@example.com"),
        phone: String::from("9962345452"),
        id: 5,
    });

    
    let index = 3; 
    match students.get(index) {
        Some(student) => {
            println!("Student ID: {}", student.id);
            println!("Name: {}", student.name);
            println!("Email: {}", student.email);
            println!("Phone: {}", student.phone);
        }
        None => {
            println!("Student not found at index {}", index);
        }
    }
}
