use std::io;
extern crate base85;
use base85::{decode,encode};


fn main(){
    let mut my_string = String::new();
    io::stdin()
        .read_line(&mut my_string)
        .expect("Failed to read input");

    //println!("input string: {}",input_string);
    //let x = dpql::write(input_string.clone());
    //println!("written program: {}",x);
    
    //let my_string = "Maman died today. Or yesterday maybe, I don't know. I got a telegram from the home: Mother deceased . Funeral tomorrow. Faithfully yours. That doesn't mean anything. Maybe it was yesterday.".to_string();
    println!("input string: {}", my_string);


    //println!("{}",my_string);

    //\\let y = dpql::read(x);

    //println!("interpreted diroql code: {}",y);

    //let inst = dpql::zip::DpqlzMeta(28,3,166,[1, 3, 2, 4, 5, 7, 8, 9, 6, 0].to_vec());

    //let my_vector: Vec<u8> = [1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1].to_vec();
    //my vector is the string of 1s and 0s
    
    //let res = dpql::zip::write_meta(&inst,&my_vector);
    //println!("inputs to write meta: 28,3,166,[1, 3, 2, 4, 5, 7, 8, 9, 6, 0]");
    //println!("result of write meta: {}",res);
    //let res2 = "DIROPQLZ00000000QU0{{R300000I|2g$0R#j900000000000Q<CThBb(-a<@&imLl4c#bDKKY*w`uv2CrOjAWy2TU++f".to_string();

    
    

    //println!("is input vector to write meta equal to output of readmeta? {}", my_vector==y);

    let z = dpql::zip::write(&my_string);
    println!("ZIP WRITE OUTPUT: {}",z);

    let zz = dpql::zip::read(&z);
    //println!("results of read meta: mlen: {}, moffset: {}, bwtidx: {}, huffmancodebook: {:?}", x.0, x.1, x.2, x.3);
    println!("output of dpql::zip::read is : {}", zz);
}



mod dpql{
    use std::collections::VecDeque;
    pub fn write(text: String) -> String{
        let mut program = String::new();
    
        for character in text.chars(){
            let character_ascii = character as u8;
            if character_ascii <128 {
                for _ctr_1 in 0..(character_ascii){
                    program.push_str("i");
                }}
            else if character_ascii >=128{
                for _ctr_2 in 0..(255-character_ascii){
                    program.push_str("d");
                }}
            program.push_str("o"); 
            program.push_str("r");
        }
        program
    }

