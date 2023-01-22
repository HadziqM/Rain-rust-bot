use serenity::utils::Colour;

pub fn color(red:&str,green:&str,blue:&str)-> Colour{
    let some_u32 = u32::from_str_radix(&[red,green,blue].concat(), 16);
    Colour::new(some_u32.unwrap())
}
