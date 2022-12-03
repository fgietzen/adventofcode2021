use std::convert::TryFrom;

#[derive(Clone)]
pub enum Command {
	HorizontalChange(i32),
	DepthChange(i32)
}

impl TryFrom<String> for Command {
	type Error = String;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		let mut split = value.split(" ");

		let command = split.next().ok_or("Could not parse command from line!")?;
		let value = split.next().ok_or("Could not read value from line!")?;
		let value = value.parse::<i32>().map_err(|_| "Could not parse value to int")?;

		if command == "forward" {
			return Ok(Command::HorizontalChange(value));
		}
		if command == "up" {
			return Ok(Command::DepthChange(-value));
		}
		if command == "down" {
			return Ok(Command::DepthChange(value));
		}

		return Err(format!("Could not find command {}", command));
	}
}
