pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield.iter().enumerate().map(|(c,r)|{
            r.chars().into_iter().enumerate().fold(String::new(),|acc,i|{
                acc+ &mines_count(minefield, i.1, (c, i.0)).to_string()
            })


    }).collect()
}

fn mines_count(minefiled: &[&str], letter: char, index: (usize, usize)) -> char {
    let mut count = 0;
    if letter == '*'{
        return '*'
    }
    for i in -1..2 as isize {
        let index_i = index.0 as isize + i;
        if index_i>=0 && index_i < minefiled.len() as isize {
            for j in -1..2 as isize {
                let index_j = index.1 as isize + j;
                if let Some('*') = minefiled[index_i as usize].chars().nth(index_j as usize) {
                    count +=1;
                }
            }
        }
    }
    if count == 0{
        return ' '
    }
    count.to_string().chars().next().unwrap()
    
}