    pub fn read(prog: String) -> String {  
        let mut mp = 0;
        let mut ip:usize = 0;
        let mut arr:[u8; 10000] = [0; 10000];
        
        let mut output_queue = VecDeque::new();
    
        let mut p_stack: Vec<usize> = Vec::new();
        let mut q_stack: Vec<usize> = Vec::new();
    
        let prog_chars: Box<[char]> = prog.chars().collect::<Vec<char>>().into_boxed_slice();
        let prog_len = prog_chars.len();
    
        while ip < prog_len{
    
            let instruction = prog_chars[ip];
            
            if instruction == 'r'{
                mp +=1;
                if mp == 10000{
                    mp = 0;}    // wrap around
            }
    
            else if instruction == 'l'{
                if mp == 0{
                    mp = 9999;} // wrap around
                else{
                    mp -=1;}
                }    
    
            else if instruction == 'd' {
                if arr[mp]  == 0{
                    arr[mp] = 255; }  //wrap around
                else{
                    arr[mp]-=1;}
                }
    
            else if instruction == 'i' {
                if arr[mp]  == 255{
                    arr[mp] = 0; }  //wrap around
                else{
                    arr[mp]+=1;}
                }
    
            else if instruction == 'o' {
                let decimal_value = arr[mp] as u32;
                let ascii_char = std::char::from_u32(decimal_value).unwrap();
                output_queue.push_back(ascii_char);
            }
    
            else if instruction == 'p'{
                if arr[mp] == 0 {
                    let mut unmatched = true; // find matching q
                    while unmatched {
                        // iterate over next instructions until matching q is found
                        if prog_chars[ip] == 'p'{
                            p_stack.push(ip);}
                        else if prog_chars[ip] == 'q'{
                            q_stack.push(ip);}
                        // we only care about the p's and q's 
                        if p_stack.len() == q_stack.len() && p_stack.len()!= 0 {
                            let next_ip: usize = q_stack.pop().unwrap(); //index of the initial p's matching q
                            ip = next_ip;
                            p_stack.clear();
                            q_stack.clear();
                            unmatched = false;}
                        else{
                            ip +=1;}
                         }
                }}    
            else if instruction == 'q'{
                if arr[mp] != 0 {
                    let mut unmatched = true; // find matching p
                    while unmatched {
                        // iterate over next instructions until matching p is found
                        if prog_chars[ip] == 'p'{
                            p_stack.push(ip);}
                        else if prog_chars[ip] == 'q'{
                            q_stack.push(ip);}
                        // we only care about the p's and q's 
                        if p_stack.len() == q_stack.len() && q_stack.len()!= 0 {
                            let next_ip: usize = p_stack.pop().unwrap(); //index of the initial q's matching p
                            ip = next_ip;
                            p_stack.clear();
                            q_stack.clear();
                            unmatched = false;}
                        else{
                            ip +=1;}
                         }
                }}          
    
            ip +=1;  // move to next instruction in diropql program
        }
    
        let mut output = String::new();      
        for symb in &output_queue {        //put contents of output queue into a string
            output += &symb.to_string();   
        }
    
        output //return string
    }

   
    pub mod zip{
        pub fn write(text: &String) -> String{
            let diropql_program = super::write(text.to_string()); //convert the input text to a diropql program
            println!("diropql_program: {}",diropql_program);

            let (bwt_encoded,bwt_idx) = crate::compressor::bwt::encode(&diropql_program); // output of bwt encode
            println!("bwt_encoded: {:?} bwt_idx: {:?}",bwt_encoded,bwt_idx);
            let mtf_dict = crate::compressor::mtf::alphabet_sort(&bwt_encoded); // alphabet
            println!("mtf_dict: {:?}",mtf_dict);  
            let mtf_encoded = crate::compressor::mtf::encode(&bwt_encoded, &mtf_dict); // output of mtf encode
            println!("mtf_encoded: {:?}",mtf_encoded);  
            let rle_encoded = crate::compressor::rle::encode(&mtf_encoded); // output of rle encode
            println!("rle_encoded: {:?}",rle_encoded); 
            let (huffman_encoded,huffman_codebook) = crate::compressor::huffman::encode(&rle_encoded); //output of huffman encode
            println!("huffman_encoded: {:?}",huffman_encoded); 

            //extracting of metadata 

            let mlen_raw:u64 = huffman_encoded.len() as u64;
            let moffset:u8 = 8 - (mlen_raw % 8) as u8;
            let mlen_fin:u64 = mlen_raw + (moffset as u64);
            let bwt_idx_u64 = bwt_idx as u64;
            
            //putting meta data in struct
            let metadata = DpqlzMeta(mlen_fin, moffset, bwt_idx_u64, huffman_codebook);
            //write the metadata
            let diropqlz_fin = write_meta(&metadata, &huffman_encoded);
            println!("{:?}",diropqlz_fin);
            diropqlz_fin  // from input text, convert to diropqlz file!
        }
        pub fn read(prog: &String) -> String{
            let (metadata, compressed_prog) = read_meta(&prog);

            let huffman_decoded = crate::compressor::huffman::decode(&compressed_prog, &metadata.3); //output of huffman decode
            println!("huffman_decoded: {:?}",huffman_decoded);
            
            let rle_decoded = crate::compressor::rle::decode(&huffman_decoded); //output of rle decode
            println!("rle_decoded: {:?}",rle_decoded);
       
            let mtf_alphabet = "dior\0".to_string(); //only makes use of dior
            let mtf_decoded = crate::compressor::mtf::decode(&rle_decoded, &mtf_alphabet); //output of mtf decode
            println!("mtf_decoded: {:?}",mtf_decoded);
          
            let bwt_idx_usize = metadata.2 as usize;
            //println!("test");
            let bwt_decoded = crate::compressor::bwt::decode(&mtf_decoded, &bwt_idx_usize); // output of bwt decode, medyo matagal to
            // the bwt decoded string above is the diropql program! time to read
            println!("bwt_decoded: {:?}",bwt_decoded);
            let final_decoded = super::read(bwt_decoded.to_string()); // this is the final deobfuscated message 

            final_decoded
        }
        pub fn write_meta(meta: &DpqlzMeta, prog: &Vec <u8>) -> String{
            let mut diropqlz = String::new();
            let mlen = meta.0; // this is in bytes
            let moffset = meta.1; //in bits
            let bwt_idx = meta.2; //not in bytes
            let huffman_tree = &meta.3;

            let mlen_as_64bits = format!("{:064b}", mlen*8);  //mlen is in bytes, convert it to # of bits - format into 64 bit binary number
            let mlen_64bit_string = format!("{}", mlen_as_64bits);
            diropqlz.push_str(&mlen_64bit_string);    //appending of mlen to diropqlz file
       
            let moffset_as_8bits = format!("{:08b}", moffset);  //moffset is in bits, convert into 8 bit binary number
            let moffset_8bit_string = format!("{}", moffset_as_8bits);
            diropqlz.push_str(&moffset_8bit_string); //appending of moffset to diropqlz file

            let bwt_idx_as_64bits = format!("{:064b}", bwt_idx);  //bwt idx expressed as 64 bit binary number
            let bwt_idx_64bit_string = format!("{}", bwt_idx_as_64bits);
            diropqlz.push_str(&bwt_idx_64bit_string); //appending of bwt_idx to the file

            let mut ctr = 0;
            for item in huffman_tree.iter(){
                let item_as_8bits =  format!("{:08b}", item);
                let item_as_8bit_string = format!("{}", item_as_8bits);
                diropqlz.push_str(&item_as_8bit_string);            
                ctr+=1;}  //appending of elements of huffman tree
            while ctr < 16{
                let item = 0;
                let item_as_8bits =  format!("{:08b}", item);
                let item_as_8bit_string = format!("{}", item_as_8bits);       
                diropqlz.push_str(&item_as_8bit_string);   // fill the rest w zeroes
                ctr+=1;
            }
            //meta data is appended ! total length is 264 bits ! 

            for element in prog.iter(){
                let string_element = element.to_string(); 
                diropqlz.push_str(&string_element); //push obfuscated message into diropqlz
            }

            let mut ctr_zeroes = 0;
            while ctr_zeroes < moffset{
                diropqlz.push('0');
                ctr_zeroes+=1;} // append any zeroes if needed since obfuscated message wont always be divisible by 8

        // b85 encoding needs a vector input of bytes

        let mut diropqlz_bytes: Vec<u8> = Vec::new();
        let mut working_byte = String::new();
        for character in diropqlz.chars(){
            let char_string = character.to_string();
            working_byte.push_str(&char_string);
            if working_byte.len() == 8 {
                let decimal = u8::from_str_radix(&working_byte, 2).unwrap();
                diropqlz_bytes.push(decimal);
                working_byte.clear();}
        }

        let base85_diropqlz = base85::encode(&diropqlz_bytes); //encoded!
        
        let diropqlz_final = "DIROPQLZ".to_owned() + &base85_diropqlz; //prepend the magic string

        diropqlz_final //diropqlz file
        }

        
        pub fn read_meta(prog: &String) -> (DpqlzMeta, Vec <u8>){
            //assuming that prog is a diropqlz file, the first 7 chars in the string should be DIROPQLZ
            let  b85_encoded_prog = &prog[8..]; //remove magic string to get base 85 encoded string
         
            let decoded_prog = base85::decode(&b85_encoded_prog); // decoded program, the output will be a vector of bytes
            let decoded_vec = decoded_prog.unwrap(); 

            let mut b85_decoded = String::new();
            
            for item in decoded_vec.iter(){
                let item_as_8bits =  format!("{:08b}", item);
                let item_as_8bit_string = format!("{}", item_as_8bits);
                b85_decoded.push_str(&item_as_8bit_string);
            } // the vector of bytes should be converted into its 8 bit representations
            
            // slice the stuff
            //[inc..] [..not]
            let mlen_string = &b85_decoded[..64];
            let moffset_string = &b85_decoded[64..72]; 
            let bwt_idx_string = &b85_decoded[72..136]; 
            let huffman_tree_string = &b85_decoded[136..264];
            let obfuscated_string = &b85_decoded[264..];
     
            //convert binary strings to correct data types
            let mlen = u64::from_str_radix(mlen_string, 2).unwrap();
            let mlen_final = mlen/8;
            let moffset = u8::from_str_radix(moffset_string, 2).unwrap();
            let bwt_idx = u64::from_str_radix(bwt_idx_string, 2).unwrap();
            let mut huffman_tree: Vec<u8> = Vec::new();
            let mut working_string = String::new();
            for bin_digit in huffman_tree_string.chars(){
                if working_string.len() != 8 {
                    let bin_digit_str = bin_digit.to_string();   //form bytes from the characters
                    working_string.push_str(&bin_digit_str);}
                if working_string.len() == 8{
                    let huffman_entry = u8::from_str_radix(&working_string, 2).unwrap();
                    huffman_tree.push(huffman_entry);  // push the bytes
                    working_string.clear();}
            }
         
            let huffman_tree_final = &huffman_tree[..10]; //only 10 elements accdg to don
    
            let mut output_vec: Vec<u8> = Vec::new();

            for bin_digit in obfuscated_string.chars(){
               output_vec.push(bin_digit.to_digit(10).unwrap() as u8);}  // put each element in the obfuscated string into a vector
            let final_index:usize = (mlen_final - (moffset as u64)) as usize; //remove offset zeroes

            let output_vec_final = &output_vec[..final_index];  //final output vector!
            
       
            return (DpqlzMeta(mlen_final, moffset, bwt_idx, huffman_tree_final.to_vec()), output_vec_final.to_vec());
        } 
        pub struct DpqlzMeta (pub u64, pub u8, pub u64, pub Vec <u8>);

    }

    
}




