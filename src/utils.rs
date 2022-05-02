use std::{error::Error, io::Write};

use chrono::Utc;

pub fn read_line() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    input.remove(input.len() - 1);
    Ok(input)
}

pub fn render_table(list: Vec<Vec<String>>) {
    if list.is_empty() {
        return;
    }

    let mut list_elem_max_length = vec![0; list.first().unwrap().len()];

    for row in list.iter() {
        for (column_index, column_content) in row.iter().enumerate() {
            if list_elem_max_length
                .get(column_index)
                .expect("Input list does not contain same column count")
                < &column_content.len()
            {
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

pub fn select_from_to_date(
    today: bool,
    week: bool,
    month: bool,
) -> (chrono::DateTime<Utc>, chrono::DateTime<Utc>) {
    use now::DateTimeNow;

    let now = Utc::now();
    let mut from = if today { Some(now) } else { None };
    let mut to = if today { Some(now) } else { None };
    from = if week {
        Some(now.beginning_of_week())
    } else {
        from
    };
    to = if week { Some(now.end_of_week()) } else { to };
    from = if month {
        Some(now.beginning_of_month())
    } else {
        from
    };
    to = if month { Some(now.end_of_month()) } else { to };
    let from = from.unwrap_or(now);
    let to = to.unwrap_or(now);
    (from, to)
}

pub fn ask_question(
    question: &str,
    validator: &dyn Fn(&str) -> Option<String>,
) -> Result<String, Box<dyn Error>> {
    loop {
        print!("{}", question);
        std::io::stdout().flush()?;
        let line = read_line()?;
        if let Some(error) = validator(&line) {
            println!("{}", error);
            continue;
        }
        return Ok(line);
    }
}
