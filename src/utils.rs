use std::{error::Error, io::Write};

pub fn read_line() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    input.remove(input.len() - 1);
    Ok(input)
}

pub fn render_table(list: Vec<Vec<String>>) {
    let mut list_elem_max_length = vec![0; list.len()];

    for row in list.iter() {
        for (column_index, column_content) in row.iter().enumerate() {
            if list_elem_max_length[column_index] < column_content.len() {
                list_elem_max_length[column_index] = column_content.len();
            }
        }
    }

    for row in list.iter() {
        for (column_index, column_content) in row.iter().enumerate() {
            print!(
                "{}{}\t",
                column_content,
                " ".repeat(list_elem_max_length[column_index] - column_content.len())
            )
        }
        println!();
    }
}

pub fn render_list_select<T>(
    list: &[T],
    headline: Vec<&str>,
    promt: &str,
    linenderer: &dyn Fn((usize, &T)) -> Vec<String>,
) -> Result<usize, Box<dyn Error>> {
    loop {
        let mut rendered_list: Vec<Vec<String>> =
            list.iter().enumerate().map(|ele| linenderer(ele)).collect();
        rendered_list.insert(0, headline.iter().map(|x| x.to_string()).collect());
        render_table(rendered_list);

        print!("{}", promt);
        std::io::stdout().flush()?;

        let index_input = read_line().map(|x| x.parse::<usize>().ok()).ok().flatten();

        if let Some(index) = index_input {
            if index < list.len() {
                return Ok(index);
            }
        }
        println!("Index Invallid")
    }
}
