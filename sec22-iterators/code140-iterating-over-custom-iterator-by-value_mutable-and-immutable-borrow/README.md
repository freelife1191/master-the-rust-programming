# Iterating over custom iterator by value, mutable and immutable borrow

### nightly 설치 및 적용

```shell
# nightly 설치
$ rustup install nightly

# nightly로 변경
$ rustc +nightly -Zunpretty=hir src/main.rs
```

**코드의 확장된 버전 생성**

```rust
#[prelude_import]
use ::std::prelude::rust_2015::*;
#[macro_use]
extern crate std;
struct Booking {
    date: String,
    guest_name: String,
    room_number: u32,
}
#[automatically_derived]
impl ::core::fmt::Debug for Booking {
    #[inline]
    fn fmt<'_, '_, '_>(self: &'_ Self, f: &'_ mut ::core::fmt::Formatter<>)
        ->
            ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(f, "Booking",
            "date", &self.date, "guest_name", &self.guest_name, "room_number",
            &&self.room_number)
    }
}


impl Booking {
    fn new(date: String, guest_name: String, room_number: u32)
        -> Self { Booking{ date,  guest_name,  room_number,} }
}


struct BookingOnDate<'a> {
    date: &'a str,
    all_bookings: &'a Vec<Booking>,
    index: usize,
}

impl <'a> BookingOnDate<'a> {
    fn new(date: &'a str, all_bookings: &'a Vec<Booking>)
        -> Self { BookingOnDate{ date,  all_bookings,  index: 0,} }
}


impl <'a> Iterator for BookingOnDate<'a> {

    type
    Item
    =
    &'a Booking;

    fn next<'_>(self: &'_ mut Self)
        ->
            Option<Self::Item> {

        loop {
                if self.index < self.all_bookings.len()
                        {
                            let booking = &self.all_bookings[self.index];
                            self.index += 1;
                            if self.date == booking.date { return Some(booking); }
                                } else { break; } }

                        None
                    }

            }

            fn main() {

                let mut all_bookings: Vec<Booking> = Vec::new();

                let booking_1 =
                    Booking::new("2023-10-30".to_string(), "Mr.X".to_string(),
                        103);
                let booking_2 =
                    Booking::new("2023-10-30".to_string(), "Mr.Y".to_string(),
                        193);
                let booking_3 =
                    Booking::new("2023-10-25".to_string(), "Mr.Z".to_string(),
                        123);

                all_bookings.push(booking_1);
                all_bookings.push(booking_2);
                all_bookings.push(booking_3);


                let bookings =
                    BookingOnDate::new("2023-10-30", &all_bookings);


                {
                        let _t =
                            match #[lang = "into_iter"](bookings) {
                                    mut iter =>
                                        loop {
                                                match #[lang = "next"](&mut iter) {
                                                        #[lang = "None"] {} => break,
                                                        #[lang = "Some"] {  0: booking } => {

                                                            // println!("{}", bookings.date); // Error
                                                            // let first_entry = bookings.next(); // Error

                                                            {
                                                                ::std::io::_print(format_arguments::new_v1(&["", "\n"],
                                                                        &[format_argument::new_debug(&booking)]));
                                                            }
                                                        }
                                                    }
                                            },
                                };
                        _t
                    }
            }
```