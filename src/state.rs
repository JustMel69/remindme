use std::{fs::File, io::{BufReader, BufWriter, BufRead, Write}, path::PathBuf};

use directories::ProjectDirs;

pub struct State {
    dir: PathBuf,

    msg: Vec<String>,
    curr_session: u32,
    last_session: u32,
}

impl State {
    pub fn load() -> std::io::Result<Self> {
        let dirs = ProjectDirs::from("com", "Mel", "RemindMe").expect("Couldn't fetch project directory paths.");
        let dir = dirs.data_dir().join("data");

        if dir.exists() {
            let reader = BufReader::new(File::open(&dir)?);
            let mut lines = reader.lines();

            let (curr_session, last_session) = parse_header(lines.next().expect("Missing header.")?.as_ref());

            let mut msg = Vec::new();
            for line in lines {
                let line = line?;
                msg.push(line);
            }

            return Ok(Self { dir, msg, curr_session, last_session });
        } else {
            return Ok(Self { dir, msg: Vec::new(), curr_session: 0, last_session: 0 });
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let mut dir = self.dir.clone();
        dir.pop();
        std::fs::create_dir_all(dir).expect("Couldn't create directories.");

        let mut writer = BufWriter::new(File::create(&self.dir)?);
        writer.write(format!("{}:{}", self.curr_session, self.last_session).as_bytes())?;
        
        for msg in &self.msg {
            writer.write(b"\n")?;
            writer.write(msg.as_bytes())?;
        }

        return Ok(());
    }

    pub fn match_session(&mut self) {
        self.last_session = self.curr_session;
    }

    pub fn matches_session(&self) -> bool {
        return self.last_session == self.curr_session;
    }

    pub fn next_session(&mut self) {
        self.curr_session += 1;
    }
 
    pub fn messages(&self) -> &[String] {
        &self.msg
    }

    pub fn push_msg(&mut self, msg: &str) {
        self.msg.push(msg.into());
    }

    pub fn clear_messages(&mut self) {
        self.msg.clear();
    }
}


fn parse_header(line: &str) -> (u32, u32) {
    let mut split = line.split(':');
    let curr_session = split.next().expect("malformed header.").parse::<u32>().unwrap();
    let last_session = split.next().expect("malformed header.").parse::<u32>().unwrap();
    return (curr_session, last_session);
}
