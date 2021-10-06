use clap::arg_enum;

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum OutputFormat {
        Table,
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
                None => OutputFormat::Table,
            },
        }
    }

    pub fn fail(&self, msg: &'static str) -> anyhow::Result<()> {
        // TODO: read self.format
        Err(anyhow::Error::msg(msg))
    }

    pub fn wrap_error(&self, e: anyhow::Error) -> anyhow::Result<()> {
        Err(e)
    }

    pub fn out<T>(&self, value: &T) -> anyhow::Result<()>
    where
        T: serde::Serialize,
    {
        // TODO: read self.format
        println!("{}", serde_json::to_string(value).unwrap());
        Ok(())
    }
}
