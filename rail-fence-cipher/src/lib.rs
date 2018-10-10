pub struct RailFence {
    rails: usize,
}

enum RailFenceAction {
    Encode,
    Decode,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        self.process(text, RailFenceAction::Encode)
    }

    pub fn decode(&self, cipher: &str) -> String {
        self.process(cipher, RailFenceAction::Decode)
    }

    fn process(&self, text: &str, action: RailFenceAction) -> String {
        let in_chars: Vec<char> = text.chars().collect();
        let mut out_chars: Vec<char> = text.chars().collect();

        let mut column;
        let (mut in_pos, mut out_pos) = (0, 0);
        let longest_step = 2 * (self.rails - 1);
        let chars_count = text.chars().count();

        for rail in 0..self.rails {
            let mut next_step = 2 * rail;

            column = rail;

            match action {
                RailFenceAction::Encode => in_pos = column,
                RailFenceAction::Decode => out_pos = column,
            };

            while column < chars_count {
                out_chars[out_pos] = in_chars[in_pos];

                if next_step != longest_step {
                    next_step = longest_step - next_step;
                }

                column += next_step;

                out_pos = match action {
                    RailFenceAction::Encode => out_pos + 1,
                    RailFenceAction::Decode => column,
                };

                in_pos = match action {
                    RailFenceAction::Encode => column,
                    RailFenceAction::Decode => in_pos + 1,
                };
            }
        }

        out_chars.iter().collect()
    }
}
