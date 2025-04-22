use clap::Args;

#[derive(Args, Debug)]
#[group(id = "filter", multiple = false, required = false)]
pub struct Filter {
    /// Find resource by their ID
    #[arg(long)]
    pub id: Option<String>,

    /// Filter resources by regex name match
    #[arg(long)]
    pub name: Option<String>,

    /// Like --name but case insensitive
    #[arg(long)]
    pub iname: Option<String>,

    /// Filter resources using a CEL filter and filter variables available on that particular
    /// resource type
    #[arg(long)]
    pub cel: Option<String>,
}

#[derive(Args, Debug)]
#[group(id = "pagination", required = false)]
pub struct Pagination {
    /// Maximum number of records to return
    #[arg(short, long, default_value_t = 50)]
    pub max_records: u32,
}