mod compressor {
    pub mod bwt {
        /*
        pub fn encode(text: &String) -> (String, usize) {
            let mut text = String::from(text);
            text.push('$'); // add sentinel character to the end of the string
            let rotations: Vec<String> = (0..text.len())
                .map(|i| text.chars().skip(i).chain(text.chars().take(i)).collect())
                .collect();
            //println!("{:?}",rotations );
            let mut sorted_rotations = rotations.clone();
            sorted_rotations.sort_by(|a, b| {
                match (a.chars().next(), b.chars().next()) {
                    (Some('$'), Some('$')) => a.cmp(b),
                    (Some('$'), _) => std::cmp::Ordering::Less,
                    (_, Some('$')) => std::cmp::Ordering::Greater,
                    _ => a.cmp(b),
                }
            });
            let index = sorted_rotations.iter().position(|s| s.ends_with('$')).unwrap() as u64;
            let encoded = sorted_rotations.iter().map(|s| s.chars().last().unwrap()).collect();
            (encoded, index as usize)
        }*/
         pub fn encode(text: &String) -> (String, usize) {
            let mut text = String::from(text);

            // Check if the last character is not '\0', then append '\0'
            if !text.ends_with('\0') {
                text.push('\0');
            }

            let rotations: Vec<String> = (0..text.len())
                .map(|i| text.chars().skip(i).chain(text.chars().take(i)).collect())
                .collect();

            let mut sorted_rotations = rotations.clone();
            sorted_rotations.sort_by(|a, b| {
                match (a.chars().next(), b.chars().next()) {
                    (Some('\0'), Some('\0')) => a.cmp(b),
                    (Some('\0'), _) => std::cmp::Ordering::Less,
                    (_, Some('\0')) => std::cmp::Ordering::Greater,
                    _ => a.cmp(b),
                }
            });

            let index = sorted_rotations.iter().position(|s| s.ends_with('\0')).unwrap() as usize;
            let encoded = sorted_rotations.iter().map(|s| s.chars().last().unwrap()).collect();
            println!("{:?}",encoded );

            (encoded, index)
        }
        /*
        pub fn decode(encoded: &str, index: &usize) -> String {
            let mut table = vec![String::new(); encoded.len()];
            for _ in 0..encoded.len() {
                for (i, row) in table.iter_mut().enumerate() {
                    let ch = encoded.chars().nth(i).unwrap();
                    row.insert(0, ch);
                }
                table.sort_by(|a, b| a.chars().map(|ch| if ch == '$' { '\0' } else { ch })
                                    .cmp(b.chars().map(|ch| if ch == '$' { '\0' } else { ch })));
            }
            let original_string = &table[*index];
            original_string.clone()
        }*/
        pub fn decode(encoded: &str, index: &usize) -> String {
            // Step 1: Create array of 2-ary tuples
            let mut tuples: Vec<(char, usize)> = encoded.chars().enumerate().map(|(i, c)| (c, i)).collect();

            // Step 2: Sort T using radix sort
            tuples.sort_by_key(|k| (k.0, k.1));

            // Step 3: Create a new array L from the second values of each element in the sorted tuple
            let l: Vec<usize> = tuples.iter().map(|&(_, idx)| idx).collect();

            // Step 4: Initialize a value Lidx
            let mut lidx = *index;
            let mut m = String::new();

            // Step 5: For a loop iterating V times
            for _ in 0..encoded.len() {
                // Get 𝐿[𝐿𝑖𝑑𝑥]
                let left_shift = l[lidx];

                // Push the character corresponding to the index 𝑉[𝐿[𝐿𝑖𝑑𝑥]]
                m.push(encoded.chars().nth(left_shift).unwrap());

                // Set the new Lidx to be equal to 𝐿[𝐿𝑖𝑑𝑥]
                lidx = left_shift;
            }

            // Step 6: M will now contain the original string
            m
        }
    }
    pub mod mtf {
        /*
        pub fn encode(text: &str) -> (Vec<u8>, String) {
            let mut alphabet: Vec<char> = text.chars().collect();
            alphabet.sort_by(|a, b| a.cmp(b));
            alphabet.sort_by(|a, b| b.is_ascii_punctuation().cmp(&a.is_ascii_punctuation()));
            let sentinel_pos = alphabet.iter().position(|&x| x == '$').unwrap();
            alphabet.remove(sentinel_pos);  // places chosen sentinel to starting position
            alphabet.insert(0, '$');
            alphabet.dedup();
            let dictionary = alphabet.clone().into_iter().collect::<String>();
            let mut output = Vec::new();
                for c in text.chars() {
                    let index = alphabet.iter().position(|&x| x == c).unwrap() as u8;
                    output.push(index);
                    alphabet.remove(index as usize);
                    alphabet.insert(0, c);
                }
            (output, dictionary)
        }*/
        pub fn alphabet_sort(text: &str) -> String {
            let mut alphabet: Vec<char> = text.chars().collect();
            alphabet.sort_by(|a, b| a.cmp(b));
            alphabet.sort_by(|a, b| b.is_ascii_punctuation().cmp(&a.is_ascii_punctuation()));
            let sentinel_pos = alphabet.iter().position(|&x| x == '\0').unwrap();
            alphabet.remove(sentinel_pos);  // places chosen sentinel to starting position
            alphabet.insert(0, '\0');
            alphabet.dedup();
            let dictionary = alphabet.clone().into_iter().collect::<String>();
            dictionary
        }


        
        pub fn encode(text: &str, alpha:&str) -> Vec<u8> {
                let mut alphabet: Vec<char> = alpha.chars().collect();            
                let mut output = Vec::new();
                for c in text.chars() {
                    let index = alphabet.iter().position(|&x| x == c).unwrap() as u8;
                    output.push(index);
                    alphabet.remove(index as usize);
                    alphabet.insert(0, c);
                }
            output
        }
        pub fn decode(encoded: &[u8], alphabet: &str) -> String {
            let mut mtf_alphabet: Vec<char> = alphabet.chars().collect();
            let mut message = String::new();
            for &index in encoded {
                let c = mtf_alphabet.remove(index as usize);
                message.push(c);
                if let Some(pos) = mtf_alphabet.iter().position(|&x| x == c) {
                    mtf_alphabet.remove(pos);
                }
                mtf_alphabet.insert(0, c);
            }
            message
        }

    }
    pub mod rle {
        fn binary_without_msb(mut n: u64) -> Vec<u8> {
            let mut result = Vec::new();
            if n == 0 {
                return result;
            }
            n += 1;
            while n > 1 {
                result.push((n % 2) as u8);
                n /= 2;
            }
            result
        }
        pub fn encode(text: &[u8]) -> Vec<u8> {
            let mut n_zero = 0u64;  // Change to u64 here            
            let mut output = Vec::new();
            for (i, &value) in text.iter().enumerate() {
                if i <= text.len() - 1 && value == 0 {
                    n_zero += 1;
                } else {
                    let binary_n_zero = binary_without_msb(n_zero);
                    for bit in binary_n_zero.into_iter().rev() {
                        output.push(bit);
                    }
                    if let Some(val) = text.get(i) {
                        output.push(val + 2);
                    }
                    n_zero = 0;
                }
            }
            let binary_n_zero = binary_without_msb(n_zero);
            for bit in binary_n_zero.into_iter().rev() {
                output.push(bit);
            }
            output
        }
        fn binary_to_decimal(binary: &[u8]) -> u64 {
            binary.iter().enumerate().fold(0_u64, |acc, (index, &value)| {
                acc + ((value as u64) << (binary.len() - 1 - index) as u64)
            }) as u64
        }

