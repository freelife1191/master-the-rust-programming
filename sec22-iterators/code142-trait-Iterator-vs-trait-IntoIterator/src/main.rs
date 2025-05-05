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
    booking_iter: std::slice::Iter<'a, Booking>,
}

impl<'a> BookingOnDate<'a> {
    fn new(date: &'a str, all_bookings: &'a Vec<Booking>) -> Self {
        BookingOnDate {date, booking_iter: all_bookings.iter()}
    }
}

struct BookingOnDateMut<'a> {
    date: &'a str,
    booking_iter: std::slice::IterMut<'a, Booking>,
}

impl<'a> BookingOnDateMut<'a> {
    fn new(date: &'a str, all_bookings: &'a mut Vec<Booking>) -> Self {
        BookingOnDateMut {date, booking_iter: all_bookings.iter_mut()}
    }
}


impl<'a> Iterator for BookingOnDate<'a> {

    type Item = &'a Booking;

    fn next(&mut self) -> Option<Self::Item> {
        // &&Booking
        self.booking_iter.find(|booking| booking.date == self.date)
    }

}

impl<'a> Iterator for BookingOnDateMut<'a> {

    type Item = &'a mut Booking;

    fn next(&mut self) -> Option<Self::Item> {
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

    // let bookings = BookingOnDate::new("2023-10-25", &all_bookings);
    let bookings = BookingOnDateMut::new("2023-10-25", &mut all_bookings);

    for booking in bookings {
        println!("{:?}", booking);
        booking.room_number = 100;
    }
    // Booking { date: "2023-10-25", guest_name: "Mr.Z", room_number: 123 }

}