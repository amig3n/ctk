use std::io;
use std::io::Write;
use std::fmt;
use console;

use crate::providers::aws::{STSResponse, Ec2Response, SsmResponse};

pub struct Table {
    format: Vec<TableColumnFormat>,
    rows: Vec<Vec<String>>,
    show_header: bool,
}

#[derive(Debug)]
pub enum TableError {
    IncorrectRowLength,
    IOError(io::Error),
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TableColumnFormat{
    #[default]
    ToLeft,
    ToRight,
}

impl fmt::Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
           TableError::IncorrectRowLength => write!(f, "Incorrect row length: row length must be equal to header length"),
           TableError::IOError(err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl std::error::Error for TableError {}

impl From<std::io::Error> for TableError {
    fn from(err: std::io::Error) -> Self {
        TableError::IOError(err)
    }
}

impl From<Ec2Response> for Table {
    fn from(response: Ec2Response) -> Self {
        let mut table = Table::new(
            vec![
                "Name".to_string(),
                "Instance ID".to_string(),
                "State".to_string(),
                "Private IP".to_string(),
            ],
            vec![
                TableColumnFormat::ToRight,
                TableColumnFormat::ToLeft,
                TableColumnFormat::ToLeft,
                TableColumnFormat::ToLeft,
            ].into(),
        );

        for instance in response.instances {
            let _ = table.push(vec![
                instance.name,
                instance.instance_id,
                instance.state,
                instance.private_ip,
            ]);
        }

        table
    }
}

impl From<SsmResponse> for Table {
    fn from(response: SsmResponse) -> Self {
        let mut table = Table::new(
            vec![
                "Name".to_string(),
                "Type".to_string(),
                "Value".to_string(),
            ],
            None,
        );

        for parameter in response.parameters {
            let _ = table.push(vec![
                parameter.name,
                parameter.r#type,
                parameter.value,
            ]);
        }

        table

    }

}
impl From<STSResponse> for Table {
    fn from(response: STSResponse) -> Self {
        let mut table: Table = Table::new(
            vec!["Param".to_string(), "Value".to_string()],
            None,
        ).set_header(false);

        table.push(vec!["AWS ARN:".to_string(), response.arn]);
        table.push(vec!["User ID:".to_string(), response.user_id]);
        table.push(vec!["Account:".to_string(), response.account]);

        table
    }
}


impl Table {
    /// Create new table object
    pub fn new(headers: Vec<String>, format: Option<Vec<TableColumnFormat>>) -> Table {
        let parsed_format = match &format {
            Some(f) => f,
            None => &vec![TableColumnFormat::default(); headers.len()],
        };

        return Table {
            format: parsed_format.to_vec(),
            rows: vec![headers],
            show_header: true,
        }
    }

    pub fn set_header(mut self, show: bool) -> Self {
        self.show_header = show;
        self
    }

    /// Push new row to the table
    pub fn push(&mut self, row: Vec<String>) -> Result<(), TableError>  {
        if row.len() == self.rows[0].len() {
            self.rows.push(row);
            Ok(())
        } else {
            Err(TableError::IncorrectRowLength)
        }
    }

    fn calculate_text_length(field: &String) -> usize {
        return console::measure_text_width(field.as_str());
    }

    /// Calculate the width of each column
    fn calculate_width(&self) -> Vec<usize> {
        // init usize vector with zeros
        let mut column_width: Vec<usize> = vec![0; self.rows[0].len()];        

        for row in &self.rows {
            for (index,field) in row.iter().enumerate() {
                // if current field is longer than current column width, update it
                let length = Self::calculate_text_length(field);
                if length > column_width[index] {
                    column_width[index] = length;
                }
            }    
        }
        return column_width; 
    }
    
    /// Render the table after all data has been loaded
    pub fn render(&self, column_padding: usize) -> Result<(), TableError> {
        // calculate each column width
        let column_width: Vec<usize> = self.calculate_width();
        
        // container for ready table
        let mut ready_table: Vec<Vec<String>> = vec![];

        let start_index = if self.show_header { 0 } else { 1 };

        for row in &self.rows[start_index..] {
            let mut current_row: Vec<String> = vec![];

            for (index,field) in row.iter().enumerate() {
                // check format strategy for each column
                let mut current_field = String::new();

                match self.format[index] {
                    TableColumnFormat::ToLeft => {
                        // insert whitespaces after field
                        current_field.push_str(field.as_str());
                        current_field.push_str(
                            " ".repeat(column_width[index] - Self::calculate_text_length(field)).as_str()
                        );
                    },
                    TableColumnFormat::ToRight => {
                        // insert whitespaces before field and padding after
                        current_field.push_str(
                            " ".repeat(column_width[index] - Self::calculate_text_length(field)).as_str()
                        );
                        current_field.push_str(field.as_str());
                    }
                }
                current_row.push(current_field);
            }
            ready_table.push(current_row);
        }

        // prepare necessities
        let mut stdout = io::stdout();
        let separator = " ".repeat(column_padding);

        // table is ready to be rendered
        for table_row in ready_table {
           stdout.write_all(table_row.join(&separator).as_bytes())?;
           stdout.write_all(b"\n")?;
        }

    
        stdout.flush()?;
        Ok(())
    }


}
