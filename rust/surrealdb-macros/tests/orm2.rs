struct Student {
    id: String,
    name: String,
}
struct User {
    id: String,
    nickie: String,
}

struct Book {
    id: String,
    title: String,
}
struct Poem {
    id: String,
    pages: u32,
}

enum Origin {
    Student(Student),
    User(User),
}
enum Destination {
    Book(Book),
    Poem(Poem),
}

struct Writes {
    id: String,
    r#in: Origin,
    out: Destination,
    extra_1: String,
    // extra_2: Datetime<Utc>,
}

struct StudentWritesBook {
    id: String,
    r#in: Student,
    out: Book,
    extra_1: String,
    //  extra_2: Datetime<Utc>
}

struct UserWritesBook {
    id: String,
    r#in: User,
    out: Book,
    extra_5: String,
}

struct UserWritesPoem {
    id: String,
    r#in: User,
    out: Poem,
    //  extra_9: Datetime<Utc>
}


enum Writes2 {
    StudentWritesBook(StudentWritesBook),
    UserWritesBook(UserWritesBook),
    UserWritesPoem(UserWritesPoem),
}

fn xxx() {
    match Writes2::StudentWritesBook(StudentWritesBook::default()) {
        Writes2::StudentWritesBook(x) => x.,
        Writes2::UserWritesBook(_) => todo!(),
        Writes2::UserWritesPoem(_) => todo!(),
    }
}