use crate::builder::*;

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

const HTML_FOOTER: &str = "</body>
</html>";

// TODO: Export as an external crate
// TODO: Add missing cells if empty

pub fn generate_html(teams: &[Team]) -> Option<String> {
  let mut html = String::new();

  html.push_str(HTML_HEADER);

  html.push_str("<table>");

  for team in teams {
    html.push_str("<tr>");

    // TODO: Remove hardcoded team name
    html.push_str(&format!("<th>{}</th>", "Team name"));

    for person in &team.people {
      html.push_str(&format!("<td>{}</td>", &person.surname));
    }

    html.push_str("</tr>");
  }

  html.push_str("</table>");

  html.push_str(HTML_FOOTER);

  Some(html)
}
