use crate::builder::*;

#[allow(dead_code)]
const HTML_HEADER: &str = "<!DOCTYPE html>
<html>
<head>
<style>
table {
  font-family: arial, sans-serif;
  border-collapse: collapse;
  width: 100%;
}

td, th {
  border: 1px solid #dddddd;
  text-align: left;
  padding: 8px;
}

tr:nth-child(even) {
  background-color: #dddddd;
}
</style>
</head>
<body>";

const HTML_FOOTER: &str = "</body>
</html>";

pub fn generate_html(teams: &Vec<Team>) {
    let mut html = String::new();

    html.push_str("<table>");

    for team in teams {
        html.push_str("<tr>");
        for student in &team.students
        {
            html.push_str(&student.surname);
        }
        html.push_str("</tr>");
    }

    html.push_str("</table>");

    println!("{}", html);
}
