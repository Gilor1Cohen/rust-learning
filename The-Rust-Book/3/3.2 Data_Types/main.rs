fn main() {
    /// Why does Rust need Data Types?
    /// To prevent compile-time errors.
    /// Example:
    let age: u8 = 21;
    let name: &str = "Rust";

    let result = age + name; // Error


// In Rust, every value has a type.
// A type tells Rust what kind of data the value is,
// and what operations are allowed on it.


// ===============================
// 1. Integer Types
// ===============================

// Signed integers: can be negative or positive.
// i = signed integer.
// The number says how many bits the type uses.

let x: i8 = -10;        // i8  = small signed integer
let x: i16 = -1000;     // i16 = signed integer
let x: i32 = -50000;    // i32 = default signed integer type
let x: i64 = -900000;   // i64 = larger signed integer
let x: i128 = -999999;  // i128 = very large signed integer
let x: isize = -5;      // isize = signed integer based on computer architecture


// Unsigned integers: only zero or positive.
// u = unsigned integer.

let x: u8 = 255;        // u8  = small positive integer, 0 to 255
let x: u16 = 1000;      // u16 = positive integer
let x: u32 = 50000;     // u32 = common positive integer
let x: u64 = 900000;    // u64 = larger positive integer
let x: u128 = 999999;   // u128 = very large positive integer
let x: usize = 5;       // usize = used often for indexes and sizes


// ===============================
// 2. Floating Point Types
// ===============================

let price: f32 = 19.99; // f32 = 32-bit decimal number
let price: f64 = 19.99; // f64 = 64-bit decimal number, default and more common


// ===============================
// 3. Boolean Type
// ===============================

let is_active: bool = true;  // bool can be true
let is_admin: bool = false;  // or false


// ===============================
// 4. Character Type
// ===============================

let letter: char = 'A';  // char = one character
let emoji: char = '🔥';  // char can also hold Unicode characters

// ===============================
// 5. String Slice
// ===============================

let name: &str = "Rust"; 
// &str = string slice.
// Usually used for fixed text.
// Text with double quotes is usually &str.

 // ===============================
 // 6. The Tuple Type
 // ===============================

 // A tuple is a group of values stored together.
 // A tuple can contain different types.

 let user: (&str, u32, bool) = ("Rust", 22, true);

 // This tuple contains:
 // user.0 = &str  -> "Rust"
 // user.1 = u32   -> 22
 // user.2 = bool  -> true

 println!("Name: {}", user.0);
 println!("Age: {}", user.1);
 println!("Is active: {}", user.2);


    // ===============================
    // 7. The Array Type
    // ===============================

    // An array is a group of values stored together.
    // In Rust, all array values must have the same type.
    // Arrays have a fixed size.

    let numbers: [i32; 5] = [10, 20, 30, 40, 50];

    // [i32; 5] means:
    // i32 = the type of each element
    // 5   = the number of elements in the array

    println!("Numbers: {:?}", numbers);

    // We access array elements using an index.
    // Index starts from 0, not 1.

    let first_number = numbers[0];  // 10
    let second_number = numbers[1]; // 20
    let third_number = numbers[2];  // 30


    // ===============================
    // Numeric Operations:
    // ===============================
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

}