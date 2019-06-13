use crate::builder::*;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

#[allow(dead_code)]
const HTML_HEADER: &str = "<!DOCTYPE html>
<html>
<head>
<title>Team Builder</title>
<style>
table {
  font-family: arial, sans-serif;
  border-collapse: collapse;
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

#[allow(dead_code)]
const HTML_FOOTER: &str = "</body>
</html>";

// TODO: Export as an external crate
// TODO: Add missing cells if empty

pub fn generate_html(teams: &Vec<Team>) {
  let mut html = String::new();

  html.push_str(HTML_HEADER);

  html.push_str("<table>");

  for team in teams {
    html.push_str("<tr>");

    html.push_str(&format!("<th>{}</th>", "Team name"));

    for student in &team.students {
      html.push_str(&format!("<td>{}</td>", &student.surname));
    }

    html.push_str("</tr>");
  }

  html.push_str("</table>");

  html.push_str(HTML_FOOTER);

  let mut file = File::create("table.html").unwrap();
  file.write_all(html.as_bytes()).unwrap();

  Command::new("open").arg("table.html").output().unwrap();
}
