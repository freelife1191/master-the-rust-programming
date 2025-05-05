#[derive(Debug)]
struct Booking {
    date: String,
    guest_name: String,
    room_number: u32,
}


impl Booking {
    fn new(date: String, guest_name: String, room_number: u32) -> Self {
        Booking {date, guest_name, room_number}
    }
}


struct BookingOnDate<'a> {
    date: &'a str,
    // all_bookings: &'a Vec<Booking>,
    // index: usize,
    booking_iter: std::slice::Iter<'a, Booking>,
}

impl<'a> BookingOnDate<'a> {
    fn new(date: &'a str, all_bookings: &'a Vec<Booking>) -> Self {
        // BookingOnDate {date, all_bookings, index: 0}
        BookingOnDate {date, booking_iter: all_bookings.iter()}
    }
}


impl<'a> Iterator for BookingOnDate<'a> {

    type Item = &'a Booking;

    fn next(&mut self) -> Option<Self::Item> {
        /*
        while self.index < self.all_bookings.len() {
            let booking = &self.all_bookings[self.index];
            self.index += 1;
            if self.date == booking.date {
                return Some(booking);
            }
        }

        None
        */
        // &&Booking
        self.booking_iter.find(|booking| booking.date == self.date)
    }

}

fn main() {

    let mut all_bookings: Vec<Booking> = Vec::new();

    let booking_1 = Booking::new("2023-10-30".to_string(), "Mr.X".to_string(), 103);
    let booking_2 = Booking::new("2023-10-30".to_string(), "Mr.Y".to_string(), 193);
    let booking_3 = Booking::new("2023-10-25".to_string(), "Mr.Z".to_string(), 123);

    all_bookings.push(booking_1);
    all_bookings.push(booking_2);
    all_bookings.push(booking_3);

    // let bookings = BookingOnDate::new("2023-10-30", &all_bookings);
    let bookings = BookingOnDate::new("2023-10-25", &all_bookings);

    for booking in bookings {
        println!("{:?}", booking)
    }

}