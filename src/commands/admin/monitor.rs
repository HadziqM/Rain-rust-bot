use crate::{MyErr,Components,SlashBundle, MONITOR};

#[hertz::hertz_auto]
async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut mon = MONITOR.lock().await;
    let res;
    if *mon {
        res = "monitor is disabled"
    }else{
        res = "monitor is enabled"
    }
    *mon = !*mon;
    Components::response(bnd, res, false).await?;
    Ok(())
}
