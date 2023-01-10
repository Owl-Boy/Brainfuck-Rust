use text_io::try_read;

#[derive(Debug)]
pub struct Memory {
    pub mem: [u8; 30_000],
    pub mem_ptr: usize,
    pub code_ptr: usize,
    pub pairs: Vec<(usize, usize)>,
    cmd: Vec<Command>
}

impl Memory {
    pub fn new(code: String) -> Result<Self, Error> {
        let mut stack = vec![];
        let mut pairs = vec![];
        let command_list: Vec<Command> = code.chars().map(|ch| char_to_command(ch)).collect();

        for (idx, command) in command_list.iter().enumerate() {
            match command {
                Command::Invalid => return Err(Error::InvalidCharErr),
                Command::LoopOpen => stack.push(idx),
                Command::LoopClose => {
                    if let Some(i) = stack.pop() {
                        pairs.push((i, idx));
                    } else {
                        return Err(Error::MatchErr);
                    }
                },
                _ => {}
            }
        }
        if !stack.is_empty() {
            Err(Error::MatchErr)
        } else {
            Ok (Memory {
                mem: [0; 30_000],
                mem_ptr: 0,
                code_ptr: 0,
                pairs,
                cmd: command_list
            })
        }
    }

    pub fn execute(&mut self, cmd: Command) -> Result<(), Error> {
        match cmd {
            Command::Add => {
                if self.mem[self.mem_ptr] == 255 {
                    self.mem[self.mem_ptr] = 0;
                } else {
                    self.mem[self.mem_ptr] += 1;
                }
                Ok(())
            },
            Command::Sub => {
                if self.mem[self.mem_ptr] == 0 {
                    self.mem[self.mem_ptr] = 255;
                } else {
                    self.mem[self.mem_ptr] -= 1;
                }
                Ok(())
            },
            Command::Right => {
                if self.mem_ptr == 30_000 {
                    Err(Error::IndexErr)
                } else {
                    self.mem_ptr += 1;
                    Ok(())
                }
            },
            Command::Left => {
                if self.mem_ptr == 0 {
                    Err(Error::IndexErr)
                } else {
                    self.mem_ptr -= 1;
                    Ok(())
                }
            },
            Command::Out => {
                print!("{}", std::char::from_u32(self.mem[self.mem_ptr] as u32).unwrap());
                Ok(())
            },
            Command::Inp => {
                let inp: Result<u8, text_io::Error> = try_read!();
                match inp {
                    Ok(val) => {
                        self.mem[self.mem_ptr] = val;
                        Ok(())
                    },
                    _ => Err(Error::InputErr)
                }
            },
            Command::LoopOpen => {
                if self.mem[self.mem_ptr] == 0 {
                    self.code_ptr = find(&self.pairs, self.code_ptr).unwrap();
                }
                Ok(())
            },
            Command::LoopClose => {
                if self.mem[self.mem_ptr] != 0 {
                    self.code_ptr = find(&self.pairs, self.code_ptr).unwrap();
                }
                Ok(())
            },
            Command::Invalid => {
                Err(Error::InvalidCharErr)
            }
        }
    }

    pub fn run(mut self) -> Result<(), Error> {
        while self.code_ptr < self.cmd.len() {
            if let Err(val) = self.execute(self.cmd[self.code_ptr]) {
                return Err(val)
            }
            self.code_ptr += 1;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Add,
    Sub,
    Right,
    Left,
    Out,
    Inp,
    LoopOpen,
    LoopClose,
    Invalid
}

fn char_to_command(ch: char) -> Command {
    match ch {
        '+' => Command::Add,
        '-' => Command::Sub,
        '>' => Command::Right,
        '<' => Command::Left,
        '.' => Command::Out,
        ',' => Command::Inp,
        '[' => Command::LoopOpen,
        ']' => Command::LoopClose,
        _   => Command::Invalid,
    }
}

fn find(ls: &Vec<(usize, usize)>, el: usize) -> Option<usize> {
    for (a, b) in ls.iter() {
        if *a == el { return Some(*b) }
        if *b == el { return Some(*a) }
    }
    return None
}

#[derive(Debug)]
pub enum Error {
    IndexErr,
    InputErr,
    MatchErr,
    InvalidCharErr
}