        pub fn decode(data: &[u8]) -> Vec<u8> {
            let mut n_zero = Vec::new();
            let mut output = Vec::new();
            let l = data.to_vec();
            let l_len = l.len();

            let mut i = 0;
            while i < l_len +1 {
                if i <= l_len - 1 && (l[i] == 0 || l[i] == 1) {
                    n_zero.push(l[i]);
                    i += 1;
                } else {
                    n_zero.reverse();
                    n_zero.push(1);
                    n_zero.reverse();
                    let n_zero_decimal = binary_to_decimal(&n_zero) - 1;
                    let zeros_to_push = if n_zero_decimal > 0 { n_zero_decimal} else { 0 };
                    for _ in 0..zeros_to_push {
                        output.push(0);
                    }
                    n_zero.clear();

                    if i < l_len {
                        output.push(l[i] - 2);
                    }
                    i += 1;
                }
            }

            if !n_zero.is_empty() {
                n_zero.push(1);
                n_zero.reverse();
                let n_zero_decimal = binary_to_decimal(&n_zero) - 1;
                for _ in 0..n_zero_decimal {
                    output.push(0);
                }
            }

            output
        }

    }
pub mod huffman {
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Eq, PartialEq)]
struct HuffmanNode {
    freq: isize,
    value: Option<u8>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(freq: isize, value: Option<u8>) -> Self {
        HuffmanNode {
            freq,
            value,
            left: None,
            right: None,
        }
    }

