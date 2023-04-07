use std::ops::Sub;
use std::{error::Error, io::Write, vec};

use chrono::{Duration, Utc};

use crate::moco::client::MocoClient;
use crate::moco::model::{Activity, Project, ProjectTask};

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
    last_week: bool,
    month: bool,
    last_month: bool,
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

    from = if last_week {
        Some(now.beginning_of_week().sub(Duration::weeks(1)))
    } else {
        from
    };
    to = if last_week {
        Some(now.end_of_week().sub(Duration::weeks(1)))
    } else {
        to
    };

    from = if month {
        Some(now.beginning_of_month())
    } else {
        from
    };
    to = if month { Some(now.end_of_month()) } else { to };

    from = if last_month {
        Some(now.beginning_of_month().sub(Duration::weeks(4)))
    } else {
        from
    };
    to = if last_month {
        Some(now.end_of_month().sub(Duration::weeks(4)))
    } else {
        to
    };

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

pub fn optional_validator(_: &str) -> Option<String> {
    None
}

pub fn mandatory_validator(input: &str) -> Option<String> {
    if input.is_empty() {
        Some("Input is required".to_string())
    } else {
        None
    }
}

pub async fn prompt_task_select(
    moco_client: &MocoClient,
    project: Option<i64>,
    task: Option<i64>,
) -> Result<(Project, ProjectTask), Box<dyn Error>> {
    let projects = moco_client.get_assigned_projects().await?;
    let project = projects.iter().find(|p| p.id == project.unwrap_or(-1));

    let project = if let Some(p) = project {
        p
    } else {
        let project_index = render_list_select(
            &projects,
            vec!["Index", "Customer", "Project", "Project ID"],
            "Chose your Project: ",
            &(|(index, project)| {
                vec![
                    index.to_string(),
                    project.customer.name.clone(),
                    project.name.clone(),
                    project.id.to_string(),
                ]
            }),
        )?;

        &projects[project_index]
    };

    let task = project.tasks.iter().find(|t| t.id == task.unwrap_or(-1));

    let task = if let Some(t) = task {
        t
    } else {
        let task_index = render_list_select(
            &project.tasks,
            vec!["Index", "Task", "Task ID"],
            "Chose your Task: ",
            &(|(index, task)| vec![index.to_string(), task.name.clone(), task.id.to_string()]),
        )?;
        &project.tasks[task_index]
    };

    Ok((project.clone(), task.clone()))
}

pub async fn prompt_activity_select(
    moco_client: &MocoClient,
    activity: Option<i64>,
) -> Result<Activity, Box<dyn Error>> {
    let now = Utc::now().format("%Y-%m-%d").to_string();

    print!("List activities from (YYYY-MM-DD) - Default 'today': ");
    std::io::stdout().flush()?;

    let mut from = read_line()?;
    if from.is_empty() {
        from = now.clone();
    }

    print!("List activities to (YYYY-MM-DD) - Default 'last answer': ");
    std::io::stdout().flush()?;

    let mut to = read_line()?;
    if to.is_empty() {
        to = from.clone();
    }

    let activities = moco_client.get_activities(from, to, None, None).await?;
    let activity = activities.iter().find(|a| a.id == activity.unwrap_or(-1));

    let activity = if let Some(a) = activity {
        a
    } else {
        let activity_index = render_list_select(
            &activities,
            vec![
                "Index",
                "Date",
                "Duration",
                "Project",
                "Task",
                "Description",
            ],
            "Choose your Acitivity: ",
            &(|(index, activity)| {
                vec![
                    index.to_string(),
                    activity.date.clone(),
                    activity.hours.to_string(),
                    activity.project.name.clone(),
                    activity.task.name.clone(),
                    activity
                        .description
                        .as_ref()
                        .unwrap_or(&String::new())
                        .to_string(),
                ]
            }),
        )?;

        &activities[activity_index]
    };

    Ok(activity.clone())
}
