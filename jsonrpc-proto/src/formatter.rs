use clap::arg_enum;

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum OutputFormat {
        Json,
        Yaml,
    }
}

pub struct Formatter {
    format: OutputFormat,
}

impl Formatter {
    pub fn new(format: Option<OutputFormat>) -> Self {
        Self {
            format: match format {
                Some(x) => x,
                None => OutputFormat::Yaml,
            },
        }
    }

    pub fn fail(&self, msg: &'static str) -> anyhow::Result<()> {
        Err(anyhow::Error::msg(msg))
    }

    pub fn wrap_error(&self, e: anyhow::Error) -> anyhow::Result<()> {
        Err(e)
    }

    pub fn out<T>(&self, value: &T) -> anyhow::Result<()>
    where
        T: serde::Serialize,
    {
        match &self.format {
            OutputFormat::Json => println!("{}", serde_json::to_string(value).unwrap()),
            OutputFormat::Yaml => println!("{}", serde_yaml::to_string(value).unwrap()),
        }
        Ok(())
    }
}
