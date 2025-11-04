fn structs() {
    let _book = Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
        publication_year: 1925,
    };

    println!("My book is {:?}", _book);

    let book = create_book(
        String::from("The Great Gatsby"),
        String::from("F. Scott Fitzgerald"),
        1925,
    );

    let book_data = get_book_data(book);
    for data in book_data {
        println!("{data}");
    }

    let tuple_book = TupleBook("Somebook".to_string(), "Wong".to_string(), 32);
    let _title = tuple_book.0;
    let _author = tuple_book.1;
    let _publication_year = tuple_book.2;

    let _unit_book = UnitBook;

    let my_rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    let area = my_rectangle.area();
    println!("The area of the rectangle is {}", area);
}

#[derive(Debug)] // With debug trait, we can print the struct in a debug format. using {:?}.
                 // Structs are used to create custom data types.
struct Book {
    title: String,
    author: String,
    publication_year: u32,
}

// Tuple structs are used to create custom data types with a fixed number of values of different types.
struct TupleBook(String, String, u32); // Only types are needed, no names are needed.

// unit structs are used to create custom data types with a single value.
struct UnitBook;

fn get_book_data(book: Book) -> [String; 3] {
    let title = book.title;
    let author = book.author;
    let publication_year = book.publication_year;

    let data: [String; 3] = [title, author, publication_year.to_string()];
    data
}

fn create_book(title: String, author: String, publication_year: u32) -> Book {
    Book {
        title,
        author,
        publication_year,
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

// implement methods for struct Rectangle
impl Rectangle {
    // &self refers to the instance that is calling this method
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
