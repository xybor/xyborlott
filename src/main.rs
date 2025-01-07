use std::time::Instant;

use crate::{
    customer::phone,
    ticket::{draw_period, ticket_system},
    ticket_number::{loto::TicketNumberLoto, number6of55::TicketNumber6Of55, TicketNumber},
};

pub mod commitment;
pub mod customer;
pub mod ticket;
pub mod ticket_number;

fn main() {
    let draw_period = draw_period::DrawPeriod::new(0, 100, commitment::default());
    let mut system = ticket_system::TicketSystem::new(draw_period);

    let ticket1 = system.new_ticket(
        phone::PhoneCustomer::from_phone(String::from("0917281911")),
        TicketNumberLoto::from([04, 09, 19, 18, 08, 01]).unwrap(),
    );

    println!("{}", ticket1.to_string());
    println!(
        "==============={}==============",
        ticket1.customer.verify(&String::from("0917281912"))
    );

    let ticket2 = system.new_ticket(
        phone::PhoneCustomer::from_phone(String::from("0917281911")),
        TicketNumberLoto::from([11, 12, 03, 16, 13, 07]).unwrap(),
    );

    println!("{}", ticket2.to_string());
    println!(
        "==============={}==============",
        ticket2.customer.verify(&String::from("0917281911"))
    );

    let draw_commitment = &system.commit();

    let start = Instant::now();
    let number = ticket_number::loto::TicketNumberLoto::draw_from(
        system.get_ticket_numbers().as_slice(),
        draw_commitment,
    );
    println!("cost={:?} number={:?}", start.elapsed(), number)
}
