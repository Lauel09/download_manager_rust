# Rust downloader manager

A download manager I have been writing for past 4-5 days.
One of my first personal projects.

# To run the program
 - Clone the git repo
 - Rust toolchain installed with 1.49+
 - Change directory to the repo's one and type `cargo build` and then you could find the binary in `target/debug/<program_name>`, you can find help there onwards.

#### Only library used here is **reqwest**, other than that it is purely based upon Rust standard library

---

## Things that could be improved
1. File handling is the one which leads to bug here most,like what if there is another file with same given name, or program doesn't has permission to write to a folder ?

2. Another important thing that could be implemented here would be checking whether the given url is valid or not, regex could be used or try to send the reqwest and cross check with the status code.

3. **Bugs** ,a lot of bugs.

---

### I learned about:-
1. Shared references in Rust
2. `std::cell` and `std::RefCell` but I avoided using them, I wanted to keep things as simple as possible.
3. I understood how difficult it is to manage large codebase, I had to rewrite it twice(250+ LOC), I wonder how projects like Linux kernel are managed.
4. A lot of fights with borrow checker, it was frustrating at first then I had to redesign everything(second time), to prevent these clashes.

---

*I am looking forward to work upon Pytesseract,OpenCV and PIL.* 
; )

