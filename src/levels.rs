use crate::blocks::*;
use bevy::prelude::*;

use Block::*;

pub const LEVEL_1: [[Option<Block>; 11]; 10] = [
    [None; 11],
    [None; 11],
    [None; 11],
    [None; 11],
    [Some(Silver); 11],
    [Some(Red); 11],
    [Some(Blue); 11],
    [Some(Orange); 11],
    [Some(Pink); 11],
    [Some(Green); 11],
];