    fn with_children(freq: isize, left: Box<HuffmanNode>, right: Box<HuffmanNode>) -> Self {
        HuffmanNode {
            freq,
            value: None,
            left: Some(left),
            right: Some(right),
        }
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq).then_with(|| other.value.cmp(&self.value))
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_frequencies(input: &Vec<u8>) -> Vec<HuffmanNode> {
    let mut frequencies = vec![0; 10];
    for &item in input {
        frequencies[item as usize] += 1;
    }
    frequencies.into_iter().enumerate().filter_map(|(i, freq)| {
        if freq > 0 {
            Some(HuffmanNode::new(freq, Some(i as u8)))
        } else {
            None
        }
    }).collect()
}

fn build_huffman_tree(nodes: Vec<HuffmanNode>) -> HuffmanNode {
    let mut heap: BinaryHeap<_> = nodes.into_iter().collect();

    while heap.len() > 1 {
        let right = heap.pop().unwrap();
        let left = heap.pop().unwrap();
        let new_node = HuffmanNode::with_children(
            left.freq + right.freq,
            Box::new(left),
            Box::new(right),
        );
        heap.push(new_node);
    }
    
    heap.pop().unwrap()
}

fn traverse_tree(node: &HuffmanNode, code: Vec<u8>, huffman_codes: &mut Vec<Option<Vec<u8>>>) {
    if let Some(value) = node.value {
        if !code.is_empty() {
            huffman_codes[value as usize] = Some(code);
        }
    } else {
        if let Some(ref left) = node.left {
            let mut new_code = code.clone();
            new_code.push(1);
            traverse_tree(left, new_code, huffman_codes);
        }
        if let Some(ref right) = node.right {
            let mut new_code = code.clone();
            new_code.push(0);
            traverse_tree(right, new_code, huffman_codes);
        }
    }
}

fn huffman_codebook(input: &Vec<u8>) -> (Vec<u8>, Vec<Vec<u8>>) {
    let nodes = calculate_frequencies(input);
    let huffman_tree = build_huffman_tree(nodes);
    let mut huffman_codes = vec![None; 10];
    traverse_tree(&huffman_tree, Vec::new(), &mut huffman_codes);

    let mut values = Vec::new();
    let mut codes = Vec::new();
    for (value, code_option) in huffman_codes.into_iter().enumerate() {
        if let Some(code) = code_option {
            values.push(value as u8);
            codes.push(code);
        }
    }

    (values, codes)
}

fn huffman_encode(input: &[u8], values: &[u8], codes: &[Vec<u8>]) -> Vec<u8> {
    let code_map: std::collections::HashMap<_, _> = values.iter().zip(codes.iter()).collect();
    input.iter().map(|&value| code_map[&value].clone())
        .flatten()
        .collect()
}



// Function to get the canonical Huffman code.

fn get_canonical_huffman(values: &Vec<u8>, huffman_codes: &Vec<Vec<u8>>) -> (Vec<u8>, Vec<Vec<u8>>) {
    // Create a vector of tuples (value, code length, code).
    let mut codes: Vec<(u8, usize, Vec<u8>)> = values.iter()
                                                       .zip(huffman_codes.iter())
                                                       .map(|(&value, code)| (value, code.len(), code.clone()))
                                                       .collect();
    // Sort the vector by code length and then by value.
    codes.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    //println!("{:?}",codes );

    // Initialize the canon code lengths and canon code book vectors with zeros.
    let mut canon_code_lengths = vec![0; 10];
    let mut canon_code_book = vec![vec![]; 10];
    
    // Iterate over the sorted codes vector.
    let mut cur_code = 0;
    let mut cur_length = 0;
    for i in 0..codes.len() {
        let &(value, code_len, _) = &codes[i];

        // Check if the code length has increased.
        if code_len > cur_length {
            cur_code <<= code_len - cur_length;
            cur_length = code_len;
        }

        // Generate the binary representation of the current code.
        let mut code = Vec::new();
        for j in 0..code_len {
            code.push((cur_code >> (code_len - j - 1)) & 1);
        }

        // Assign the current code to the value and store the code in the code book.
        canon_code_lengths[value as usize] = code_len as u8;
        canon_code_book[value as usize] = code;

        // Increment the current code.
        cur_code += 1 << (cur_length - code_len);
    }
    canon_code_book.retain(|x| !x.is_empty());
    (canon_code_lengths, canon_code_book)
}





pub fn encode(text: &Vec <u8>) -> (Vec <u8>, Vec <u8>){
    let (values, codes) = huffman_codebook(&text);
    let (canon_code_lengths, canon_code_book) = get_canonical_huffman(&values, &codes);
    let huffman_message = huffman_encode(&text,&values,&canon_code_book);
    (huffman_message, canon_code_lengths)
}



fn calculate_codebook(canonical_lengths: &Vec<u8>) -> (Vec<u8>, Vec<Vec<u8>>) {
    let alphabet: Vec<u8> = (0..10).collect();

    let mut mapping: Vec<(u8, u8)> = alphabet
        .iter()
        .zip(canonical_lengths.iter())
        .filter(|&(_, &length)| length > 0)
        .map(|(&symbol, &length)| (symbol, length))
        .collect();

    mapping.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    let mut codebook: Vec<Vec<u8>> = vec![vec![]; mapping.len()];

    let mut c_canon = 0u8;
    let mut l_canon = mapping[0].1;

    for (i, (_, length)) in mapping.iter().enumerate() {
        if *length > l_canon {
            c_canon <<= length - l_canon;
            l_canon = *length;
        }

        let mut code = Vec::new();
        for j in 0..*length {
            code.push((c_canon >> j) & 1);
        }
        code.reverse();
        codebook[i] = code;
        
        c_canon += 1;
    }

    let alphabet: Vec<u8> = mapping.iter().map(|&(symbol, _)| symbol).collect();

    (alphabet, codebook)
}



fn retranslate(codebook: &Vec<Vec<u8>>, alphabet: &Vec<u8>, data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut temp: Vec<u8> = Vec::new();

    for &bit in data {
        temp.push(bit);
        if let Some(index) = codebook.iter().position(|code| *code == temp) {
            result.push(alphabet[index]);
            temp.clear();
        }
    }

    result
}

pub fn decode (data: &Vec <u8>, canon_freqs: &Vec <u8>) -> Vec <u8>{
    let(alphabet, codebook) = calculate_codebook(&canon_freqs);
    let result = retranslate(&codebook,&alphabet,&data);
    result
}

fn binary_to_decimal(binary: &[u8]) -> u8 {
    binary.iter().enumerate().fold(0, |acc, (index, &value)| {
        acc + (value << (binary.len() - 1 - index))
    })
}

fn canonical_codebook_lengths_decimal(canon_freqs: &Vec<Vec<u8>>) -> Vec<u8> {
    canon_freqs.iter().map(|binary| binary_to_decimal(binary)).collect()
}

fn decimal_to_binary(mut n: u8) -> Vec<u8> {
    let mut result = Vec::new();

    while n > 0 {
        result.push(n % 2);
        n /= 2;
    }

    result.reverse();
    result
}

fn canonical_codebook_lengths_binary(canon_freqs: &Vec<u8>) -> Vec<Vec<u8>> {
    canon_freqs.iter().map(|&n| decimal_to_binary(n)).collect()
}


}
}