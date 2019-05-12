#![allow(dead_code)]
#![allow(unused_variables)]

use archery::*;

struct Image {}

// The type parameter `P` will allow you to choose between an `Rc` or `Arc` pointer:
struct Book<P: SharedPointerKind> {
    // A `SharedPointer` is a reference-counting pointer that can be atomic or non-atomic depending
    // on the second type argument:
    cover: SharedPointer<Image, P>,
    text: Vec<String>,
}

impl<P: SharedPointerKind> Book<P> {
    fn new(cover: Image, text: Vec<String>) -> Book<P> {
        Book {
            // Creates a shared pointer of kind `P`:
            cover: SharedPointer::new(cover),
            text,
        }
    }
}

fn main() {
    // To create a book that uses a non-atomic reference-counting pointer you specify the
    // `SharedPointerKindRc` type argument.
    let book_rc = Book::<SharedPointerKindRc>::new(Image {}, Vec::new());

    // Similarly, you can create an atomic reference-counting pointer with the
    // `SharedPointerKindArc` type argument.
    let book_arc = Book::<SharedPointerKindArc>::new(Image {}, Vec::new());

    // `book_arc` will have a `cover` that is backed by an `Arc` pointer, thus implementing `Sync`:
    let _: Box<dyn Sync> = Box::new(book_arc);
}
