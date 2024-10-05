use std::{
    fs::{self},
    io::ErrorKind,
};

pub enum Command {
    ECHO,
    CAT,
    LS,
    UNSUPPORTTED,
}
#[derive(Debug)]
pub struct Commands {
    pub args: Vec<String>,
}
impl Commands {
    pub fn execute(&self) -> Result<(), &'static str> {
        let command = self.get_command();
        match command {
            Command::ECHO => self.echo(),
            Command::CAT => self.cat(),
            Command::LS => self.ls(),
            Command::UNSUPPORTTED => Err("Command is not support"),
        }
    }
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            Err("Se requiere al menos un argumento de comando.")
        } else {
            Ok(Self {
                args: args.to_vec(),
            })
        }
    }
    fn get_command(&self) -> Command {
        match self.args[1].to_lowercase().as_str() {
            "echo" => Command::ECHO,
            "cat" => Command::CAT,
            "ls" => Command::LS,
            _ => Command::UNSUPPORTTED,
        }
    }
    fn cat(&self) -> Result<(), &'static str> {
        match fs::read_to_string(self.args[2].to_lowercase().as_str()) {
            Ok(file) => println!("{}", file),
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    println!("The file not exist")
                } else {
                    println!("Error to read the file {}", err)
                }
            }
        }
        Ok(())
    }
    fn ls(&self) -> Result<(), &'static str> {
        match fs::read_dir(self.args[2].to_lowercase().as_str()) {
            Ok(file) => {
                for entry in file {
                    match entry {
                        Ok(entry) => match entry.file_name().to_str() {
                            Some(name) => println!("{}", name),
                            None => println!("Error al convertir el nombre del archivo a str"),
                        },
                        Err(err) => {
                            if err.kind() == ErrorKind::NotFound {
                                println!("La ruta no existe");
                            } else {
                                println!("Error al leer la entrada del directorio: {}", err);
                            }
                        }
                    }
                }
                Ok(())
            }
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    println!("La ruta no existe");
                } else {
                    println!("Error al leer el directorio: {}", err);
                }
                Ok(())
            }
        }
    }
    fn echo(&self) -> Result<(), &'static str> {
        let output = self
            .args
            .iter()
            .skip(2)
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", output);
        Ok(())
    }
}
