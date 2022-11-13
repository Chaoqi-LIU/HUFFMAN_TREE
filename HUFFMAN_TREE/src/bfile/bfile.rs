use crate::huffmantree::huffmantree::HuffmanTree;

use std::fs;
use std::fs::File;
use std::io::{Write, Error};

pub fn read_from_file(file : &str) -> String {
    return fs::read_to_string(file).unwrap();
}

pub fn write_to_file(file : &str, content : String) -> Result<(), Error> {
    let mut output = File::create(file)?;
    return write!(output, "{}", content);
}

pub fn read_huffmantree(file : &str, huffman_tree : &mut HuffmanTree) -> Result<(), Error> {
    let mut idx : usize = 0;
    let mut reader = fs::read_to_string(file).unwrap();
    reader.pop();
    // println!("reader: {}", reader);
    for line in reader.split("🦀") {
        let comps : Vec<&str> = line.split("|").collect();
        idx = huffman_tree.insert(comps[2].to_string().parse::<i32>().unwrap(), comps[3].to_string());
        huffman_tree.set_left(idx, comps[0].to_string().parse::<usize>().unwrap());
        huffman_tree.set_right(idx, comps[1].to_string().parse::<usize>().unwrap());
    }
    huffman_tree.set_root(idx);
    huffman_tree.build_map(huffman_tree.get_root(), &mut vec![]);
    return Ok(());
}

pub fn huffmantree_to_stirng(huffman_tree : &HuffmanTree) -> String {
    let mut content : String = "".to_string();
    for node in &huffman_tree.get_nodes()[1..(huffman_tree.get_idx() + 1)] {
        content += &(node.left_.to_string() + &"|");
        content += &(node.right_.to_string() + &"|");
        content += &(node.freq_.get_frequancy().to_string() + &"|");
        content += &(node.freq_.get_charactor().to_string() + &"🦀");
    }
    return content;
}

/*
file_name
---------------------------------------------
left_node right_node freq char 
*/
pub fn write_huffmantree(file : &str, huffman_tree : &HuffmanTree) -> Result<(), Error> {
    if huffman_tree.get_root() == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "empty Huffman Tree",
        ));
    }
    let mut output = File::create(file)?;
    return write!(output, "{}", huffmantree_to_stirng(huffman_tree));
}
