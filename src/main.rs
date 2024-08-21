type PiecePosition = u64;
struct Piece {
    position: usize
}

// fn bitstream_to_position(bit: PiecePosition)->Result<String, String> {
//     if bit == 0 {
// return Err("no piece".to_string());
//     } else {
//         let onebit_index= bit_scan(bit);
//         return Ok(index_to_position(onebit_index));
//         }
// }

fn bit_scan () {todo!()}

static COLMAP: [char; 8]=['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

fn index_to_position (index: usize)-> String {
    let column=index % 8;
    let row = index/8+1;
return format!("{}{}", COLMAP[column], row);

}
fn main() {
    let position = 1 << 4;//1<<0000

    println!("{}", (1 as u64)<< 55);
}
