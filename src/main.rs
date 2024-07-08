use miette::Result;
use orogene::Orogene;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[async_std::main]
async fn main() -> Result<()> {
    Ok(Orogene::load().await?)
}
