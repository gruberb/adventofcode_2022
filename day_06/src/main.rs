fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
            
    let mut marker = "".to_string();
    let mut counter = 0;

    // Part 1
    // for c in contents.chars() {
    //     if marker.len() < 4 {
    //         counter += 1;
    //         if !marker.contains(c) {
    //             marker.push(c);
    //         } else {
    //             let index = marker.find(c).unwrap();
    //             marker.drain(..index+1);
    //             marker.push(c);
    //         }
    //     } else {
    //         println!("{}, {}", counter, marker);
    //         break;
    //     }
    // }

    // Part 2
    for c in contents.chars() {
        if marker.len() < 14 {
            counter += 1;
            if !marker.contains(c) {
                marker.push(c);
            } else {
                let index = marker.find(c).unwrap();
                marker.drain(..index+1);
                marker.push(c);
            }
        } else {
            println!("{}, {}", counter, marker);
            break;
        }
    }
}
