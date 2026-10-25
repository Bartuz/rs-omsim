use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub offset: usize,
    pub message: &'static str,
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at byte {}", self.message, self.offset)
    }
}
impl std::error::Error for ParseError {}
type Result<T> = std::result::Result<T, ParseError>;

// Strings are byte slices: the format does not require valid UTF-8. They borrow
// the input, avoiding string allocations while retaining safe ownership.
#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle<'a> {
    pub name: &'a [u8],
    pub creator: u64,
    pub parts_available: u64,
    pub inputs: Vec<Molecule>,
    pub outputs: Vec<Molecule>,
    pub output_scale: u32,
    pub production: Option<Production<'a>>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Atom {
    pub kind: u8,
    pub offset: [i8; 2],
}
#[derive(Debug, PartialEq, Eq)]
pub struct Bond {
    pub kind: u8,
    pub from: [i8; 2],
    pub to: [i8; 2],
}
#[derive(Debug, PartialEq, Eq)]
pub struct Production<'a> {
    pub shrink_left: bool,
    pub shrink_right: bool,
    pub isolate_inputs_from_outputs: bool,
    pub cabinets: Vec<Cabinet<'a>>,
    pub conduits: Vec<Conduit>,
    pub vials: Vec<Vial>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Cabinet<'a> {
    pub position: [i8; 2],
    pub kind: &'a [u8],
}
#[derive(Debug, PartialEq, Eq)]
pub struct Conduit {
    pub starting_position_a: [i8; 2],
    pub starting_position_b: [i8; 2],
    pub hexes: Vec<[i8; 2]>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Vial {
    pub position: [i8; 2],
    pub style: u8,
    pub count: u32,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Solution<'a> {
    pub puzzle: &'a [u8],
    pub name: &'a [u8],
    pub metrics: Option<Metrics>,
    pub parts: Vec<Part<'a>>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Metrics {
    pub cycles: u32,
    pub cost: u32,
    pub area: u32,
    pub instructions: u32,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Part<'a> {
    pub name: &'a [u8],
    pub position: [i32; 2],
    pub size: u32,
    pub rotation: i32,
    pub which_input_or_output: u32,
    pub instructions: Vec<Instruction>,
    pub track_hexes: Vec<[i32; 2]>,
    pub arm_number: u32,
    pub conduit_id: u32,
    pub conduit_hexes: Vec<[i32; 2]>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub index: i32,
    pub instruction: u8,
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}
impl<'a> Reader<'a> {
    fn error(&self, message: &'static str) -> ParseError {
        ParseError {
            offset: self.offset,
            message,
        }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let end = self
            .offset
            .checked_add(n)
            .ok_or_else(|| self.error("length overflow"))?;
        let data = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| self.error("unexpected end of file"))?;
        self.offset = end;
        Ok(data)
    }
    fn byte(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }
    fn boolean(&mut self) -> Result<bool> {
        Ok(self.byte()? != 0)
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn i32(&mut self) -> Result<i32> {
        Ok(self.u32()? as i32)
    }
    fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn small_position(&mut self) -> Result<[i8; 2]> {
        Ok([self.byte()? as i8, self.byte()? as i8])
    }
    fn position(&mut self) -> Result<[i32; 2]> {
        Ok([self.i32()?, self.i32()?])
    }
    fn string(&mut self) -> Result<&'a [u8]> {
        let mut len = 0usize;
        for shift in (0..usize::BITS).step_by(7) {
            let byte = self.byte()?;
            let value = (byte & 0x7f) as usize;
            if value > (usize::MAX >> shift) {
                return Err(self.error("string length overflow"));
            }
            len |= value << shift;
            if byte & 0x80 == 0 {
                return self.take(len);
            }
        }
        Err(self.error("string length overflow"))
    }
    // Bound allocations by the bytes remaining, before reserving memory. Each
    // collection supplies its minimum serialized element size and format limit.
    fn list<T>(
        &mut self,
        min_size: usize,
        limit: u32,
        mut read: impl FnMut(&mut Self) -> Result<T>,
    ) -> Result<Vec<T>> {
        let count = self.u32()?;
        if count > limit {
            return Err(self.error("collection exceeds format limit"));
        }
        if count as usize > (self.bytes.len() - self.offset) / min_size {
            return Err(self.error("collection exceeds remaining bytes"));
        }
        let mut values = Vec::new();
        values
            .try_reserve_exact(count as usize)
            .map_err(|_| self.error("collection allocation failed"))?;
        for _ in 0..count {
            values.push(read(self)?);
        }
        Ok(values)
    }
    fn molecule(&mut self) -> Result<Molecule> {
        Ok(Molecule {
            atoms: self.list(3, u32::MAX, |r| {
                Ok(Atom {
                    kind: r.byte()?,
                    offset: r.small_position()?,
                })
            })?,
            bonds: self.list(5, u32::MAX, |r| {
                Ok(Bond {
                    kind: r.byte()?,
                    from: r.small_position()?,
                    to: r.small_position()?,
                })
            })?,
        })
    }
    fn production(&mut self) -> Result<Production<'a>> {
        Ok(Production {
            shrink_left: self.boolean()?,
            shrink_right: self.boolean()?,
            isolate_inputs_from_outputs: self.boolean()?,
            cabinets: self.list(3, u32::MAX, |r| {
                Ok(Cabinet {
                    position: r.small_position()?,
                    kind: r.string()?,
                })
            })?,
            conduits: self.list(8, u32::MAX, |r| {
                Ok(Conduit {
                    starting_position_a: r.small_position()?,
                    starting_position_b: r.small_position()?,
                    hexes: r.list(2, u32::MAX, Self::small_position)?,
                })
            })?,
            vials: self.list(7, u32::MAX, |r| {
                Ok(Vial {
                    position: r.small_position()?,
                    style: r.byte()?,
                    count: r.u32()?,
                })
            })?,
        })
    }
    fn part(&mut self) -> Result<Part<'a>> {
        let name = self.string()?;
        if self.byte()? != 1 {
            return Err(self.error("unsupported part version"));
        }
        let position = self.position()?;
        let size = self.u32()?;
        let rotation = self.i32()?;
        let which_input_or_output = self.u32()?;
        let instructions = self.list(5, 99_999, |r| {
            Ok(Instruction {
                index: r.i32()?,
                instruction: r.byte()?,
            })
        })?;
        let track_hexes = if name == b"track" {
            self.list(8, 9_999, Self::position)?
        } else {
            Vec::new()
        };
        let arm_number = self.u32()?;
        let (conduit_id, conduit_hexes) = if name == b"pipe" {
            (self.u32()?, self.list(8, 9_999, Self::position)?)
        } else {
            (0, Vec::new())
        };
        Ok(Part {
            name,
            position,
            size,
            rotation,
            which_input_or_output,
            instructions,
            track_hexes,
            arm_number,
            conduit_id,
            conduit_hexes,
        })
    }
}

