mod chunk;
mod iter;

pub use chunk::Chunk;
pub use iter::Chunks;

use crate::line::Line;

pub struct ChunkLine<'a> {
    pub line: &'a Line<'a>,
    pub iterator_index: usize
}

impl<'a> ChunkLine<'a> {
    pub fn new(line: &'a Line<'a>, iterator_index: usize) -> Self {
        ChunkLine { line, iterator_index }
    }


}

impl<'a> PartialEq for ChunkLine<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl<'a> PartialOrd for ChunkLine<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Eq for ChunkLine<'a> {}

impl<'a> Ord for ChunkLine<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.line.cmp(self.line)
    }
}