/// Parse puzzle format version 3. Unknown trailing extension bytes are allowed,
/// as in upstream; missing required fields and oversized allocations are errors.
pub fn puzzle(bytes: &[u8]) -> Result<Puzzle<'_>> {
    let mut r = Reader { bytes, offset: 0 };
    if r.u32()? != 3 {
        return Err(r.error("unsupported puzzle version"));
    }
    Ok(Puzzle {
        name: r.string()?,
        creator: r.u64()?,
        parts_available: r.u64()?,
        inputs: r.list(8, u32::MAX, Reader::molecule)?,
        outputs: r.list(8, u32::MAX, Reader::molecule)?,
        output_scale: r.u32()?,
        production: if r.boolean()? {
            Some(r.production()?)
        } else {
            None
        },
    })
}
/// Parse solution format version 7, borrowing names from the input buffer.
pub fn solution(bytes: &[u8]) -> Result<Solution<'_>> {
    let mut r = Reader { bytes, offset: 0 };
    if r.u32()? != 7 {
        return Err(r.error("unsupported solution version"));
    }
    let puzzle = r.string()?;
    let name = r.string()?;
    let metrics = if r.u32()? != 0 {
        let mut values = [0; 4];
        for (tag, value) in values.iter_mut().enumerate() {
            if r.u32()? != tag as u32 {
                return Err(r.error("invalid metric tag"));
            }
            *value = r.u32()?;
        }
        Some(Metrics {
            cycles: values[0],
            cost: values[1],
            area: values[2],
            instructions: values[3],
        })
    } else {
        None
    };
    Ok(Solution {
        puzzle,
        name,
        metrics,
        parts: r.list(30, 9_999, Reader::part)?,
    })
}
impl Solution<'_> {
    /// Recompute the parts cost, matching upstream `solution_file_cost`.
    /// Unknown parts cost zero, following the original function's behavior.
    pub fn cost(&self) -> u64 {
        self.parts
            .iter()
            .map(|part| match part.name {
                b"glyph-calcification" | b"bonder" | b"unbonder" => 10,
                b"glyph-life-and-death"
                | b"glyph-projection"
                | b"glyph-dispersion"
                | b"glyph-purification"
                | b"glyph-duplication"
                | b"glyph-unification"
                | b"bonder-prisma"
                | b"arm1"
                | b"glyph-rejection"
                | b"glyph-division" => 20,
                b"bonder-speed" | b"arm2" | b"arm3" | b"arm6" | b"baron" | b"ravari" => 30,
                b"piston" | b"glyph-proliferation" => 40,
                b"track" => part.track_hexes.len() as u64 * 5,
                _ => 0,
            })
            .sum()
    }
}
